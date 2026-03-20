//! Python binding for rdapify — built with PyO3.
//!
//! Exposes all 5 query types as synchronous Python functions
//! (using tokio::runtime::Runtime under the hood).

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use rdapify::RdapClient;

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().expect("failed to create Tokio runtime")
}

fn client() -> PyResult<RdapClient> {
    RdapClient::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

fn to_py_dict<T: serde::Serialize>(py: Python<'_>, value: &T) -> PyResult<Py<PyDict>> {
    let json = serde_json::to_string(value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    let json_module = PyModule::import_bound(py, "json")?;
    let result = json_module.call_method1("loads", (json,))?;
    result.extract::<Py<PyDict>>()
}

/// Query RDAP information for a domain name.
///
/// :param domain_name: Domain name (e.g. "example.com"). Unicode IDNs supported.
/// :returns: Dictionary with normalised RDAP domain data.
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
/// :param ip_address: IP address string (e.g. "8.8.8.8").
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
/// :param asn_value: ASN as string: "15169" or "AS15169".
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

/// rdapify — Unified RDAP client for Python, powered by Rust.
#[pymodule]
#[pyo3(name = "rdapify")]
fn rdapify_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(domain, m)?)?;
    m.add_function(wrap_pyfunction!(ip, m)?)?;
    m.add_function(wrap_pyfunction!(asn, m)?)?;
    m.add_function(wrap_pyfunction!(nameserver, m)?)?;
    m.add_function(wrap_pyfunction!(entity, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
