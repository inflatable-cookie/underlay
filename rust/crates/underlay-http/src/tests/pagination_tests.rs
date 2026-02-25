    use super::*;

    #[test]
    fn test_offset_calculation() {
        let params = PaginationParams { page: 1, limit: 20 };
        assert_eq!(params.offset(), 0);

        let params = PaginationParams { page: 2, limit: 20 };
        assert_eq!(params.offset(), 20);

        let params = PaginationParams { page: 3, limit: 50 };
        assert_eq!(params.offset(), 100);
    }

    #[test]
    fn test_default_values() {
        let params = PaginationParams::default();
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 20);
    }

    #[test]
    fn test_max_limit() {
        let params = PaginationParams {
            page: 1,
            limit: 200,
        };
        let limited = params.with_max_limit(100);
        assert_eq!(limited.limit, 100);

        let params = PaginationParams { page: 1, limit: 50 };
        let limited = params.with_max_limit(100);
        assert_eq!(limited.limit, 50);
    }

    #[test]
    fn test_wrap_creates_paginated_response() {
        let params = PaginationParams { page: 1, limit: 20 };
        let data = vec![1, 2, 3, 4, 5];
        let response = params.wrap(data, 45);

        assert_eq!(response.data.len(), 5);
        assert_eq!(response.pagination.page, 1);
        assert_eq!(response.pagination.limit, 20);
        assert_eq!(response.pagination.total, 45);
        assert_eq!(response.pagination.total_pages, 3);
    }

    #[test]
    fn test_total_pages_calculation() {
        let params = PaginationParams { page: 1, limit: 20 };

        // Exact multiple
        let response: Paginated<()> = params.clone().wrap(vec![], 60);
        assert_eq!(response.pagination.total_pages, 3);

        // Partial page
        let response: Paginated<()> = params.clone().wrap(vec![], 61);
        assert_eq!(response.pagination.total_pages, 4);

        // Empty
        let response: Paginated<()> = params.clone().wrap(vec![], 0);
        assert_eq!(response.pagination.total_pages, 0);
    }

    #[test]
    fn test_limit_i64() {
        let params = PaginationParams { page: 1, limit: 20 };
        assert_eq!(params.limit_i64(), 20i64);

        let params = PaginationParams {
            page: 1,
            limit: 100,
        };
        assert_eq!(params.limit_i64(), 100i64);
    }

    #[test]
    fn test_offset_i64() {
        let params = PaginationParams { page: 1, limit: 20 };
        assert_eq!(params.offset_i64(), 0i64);

        let params = PaginationParams { page: 3, limit: 25 };
        assert_eq!(params.offset_i64(), 50i64);
    }

    #[test]
    fn test_sql_clause() {
        let params = PaginationParams { page: 1, limit: 20 };
        assert_eq!(params.sql_clause(), "LIMIT 20 OFFSET 0");

        let params = PaginationParams { page: 2, limit: 50 };
        assert_eq!(params.sql_clause(), "LIMIT 50 OFFSET 50");

        let params = PaginationParams { page: 5, limit: 10 };
        assert_eq!(params.sql_clause(), "LIMIT 10 OFFSET 40");
    }

    #[test]
    fn test_sql_clause_params() {
        let params = PaginationParams { page: 1, limit: 20 };
        assert_eq!(params.sql_clause_params(1, 2), "LIMIT $1 OFFSET $2");
        assert_eq!(params.sql_clause_params(5, 6), "LIMIT $5 OFFSET $6");
    }

    #[test]
    fn test_clamped() {
        // Under limit - unchanged
        let params = PaginationParams { page: 1, limit: 50 };
        let clamped = params.clamped();
        assert_eq!(clamped.limit, 50);

        // Over limit - clamped to 100
        let params = PaginationParams {
            page: 1,
            limit: 500,
        };
        let clamped = params.clamped();
        assert_eq!(clamped.limit, DEFAULT_MAX_LIMIT);
        assert_eq!(clamped.limit, 100);

        // Exactly at limit - unchanged
        let params = PaginationParams {
            page: 1,
            limit: 100,
        };
        let clamped = params.clamped();
        assert_eq!(clamped.limit, 100);
    }

    #[test]
    fn test_wrap_i64() {
        let params = PaginationParams { page: 1, limit: 20 };
        let data = vec!["a", "b", "c"];

        // Positive i64 total
        let response = params.clone().wrap_i64(data.clone(), 100i64);
        assert_eq!(response.pagination.total, 100u64);
        assert_eq!(response.pagination.total_pages, 5);

        // Negative i64 total (edge case) - should clamp to 0
        let response = params.clone().wrap_i64(data.clone(), -5i64);
        assert_eq!(response.pagination.total, 0u64);
        assert_eq!(response.pagination.total_pages, 0);

        // Zero
        let response = params.wrap_i64(data, 0i64);
        assert_eq!(response.pagination.total, 0u64);
    }

    #[test]
    fn test_default_max_limit_constant() {
        assert_eq!(DEFAULT_MAX_LIMIT, 100);
    }