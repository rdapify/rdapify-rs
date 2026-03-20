# rdapify

A fast, secure, production-ready [RDAP](https://rdap.org) client library for Rust.

RDAP (Registration Data Access Protocol) is the modern replacement for WHOIS, defined in [RFC 9083](https://www.rfc-editor.org/rfc/rfc9083) and [RFC 9224](https://www.rfc-editor.org/rfc/rfc9224).

[![Crates.io](https://img.shields.io/crates/v/rdapify)](https://crates.io/crates/rdapify)
[![docs.rs](https://img.shields.io/docsrs/rdapify)](https://docs.rs/rdapify)
[![CI](https://github.com/rdapify/rdapify/actions/workflows/ci.yml/badge.svg)](https://github.com/rdapify/rdapify/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **5 query types** — domain, IP, ASN, nameserver, entity
- **IANA Bootstrap** (RFC 9224) — automatic server discovery, no manual configuration needed
- **SSRF protection** — blocks requests to private, loopback, and link-local addresses
- **In-memory cache** — configurable TTL and capacity, lock-free via `DashMap`
- **IDN support** — accepts Unicode domain names, normalises to Punycode automatically
- **Retry with back-off** — exponential back-off on network errors and 5xx/429 responses
- **Zero OpenSSL** — uses `rustls` (pure Rust TLS)
- **Async-first** — built on `tokio`

## Installation

```toml
[dependencies]
rdapify = "0.1"
```

## Quick Start

```rust
use rdapify::RdapClient;

#[tokio::main]
async fn main() -> rdapify::Result<()> {
    let client = RdapClient::new();

    // Query a domain
    let domain = client.domain("example.com").await?;
    println!("Registrar: {:?}", domain.registrar);
    println!("Expires:   {:?}", domain.expiration_date());

    // Query an IP address
    let ip = client.ip("8.8.8.8").await?;
    println!("Network: {:?}", ip.name);
    println!("Country: {:?}", ip.country);

    // Query an ASN
    let asn = client.asn("AS15169").await?;
    println!("ASN name: {:?}", asn.name);

    Ok(())
}
```

## Usage

### Domain Query

```rust
let res = client.domain("rust-lang.org").await?;

println!("{}", res.ldh_name.as_deref().unwrap_or("-"));
println!("{:?}", res.status);
println!("{:?}", res.expiration_date());

if let Some(r) = &res.registrar {
    println!("Registrar: {}", r.name.as_deref().unwrap_or("-"));
}
```

### IP Address Query

```rust
// IPv4
let res = client.ip("1.1.1.1").await?;

// IPv6
let res = client.ip("2606:4700::1111").await?;

println!("CIDR:    {:?}", res.cidr);
println!("Country: {:?}", res.country);
```

### ASN Query

```rust
// Both formats accepted
let res = client.asn("15169").await?;
let res = client.asn("AS15169").await?;

println!("Name: {:?}", res.name);
```

### Nameserver Query

```rust
let res = client.nameserver("ns1.example.com").await?;
println!("IPs: {:?}", res.ip_addresses);
```

### Entity Query

```rust
let res = client.entity("ARIN-CHA-1", "https://rdap.arin.net/registry").await?;
println!("Name:  {:?}", res.name);
println!("Roles: {:?}", res.roles);
```

## Configuration

```rust
use rdapify::{RdapClient, ClientConfig, FetcherConfig, SsrfConfig};
use std::time::Duration;

let client = RdapClient::with_config(ClientConfig {
    cache: true,
    fetcher: FetcherConfig {
        timeout: Duration::from_secs(10),
        max_attempts: 3,
        ..Default::default()
    },
    ssrf: SsrfConfig {
        enabled: true,
        ..Default::default()
    },
    ..Default::default()
})?;
```

## CLI

Enable the `cli` feature to build the `rdapify` binary:

```toml
rdapify = { version = "0.1", features = ["cli"] }
```

Or install it directly:

```bash
cargo install rdapify --features cli
```

```bash
rdapify domain example.com
rdapify ip 8.8.8.8
rdapify asn AS15169
rdapify nameserver ns1.example.com
rdapify entity ARIN-CHA-1 https://rdap.arin.net/registry

# Machine-readable JSON output
rdapify domain example.com --raw
```

## MSRV

Minimum supported Rust version: **1.75**

## License

MIT — see [LICENSE](LICENSE)
