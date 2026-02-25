    use super::*;

    #[test]
    fn test_field_error_creation() {
        let err = FieldError::new("Invalid email");
        assert_eq!(err.message, "Invalid email");
        assert!(err.code.is_none());

        let err = FieldError::with_code("Invalid email", "email.invalid");
        assert_eq!(err.message, "Invalid email");
        assert_eq!(err.code, Some("email.invalid".to_string()));
    }

    #[test]
    fn test_validation_error_builder() {
        let mut err = ValidationError::new();
        assert!(!err.has_errors());

        err.add_field("email", "Invalid email");
        err.add_field(
            "password",
            FieldError::with_code("Too short", "password.short"),
        );

        assert!(err.has_errors());
        assert_eq!(err.field_count(), 2);
        assert!(err.has_field("email"));
        assert!(err.has_field("password"));
        assert!(!err.has_field("username"));
    }

    #[test]
    fn test_into_result() {
        let err = ValidationError::new();
        assert!(err.into_result().is_ok());

        let mut err = ValidationError::new();
        err.add_field("email", "Invalid");
        assert!(err.into_result().is_err());
    }

    #[test]
    fn test_nested_errors() {
        let mut err = ValidationError::new();
        err.add_nested("user", "email", "Invalid email");
        err.add_nested("user", "name", "Required");

        assert!(err.has_field("user.email"));
        assert!(err.has_field("user.name"));
    }

    #[test]
    fn test_merge() {
        let mut err1 = ValidationError::new();
        err1.add_field("email", "Invalid");

        let mut err2 = ValidationError::new();
        err2.add_field("password", "Too short");

        err1.merge(err2);
        assert_eq!(err1.field_count(), 2);
    }

    #[test]
    fn test_serialization() {
        let mut err = ValidationError::new();
        err.add_field(
            "email",
            FieldError::with_code("Invalid email", "email.invalid"),
        );

        let json = serde_json::to_string(&err.finalize()).unwrap();
        assert!(json.contains("field_errors"));
        assert!(json.contains("email.invalid"));
    }