    use crate::validate_table_name;

    #[test]
    fn valid_table_names() {
        assert!(validate_table_name("platform.audit_log").is_ok());
        assert!(validate_table_name("audit_log").is_ok());
    }

    #[test]
    fn invalid_table_names() {
        assert!(validate_table_name("audit; DROP TABLE users").is_err());
        assert!(validate_table_name("audit-log").is_err());
    }