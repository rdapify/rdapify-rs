//! Public RDAP response types.

pub mod asn;
pub mod availability;
pub mod common;
pub mod domain;
pub mod entity;
pub mod ip;
pub mod nameserver;

// Flatten commonly-used types to the top of the `types` module.
pub use asn::AsnResponse;
pub use availability::AvailabilityResult;
pub use common::{RdapEntity, RdapEvent, RdapLink, RdapRemark, RdapRole, RdapStatus, ResponseMeta};
pub use domain::{DomainResponse, RegistrarSummary};
pub use entity::EntityResponse;
pub use ip::{IpResponse, IpVersion};
pub use nameserver::{NameserverIpAddresses, NameserverResponse};
