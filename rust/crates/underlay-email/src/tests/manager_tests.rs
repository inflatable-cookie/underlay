use super::*;
use crate::adapter::NoopAdapter;

#[tokio::test]
async fn test_email_manager_send() {
    let adapter = Arc::new(NoopAdapter::new());
    let from = EmailAddress::new("noreply@example.com").unwrap();
    let manager = EmailManager::new(adapter, from);

    let to = EmailAddress::new("user@example.com").unwrap();
    let result = manager
        .send_email(to, "Test Subject", "Hello, World!", None)
        .await
        .unwrap();

    assert!(result.success);
}

#[tokio::test]
async fn test_email_manager_builder() {
    let adapter = Arc::new(NoopAdapter::new());
    let from = EmailAddress::with_name("noreply@example.com", "My App").unwrap();
    let manager = EmailManager::new(adapter, from.clone());

    let email = manager
        .email_builder()
        .to(EmailAddress::new("user@example.com").unwrap())
        .subject("Test")
        .text_body("Hello")
        .build()
        .unwrap();

    assert_eq!(email.from.email, "noreply@example.com");
    assert_eq!(email.from.name, Some("My App".to_string()));
}

#[test]
fn test_config_defaults() {
    let smtp = SmtpConfig::default();
    assert_eq!(smtp.host(), "localhost");
    assert_eq!(smtp.port(), 587);
    assert_eq!(smtp.tls_mode(), TlsMode::Required);

    let ses = SesConfig::default();
    assert_eq!(ses.region(), "us-east-1");

    let dev = DevCaptureConfig::default();
    assert!(dev.whitelist().is_empty());
    assert!(!dev.use_fallback());
}
