    use super::*;

    #[test]
    fn test_parse_uuid_path_valid() {
        let valid_uuid = "01234567-89ab-cdef-0123-456789abcdef";
        let result = parse_uuid_path(valid_uuid, "testId");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.to_string(), valid_uuid);
    }

    #[test]
    fn test_parse_uuid_path_invalid() {
        let invalid = "not-a-uuid";
        let result = parse_uuid_path(invalid, "testId");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_uuid_path_empty() {
        let result = parse_uuid_path("", "testId");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_uuid_path_raw_valid() {
        let valid_uuid = "01234567-89ab-cdef-0123-456789abcdef";
        let result = parse_uuid_path_raw(valid_uuid, "testId");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), valid_uuid);
    }

    #[test]
    fn test_parse_uuid_path_raw_invalid() {
        let result = parse_uuid_path_raw("invalid", "testId");
        assert!(result.is_err());
    }