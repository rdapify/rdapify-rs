//! Node.js binding for rdapify — built with napi-rs.
//!
//! Exposes all 5 query types as async JavaScript functions.
//! Each function returns a plain JavaScript object (JSON-serialised).
//!
//! # Usage (JavaScript/TypeScript)
//! ```js
//! const { domain, ip, asn, nameserver, entity } = require('@rdapify/core');
//!
//! const result = await domain('example.com');
//! console.log(result.registrar?.name);
//!
//! const ipResult = await ip('8.8.8.8');
//! console.log(ipResult.country);
//!
//! const asnResult = await asn('AS15169');
//! console.log(asnResult.name);
//! ```

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use rdapify::RdapClient;

// ── Lazy client — one instance per process ────────────────────────────────────

fn get_client() -> napi::Result<RdapClient> {
    RdapClient::new().map_err(|e| napi::Error::from_reason(e.to_string()))
}

// ── Helper: serialise to napi Object ─────────────────────────────────────────

fn to_js_object<T: serde::Serialize>(env: Env, value: &T) -> napi::Result<Object> {
    let json = serde_json::to_string(value)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let js_string = env.create_string(&json)?;

    // Use JSON.parse on the V8 side for maximum compatibility.
    let global = env.get_global()?;
    let json_obj: Object = global.get_named_property("JSON")?;
    let parse_fn: Function<JsString, Object> = json_obj.get_named_property("parse")?;
    parse_fn.call(Some(&json_obj), &[js_string])
}

// ── Exported functions ────────────────────────────────────────────────────────

/// Query RDAP information for a domain name.
///
/// @param domain - Domain name (e.g. "example.com", Unicode IDNs supported)
/// @returns Normalised RDAP domain object
#[napi]
pub async fn domain(env: Env, domain_name: String) -> napi::Result<Object> {
    let client = get_client()?;

    let result = client
        .domain(&domain_name)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    to_js_object(env, &result)
}

/// Query RDAP information for an IP address (IPv4 or IPv6).
///
/// @param ip - IP address (e.g. "8.8.8.8", "2001:4860:4860::8888")
/// @returns Normalised RDAP IP network object
#[napi]
pub async fn ip(env: Env, ip_address: String) -> napi::Result<Object> {
    let client = get_client()?;

    let result = client
        .ip(&ip_address)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    to_js_object(env, &result)
}

/// Query RDAP information for an Autonomous System Number.
///
/// @param asn - ASN number or prefixed form (e.g. "15169", "AS15169")
/// @returns Normalised RDAP autnum object
#[napi]
pub async fn asn(env: Env, asn_value: String) -> napi::Result<Object> {
    let client = get_client()?;

    let result = client
        .asn(&asn_value)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    to_js_object(env, &result)
}

/// Query RDAP information for a nameserver hostname.
///
/// @param hostname - Nameserver hostname (e.g. "ns1.google.com")
/// @returns Normalised RDAP nameserver object
#[napi]
pub async fn nameserver(env: Env, hostname: String) -> napi::Result<Object> {
    let client = get_client()?;

    let result = client
        .nameserver(&hostname)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    to_js_object(env, &result)
}

/// Query RDAP information for an entity (contact / registrar).
///
/// Entities have no global bootstrap registry, so an explicit server URL is required.
///
/// @param handle     - Entity handle (e.g. "ARIN-HN-1")
/// @param server_url - RDAP server base URL (e.g. "https://rdap.arin.net/registry")
/// @returns Normalised RDAP entity object
#[napi]
pub async fn entity(env: Env, handle: String, server_url: String) -> napi::Result<Object> {
    let client = get_client()?;

    let result = client
        .entity(&handle, &server_url)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    to_js_object(env, &result)
}
