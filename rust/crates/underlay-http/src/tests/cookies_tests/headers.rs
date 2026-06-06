use super::*;

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
