//! HIBP (Have I Been Pwned) k-anonymity password checking.

use crate::errors::{PasswordAuthError, PasswordAuthResult};

/// Check a password against the HIBP Pwned Passwords k-anonymity range API.
///
/// Only the first 5 hex characters of the SHA-1 hash are sent over the network.
pub async fn hibp_k_anonymity_check(
    password: &str,
    api_base_url: &str,
    user_agent: &str,
) -> PasswordAuthResult<bool> {
    use reqwest::header::HeaderValue;
    use sha1::{Digest, Sha1};

    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    let digest = hasher.finalize();

    let mut hex = String::with_capacity(digest.len() * 2);
    for b in digest {
        hex.push_str(&format!("{:02X}", b));
    }

    let (prefix, suffix) = hex.split_at(5);

    let base = api_base_url.trim_end_matches('/');
    let url = format!("{}/range/{}", base, prefix);

    let client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .map_err(|e| PasswordAuthError::Internal(format!("failed to build HIBP client: {}", e)))?;

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

    let body = resp
        .text()
        .await
        .map_err(|e| PasswordAuthError::Internal(format!("HIBP response read failed: {}", e)))?;

    Ok(hibp_range_body_contains_suffix(&body, suffix))
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
    use super::{hibp_k_anonymity_check, hibp_range_body_contains_suffix};

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

    #[tokio::test]
    async fn hibp_k_anonymity_check_can_run_against_local_server() {
        use sha1::{Digest, Sha1};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let password = "password123";

        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let digest = hasher.finalize();

        let mut hex = String::with_capacity(digest.len() * 2);
        for b in digest {
            hex.push_str(&format!("{:02X}", b));
        }

        let (prefix, suffix) = hex.split_at(5);

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
