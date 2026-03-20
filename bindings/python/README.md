# rdapify

A fast, secure RDAP client for Python — powered by Rust.

## Installation

```bash
pip install rdapify
```

## Usage

```python
import rdapify

# Query a domain
result = rdapify.domain("example.com")
print(result["registrar"]["name"])
print(result["expiration_date"])

# Query an IP address
ip = rdapify.ip("8.8.8.8")
print(ip["country"])

# Query an ASN
asn = rdapify.asn("AS15169")
print(asn["name"])

# Query a nameserver
ns = rdapify.nameserver("ns1.google.com")
print(ns["ip_addresses"])

# Query an entity
entity = rdapify.entity("ARIN-HN-1", "https://rdap.arin.net/registry")
print(entity["handle"])
```

## Features

- 5 query types: domain, IP, ASN, nameserver, entity
- IANA Bootstrap — automatic server discovery
- SSRF protection built-in
- In-memory cache
- IDN/Punycode support
- Zero OpenSSL dependency (rustls)

## License

MIT
