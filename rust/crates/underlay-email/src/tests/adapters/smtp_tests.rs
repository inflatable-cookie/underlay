use super::*;

#[test]
fn test_smtp_config_builder() {
    let default_config = SmtpConfig::new("smtp.example.com");
    assert_eq!(default_config.timeout(), std::time::Duration::from_secs(30));

    let config = SmtpConfig::new("smtp.example.com")
        .with_port(587)
        .with_credentials("user", "pass")
        .with_tls_mode(TlsMode::Required)
        .with_timeout(std::time::Duration::from_secs(12));

    assert_eq!(config.host(), "smtp.example.com");
    assert_eq!(config.port(), 587);
    assert_eq!(config.username(), Some("user"));
    assert_eq!(config.password(), Some("pass"));
    assert_eq!(config.timeout(), std::time::Duration::from_secs(12));
}

#[test]
fn test_to_mailbox() {
    let addr = EmailAddress::new("user@example.com").unwrap();
    let mailbox = SmtpAdapter::to_mailbox(&addr).unwrap();
    assert_eq!(mailbox.email.to_string(), "user@example.com");
    assert!(mailbox.name.is_none());

    let addr_with_name = EmailAddress::with_name("user@example.com", "John Doe").unwrap();
    let mailbox_with_name = SmtpAdapter::to_mailbox(&addr_with_name).unwrap();
    assert_eq!(mailbox_with_name.name, Some("John Doe".to_string()));
}

#[test]
fn test_build_message_text_only() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .header("X-Trace", "smtp-test")
        .text_body("Hello, World!")
        .build()
        .unwrap();

    let message = SmtpAdapter::build_message(&email).unwrap();
    assert_eq!(message.headers().get_raw("X-Trace"), Some("smtp-test"));
}

#[test]
fn test_build_message_multipart() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .text_body("Hello, World!")
        .html_body("<h1>Hello, World!</h1>")
        .build()
        .unwrap();

    let message = SmtpAdapter::build_message(&email).unwrap();
    assert!(message.headers().get_raw("Subject").is_some());
}

#[tokio::test]
async fn smtp_timeout_bounds_send_and_health_check_when_server_stalls_after_connect() {
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // Accept TCP connections and hold them open without sending an SMTP greeting.
    let server = tokio::spawn(async move {
        let mut sockets = Vec::new();
        while let Ok((socket, _)) = listener.accept().await {
            sockets.push(socket);
        }
    });

    let config = SmtpConfig::new("127.0.0.1")
        .with_port(port)
        .with_tls_mode(TlsMode::None)
        .with_timeout(Duration::from_millis(100));
    let adapter = SmtpAdapter::new(&config).unwrap();

    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Timeout Test")
        .text_body("Body")
        .build()
        .unwrap();

    let send_result = tokio::time::timeout(Duration::from_secs(2), adapter.send(&email))
        .await
        .expect("send must not hang past configured timeout");
    assert!(
        matches!(send_result, Err(EmailError::ConnectionFailed(ref msg)) if msg.contains("timed out")),
        "expected ConnectionFailed timeout error, got {send_result:?}"
    );

    let health_result = tokio::time::timeout(Duration::from_secs(2), adapter.health_check())
        .await
        .expect("health_check must not hang past configured timeout");
    assert!(
        matches!(health_result, Err(EmailError::ConnectionFailed(ref msg)) if msg.contains("timed out")),
        "expected ConnectionFailed timeout error, got {health_result:?}"
    );

    server.abort();
}
