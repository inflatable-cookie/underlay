use super::*;

#[test]
fn refresh_token_cookie_secure() {
    let config = AuthCookieConfig::default();
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

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
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("refresh_token=test-token"));
    assert!(cookie.contains("HttpOnly"));
    assert!(!cookie.contains("Secure"));
}

#[test]
fn logged_in_cookie_not_httponly() {
    let config = AuthCookieConfig::default();
    let cookie = logged_in_cookie(&config).unwrap();

    assert!(cookie.contains("logged_in=1"));
    assert!(!cookie.contains("HttpOnly"));
    assert!(cookie.contains("Path=/"));
}

#[test]
fn csrf_token_cookie_not_httponly() {
    let config = AuthCookieConfig::default();
    let cookie = csrf_token_cookie("csrf-token", &config).unwrap();

    assert!(cookie.contains("csrf_token=csrf-token"));
    assert!(!cookie.contains("HttpOnly"));
    assert!(cookie.contains("Secure"));
    assert!(cookie.contains("SameSite=Lax"));
    assert!(cookie.contains("Path=/"));
    assert!(cookie.contains("Max-Age="));
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
fn extract_csrf_token_present() {
    let config = AuthCookieConfig::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        HeaderValue::from_static("other=value; csrf_token=my-csrf; another=thing"),
    );

    let token = extract_csrf_token(&headers, &config);
    assert_eq!(token, Some("my-csrf".to_string()));
}

#[test]
fn custom_cookie_prefix() {
    let config = AuthCookieConfig::default().with_cookie_prefix("acme_");
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("acme_refresh_token=test-token"));
    // Verify the cookie starts with the prefixed name (not an unprefixed "refresh_token=")
    assert!(cookie.starts_with("acme_refresh_token="));
    assert_eq!(config.csrf_token_name(), "acme_csrf_token");
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
fn extract_csrf_token_with_prefix() {
    let config = AuthCookieConfig::default().with_cookie_prefix("acme_");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        HeaderValue::from_static("other=value; acme_csrf_token=my-csrf; another=thing"),
    );

    let token = extract_csrf_token(&headers, &config);
    assert_eq!(token, Some("my-csrf".to_string()));
}

#[test]
fn same_site_strict() {
    let config = AuthCookieConfig::default().with_same_site(SameSite::Strict);
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("SameSite=Strict"));
}

#[test]
fn custom_refresh_token_path() {
    let config = AuthCookieConfig::default().with_refresh_token_path("/api/auth");
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("Path=/api/auth"));
}

#[test]
fn same_site_none_requires_secure() {
    let config = AuthCookieConfig::local_dev().with_same_site(SameSite::None);

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::SameSiteNoneRequiresSecure)
    );
}

#[test]
fn rejects_invalid_cookie_prefix() {
    let config = AuthCookieConfig::default().with_cookie_prefix("bad;");

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::InvalidCookiePrefix)
    );
}

#[test]
fn rejects_invalid_refresh_token_path() {
    let config = AuthCookieConfig::default().with_refresh_token_path("api/auth");

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::InvalidPath)
    );
}

#[test]
fn rejects_invalid_domain() {
    let config = AuthCookieConfig::default().with_domain("bad;domain");

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::InvalidDomain)
    );
}

#[test]
fn rejects_invalid_domain_label() {
    let config = AuthCookieConfig::default().with_domain("example-.com");

    assert_eq!(
        refresh_token_cookie("test-token", &config),
        Err(AuthCookieError::InvalidDomain)
    );
}

#[test]
fn rejects_invalid_cookie_value() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        refresh_token_cookie("bad;token", &config),
        Err(AuthCookieError::InvalidValue)
    );
}

#[test]
fn rejects_empty_cookie_value() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        refresh_token_cookie("", &config),
        Err(AuthCookieError::InvalidValue)
    );
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
    assert_eq!(config.csrf_token_name(), "app_csrf_token");
}

#[test]
fn typed_cookie_fields_validate_at_construction() {
    let domain = CookieDomain::parse(".example.com").unwrap();
    let path = CookiePath::parse("/v1/auth").unwrap();
    let name = CookieName::parse("app_refresh_token").unwrap();

    assert_eq!(domain.as_str(), ".example.com");
    assert_eq!(path.as_str(), "/v1/auth");
    assert_eq!(name.as_str(), "app_refresh_token");
    assert_eq!(name.to_string(), "app_refresh_token");

    assert_eq!(
        CookieDomain::parse("bad;domain"),
        Err(AuthCookieError::InvalidDomain)
    );
    assert_eq!(
        CookiePath::parse("v1/auth"),
        Err(AuthCookieError::InvalidPath)
    );
    assert_eq!(
        CookieName::parse("bad;name"),
        Err(AuthCookieError::InvalidCookiePrefix)
    );
}

#[test]
fn auth_cookie_config_try_builders_validate_early() {
    let config = AuthCookieConfig::default()
        .try_with_domain(".example.com")
        .unwrap()
        .try_with_cookie_prefix("app_")
        .unwrap()
        .try_with_refresh_token_path("/api/auth")
        .unwrap();

    assert_eq!(config.domain.as_deref(), Some(".example.com"));
    assert_eq!(config.cookie_prefix, "app_");
    assert_eq!(config.refresh_token_path, "/api/auth");
    assert_eq!(
        config.refresh_token_cookie_name().unwrap().as_str(),
        "app_refresh_token"
    );
    assert_eq!(
        config.logged_in_cookie_name().unwrap().as_str(),
        "app_logged_in"
    );
    assert_eq!(
        config.csrf_token_cookie_name().unwrap().as_str(),
        "app_csrf_token"
    );

    assert_eq!(
        AuthCookieConfig::default()
            .try_with_cookie_prefix("bad;")
            .unwrap_err(),
        AuthCookieError::InvalidCookiePrefix
    );
}

#[test]
fn set_and_clear_csrf_cookie_append_headers() {
    let config = AuthCookieConfig::default();
    let mut headers = HeaderMap::new();

    set_csrf_cookie(&mut headers, "csrf-token", &config).unwrap();
    clear_csrf_cookie(&mut headers, &config).unwrap();

    let cookies = headers
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|value| value.to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    assert_eq!(cookies.len(), 2);
    assert!(cookies[0].contains("csrf_token=csrf-token"));
    assert!(cookies[1].contains("csrf_token=;"));
    assert!(cookies[1].contains("Max-Age=0"));
}

#[test]
fn csrf_cookie_reuses_value_validation() {
    let config = AuthCookieConfig::default();

    assert_eq!(
        csrf_token_cookie("bad;token", &config),
        Err(AuthCookieError::InvalidValue)
    );
}
