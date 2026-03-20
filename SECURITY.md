# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | ✅        |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Send a report to **security@rdapify.com** with:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within **48 hours**, and a patch within **7 days** for confirmed vulnerabilities.

## Security Features

- **SSRF Protection** — all outbound URLs are validated against private/loopback/link-local ranges before any network request
- **Zero OpenSSL** — uses `rustls` (pure Rust TLS), eliminating a large class of C-level vulnerabilities
- **No unsafe code** — `#![forbid(unsafe_code)]` enforced at the crate level
- **Dependency auditing** — `cargo-audit` runs on every CI push via GitHub Actions
