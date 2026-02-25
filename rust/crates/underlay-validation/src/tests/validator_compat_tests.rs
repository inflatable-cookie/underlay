    use super::*;

    #[test]
    fn test_validation_to_app_error_creates_correct_structure() {
        let mut errors = validator::ValidationErrors::new();
        let mut field_error = validator::ValidationError::new("required");
        field_error.message = Some("Name is required".into());
        errors.add("name", field_error);

        let app_error =
            validation_to_app_error(&errors, "test.validation_failed", "Validation failed");

        assert_eq!(app_error.code, "test.validation_failed");
        assert_eq!(app_error.message, "Validation failed");
        assert!(app_error.field_errors.is_some());

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(
            field_errors.get("name"),
            Some(&"Name is required".to_string())
        );
    }

    #[test]
    fn test_validation_to_app_error_default_message() {
        let mut errors = validator::ValidationErrors::new();
        let field_error = validator::ValidationError::new("length");
        errors.add("email", field_error);

        let app_error =
            validation_to_app_error(&errors, "test.validation_failed", "Validation failed");

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(
            field_errors.get("email"),
            Some(&"Invalid value".to_string())
        );
    }

    #[test]
    fn test_validation_to_app_error_multiple_fields() {
        let mut errors = validator::ValidationErrors::new();

        let mut name_error = validator::ValidationError::new("required");
        name_error.message = Some("Name is required".into());
        errors.add("name", name_error);

        let mut email_error = validator::ValidationError::new("email");
        email_error.message = Some("Invalid email format".into());
        errors.add("email", email_error);

        let app_error = validation_to_app_error(
            &errors,
            "user.invalid",
            "There is a problem with one or more fields.",
        );

        let field_errors = app_error.field_errors.unwrap();
        assert_eq!(field_errors.len(), 2);
        assert_eq!(
            field_errors.get("name"),
            Some(&"Name is required".to_string())
        );
        assert_eq!(
            field_errors.get("email"),
            Some(&"Invalid email format".to_string())
        );
    }