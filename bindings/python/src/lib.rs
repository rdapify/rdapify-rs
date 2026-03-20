//! Python binding for rdapify — built with PyO3.
//!
//! Exposes all 5 query types as synchronous Python functions
//! (using tokio::runtime::Runtime under the hood).
//!
//! # Usage (Python)
//! ```python
//! import rdapify
//!
//! result = rdapify.domain("example.com")
//! print(result["registrar"]["name"])
//!
//! ip_result = rdapify.ip("8.8.8.8")
//! print(ip_result["country"])
//!
//! asn_result = rdapify.asn("AS15169")
//! print(asn_result["name"])
//! ```

use pyo3::prelude::*;
use pyo3::types::PyDict;
use rdapify::RdapClient;

// ── Shared tokio runtime ──────────────────────────────────────────────────────

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("failed to create Tokio runtime")
}

fn client() -> PyResult<RdapClient> {
    RdapClient::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

// ── Helper: Rust value → Python dict ─────────────────────────────────────────

fn to_py_dict<T: serde::Serialize>(py: Python<'_>, value: &T) -> PyResult<Py<PyDict>> {
    let json = serde_json::to_string(value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Use Python's json.loads for maximum compatibility.
    let json_module = py.import("json")?;
    let dict: Py<PyDict> = json_module
        .call_method1("loads", (json,))?
        .downcast::<PyDict>()?
        .into();

    Ok(dict)
}

// ── Exported functions ────────────────────────────────────────────────────────

/// Query RDAP information for a domain name.
///
/// :param domain: Domain name (e.g. "example.com"). Unicode IDNs are supported.
/// :returns: Dictionary with normalised RDAP domain data.
/// :raises RuntimeError: On network failure or invalid input.
#[pyfunction]
fn domain(py: Python<'_>, domain_name: &str) -> PyResult<Py<PyDict>> {
    let c = client()?;
    let result = runtime()
        .block_on(c.domain(domain_name))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    to_py_dict(py, &result)
}

/// Query RDAP information for an IP address (IPv4 or IPv6).
///
/// :param ip: IP address string (e.g. "8.8.8.8").
/// :returns: Dictionary with normalised RDAP IP network data.
#[pyfunction]
fn ip(py: Python<'_>, ip_address: &str) -> PyResult<Py<PyDict>> {
    let c = client()?;
    let result = runtime()
        .block_on(c.ip(ip_address))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    to_py_dict(py, &result)
}

/// Query RDAP information for an Autonomous System Number.
///
/// :param asn: ASN as string: "15169" or "AS15169".
/// :returns: Dictionary with normalised RDAP autnum data.
#[pyfunction]
fn asn(py: Python<'_>, asn_value: &str) -> PyResult<Py<PyDict>> {
    let c = client()?;
    let result = runtime()
        .block_on(c.asn(asn_value))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    to_py_dict(py, &result)
}

/// Query RDAP information for a nameserver hostname.
///
/// :param hostname: Nameserver hostname (e.g. "ns1.google.com").
/// :returns: Dictionary with normalised RDAP nameserver data.
#[pyfunction]
fn nameserver(py: Python<'_>, hostname: &str) -> PyResult<Py<PyDict>> {
    let c = client()?;
    let result = runtime()
        .block_on(c.nameserver(hostname))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    to_py_dict(py, &result)
}

/// Query RDAP information for an entity (contact / registrar).
///
/// Entities have no global bootstrap registry; the server URL is required.
///
/// :param handle: Entity handle (e.g. "ARIN-HN-1").
/// :param server_url: RDAP server base URL (e.g. "https://rdap.arin.net/registry").
/// :returns: Dictionary with normalised RDAP entity data.
#[pyfunction]
fn entity(py: Python<'_>, handle: &str, server_url: &str) -> PyResult<Py<PyDict>> {
    let c = client()?;
    let result = runtime()
        .block_on(c.entity(handle, server_url))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    to_py_dict(py, &result)
}

// ── Module definition ─────────────────────────────────────────────────────────

/// rdapify — Unified RDAP client for Python, powered by Rust.
#[pymodule]
fn rdapify(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(domain, m)?)?;
    m.add_function(wrap_pyfunction!(ip, m)?)?;
    m.add_function(wrap_pyfunction!(asn, m)?)?;
    m.add_function(wrap_pyfunction!(nameserver, m)?)?;
    m.add_function(wrap_pyfunction!(entity, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
