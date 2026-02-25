    use super::*;

    #[test]
    fn test_default_query() {
        let query = SuggestionQuery::new();
        assert!(query.hint_ids().is_empty());
        assert_eq!(query.limit(), 15);
        assert_eq!(query.order(), SuggestionOrder::HintsThenRecent);
    }

    #[test]
    fn test_with_hints() {
        let query = SuggestionQuery::new().with_hints(vec!["a", "b", "c"]);

        assert_eq!(query.hint_ids(), &["a", "b", "c"]);
        assert!(query.has_hints());
        assert_eq!(query.hint_count(), 3);
    }

    #[test]
    fn test_fill_limit() {
        let query = SuggestionQuery::new()
            .with_hints(vec!["a", "b", "c"])
            .with_limit(10);

        assert_eq!(query.fill_limit(3), 7);
        assert_eq!(query.fill_limit(0), 10);
        assert_eq!(query.fill_limit(10), 0);
        assert_eq!(query.fill_limit(15), 0); // saturating
    }

    #[test]
    fn test_hint_order_sql() {
        let query = SuggestionQuery::new().with_hints(vec!["id1", "id2"]);

        let sql = query.hint_order_sql("level_id");
        assert!(sql.contains("CASE level_id"));
        assert!(sql.contains("WHEN 'id1' THEN 0"));
        assert!(sql.contains("WHEN 'id2' THEN 1"));
    }

    #[test]
    fn test_hint_order_sql_empty() {
        let query = SuggestionQuery::new();
        assert_eq!(query.hint_order_sql("id"), "0");
    }

    #[test]
    fn test_hint_array_sql() {
        let query = SuggestionQuery::new().with_hints(vec!["id1", "id2"]);

        let sql = query.hint_array_sql();
        assert_eq!(sql, "ARRAY['id1','id2']");
    }

    #[test]
    fn test_hint_array_sql_empty() {
        let query = SuggestionQuery::new();
        assert_eq!(query.hint_array_sql(), "ARRAY[]::text[]");
    }

    #[test]
    fn test_sql_injection_prevention() {
        let query = SuggestionQuery::new().with_hints(vec!["id'; DROP TABLE users; --"]);

        let sql = query.hint_order_sql("id");
        assert!(sql.contains("id''; DROP TABLE users; --"));
        assert!(!sql.contains("id'; DROP"));
    }