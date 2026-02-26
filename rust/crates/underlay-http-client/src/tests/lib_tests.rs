use super::HttpClient;

#[test]
fn default_client_builds_requests() {
    let client = HttpClient::new();
    client
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn custom_user_agent_client_builds_requests() {
    let client = HttpClient::with_user_agent("test-agent/1.0");
    client
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn try_new_builds_requests() {
    let client = HttpClient::try_new().expect("client should build");
    client
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn try_with_user_agent_builds_requests() {
    let client = HttpClient::try_with_user_agent("try-agent/1.0").expect("client should build");
    client
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn default_matches_new() {
    HttpClient::default()
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
    HttpClient::new()
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn deref_and_inner_reference_same_client() {
    let client = HttpClient::with_user_agent("deref-test/1.0");
    assert!(std::ptr::eq(client.inner(), &*client));
}

#[test]
fn into_inner_exposes_reqwest_client() {
    let client = HttpClient::with_user_agent("deref-test/1.0");

    client
        .get("https://example.com")
        .build()
        .expect("request should build via deref");

    let inner = client.into_inner();
    inner
        .get("https://example.com")
        .build()
        .expect("request should build");
}
