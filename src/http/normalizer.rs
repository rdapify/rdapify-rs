//! Response normaliser — converts raw RDAP JSON into typed response structs.
//!
//! The normaliser extracts the minimum necessary fields from the raw response
//! and returns a typed, consistent struct regardless of which RDAP server
//! produced the response.

use chrono::Utc;
use serde_json::Value;

use crate::error::{RdapError, Result};
use crate::types::{
    asn::AsnResponse,
    common::{RdapEntity, RdapEvent, RdapLink, RdapRemark, RdapRole, RdapStatus, ResponseMeta},
    domain::{DomainResponse, RegistrarSummary},
    entity::EntityResponse,
    ip::{IpResponse, IpVersion},
    nameserver::{NameserverIpAddresses, NameserverResponse},
};

/// Normalises raw RDAP responses into typed structs.
#[derive(Debug, Clone, Default)]
pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    // ── Public normalisation methods ──────────────────────────────────────────

    pub fn domain(
        &self,
        query: &str,
        raw: Value,
        source: &str,
        cached: bool,
    ) -> Result<DomainResponse> {
        let meta = make_meta(source, cached);
        let obj = require_object(&raw)?;

        let entities = parse_entities(obj.get("entities"));
        let events = parse_events(obj.get("events"));

        let nameservers = obj
            .get("nameservers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|ns| {
                        ns.get("ldhName")
                            .or_else(|| ns.get("unicodeName"))
                            .and_then(|v| v.as_str())
                            .map(str::to_lowercase)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let registrar = extract_registrar(&entities);

        Ok(DomainResponse {
            query: query.to_string(),
            ldh_name: string_field(obj, "ldhName"),
            unicode_name: string_field(obj, "unicodeName"),
            handle: string_field(obj, "handle"),
            status: parse_status(obj.get("status")),
            nameservers,
            registrar,
            entities,
            events,
            links: parse_links(obj.get("links")),
            remarks: parse_remarks(obj.get("remarks")),
            meta,
        })
    }

    pub fn ip(&self, query: &str, raw: Value, source: &str, cached: bool) -> Result<IpResponse> {
        let meta = make_meta(source, cached);
        let obj = require_object(&raw)?;

        let ip_version = obj
            .get("ipVersion")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "v4" => IpVersion::V4,
                _ => IpVersion::V6,
            });

        Ok(IpResponse {
            query: query.to_string(),
            handle: string_field(obj, "handle"),
            start_address: string_field(obj, "startAddress"),
            end_address: string_field(obj, "endAddress"),
            ip_version,
            name: string_field(obj, "name"),
            allocation_type: string_field(obj, "type"),
            country: string_field(obj, "country"),
            parent_handle: string_field(obj, "parentHandle"),
            status: parse_status(obj.get("status")),
            entities: parse_entities(obj.get("entities")),
            events: parse_events(obj.get("events")),
            links: parse_links(obj.get("links")),
            remarks: parse_remarks(obj.get("remarks")),
            meta,
        })
    }

    pub fn asn(&self, query: u32, raw: Value, source: &str, cached: bool) -> Result<AsnResponse> {
        let meta = make_meta(source, cached);
        let obj = require_object(&raw)?;

        Ok(AsnResponse {
            query,
            handle: string_field(obj, "handle"),
            start_autnum: obj
                .get("startAutnum")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32),
            end_autnum: obj
                .get("endAutnum")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32),
            name: string_field(obj, "name"),
            autnum_type: string_field(obj, "type"),
            country: string_field(obj, "country"),
            status: parse_status(obj.get("status")),
            entities: parse_entities(obj.get("entities")),
            events: parse_events(obj.get("events")),
            links: parse_links(obj.get("links")),
            remarks: parse_remarks(obj.get("remarks")),
            meta,
        })
    }

    pub fn nameserver(
        &self,
        query: &str,
        raw: Value,
        source: &str,
        cached: bool,
    ) -> Result<NameserverResponse> {
        let meta = make_meta(source, cached);
        let obj = require_object(&raw)?;

        let ip_addresses = {
            let ip_obj = obj.get("ipAddresses").and_then(|v| v.as_object());
            NameserverIpAddresses {
                v4: ip_obj
                    .and_then(|o| o.get("v4"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
                v6: ip_obj
                    .and_then(|o| o.get("v6"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        };

        Ok(NameserverResponse {
            query: query.to_string(),
            handle: string_field(obj, "handle"),
            ldh_name: string_field(obj, "ldhName"),
            unicode_name: string_field(obj, "unicodeName"),
            ip_addresses,
            status: parse_status(obj.get("status")),
            entities: parse_entities(obj.get("entities")),
            events: parse_events(obj.get("events")),
            links: parse_links(obj.get("links")),
            remarks: parse_remarks(obj.get("remarks")),
            meta,
        })
    }

    pub fn entity(
        &self,
        query: &str,
        raw: Value,
        source: &str,
        cached: bool,
    ) -> Result<EntityResponse> {
        let meta = make_meta(source, cached);
        let obj = require_object(&raw)?;

        let roles = obj
            .get("roles")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| serde_json::from_value::<RdapRole>(v.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(EntityResponse {
            query: query.to_string(),
            handle: string_field(obj, "handle"),
            vcard_array: obj.get("vcardArray").cloned(),
            roles,
            status: parse_status(obj.get("status")),
            entities: parse_entities(obj.get("entities")),
            events: parse_events(obj.get("events")),
            links: parse_links(obj.get("links")),
            remarks: parse_remarks(obj.get("remarks")),
            meta,
        })
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn make_meta(source: &str, cached: bool) -> ResponseMeta {
    ResponseMeta {
        source: source.to_string(),
        queried_at: Utc::now().to_rfc3339(),
        cached,
    }
}

fn require_object(value: &Value) -> Result<&serde_json::Map<String, Value>> {
    value.as_object().ok_or_else(|| RdapError::ParseError {
        reason: "Expected a JSON object at the response root".to_string(),
    })
}

fn string_field(obj: &serde_json::Map<String, Value>, key: &str) -> Option<String> {
    obj.get(key).and_then(|v| v.as_str()).map(str::to_string)
}

fn parse_status(value: Option<&Value>) -> Vec<RdapStatus> {
    value
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value::<RdapStatus>(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn parse_events(value: Option<&Value>) -> Vec<RdapEvent> {
    value
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value::<RdapEvent>(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn parse_links(value: Option<&Value>) -> Vec<RdapLink> {
    value
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value::<RdapLink>(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn parse_remarks(value: Option<&Value>) -> Vec<RdapRemark> {
    value
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value::<RdapRemark>(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn parse_entities(value: Option<&Value>) -> Vec<RdapEntity> {
    value
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value::<RdapEntity>(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// Extracts a condensed `RegistrarSummary` from the entities list.
fn extract_registrar(entities: &[RdapEntity]) -> Option<RegistrarSummary> {
    let registrar_entity = entities
        .iter()
        .find(|e| e.roles.iter().any(|r| matches!(r, RdapRole::Registrar)))?;

    // Try to get the name from vCard
    let name = registrar_entity
        .vcard_array
        .as_ref()
        .and_then(extract_vcard_name);

    // Try to get the URL from links
    let url = registrar_entity
        .links
        .iter()
        .find(|l| l.rel.as_deref() == Some("self"))
        .map(|l| l.href.clone());

    Some(RegistrarSummary {
        name,
        handle: registrar_entity.handle.clone(),
        url,
        abuse_email: None,
        abuse_phone: None,
    })
}

/// Tries to extract the `fn` (full name) property from a vCard array.
fn extract_vcard_name(vcard: &Value) -> Option<String> {
    let outer = vcard.as_array()?;
    // vCard format: ["vcard", [[property, {}, type, value], ...]]
    let props = outer.get(1)?.as_array()?;

    for prop in props {
        let arr = prop.as_array()?;
        if arr.first()?.as_str()? == "fn" {
            return arr.get(3).and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}
