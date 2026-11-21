//! HIBP (Have I Been Pwned) k-anonymity password checking.

use crate::errors::{PasswordAuthError, PasswordAuthResult};
use async_trait::async_trait;
use sha1::{Digest, Sha1};
use underlay_http_client::{reqwest::Url, HttpClient};

fn http_host_is_local(url: &Url) -> bool {
    url.scheme() == "http"
        && url.host_str().is_some_and(|host| {
            matches!(host, "localhost" | "127.0.0.1" | "0.0.0.0" | "[::1]")
                || host.ends_with(".localhost")
        })
}

/// Minimal interface for HIBP range retrieval.
///
/// This enables deterministic tests without live network calls.
#[async_trait]
pub trait HibpRangeClient: Send + Sync {
    async fn fetch_range(&self, prefix: &str) -> PasswordAuthResult<String>;
}

#[derive(Debug, Clone)]
pub(crate) struct HttpHibpRangeClient {
    api_base_url: Url,
    client: HttpClient,
}

impl HttpHibpRangeClient {
    pub(crate) fn new(api_base_url: &str, user_agent: &str) -> PasswordAuthResult<Self> {
        let api_base_url = Url::parse(api_base_url.trim())
            .map_err(|_| PasswordAuthError::Internal("Invalid HIBP api_base_url".into()))?;
        if api_base_url.scheme() != "https" && !http_host_is_local(&api_base_url) {
            return Err(PasswordAuthError::Internal(
                "HIBP api_base_url must use https (http is only allowed for localhost)".into(),
            ));
        }
        if api_base_url.query().is_some()
            || api_base_url.fragment().is_some()
            || !api_base_url.username().is_empty()
            || api_base_url.password().is_some()
        {
            return Err(PasswordAuthError::Internal(
                "Invalid HIBP api_base_url".into(),
            ));
        }
        let client = HttpClient::try_with_user_agent(user_agent)
            .map_err(|e| PasswordAuthError::Internal(format!("HIBP client setup failed: {e}")))?;
        Ok(Self {
            api_base_url,
            client,
        })
    }
}

#[async_trait]
impl HibpRangeClient for HttpHibpRangeClient {
    async fn fetch_range(&self, prefix: &str) -> PasswordAuthResult<String> {
        use underlay_http_client::reqwest::header::HeaderValue;

        let mut url = self.api_base_url.clone();
        url.path_segments_mut()
            .map_err(|()| PasswordAuthError::Internal("Invalid HIBP api_base_url".into()))?
            .pop_if_empty()
            .extend(["range", prefix]);

        let resp = self
            .client
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
/// Only the first 5 hex characters of the SHA-1 hash are sent to the client.
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

        let Some((candidate_suffix, count_str)) = line.split_once(':') else {
            continue;
        };

        // HIBP `Add-Padding: true` appends synthetic `SUFFIX:0` lines; only
        // count > 0 represents an actual breach match.
        let count: u64 = count_str.trim().parse().unwrap_or(0);
        if count > 0 && candidate_suffix.trim().eq_ignore_ascii_case(suffix) {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[path = "tests/hibp_tests.rs"]
mod tests;
