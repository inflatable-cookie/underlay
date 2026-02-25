    use super::*;

    #[test]
    fn filters_builder() {
        let filters = AuditLogFilters::new()
            .with_action("create")
            .with_resource_type("pathway")
            .with_pagination(100, 50);

        assert_eq!(filters.action, Some("create".to_string()));
        assert_eq!(filters.resource_type, Some("pathway".to_string()));
        assert_eq!(filters.limit, 100);
        assert_eq!(filters.offset, 50);
    }