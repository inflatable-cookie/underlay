//! Shared HTTP client for Underlay crates.
//!
//! Provides a pre-configured [`reqwest::Client`] with sensible defaults
//! (rustls TLS, JSON support, user-agent, connect/total timeouts) so that
//! consumers don't each configure their own.
//!
//! Two profiles:
//! - [`HttpClient::new`] - internal/trusted targets. Timeouts, normal
//!   redirects.
//! - [`HttpClient::external`] - untrusted/user-influenced targets. Adds SSRF
//!   guards: private/loopback/link-local hosts are rejected and redirects are
//!   constrained and re-checked at every hop.
//!
//! # Example
//!
//! ```rust
//! use underlay_http_client::HttpClient;
//!
//! let client = HttpClient::new();
//! let inner: &reqwest::Client = client.inner();
//! ```

use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;

pub use reqwest;

/// Default timeout for establishing a connection.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Default overall request timeout (connect + transfer).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum redirect hops allowed for the external (SSRF-guarded) profile.
pub const EXTERNAL_MAX_REDIRECTS: usize = 3;

/// Error building or validating an HTTP client / URL.
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error(transparent)]
    Build(#[from] reqwest::Error),
    #[error("blocked SSRF target: {0}")]
    BlockedTarget(String),
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
}

/// A pre-configured HTTP client wrapping [`reqwest::Client`].
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    /// Create a new HTTP client with default settings (timeouts + user-agent).
    pub fn new() -> Self {
        Self::try_new().unwrap_or_else(|err| {
            tracing::warn!(%err, "configured HTTP client build failed; falling back to reqwest defaults (custom timeouts/user-agent lost)");
            Self {
                inner: reqwest::Client::new(),
            }
        })
    }

    /// Try to create a new HTTP client with default settings.
    pub fn try_new() -> Result<Self, reqwest::Error> {
        base_builder(concat!("underlay/", env!("CARGO_PKG_VERSION")).to_string())
            .build()
            .map(|inner| Self { inner })
    }

    /// Create a new HTTP client with a custom user-agent.
    pub fn with_user_agent(user_agent: impl Into<String>) -> Self {
        Self::try_with_user_agent(user_agent).unwrap_or_else(|err| {
            tracing::warn!(%err, "configured HTTP client build failed; falling back to reqwest defaults (custom timeouts/user-agent lost)");
            Self {
                inner: reqwest::Client::new(),
            }
        })
    }

    /// Try to create a new HTTP client with a custom user-agent.
    pub fn try_with_user_agent(user_agent: impl Into<String>) -> Result<Self, reqwest::Error> {
        base_builder(user_agent.into())
            .build()
            .map(|inner| Self { inner })
    }

    /// Build a client for **untrusted / user-influenced** outbound targets.
    ///
    /// Adds SSRF protection on top of the default timeouts: redirects are
    /// capped at [`EXTERNAL_MAX_REDIRECTS`] and every hop's host is
    /// re-checked, rejecting private, loopback, link-local, and unspecified
    /// addresses (including the cloud metadata endpoint `169.254.169.254`).
    ///
    /// Callers should still pass each request URL through
    /// [`validate_external_url`] before sending, since DNS for the initial
    /// host is resolved there; the redirect policy covers subsequent hops.
    pub fn external() -> Result<Self, HttpClientError> {
        let inner = base_builder(concat!("underlay/", env!("CARGO_PKG_VERSION")).to_string())
            .redirect(external_redirect_policy())
            .build()?;
        Ok(Self { inner })
    }

    /// Get a reference to the underlying [`reqwest::Client`].
    pub fn inner(&self) -> &reqwest::Client {
        &self.inner
    }

    /// Consume the wrapper and return the underlying [`reqwest::Client`].
    pub fn into_inner(self) -> reqwest::Client {
        self.inner
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for HttpClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn base_builder(user_agent: String) -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .user_agent(user_agent)
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .timeout(DEFAULT_TIMEOUT)
}

fn external_redirect_policy() -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(|attempt| {
        if attempt.previous().len() >= EXTERNAL_MAX_REDIRECTS {
            return attempt.error("too many redirects");
        }
        match host_is_blocked(attempt.url()) {
            Ok(true) => attempt.stop(),
            Ok(false) => attempt.follow(),
            // Unresolvable host on a redirect: refuse rather than follow.
            Err(_) => attempt.stop(),
        }
    })
}

/// Returns true if a URL's host resolves to (or is) a non-public address.
fn host_is_blocked(url: &reqwest::Url) -> Result<bool, HttpClientError> {
    let host = url
        .host_str()
        .ok_or_else(|| HttpClientError::InvalidUrl(url.to_string()))?;

    // IP literal: classify directly.
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(!is_public_ip(ip));
    }

    // Hostname: resolve every A/AAAA record and block if ANY is non-public
    // (defends against split-horizon / partial-rebind tricks).
    let port = url.port_or_known_default().unwrap_or(80);
    let addrs = (host, port)
        .to_socket_addrs()
        .map_err(|e| HttpClientError::InvalidUrl(format!("{host}: {e}")))?;

    let mut saw_any = false;
    for addr in addrs {
        saw_any = true;
        if !is_public_ip(addr.ip()) {
            return Ok(true);
        }
    }

    if !saw_any {
        return Err(HttpClientError::InvalidUrl(format!(
            "{host}: no addresses resolved"
        )));
    }

    Ok(false)
}

/// Validate a URL for the external profile before sending.
///
/// Rejects non-http(s) schemes and any host that resolves to a private,
/// loopback, link-local, or unspecified address.
pub fn validate_external_url(url: &str) -> Result<reqwest::Url, HttpClientError> {
    let parsed =
        reqwest::Url::parse(url).map_err(|e| HttpClientError::InvalidUrl(e.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => {}
        other => {
            return Err(HttpClientError::InvalidUrl(format!(
                "unsupported scheme: {other}"
            )))
        }
    }

    if host_is_blocked(&parsed)? {
        return Err(HttpClientError::BlockedTarget(
            parsed.host_str().unwrap_or(url).to_string(),
        ));
    }

    Ok(parsed)
}

/// Classify an IP address as publicly routable.
///
/// Blocks loopback, private (RFC 1918 / ULA), link-local (incl.
/// `169.254.0.0/16` metadata), unspecified, and other non-global ranges.
fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            !(v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
                // Carrier-grade NAT 100.64.0.0/10
                || (v4.octets()[0] == 100 && (v4.octets()[1] & 0xC0) == 64)
                // Benchmarking 198.18.0.0/15
                || (v4.octets()[0] == 198 && (v4.octets()[1] & 0xFE) == 18))
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() || v6.is_unspecified() {
                return false;
            }
            // Unique local fc00::/7
            let seg0 = v6.segments()[0];
            if (seg0 & 0xFE00) == 0xFC00 {
                return false;
            }
            // Link-local fe80::/10
            if (seg0 & 0xFFC0) == 0xFE80 {
                return false;
            }
            // IPv4-mapped: classify the embedded v4 address.
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_public_ip(IpAddr::V4(v4));
            }
            true
        }
    }
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
