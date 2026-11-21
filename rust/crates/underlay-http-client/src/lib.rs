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

use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::Arc;
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
    ///
    /// # Panics
    ///
    /// Panics if reqwest cannot build even the timeout-only fallback client.
    /// Returning an unbounded client is not permitted.
    pub fn new() -> Self {
        Self::try_new().unwrap_or_else(bounded_fallback)
    }

    /// Try to create a new HTTP client with default settings.
    pub fn try_new() -> Result<Self, reqwest::Error> {
        base_builder(concat!("underlay/", env!("CARGO_PKG_VERSION")).to_string())
            .build()
            .map(|inner| Self { inner })
    }

    /// Create a new HTTP client with a custom user-agent.
    ///
    /// # Panics
    ///
    /// Panics if reqwest cannot build even the timeout-only fallback client.
    /// Returning an unbounded client is not permitted.
    pub fn with_user_agent(user_agent: impl Into<String>) -> Self {
        Self::try_with_user_agent(user_agent).unwrap_or_else(bounded_fallback)
    }

    /// Try to create a new HTTP client with a custom user-agent.
    pub fn try_with_user_agent(user_agent: impl Into<String>) -> Result<Self, reqwest::Error> {
        base_builder(user_agent.into())
            .build()
            .map(|inner| Self { inner })
    }

    /// Build a client for **untrusted / user-influenced** outbound targets.
    ///
    /// Adds SSRF protection on top of the default timeouts:
    /// - Disables system/environment proxies (`.no_proxy()`) so `HTTP_PROXY` /
    ///   `HTTPS_PROXY` cannot bypass [`ExternalDnsResolver`].
    /// - Installs a custom async DNS resolver that validates every resolved IP
    ///   at connect time, rejecting private, loopback, link-local, and
    ///   unspecified addresses (closing the DNS-rebinding TOCTOU window).
    /// - Caps redirects at [`EXTERNAL_MAX_REDIRECTS`] and rejects any hop that
    ///   targets a non-http(s) scheme or non-public IP literal with an error.
    ///
    /// Callers should still pass each request URL through
    /// [`validate_external_url_async`] (or [`validate_external_url`] in sync
    /// code) before sending so IP-literal targets and bad schemes fail before
    /// any network I/O.
    pub fn external() -> Result<Self, HttpClientError> {
        let inner = base_builder(concat!("underlay/", env!("CARGO_PKG_VERSION")).to_string())
            .no_proxy()
            .redirect(external_redirect_policy())
            .dns_resolver(Arc::new(ExternalDnsResolver))
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
    bounded_builder().user_agent(user_agent)
}

fn bounded_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .timeout(DEFAULT_TIMEOUT)
}

fn bounded_fallback(build_error: reqwest::Error) -> HttpClient {
    tracing::warn!(
        %build_error,
        "configured HTTP client build failed; using timeout-bounded fallback without a user-agent"
    );

    let inner = bounded_builder().build().unwrap_or_else(|fallback_error| {
        panic!(
            "bounded HTTP client fallback failed after configured build error ({build_error}): {fallback_error}"
        )
    });

    HttpClient { inner }
}

/// Async DNS resolver for [`HttpClient::external`] that validates all resolved
/// addresses at connect time, avoiding blocking `ToSocketAddrs` calls on Tokio
/// worker threads and closing the DNS-rebinding TOCTOU window.
struct ExternalDnsResolver;

impl reqwest::dns::Resolve for ExternalDnsResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> reqwest::dns::Resolving {
        Box::pin(async move {
            let host = name.as_str();
            let addrs: Vec<SocketAddr> = tokio::net::lookup_host((host, 0))
                .await
                .map_err(|err| -> Box<dyn std::error::Error + Send + Sync> {
                    Box::new(HttpClientError::InvalidUrl(format!("{host}: {err}")))
                })?
                .collect();

            if addrs.is_empty() {
                return Err(Box::new(HttpClientError::InvalidUrl(format!(
                    "{host}: no addresses resolved"
                )))
                    as Box<dyn std::error::Error + Send + Sync>);
            }

            if addrs.iter().any(|addr| !is_public_ip(addr.ip())) {
                return Err(Box::new(HttpClientError::BlockedTarget(host.to_string()))
                    as Box<dyn std::error::Error + Send + Sync>);
            }

            let addrs: reqwest::dns::Addrs = Box::new(addrs.into_iter());
            Ok(addrs)
        })
    }
}

fn validate_redirect_target(url: &reqwest::Url) -> Result<(), HttpClientError> {
    match url.scheme() {
        "http" | "https" => {}
        other => {
            return Err(HttpClientError::InvalidUrl(format!(
                "unsupported scheme: {other}"
            )));
        }
    }

    let host = url
        .host()
        .ok_or_else(|| HttpClientError::InvalidUrl(url.to_string()))?;

    match host {
        url::Host::Ipv4(v4) if !is_public_ip(IpAddr::V4(v4)) => {
            Err(HttpClientError::BlockedTarget(v4.to_string()))
        }
        url::Host::Ipv6(v6) if !is_public_ip(IpAddr::V6(v6)) => {
            Err(HttpClientError::BlockedTarget(v6.to_string()))
        }
        // Public IP literals are allowed; domain names are validated
        // asynchronously at connect time by `ExternalDnsResolver`.
        url::Host::Ipv4(_) | url::Host::Ipv6(_) | url::Host::Domain(_) => Ok(()),
    }
}

fn external_redirect_policy() -> reqwest::redirect::Policy {
    reqwest::redirect::Policy::custom(|attempt| {
        if attempt.previous().len() >= EXTERNAL_MAX_REDIRECTS {
            return attempt.error("too many redirects");
        }
        match validate_redirect_target(attempt.url()) {
            Ok(()) => attempt.follow(),
            Err(err) => attempt.error(err),
        }
    })
}

/// Returns true if a URL's host resolves to (or is) a non-public address.
fn host_is_blocked(url: &reqwest::Url) -> Result<bool, HttpClientError> {
    let host = url
        .host()
        .ok_or_else(|| HttpClientError::InvalidUrl(url.to_string()))?;

    match host {
        url::Host::Ipv4(v4) => Ok(!is_public_ip(IpAddr::V4(v4))),
        url::Host::Ipv6(v6) => Ok(!is_public_ip(IpAddr::V6(v6))),
        url::Host::Domain(domain) => {
            // Hostname: resolve every A/AAAA record and block if ANY is non-public
            // (defends against split-horizon / partial-rebind tricks).
            let port = url.port_or_known_default().unwrap_or(80);
            let addrs = (domain, port)
                .to_socket_addrs()
                .map_err(|e| HttpClientError::InvalidUrl(format!("{domain}: {e}")))?;

            let mut saw_any = false;
            for addr in addrs {
                saw_any = true;
                if !is_public_ip(addr.ip()) {
                    return Ok(true);
                }
            }

            if !saw_any {
                return Err(HttpClientError::InvalidUrl(format!(
                    "{domain}: no addresses resolved"
                )));
            }

            Ok(false)
        }
    }
}

/// Parse a URL and ensure it uses an `http` or `https` scheme.
fn parse_external_url(url: &str) -> Result<reqwest::Url, HttpClientError> {
    let parsed =
        reqwest::Url::parse(url).map_err(|e| HttpClientError::InvalidUrl(e.to_string()))?;

    match parsed.scheme() {
        "http" | "https" => Ok(parsed),
        other => Err(HttpClientError::InvalidUrl(format!(
            "unsupported scheme: {other}"
        ))),
    }
}

/// Async version of [`host_is_blocked`] using [`tokio::net::lookup_host`].
async fn host_is_blocked_async(url: &reqwest::Url) -> Result<bool, HttpClientError> {
    let host = url
        .host()
        .ok_or_else(|| HttpClientError::InvalidUrl(url.to_string()))?;

    match host {
        url::Host::Ipv4(v4) => Ok(!is_public_ip(IpAddr::V4(v4))),
        url::Host::Ipv6(v6) => Ok(!is_public_ip(IpAddr::V6(v6))),
        url::Host::Domain(domain) => {
            let port = url.port_or_known_default().unwrap_or(80);
            let addrs = tokio::net::lookup_host((domain, port))
                .await
                .map_err(|e| HttpClientError::InvalidUrl(format!("{domain}: {e}")))?;

            let mut saw_any = false;
            for addr in addrs {
                saw_any = true;
                if !is_public_ip(addr.ip()) {
                    return Ok(true);
                }
            }

            if !saw_any {
                return Err(HttpClientError::InvalidUrl(format!(
                    "{domain}: no addresses resolved"
                )));
            }

            Ok(false)
        }
    }
}

/// Validate a URL for the external profile before sending (blocking).
///
/// Rejects non-http(s) schemes and any host that resolves to a private,
/// loopback, link-local, or unspecified address.
///
/// **Blocking:** resolves domain names synchronously via [`ToSocketAddrs`],
/// which blocks the calling thread. Async handlers should use
/// [`validate_external_url_async`] to avoid blocking Tokio worker threads.
pub fn validate_external_url(url: &str) -> Result<reqwest::Url, HttpClientError> {
    let parsed = parse_external_url(url)?;

    if host_is_blocked(&parsed)? {
        return Err(HttpClientError::BlockedTarget(
            parsed.host_str().unwrap_or(url).to_string(),
        ));
    }

    Ok(parsed)
}

/// Validate a URL for the external profile before sending without blocking
/// Tokio worker threads.
///
/// Rejects non-http(s) schemes and any host that resolves to a private,
/// loopback, link-local, or unspecified address using
/// [`tokio::net::lookup_host`].
pub async fn validate_external_url_async(url: &str) -> Result<reqwest::Url, HttpClientError> {
    let parsed = parse_external_url(url)?;

    if host_is_blocked_async(&parsed).await? {
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
            let octets = v4.octets();
            !(v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
                || v4.is_multicast()
                // "This network" 0.0.0.0/8
                || octets[0] == 0
                // Reserved 240.0.0.0/4
                || octets[0] >= 240
                // Carrier-grade NAT 100.64.0.0/10
                || (octets[0] == 100 && (octets[1] & 0xC0) == 64)
                // Benchmarking 198.18.0.0/15
                || (octets[0] == 198 && (octets[1] & 0xFE) == 18))
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() || v6.is_unspecified() || v6.is_multicast() {
                return false;
            }
            let segments = v6.segments();
            let seg0 = segments[0];
            // Unique local fc00::/7
            if (seg0 & 0xFE00) == 0xFC00 {
                return false;
            }
            // Link-local fe80::/10
            if (seg0 & 0xFFC0) == 0xFE80 {
                return false;
            }
            // NAT64 well-known prefix 64:ff9b::/96
            if segments[0] == 0x0064
                && segments[1] == 0xFF9B
                && segments[2] == 0
                && segments[3] == 0
                && segments[4] == 0
                && segments[5] == 0
            {
                return false;
            }
            // Local-use NAT64 64:ff9b:1::/48 (RFC 8215)
            if segments[0] == 0x0064 && segments[1] == 0xFF9B && segments[2] == 0x0001 {
                return false;
            }
            // 6to4 2002::/16 (RFC 3056, embeds an IPv4 address)
            if seg0 == 0x2002 {
                return false;
            }
            // Teredo 2001::/32 (RFC 4380) and Documentation 2001:db8::/32 (RFC 3849)
            if seg0 == 0x2001 && (segments[1] == 0x0000 || segments[1] == 0x0DB8) {
                return false;
            }
            // IPv4-mapped ::ffff:0:0/96
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_public_ip(IpAddr::V4(v4));
            }
            // Deprecated IPv4-compatible ::/96
            segments[0..6] != [0, 0, 0, 0, 0, 0]
        }
    }
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
