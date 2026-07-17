use axum::http::HeaderMap;
use std::net::IpAddr;
use uuid::Uuid;

use super::headers;

/// Extract request ID from headers or generate a new one
pub(in crate::context) fn extract_request_id(headers: &HeaderMap) -> String {
    headers
        .get(headers::X_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

/// Trusted-proxy boundary for client IP resolution.
///
/// Client-supplied forwarding headers are attacker-controlled unless a trusted
/// proxy in front of the app strips or overwrites them. Rate limiting and
/// security alerting key on the resolved IP, so trusting the wrong header lets
/// an attacker rotate identities freely. Declare the actual deployment
/// topology; the default trusts no headers at all.
///
/// Install as a request extension, e.g.:
///
/// ```rust,ignore
/// use axum::Extension;
/// use underlay_http::TrustedProxyConfig;
///
/// let app = router.layer(Extension(TrustedProxyConfig::ForwardedFor { trusted_hops: 1 }));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TrustedProxyConfig {
    /// Trust no forwarding headers (default). The client IP is the socket
    /// peer address (requires serving with
    /// `into_make_service_with_connect_info::<SocketAddr>()`).
    #[default]
    None,
    /// Deployment fronted exclusively by Cloudflare: trust `CF-Connecting-IP`.
    /// Only safe when the origin accepts traffic from Cloudflare alone.
    CloudflareHeader,
    /// A directly-fronting trusted proxy (e.g. nginx) sets `X-Real-IP` after
    /// stripping any client-supplied value.
    RealIpHeader,
    /// `trusted_hops` trusted proxies each append the peer they saw to
    /// `X-Forwarded-For`; the client IP is the `trusted_hops`-th entry from
    /// the right. Entries further left are client-supplied and ignored.
    ForwardedFor { trusted_hops: usize },
}

/// Resolve the client IP from the configured trusted-proxy boundary.
///
/// `socket_ip` is the connection peer address when available; it is the
/// fallback whenever the configured header is absent or unparseable.
pub(in crate::context) fn resolve_ip_address(
    headers: &HeaderMap,
    config: &TrustedProxyConfig,
    socket_ip: Option<IpAddr>,
) -> Option<IpAddr> {
    match config {
        TrustedProxyConfig::None => socket_ip,
        TrustedProxyConfig::CloudflareHeader => headers
            .get(headers::CF_CONNECTING_IP)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.trim().parse().ok())
            .or(socket_ip),
        TrustedProxyConfig::RealIpHeader => headers
            .get(headers::X_REAL_IP)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.trim().parse().ok())
            .or(socket_ip),
        TrustedProxyConfig::ForwardedFor { trusted_hops } => {
            forwarded_for_client_ip(headers, *trusted_hops).or(socket_ip)
        }
    }
}

fn forwarded_for_client_ip(headers: &HeaderMap, trusted_hops: usize) -> Option<IpAddr> {
    if trusted_hops == 0 {
        return None;
    }

    let entries: Vec<&str> = headers
        .get_all(headers::X_FORWARDED_FOR)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(','))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // Each trusted proxy appends exactly one entry, so the client as seen by
    // the outermost trusted proxy sits `trusted_hops` from the right. Fewer
    // entries than trusted hops means the header was not fully populated by
    // the trusted chain; fall back to the socket peer rather than trusting a
    // client-supplied value.
    let index = entries.len().checked_sub(trusted_hops)?;
    entries[index].parse().ok()
}
