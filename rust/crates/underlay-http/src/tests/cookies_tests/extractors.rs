use super::*;

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
fn extract_refresh_token_with_prefix() {
    let config = AuthCookieConfig::default()
        .try_with_cookie_prefix("acme_")
        .unwrap();
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
    let config = AuthCookieConfig::default()
        .try_with_cookie_prefix("acme_")
        .unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        HeaderValue::from_static("other=value; acme_csrf_token=my-csrf; another=thing"),
    );

    let token = extract_csrf_token(&headers, &config);
    assert_eq!(token, Some("my-csrf".to_string()));
}

#[test]
fn extract_tokens_across_multiple_cookie_headers() {
    let config = AuthCookieConfig::default();
    let mut headers = HeaderMap::new();
    headers.append(header::COOKIE, HeaderValue::from_static("theme=dark"));
    headers.append(
        header::COOKIE,
        HeaderValue::from_static("refresh_token=h2-refresh"),
    );
    headers.append(
        header::COOKIE,
        HeaderValue::from_static("csrf_token=h2-csrf"),
    );

    assert_eq!(
        extract_refresh_token(&headers, &config),
        Some("h2-refresh".to_string())
    );
    assert_eq!(
        extract_csrf_token(&headers, &config),
        Some("h2-csrf".to_string())
    );
}
