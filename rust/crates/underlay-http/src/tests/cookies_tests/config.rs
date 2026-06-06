use super::*;

#[test]
fn custom_cookie_prefix() {
    let config = AuthCookieConfig::default().with_cookie_prefix("acme_");
    let cookie = refresh_token_cookie("test-token", &config).unwrap();

    assert!(cookie.contains("acme_refresh_token=test-token"));
    assert!(cookie.starts_with("acme_refresh_token="));
    assert_eq!(config.csrf_token_name(), "acme_csrf_token");
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
