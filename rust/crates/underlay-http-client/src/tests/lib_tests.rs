use std::time::Duration;

use super::{HttpClient, DEFAULT_CONNECT_TIMEOUT, DEFAULT_TIMEOUT};

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
fn try_with_user_agent_rejects_invalid_header_value() {
    let error = HttpClient::try_with_user_agent("invalid\nuser-agent")
        .expect_err("invalid user-agent should fail the fallible constructor");
    assert!(error.is_builder(), "expected a builder error, got: {error}");
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

#[test]
fn external_client_builds() {
    let client = super::HttpClient::external().expect("external client should build");
    client
        .inner()
        .get("https://example.com")
        .build()
        .expect("request should build");
}

#[test]
fn validate_external_url_rejects_private_and_metadata_targets() {
    use super::{validate_external_url, HttpClientError};

    for blocked in [
        "http://127.0.0.1/",
        "http://localhost/",                        // resolves to loopback
        "http://169.254.169.254/latest/meta-data/", // cloud metadata
        "http://10.0.0.5/",
        "http://192.168.1.1/",
        "http://172.16.0.1/",
        "http://[::1]/",
        "http://0.0.0.0/",
        "http://100.64.0.1/", // CGNAT
    ] {
        let err = validate_external_url(blocked).expect_err(blocked);
        assert!(
            matches!(
                err,
                HttpClientError::BlockedTarget(_) | HttpClientError::InvalidUrl(_)
            ),
            "{blocked} -> {err:?}"
        );
    }
}

#[test]
fn validate_external_url_rejects_non_http_schemes() {
    use super::{validate_external_url, HttpClientError};
    let err = validate_external_url("file:///etc/passwd").expect_err("file scheme");
    assert!(matches!(err, HttpClientError::InvalidUrl(_)));
    let err = validate_external_url("gopher://example.com/").expect_err("gopher scheme");
    assert!(matches!(err, HttpClientError::InvalidUrl(_)));
}

#[test]
fn is_public_ip_classifies_correctly() {
    use super::is_public_ip;
    use std::net::IpAddr;

    // Public
    assert!(is_public_ip("1.1.1.1".parse::<IpAddr>().unwrap()));
    assert!(is_public_ip("8.8.8.8".parse::<IpAddr>().unwrap()));
    assert!(is_public_ip(
        "2606:4700:4700::1111".parse::<IpAddr>().unwrap()
    ));

    // Non-public
    assert!(!is_public_ip("127.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("10.0.0.1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("192.168.0.1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("172.16.0.1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("169.254.169.254".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("::1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("fc00::1".parse::<IpAddr>().unwrap()));
    assert!(!is_public_ip("fe80::1".parse::<IpAddr>().unwrap()));
    // IPv4-mapped loopback must also be blocked.
    assert!(!is_public_ip("::ffff:127.0.0.1".parse::<IpAddr>().unwrap()));
}

#[test]
fn default_timeouts_are_bounded() {
    // bounded_builder applies these to configured and fallback clients; guard
    // the values so a refactor cannot silently drop or loosen them.
    assert_eq!(DEFAULT_CONNECT_TIMEOUT, Duration::from_secs(10));
    assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));
}

/// A server that accepts connections, reads the first request, and then never
/// responds. It returns its address and a flag that flips once the first
/// request has been read, which tells the caller the client's connect phase is
/// complete and its connect timeout has been disarmed.
async fn stalled_server() -> (
    std::net::SocketAddr,
    std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::io::AsyncReadExt;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let addr = listener.local_addr().expect("addr");
    let request_seen = Arc::new(AtomicBool::new(false));
    let seen = Arc::clone(&request_seen);
    tokio::spawn(async move {
        let mut held = Vec::new();
        let mut first = true;
        while let Ok((mut socket, _)) = listener.accept().await {
            if first {
                first = false;
                let mut buf = [0u8; 1024];
                // A client only writes its request once the connection is
                // established, so a successful read proves reqwest's connect
                // timeout has been disarmed.
                let _ = socket.read(&mut buf).await;
                seen.store(true, Ordering::SeqCst);
            }
            held.push(socket);
        }
    });
    (addr, request_seen)
}

#[tokio::test]
async fn invalid_user_agent_fallback_retains_default_timeout() {
    use std::sync::atomic::Ordering;

    let (addr, request_seen) = stalled_server().await;
    let client = HttpClient::with_user_agent("invalid\nuser-agent");
    let started = tokio::time::Instant::now();

    let mut request =
        tokio::spawn(async move { client.get(format!("http://{addr}/stall")).send().await });

    // Wait in real time until the request bytes reach the server, then freeze
    // the clock. `start_paused` alone races the paused clock against the real
    // TCP handshake: auto-advance can hit the 10s connect timeout before the
    // connection exists, so the request fails at ~10s and the old assertion
    // flaked. Freezing once the request has been read leaves the 30s total
    // timeout as the only pending deadline.
    tokio::time::timeout(Duration::from_secs(5), async {
        while !request_seen.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("server should receive the request");
    tokio::time::pause();

    // Advance to just below the default total timeout. With the connect timeout
    // disarmed, a fallback that retained only the connect timeout (or any
    // shorter total timeout) has already resolved here.
    let up_to = DEFAULT_TIMEOUT
        .saturating_sub(started.elapsed())
        .saturating_sub(Duration::from_secs(5));
    tokio::time::advance(up_to).await;
    for _ in 0..3 {
        tokio::task::yield_now().await;
    }
    assert!(
        !request.is_finished(),
        "fallback failed before the default total timeout of {DEFAULT_TIMEOUT:?}"
    );

    // Let the paused clock reach the remaining deadline and confirm it is the
    // default total timeout. The outer timeout turns an unbounded fallback into
    // a failure instead of a hang.
    let joined = tokio::time::timeout(DEFAULT_TIMEOUT, &mut request)
        .await
        .expect("fallback did not time out within twice the default total timeout");
    let err = joined
        .expect("request task should not panic")
        .expect_err("bounded fallback must time out on a stalled server");
    assert!(err.is_timeout(), "expected a timeout error, got: {err}");
    assert!(
        started.elapsed() >= DEFAULT_TIMEOUT,
        "fallback timed out before the default total timeout"
    );
    assert!(
        started.elapsed() < DEFAULT_TIMEOUT + Duration::from_secs(5),
        "fallback did not retain the default total timeout"
    );
}

#[tokio::test]
async fn timeout_fires_on_stalled_server() {
    // A server that accepts a connection, reads the request, but never
    // responds. The per-request timeout is shrunk from the 30s default so the
    // test runs fast; the mechanism exercised (reqwest total-timeout on our
    // constructed client) is the same one the default arms.
    let (addr, _request_seen) = stalled_server().await;

    let client = HttpClient::new();
    let err = client
        .get(format!("http://{addr}/stall"))
        .timeout(Duration::from_millis(300))
        .send()
        .await
        .expect_err("stalled server must not succeed");
    assert!(err.is_timeout(), "expected a timeout error, got: {err}");
}
