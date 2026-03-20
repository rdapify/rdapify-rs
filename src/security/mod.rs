//! Security primitives: SSRF protection.

pub mod ssrf;

pub use ssrf::{SsrfConfig, SsrfGuard};
