    use super::*;

    #[test]
    fn refresh_token_cookie_secure() {
        let config = AuthCookieConfig::default();
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("refresh_token=test-token"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("Secure"));
        assert!(cookie.contains("SameSite=Lax"));
        assert!(cookie.contains("Path=/v1/auth"));
        assert!(cookie.contains("Max-Age="));
    }

    #[test]
    fn refresh_token_cookie_local_dev() {
        let config = AuthCookieConfig::local_dev();
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("refresh_token=test-token"));
        assert!(cookie.contains("HttpOnly"));
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn logged_in_cookie_not_httponly() {
        let config = AuthCookieConfig::default();
        let cookie = logged_in_cookie(&config);

        assert!(cookie.contains("logged_in=1"));
        assert!(!cookie.contains("HttpOnly"));
        assert!(cookie.contains("Path=/"));
    }

    #[test]
    fn extract_refresh_token_present() {
        let config = AuthCookieConfig::default();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=value; refresh_token=my-token; another=thing"),
        );

        let token = extract_refresh_token(&headers, &config);
        assert_eq!(token, Some("my-token".to_string()));
    }

    #[test]
    fn extract_refresh_token_missing() {
        let config = AuthCookieConfig::default();
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=value; another=thing"),
        );

        let token = extract_refresh_token(&headers, &config);
        assert_eq!(token, None);
    }

    #[test]
    fn extract_refresh_token_no_cookie_header() {
        let config = AuthCookieConfig::default();
        let headers = HeaderMap::new();
        let token = extract_refresh_token(&headers, &config);
        assert_eq!(token, None);
    }

    #[test]
    fn custom_cookie_prefix() {
        let config = AuthCookieConfig::default().with_cookie_prefix("acme_");
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("acme_refresh_token=test-token"));
        // Verify the cookie starts with the prefixed name (not an unprefixed "refresh_token=")
        assert!(cookie.starts_with("acme_refresh_token="));
    }

    #[test]
    fn extract_refresh_token_with_prefix() {
        let config = AuthCookieConfig::default().with_cookie_prefix("acme_");
        let mut headers = HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("other=value; acme_refresh_token=my-token; another=thing"),
        );

        let token = extract_refresh_token(&headers, &config);
        assert_eq!(token, Some("my-token".to_string()));
    }

    #[test]
    fn same_site_strict() {
        let config = AuthCookieConfig::default().with_same_site(SameSite::Strict);
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("SameSite=Strict"));
    }

    #[test]
    fn custom_refresh_token_path() {
        let config = AuthCookieConfig::default().with_refresh_token_path("/api/auth");
        let cookie = refresh_token_cookie("test-token", &config);

        assert!(cookie.contains("Path=/api/auth"));
    }

    #[test]
    fn builder_chain() {
        let config = AuthCookieConfig::new()
            .with_domain(".example.com")
            .with_secure(true)
            .with_refresh_token_max_age(14 * 24 * 60 * 60)
            .with_same_site(SameSite::Strict)
            .with_cookie_prefix("app_")
            .with_refresh_token_path("/auth");

        assert_eq!(config.domain, Some(".example.com".to_string()));
        assert!(config.secure);
        assert_eq!(config.refresh_token_max_age, 14 * 24 * 60 * 60);
        assert_eq!(config.same_site, SameSite::Strict);
        assert_eq!(config.cookie_prefix, "app_");
        assert_eq!(config.refresh_token_path, "/auth");
        assert_eq!(config.refresh_token_name(), "app_refresh_token");
        assert_eq!(config.logged_in_name(), "app_logged_in");
    }