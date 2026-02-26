//! HIBP (Have I Been Pwned) k-anonymity password checking.

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use async_trait::async_trait;
use sha1::{Digest, Sha1};

/// Check a password against the HIBP Pwned Passwords k-anonymity range API.
///
/// Only the first 5 hex characters of the SHA-1 hash are sent over the network.
pub async fn hibp_k_anonymity_check(
    password: &str,
    api_base_url: &str,
    user_agent: &str,
) -> PasswordAuthResult<bool> {
    let client = HttpHibpRangeClient::new(api_base_url, user_agent);
    hibp_k_anonymity_check_with_client(password, &client).await
}

/// Minimal interface for HIBP range retrieval.
///
/// This enables deterministic tests without live network calls.
#[async_trait]
pub trait HibpRangeClient: Send + Sync {
    async fn fetch_range(&self, prefix: &str) -> PasswordAuthResult<String>;
}

#[derive(Debug, Clone)]
struct HttpHibpRangeClient {
    api_base_url: String,
    user_agent: String,
}

impl HttpHibpRangeClient {
    fn new(api_base_url: &str, user_agent: &str) -> Self {
        Self {
            api_base_url: api_base_url.trim_end_matches('/').to_string(),
            user_agent: user_agent.to_string(),
        }
    }
}

#[async_trait]
impl HibpRangeClient for HttpHibpRangeClient {
    async fn fetch_range(&self, prefix: &str) -> PasswordAuthResult<String> {
        use underlay_http_client::reqwest::header::HeaderValue;

        let url = format!("{}/range/{}", self.api_base_url, prefix);
        let client = underlay_http_client::HttpClient::with_user_agent(&self.user_agent);

        let resp = client
            .get(url)
            // Request padding to reduce information disclosure.
            .header("Add-Padding", HeaderValue::from_static("true"))
            .send()
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("HIBP request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(PasswordAuthError::Internal(format!(
                "HIBP returned {}",
                resp.status()
            )));
        }

        resp.text()
            .await
            .map_err(|e| PasswordAuthError::Internal(format!("HIBP response read failed: {}", e)))
    }
}

/// Check a password against HIBP using an injected client implementation.
///
/// Useful for end-to-end checks without relying on external network.
pub async fn hibp_k_anonymity_check_with_client<C>(
    password: &str,
    client: &C,
) -> PasswordAuthResult<bool>
where
    C: HibpRangeClient + ?Sized,
{
    let hex = password_sha1_hex(password);
    let (prefix, suffix) = hex.split_at(5);
    let body = client.fetch_range(prefix).await?;

    Ok(hibp_range_body_contains_suffix(&body, suffix))
}

fn password_sha1_hex(password: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();

    let mut hex = String::with_capacity(digest.len() * 2);
    for b in digest {
        hex.push_str(&format!("{:02X}", b));
    }
    hex
}

fn hibp_range_body_contains_suffix(body: &str, suffix: &str) -> bool {
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((candidate_suffix, _count)) = line.split_once(':') else {
            continue;
        };

        if candidate_suffix.trim().eq_ignore_ascii_case(suffix) {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[path = "tests/hibp_tests.rs"]
mod tests;
