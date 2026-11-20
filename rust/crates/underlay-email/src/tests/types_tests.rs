use super::*;

#[test]
fn test_email_address_new() {
    let addr = EmailAddress::new("user@example.com").unwrap();
    assert_eq!(addr.email, "user@example.com");
    assert!(addr.name.is_none());
}

#[test]
fn test_email_address_with_name() {
    let addr = EmailAddress::with_name("user@example.com", "John Doe").unwrap();
    assert_eq!(addr.email, "user@example.com");
    assert_eq!(addr.name, Some("John Doe".to_string()));
    assert_eq!(addr.formatted(), "John Doe <user@example.com>");
}

#[test]
fn test_email_address_invalid() {
    assert!(EmailAddress::new("").is_err());
    assert!(EmailAddress::new("invalid").is_err());
    assert!(EmailAddress::new("missing@domain").is_err());
    assert!(EmailAddress::new("@example.com").is_err());
    assert!(EmailAddress::new("user@").is_err());
}

#[test]
fn email_address_deserialization_uses_constructor_validation() {
    let valid = serde_json::json!({"email": "user@example.com", "name": "Example User"});
    assert_eq!(
        serde_json::from_value::<EmailAddress>(valid).unwrap(),
        EmailAddress::with_name("user@example.com", "Example User").unwrap()
    );

    for fields in [
        serde_json::json!({"email": "user@example.com\r\n"}),
        serde_json::json!({"email": "user@example.com", "name": "Example\u{7f} User"}),
        serde_json::json!({"email": "missing-domain"}),
    ] {
        assert!(serde_json::from_value::<EmailAddress>(fields).is_err());
    }
}

#[test]
fn email_builder_rejects_controls_in_subject_and_headers() {
    let builder = || {
        Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Subject")
            .text_body("Body")
    };

    assert!(matches!(
        builder().subject("Subject\r\n").build(),
        Err(EmailError::ConfigError(_))
    ));
    assert!(matches!(
        builder().header("X-Test\n", "value").build(),
        Err(EmailError::ConfigError(_))
    ));
    assert!(matches!(
        builder().header("X-Test", "value\u{7f}").build(),
        Err(EmailError::ConfigError(_))
    ));
    assert!(matches!(
        builder().header("sUbJeCt", "value").build(),
        Err(EmailError::ConfigError(_))
    ));
}

#[test]
fn test_email_builder() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .text_body("Hello, World!")
        .build()
        .unwrap();

    assert_eq!(email.from.email, "sender@example.com");
    assert_eq!(email.to.len(), 1);
    assert_eq!(email.to[0].email, "recipient@example.com");
    assert_eq!(email.subject, "Test Subject");
    assert_eq!(email.text_body, Some("Hello, World!".to_string()));
}

#[test]
fn test_email_builder_missing_from() {
    let result = Email::builder()
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test")
        .text_body("Body")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_email_builder_missing_to() {
    let result = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .subject("Test")
        .text_body("Body")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_email_builder_missing_body() {
    let result = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test")
        .build();

    assert!(result.is_err());
}
