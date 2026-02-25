//! HIBP (Have I Been Pwned) k-anonymity password checking.

use async_trait::async_trait;
use crate::errors::{PasswordAuthError, PasswordAuthResult};
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
mod tests {
    use super::{
        hibp_k_anonymity_check, hibp_k_anonymity_check_with_client, hibp_range_body_contains_suffix,
        password_sha1_hex, HibpRangeClient,
    };
    use crate::errors::{PasswordAuthError, PasswordAuthResult};
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    #[test]
    fn hibp_range_body_parser_matches_suffix_case_insensitive() {
        let body = "ABCDEF1234567890:2\nFEDCBA9876543210:10\n";
        assert!(hibp_range_body_contains_suffix(body, "abcdef1234567890"));
        assert!(hibp_range_body_contains_suffix(body, "ABCDEF1234567890"));
        assert!(!hibp_range_body_contains_suffix(body, "00000"));
    }

    #[test]
    fn hibp_range_body_parser_ignores_bad_lines() {
        let body = "\n\nNOT_A_MATCH\nAAAAA:1\nBBBBB : 2\n";
        assert!(hibp_range_body_contains_suffix(body, "AAAAA"));
        assert!(hibp_range_body_contains_suffix(body, "BBBBB"));
    }

    #[derive(Clone, Default)]
    struct MockHibpClient {
        prefixes: Arc<Mutex<Vec<String>>>,
        body: String,
        fail: bool,
    }

    #[async_trait]
    impl HibpRangeClient for MockHibpClient {
        async fn fetch_range(&self, prefix: &str) -> PasswordAuthResult<String> {
            self.prefixes.lock().expect("prefix lock poisoned").push(prefix.to_string());
            if self.fail {
                return Err(PasswordAuthError::Internal("mock failure".to_string()));
            }
            Ok(self.body.clone())
        }
    }

    #[tokio::test]
    async fn hibp_k_anonymity_check_with_client_uses_prefix_and_matches_suffix() {
        let password = "password123";
        let digest = password_sha1_hex(password);
        let (prefix, suffix) = digest.split_at(5);
        let mock = MockHibpClient {
            body: format!("{suffix}:42\n"),
            ..Default::default()
        };

        let compromised = hibp_k_anonymity_check_with_client(password, &mock)
            .await
            .expect("mock hibp check should succeed");

        assert!(compromised);
        let seen = mock.prefixes.lock().expect("prefix lock poisoned");
        assert_eq!(seen.as_slice(), &[prefix.to_string()]);
    }

    #[tokio::test]
    async fn hibp_k_anonymity_check_with_client_propagates_client_error() {
        let mock = MockHibpClient {
            fail: true,
            ..Default::default()
        };
        let err = hibp_k_anonymity_check_with_client("password123", &mock)
            .await
            .expect_err("mock hibp check should fail");
        assert!(format!("{err}").contains("mock failure"));
    }

    #[tokio::test]
    async fn hibp_k_anonymity_check_can_run_against_local_server() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let password = "password123";
        let digest = password_sha1_hex(password);
        let (prefix, suffix) = digest.split_at(5);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let base = format!("http://{}", addr);

        // Serve exactly one request.
        let expected_path = format!("/range/{prefix}");
        let suffix_line = format!("{suffix}:42\n");

        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            let mut buf = vec![0_u8; 4096];
            let n = socket.read(&mut buf).await.unwrap();
            let req = String::from_utf8_lossy(&buf[..n]);

            assert!(req.starts_with("GET "));
            assert!(req.contains(&expected_path));

            let body = suffix_line;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );

            socket.write_all(resp.as_bytes()).await.unwrap();
        });

        let ok = hibp_k_anonymity_check(password, &base, "underlay-test")
            .await
            .unwrap();

        assert!(ok);

        server.await.unwrap();
    }
}
