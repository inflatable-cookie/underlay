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