use serde::{Deserialize, Serialize};

/// Query parameters for pagination
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,

    /// Items per page
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

/// Default maximum limit for pagination
pub const DEFAULT_MAX_LIMIT: u32 = 100;

impl PaginationParams {
    /// Calculate the offset for database queries
    ///
    /// # Example
    /// ```
    /// use underlay_http::pagination::PaginationParams;
    ///
    /// let params = PaginationParams { page: 2, limit: 20 };
    /// assert_eq!(params.offset(), 20);
    /// ```
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)).saturating_mul(self.limit)
    }

    /// Get limit as i64 for SQL binding
    pub fn limit_i64(&self) -> i64 {
        self.limit as i64
    }

    /// Get offset as i64 for SQL binding
    pub fn offset_i64(&self) -> i64 {
        self.offset() as i64
    }

    /// Ensure limit doesn't exceed maximum
    pub fn with_max_limit(mut self, max: u32) -> Self {
        if self.limit > max {
            self.limit = max;
        }
        self
    }

    /// Clamp limit to default maximum (100) and return self
    pub fn clamped(self) -> Self {
        self.with_max_limit(DEFAULT_MAX_LIMIT)
    }

    /// Generate SQL LIMIT OFFSET clause
    ///
    /// # Example
    /// ```
    /// use underlay_http::pagination::PaginationParams;
    ///
    /// let params = PaginationParams { page: 2, limit: 20 };
    /// assert_eq!(params.sql_clause(), "LIMIT 20 OFFSET 20");
    /// ```
    pub fn sql_clause(&self) -> String {
        format!("LIMIT {} OFFSET {}", self.limit, self.offset())
    }

    /// Generate SQL LIMIT OFFSET clause with parameter placeholders
    ///
    /// # Arguments
    /// * `limit_idx` - The parameter index for LIMIT (e.g., 1 for $1)
    /// * `offset_idx` - The parameter index for OFFSET (e.g., 2 for $2)
    ///
    /// # Example
    /// ```
    /// use underlay_http::pagination::PaginationParams;
    ///
    /// let params = PaginationParams { page: 1, limit: 20 };
    /// assert_eq!(params.sql_clause_params(3, 4), "LIMIT $3 OFFSET $4");
    /// ```
    pub fn sql_clause_params(&self, limit_idx: u32, offset_idx: u32) -> String {
        format!("LIMIT ${} OFFSET ${}", limit_idx, offset_idx)
    }

    /// Wrap data and total count into a paginated response
    ///
    /// # Example
    /// ```
    /// use underlay_http::pagination::{PaginationParams, Paginated};
    ///
    /// let params = PaginationParams { page: 1, limit: 20 };
    /// let data = vec![1, 2, 3];
    /// let response: Paginated<i32> = params.wrap(data, 45);
    ///
    /// assert_eq!(response.pagination.total, 45);
    /// assert_eq!(response.pagination.total_pages, 3);
    /// ```
    pub fn wrap<T>(self, data: Vec<T>, total: u64) -> Paginated<T> {
        Paginated {
            data,
            pagination: PaginationMeta {
                page: self.page,
                limit: self.limit,
                total,
                total_pages: if self.limit == 0 {
                    0
                } else {
                    (total as f64 / self.limit as f64).ceil() as u32
                },
            },
        }
    }

    /// Wrap data with i64 total (common from COUNT(*) queries)
    pub fn wrap_i64<T>(self, data: Vec<T>, total: i64) -> Paginated<T> {
        self.wrap(data, total.max(0) as u64)
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize)]
pub struct Paginated<T> {
    /// The data items for this page
    pub data: Vec<T>,

    /// Pagination metadata
    pub pagination: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    /// Current page number (1-indexed)
    pub page: u32,

    /// Items per page
    pub limit: u32,

    /// Total number of items across all pages
    pub total: u64,

    /// Total number of pages
    pub total_pages: u32,
}

#[cfg(test)]
mod tests {
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
}
