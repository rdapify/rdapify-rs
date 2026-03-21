//! Node.js binding for rdapify — built with napi-rs.
//!
//! Exposes all 5 query types as async JavaScript functions.
//! Each function returns a plain JavaScript object (via serde-json).
//!
//! # Usage (JavaScript/TypeScript)
//! ```js
//! const { domain, ip, asn, nameserver, entity } = require('rdapify-nd');
//!
//! const result = await domain('example.com');
//! console.log(result.registrar?.name);
//!
//! const ipResult = await ip('8.8.8.8');
//! console.log(ipResult.country);
//! ```

#![deny(clippy::all)]

use napi_derive::napi;
use rdapify::RdapClient;

fn get_client() -> napi::Result<RdapClient> {
    RdapClient::new().map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Query RDAP information for a domain name.
///
/// @param domainName - Domain name (e.g. "example.com", Unicode IDNs supported)
/// @returns Normalised RDAP domain object
#[napi]
pub async fn domain(domain_name: String) -> napi::Result<serde_json::Value> {
    let client = get_client()?;
    let result = client
        .domain(&domain_name)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_value(result).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Query RDAP information for an IP address (IPv4 or IPv6).
///
/// @param ipAddress - IP address (e.g. "8.8.8.8", "2001:4860:4860::8888")
/// @returns Normalised RDAP IP network object
#[napi]
pub async fn ip(ip_address: String) -> napi::Result<serde_json::Value> {
    let client = get_client()?;
    let result = client
        .ip(&ip_address)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_value(result).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Query RDAP information for an Autonomous System Number.
///
/// @param asnValue - ASN number or prefixed form (e.g. "15169", "AS15169")
/// @returns Normalised RDAP autnum object
#[napi]
pub async fn asn(asn_value: String) -> napi::Result<serde_json::Value> {
    let client = get_client()?;
    let result = client
        .asn(&asn_value)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_value(result).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Query RDAP information for a nameserver hostname.
///
/// @param hostname - Nameserver hostname (e.g. "ns1.google.com")
/// @returns Normalised RDAP nameserver object
#[napi]
pub async fn nameserver(hostname: String) -> napi::Result<serde_json::Value> {
    let client = get_client()?;
    let result = client
        .nameserver(&hostname)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_value(result).map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// Query RDAP information for an entity (contact / registrar).
///
/// Entities have no global bootstrap registry — an explicit server URL is required.
///
/// @param handle    - Entity handle (e.g. "ARIN-HN-1")
/// @param serverUrl - RDAP server base URL (e.g. "https://rdap.arin.net/registry")
/// @returns Normalised RDAP entity object
#[napi]
pub async fn entity(handle: String, server_url: String) -> napi::Result<serde_json::Value> {
    let client = get_client()?;
    let result = client
        .entity(&handle, &server_url)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_json::to_value(result).map_err(|e| napi::Error::from_reason(e.to_string()))
}
