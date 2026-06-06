use super::*;

#[test]
fn test_smtp_config_builder() {
    let config = SmtpConfig::new("smtp.example.com")
        .with_port(587)
        .with_credentials("user", "pass")
        .with_tls_mode(TlsMode::Required);

    assert_eq!(config.host(), "smtp.example.com");
    assert_eq!(config.port(), 587);
    assert_eq!(config.username(), Some("user"));
    assert_eq!(config.password(), Some("pass"));
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
        .text_body("Hello, World!")
        .build()
        .unwrap();

    let message = SmtpAdapter::build_message(&email).unwrap();
    // Message builds successfully
    assert!(message.headers().get_raw("Subject").is_some());
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
