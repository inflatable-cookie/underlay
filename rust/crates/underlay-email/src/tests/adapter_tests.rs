    use super::*;
    use crate::types::EmailAddress;

    #[tokio::test]
    async fn test_noop_adapter() {
        let adapter = NoopAdapter::new();
        assert_eq!(adapter.name(), "noop");

        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test")
            .text_body("Hello")
            .build()
            .unwrap();

        let result = adapter.send(&email).await.unwrap();
        assert!(result.success);
        assert_eq!(result.message_id, Some("noop".to_string()));
    }

    #[tokio::test]
    async fn test_noop_adapter_health_check() {
        let adapter = NoopAdapter::new();
        assert!(adapter.health_check().await.is_ok());
    }