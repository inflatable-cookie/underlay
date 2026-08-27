use super::*;
use axum::http::HeaderValue;
use crate::context::TrustedProxyConfig;
use std::net::IpAddr;

fn socket(ip: &str) -> Option<IpAddr> {
    Some(ip.parse().unwrap())
}

#[test]
fn test_default_config_ignores_all_forwarding_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("1.1.1.1"),
    );
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("2.2.2.2"));
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("3.3.3.3"),
    );

    let ip = resolve_ip_address(&headers, &TrustedProxyConfig::None, socket("9.9.9.9"));
    assert_eq!(ip, socket("9.9.9.9"));

    let no_socket = resolve_ip_address(&headers, &TrustedProxyConfig::None, None);
    assert!(no_socket.is_none());
}

#[test]
fn test_cloudflare_config_uses_cf_connecting_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::CF_CONNECTING_IP,
        HeaderValue::from_static("192.168.1.1"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::CloudflareHeader,
        socket("9.9.9.9"),
    );
    assert_eq!(ip, Some("192.168.1.1".parse().unwrap()));

    let fallback = resolve_ip_address(
        &HeaderMap::new(),
        &TrustedProxyConfig::CloudflareHeader,
        socket("9.9.9.9"),
    );
    assert_eq!(fallback, socket("9.9.9.9"));
}

#[test]
fn test_real_ip_config_uses_x_real_ip() {
    let mut headers = HeaderMap::new();
    headers.insert(headers::X_REAL_IP, HeaderValue::from_static("10.0.0.1"));

    let ip = resolve_ip_address(&headers, &TrustedProxyConfig::RealIpHeader, None);
    assert_eq!(ip, Some("10.0.0.1".parse().unwrap()));
}

#[test]
fn test_forwarded_for_takes_rightmost_untrusted_hop() {
    let mut headers = HeaderMap::new();
    // client-supplied garbage, then the real client as appended by the
    // single trusted proxy
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 203.0.113.195"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 1 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip, Some("203.0.113.195".parse().unwrap()));

    // two trusted hops: the second entry from the right is the client
    let ip2 = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 2 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip2, Some("6.6.6.6".parse().unwrap()));
}

#[test]
fn test_forwarded_for_spoofed_prefix_cannot_change_resolved_ip() {
    // Attacker sends a forged XFF; the trusted proxy appends the real peer.
    let mut spoofed = HeaderMap::new();
    spoofed.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 7.7.7.7, 203.0.113.195"),
    );

    let mut clean = HeaderMap::new();
    clean.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("203.0.113.195"),
    );

    let config = TrustedProxyConfig::ForwardedFor { trusted_hops: 1 };
    assert_eq!(
        resolve_ip_address(&spoofed, &config, None),
        resolve_ip_address(&clean, &config, None),
    );
}

#[test]
fn test_forwarded_for_falls_back_to_socket_when_underpopulated() {
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("203.0.113.195"),
    );

    let ip = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 2 },
        socket("9.9.9.9"),
    );
    assert_eq!(ip, socket("9.9.9.9"));

    let zero_hops = resolve_ip_address(
        &headers,
        &TrustedProxyConfig::ForwardedFor { trusted_hops: 0 },
        socket("9.9.9.9"),
    );
    assert_eq!(zero_hops, socket("9.9.9.9"));
}

#[test]
fn test_public_resolve_client_ip_matches_extractor_resolution() {
    // The public wrapper must produce the same result the extractor would.
    let mut headers = HeaderMap::new();
    headers.insert(
        headers::X_FORWARDED_FOR,
        HeaderValue::from_static("6.6.6.6, 203.0.113.195"),
    );
    let config = TrustedProxyConfig::ForwardedFor { trusted_hops: 1 };

    assert_eq!(
        resolve_client_ip(&headers, &config, socket("9.9.9.9")),
        resolve_ip_address(&headers, &config, socket("9.9.9.9")),
    );
    // Fail-closed default ignores the spoofable header and uses the socket peer.
    assert_eq!(
        resolve_client_ip(&headers, &TrustedProxyConfig::None, socket("9.9.9.9")),
        socket("9.9.9.9"),
    );
}

#[test]
fn test_trusted_proxy_from_env_parsing() {
    let parse = TrustedProxyConfig::parse_env;

    // Unset / off / none / blank -> fail-closed default, all recognised.
    assert_eq!(parse(None, None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some(""), None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some("none"), None), (TrustedProxyConfig::None, true));
    assert_eq!(parse(Some("off"), None), (TrustedProxyConfig::None, true));

    // Case-insensitive, whitespace-trimmed aliases.
    assert_eq!(
        parse(Some("  CloudFlare "), None),
        (TrustedProxyConfig::CloudflareHeader, true)
    );
    assert_eq!(
        parse(Some("cf"), None),
        (TrustedProxyConfig::CloudflareHeader, true)
    );
    assert_eq!(
        parse(Some("X-Real-IP"), None),
        (TrustedProxyConfig::RealIpHeader, true)
    );

    // forwarded-for: hops default 1, honour override, ignore garbage.
    assert_eq!(
        parse(Some("forwarded-for"), None),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 1 }, true)
    );
    assert_eq!(
        parse(Some("xff"), Some("3")),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 3 }, true)
    );
    assert_eq!(
        parse(Some("forwarded-for"), Some("notanumber")),
        (TrustedProxyConfig::ForwardedFor { trusted_hops: 1 }, true)
    );

    // Unrecognised mode -> fail-closed None, flagged unrecognised for the warn.
    assert_eq!(
        parse(Some("cloudfare"), None),
        (TrustedProxyConfig::None, false)
    );
}
