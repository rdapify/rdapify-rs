//! HTTP fetcher — issues RDAP requests and returns raw JSON values.
//!
//! All URLs are validated by the [`SsrfGuard`] before the request is sent.
//! Retry logic with exponential back-off is built in.

use std::time::Duration;

use serde_json::Value;

use crate::error::{RdapError, Result};
use crate::security::SsrfGuard;

/// Configuration for the HTTP fetcher.
#[derive(Debug, Clone)]
pub struct FetcherConfig {
    /// Per-request timeout.
    pub timeout: Duration,
    /// `User-Agent` header value.
    pub user_agent: String,
    /// Maximum number of retry attempts (1 = no retries).
    pub max_attempts: u32,
    /// Initial back-off delay before the first retry.
    pub initial_backoff: Duration,
    /// Maximum back-off delay cap.
    pub max_backoff: Duration,
}

impl Default for FetcherConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            user_agent: format!(
                "rdapify/{} (https://rdapify.com)",
                env!("CARGO_PKG_VERSION")
            ),
            max_attempts: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(8),
        }
    }
}

/// HTTP fetcher with SSRF protection and retry logic.
#[derive(Debug, Clone)]
pub struct Fetcher {
    client: reqwest::Client,
    ssrf: SsrfGuard,
    config: FetcherConfig,
}

impl Fetcher {
    /// Creates a fetcher using the default configuration.
    pub fn new(ssrf: SsrfGuard) -> Result<Self> {
        Self::with_config(ssrf, FetcherConfig::default())
    }

    /// Creates a fetcher with a custom configuration.
    pub fn with_config(ssrf: SsrfGuard, config: FetcherConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            // Prefer rustls (pure Rust TLS) — no OpenSSL required.
            .use_rustls_tls()
            // Automatically handle gzip/deflate responses.
            .gzip(true)
            .build()
            .map_err(RdapError::Network)?;

        Ok(Self {
            client,
            ssrf,
            config,
        })
    }

    /// Fetches and deserialises a JSON response from `url`.
    ///
    /// Validates the URL with the SSRF guard before sending, and retries on
    /// transient network errors using exponential back-off.
    pub async fn fetch(&self, url: &str) -> Result<Value> {
        // Always validate before any network call.
        self.ssrf.validate(url)?;

        let mut attempt = 0u32;
        loop {
            attempt += 1;
            match self.do_fetch(url).await {
                Ok(value) => return Ok(value),
                Err(err) if attempt < self.config.max_attempts && is_retryable(&err) => {
                    let delay = backoff(
                        attempt,
                        self.config.initial_backoff,
                        self.config.max_backoff,
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(err) => return Err(err),
            }
        }
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    async fn do_fetch(&self, url: &str) -> Result<Value> {
        let response = self
            .client
            .get(url)
            .header("Accept", "application/rdap+json, application/json")
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    RdapError::Timeout {
                        millis: self.config.timeout.as_millis() as u64,
                        url: url.to_string(),
                    }
                } else {
                    RdapError::Network(e)
                }
            })?;

        let status = response.status();

        if !status.is_success() {
            return Err(RdapError::HttpStatus {
                status: status.as_u16(),
                url: url.to_string(),
            });
        }

        response
            .json::<Value>()
            .await
            .map_err(|e| RdapError::ParseError {
                reason: e.to_string(),
            })
    }

    /// Exposes the inner `reqwest::Client` so `Bootstrap` can reuse it.
    pub fn reqwest_client(&self) -> reqwest::Client {
        self.client.clone()
    }
}

// ── Retry utilities ───────────────────────────────────────────────────────────

/// Returns `true` for errors that are safe to retry.
fn is_retryable(err: &RdapError) -> bool {
    match err {
        RdapError::Network(_) | RdapError::Timeout { .. } => true,
        RdapError::HttpStatus { status, .. } => {
            matches!(status, 429 | 500 | 502 | 503 | 504)
        }
        _ => false,
    }
}

/// Exponential back-off: `min(initial * 2^(attempt-1), max)`.
fn backoff(attempt: u32, initial: Duration, max: Duration) -> Duration {
    let millis = initial.as_millis() as u64 * 2u64.saturating_pow(attempt - 1);
    Duration::from_millis(millis).min(max)
}

#[cfg(test)]
mod tests {
    use super::backoff;
    use std::time::Duration;

    #[test]
    fn backoff_grows_exponentially() {
        let base = Duration::from_millis(500);
        let cap = Duration::from_secs(8);
        assert_eq!(backoff(1, base, cap), Duration::from_millis(500));
        assert_eq!(backoff(2, base, cap), Duration::from_millis(1000));
        assert_eq!(backoff(3, base, cap), Duration::from_millis(2000));
        assert_eq!(backoff(4, base, cap), Duration::from_millis(4000));
        assert_eq!(backoff(5, base, cap), Duration::from_millis(8000));
        // Capped at max
        assert_eq!(backoff(6, base, cap), Duration::from_secs(8));
    }
}
