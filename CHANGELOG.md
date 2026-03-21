# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] — 2026-03-21

### Changed

- **Rename**: Node.js binding renamed from `@rdapify/core` → `rdapify-nd` on npm
- **Rename**: Python binding renamed from `rdapify` → `rdapify-py` on PyPI; Python import name changed from `rdapify` → `rdapify_py`
- **Performance**: `rdapify-nd` napi binding now uses a module-level `OnceLock<RdapClient>` singleton — eliminates per-call client construction overhead

### Fixed

- **CI**: fixed duplicate `aarch64-apple-darwin` target in `bindings.yml` napi build matrix (was also listed in `napi.triples.defaults`)

### Documentation

- Added full usage examples for `rdapify-nd` (Node.js) and `rdapify-py` (Python) in README

## [0.1.1] — 2026-03-21

### Fixed

- **Security**: upgraded `idna` to resolve GHSA advisory for invalid domain label processing
- **Security**: upgraded `rustls-webpki` to resolve GHSA advisory for CPU exhaustion via crafted certificate chains
- **CI**: fixed MSRV job to allow transient network failures gracefully (`CARGO_NET_RETRY=10`)
- **CI**: fixed live-test workflow (added `#[ignore]` to integration tests that hit live servers)
- **CI**: added `cargo fetch` step to improve reliability on slow/flaky runners

### Changed

- Bindings CI/CD workflow now publishes `rdapify-nd` (npm) and `rdapify-py` (PyPI) automatically on version tags

## [0.1.0] — 2026-03-20

### Added

- **5 query types** via `RdapClient`: `domain()`, `ip()`, `asn()`, `nameserver()`, `entity()`
- **IANA Bootstrap** (RFC 9224) for automatic RDAP server discovery — DNS, IPv4, IPv6, ASN
- **SSRF protection** — blocks requests to loopback, private, link-local, and broadcast addresses for both IPv4 and IPv6; uses typed `url::Host` enum to avoid re-parsing
- **In-memory cache** backed by `DashMap` — configurable TTL (default 5 min) and max entries (default 1 000); lazy expiry on read, eager eviction at capacity
- **IDN / Punycode normalisation** via `idna` crate (RFC 5891) — accepts Unicode domain names transparently
- **Exponential back-off retry** — configurable max attempts, initial delay, and max delay; retries on network errors and 429/5xx HTTP status codes
- **Typed response structs** with serde: `DomainResponse`, `IpResponse`, `AsnResponse`, `NameserverResponse`, `EntityResponse`; common types `RdapStatus`, `RdapRole`, `RdapEvent`, `RdapLink`, `RdapRemark`, `RdapEntity`
- **`RegistrarSummary`** extracted automatically from domain entity list (name, handle, URL, abuse contact)
- **`ResponseMeta`** on every response: source URL, queried-at timestamp, cached flag
- **CLI binary** (`rdapify`) with subcommands `domain`, `ip`, `asn`, `nameserver`, `entity`; `--raw` flag for machine-readable JSON output; enabled via `cli` feature flag
- **Node.js binding** (`rdapify-nd`) via `napi-rs` — 5 async JS functions, full TypeScript type definitions, multi-platform prebuilt binary support
- **Python binding** (`rdapify-py`) via `PyO3` + `maturin` — 5 synchronous Python functions backed by a `tokio` runtime; abi3-py38 wheel for broad Python compatibility
- **43 integration tests** using `mockito` HTTP mock server — happy paths for all 5 query types, 404 / no-server error paths, IDN normalisation, SSRF blocking, cache deduplication
- **GitHub Actions CI** — multi-platform matrix (Ubuntu, macOS, Windows) + MSRV 1.75 job; lint (`rustfmt` + `clippy -D warnings`); security audit (`cargo-audit`); coverage (`cargo-tarpaulin` → Codecov)
- **Automated release workflow** — triggered on `v*.*.*` tags; verifies tag matches `Cargo.toml` version; publishes to crates.io; creates GitHub Release with CHANGELOG entry
- **Daily live-test workflow** — runs against real RDAP servers at 06:00 UTC; opens a GitHub Issue on failure

[Unreleased]: https://github.com/rdapify/rdapify-rs/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/rdapify/rdapify-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/rdapify/rdapify-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/rdapify/rdapify-rs/releases/tag/v0.1.0
