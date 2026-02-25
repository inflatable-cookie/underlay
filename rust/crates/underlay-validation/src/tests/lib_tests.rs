    use super::*;

    struct TestRequest {
        email: String,
        password: String,
        age: i32,
    }

    impl Validate for TestRequest {
        fn validate(&self) -> ValidationResult<()> {
            let mut errors = ValidationError::new();

            if let Err(e) = validators::email(&self.email) {
                errors.add_field("email", e);
            }

            if let Err(e) = validators::length(&self.password, Some(8), Some(100)) {
                errors.add_field("password", e);
            }

            if let Err(e) = validators::range(self.age, Some(18), Some(120)) {
                errors.add_field("age", e);
            }

            errors.into_result()
        }
    }

    #[test]
    fn test_valid_request() {
        let req = TestRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let req = TestRequest {
            email: "not-an-email".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        let err = req.validate().unwrap_err();
        assert!(err.has_field("email"));
        assert!(!err.has_field("password"));
    }

    #[test]
    fn test_multiple_errors() {
        let req = TestRequest {
            email: "bad".to_string(),
            password: "short".to_string(),
            age: 10,
        };

        let err = req.validate().unwrap_err();
        assert!(err.has_field("email"));
        assert!(err.has_field("password"));
        assert!(err.has_field("age"));
        assert_eq!(err.field_count(), 3);
    }

    #[test]
    fn test_validated_method() {
        let req = TestRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            age: 25,
        };

        let result = req.validated();
        assert!(result.is_ok());
    }