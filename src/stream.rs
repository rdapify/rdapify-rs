//! Async streaming API for batch RDAP queries.
//!
//! The [`RdapClient::stream_domain`] and [`RdapClient::stream_ip`] methods
//! return [`tokio_stream::wrappers::ReceiverStream`] values that yield events
//! as results arrive.
//!
//! # Back-pressure
//! Each stream is backed by a bounded [`tokio::sync::mpsc`] channel.
//! [`StreamConfig::buffer_size`] controls the channel capacity — when the
//! buffer is full the producer will wait until the consumer has read at least
//! one item, providing natural back-pressure.
//!
//! # Cancellation
//! Dropping the stream before it is exhausted is safe: the background task
//! that performs the queries detects the closed channel and exits cleanly,
//! without leaking any resources.

use crate::error::RdapError;
use crate::types::{DomainResponse, IpResponse};

// ── Events ────────────────────────────────────────────────────────────────────

/// Emitted by [`RdapClient::stream_domain`] for each queried domain.
#[derive(Debug)]
pub enum DomainEvent {
    /// Successful RDAP response for the queried domain.
    Result(DomainResponse),
    /// The query for this domain failed.
    Error {
        /// The domain name that was queried.
        query: String,
        /// The error that occurred.
        error: RdapError,
    },
}

/// Emitted by [`RdapClient::stream_ip`] for each queried IP address.
#[derive(Debug)]
pub enum IpEvent {
    /// Successful RDAP response for the queried IP.
    Result(IpResponse),
    /// The query for this IP address failed.
    Error {
        /// The IP address that was queried.
        query: String,
        /// The error that occurred.
        error: RdapError,
    },
}

// ── Configuration ─────────────────────────────────────────────────────────────

/// Configuration for streaming queries.
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Channel buffer size (controls back-pressure).
    ///
    /// A larger buffer allows the producer to run further ahead of the
    /// consumer; a smaller buffer keeps memory usage lower at the cost of
    /// potential producer stalls.  @default 32
    pub buffer_size: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self { buffer_size: 32 }
    }
}
