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

/// Extract client IP address from various headers
///
/// Checks headers in order of priority:
/// 1. CF-Connecting-IP (Cloudflare)
/// 2. X-Real-IP (nginx)
/// 3. X-Forwarded-For (first IP)
pub(in crate::context) fn extract_ip_address(headers: &HeaderMap) -> Option<IpAddr> {
    // Try CF-Connecting-IP first (Cloudflare)
    if let Some(ip) = headers
        .get(headers::CF_CONNECTING_IP)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    // Try X-Real-IP (common in nginx setups)
    if let Some(ip) = headers
        .get(headers::X_REAL_IP)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    // Try X-Forwarded-For (take first IP)
    if let Some(ip) = headers
        .get(headers::X_FORWARDED_FOR)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .and_then(|s| s.parse().ok())
    {
        return Some(ip);
    }

    None
}
