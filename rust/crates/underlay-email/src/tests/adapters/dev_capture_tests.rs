    use super::*;
    use crate::types::EmailAddress;

    fn create_test_email(to: &str) -> Email {
        Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new(to).unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .build()
            .unwrap()
    }

    #[test]
    fn test_whitelist_lookup() {
        let config = DevCaptureConfig {
            whitelist: vec![
                "dev@example.com".to_string(),
                "test@example.com".to_string(),
            ],
            use_fallback: true,
        };
        let lookup = WhitelistLookup::from_config(&config);

        assert!(lookup.is_whitelisted("dev@example.com"));
        assert!(lookup.is_whitelisted("test@example.com"));
        assert!(!lookup.is_whitelisted("other@example.com"));
    }

    #[test]
    fn test_whitelist_any_whitelisted() {
        let config = DevCaptureConfig {
            whitelist: vec!["dev@example.com".to_string()],
            use_fallback: true,
        };
        let lookup = WhitelistLookup::from_config(&config);

        assert!(lookup.any_whitelisted(["other@example.com", "dev@example.com"]));
        assert!(!lookup.any_whitelisted(["other@example.com", "another@example.com"]));
    }

    #[test]
    fn test_captured_email_from_email() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .cc(EmailAddress::new("cc@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello")
            .html_body("<p>Hello</p>")
            .build()
            .unwrap();

        let captured = CapturedEmail::from_email(&email);

        assert_eq!(captured.email_id, email.id);
        assert_eq!(captured.from_address, "sender@example.com");
        assert_eq!(captured.to_addresses, vec!["recipient@example.com"]);
        assert_eq!(captured.cc_addresses, vec!["cc@example.com"]);
        assert_eq!(captured.subject, "Test Subject");
        assert_eq!(captured.text_body, Some("Hello".to_string()));
        assert_eq!(captured.html_body, Some("<p>Hello</p>".to_string()));
        assert!(!captured.was_delivered);
        assert!(captured.delivery_error.is_none());
    }

    #[tokio::test]
    async fn test_dev_capture_adapter_capture_only() {
        let store = Arc::new(InMemoryEmailStore::new());
        let adapter = DevCaptureAdapter::capture_only(store.clone());

        let email = create_test_email("recipient@example.com");
        let result = adapter.send(&email).await.unwrap();

        assert!(result.success);
        assert_eq!(result.message_id, Some("dev_capture".to_string()));
        assert_eq!(store.count(), 1);

        let captured = &store.emails()[0];
        assert!(!captured.was_delivered);
    }

    #[tokio::test]
    async fn test_dev_capture_adapter_with_whitelist_no_match() {
        let store = Arc::new(InMemoryEmailStore::new());
        let config = DevCaptureConfig {
            whitelist: vec!["dev@example.com".to_string()],
            use_fallback: true,
        };
        let adapter = DevCaptureAdapter::new(store.clone(), config, None);

        let email = create_test_email("other@example.com");
        let result = adapter.send(&email).await.unwrap();

        assert!(result.success);
        assert_eq!(store.count(), 1);

        let captured = &store.emails()[0];
        assert!(!captured.was_delivered);
    }

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryEmailStore::new();
        assert_eq!(store.count(), 0);

        let email = create_test_email("test@example.com");
        let captured = CapturedEmail::from_email(&email);
        store.store(captured).await.unwrap();

        assert_eq!(store.count(), 1);

        store.clear();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_captured_email_mark_delivered() {
        let email = create_test_email("test@example.com");
        let mut captured = CapturedEmail::from_email(&email);

        assert!(!captured.was_delivered);
        captured.mark_delivered();
        assert!(captured.was_delivered);
    }

    #[test]
    fn test_captured_email_mark_delivery_failed() {
        let email = create_test_email("test@example.com");
        let mut captured = CapturedEmail::from_email(&email);

        captured.mark_delivery_failed("Connection refused");
        assert!(!captured.was_delivered);
        assert_eq!(
            captured.delivery_error,
            Some("Connection refused".to_string())
        );
    }