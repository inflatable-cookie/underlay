use super::cursor::Cursor;
use super::errors::CursorError;
use super::params::{CursorPaginationParams, PaginationDirection};
use super::response::PaginatedResponse;
use super::typed_cursors::{TimestampCursor, WeightCursor};

/// Helper for building paginated queries with a common pattern.
///
/// This is a convenience struct for the common case of paginating by
/// (weight, id) or (created_at, id).
#[derive(Debug, Clone)]
pub struct PaginationBuilder {
    params: CursorPaginationParams,
}

impl PaginationBuilder {
    /// Create a new pagination builder from params.
    pub fn new(params: CursorPaginationParams) -> Self {
        Self { params }
    }

    /// Get the effective LIMIT value (limit + 1 to detect has_more).
    pub fn query_limit(&self) -> i64 {
        self.params.effective_limit() + 1
    }

    /// Get the SQL OFFSET value for page-number pagination.
    pub fn query_offset(&self) -> i64 {
        self.params.offset()
    }

    /// Get the page size (without the +1).
    pub fn page_size(&self) -> i64 {
        self.params.effective_limit()
    }

    /// Check if we should include total count.
    pub fn should_count(&self) -> bool {
        self.params.include_total
    }

    /// Get the direction.
    pub fn direction(&self) -> PaginationDirection {
        self.params.direction
    }

    /// Decode cursor for weight-based pagination.
    pub fn decode_weight_cursor(&self) -> Result<Option<WeightCursor>, CursorError> {
        self.params.decode_cursor()
    }

    /// Decode cursor for timestamp-based pagination.
    pub fn decode_timestamp_cursor(&self) -> Result<Option<TimestampCursor>, CursorError> {
        self.params.decode_cursor()
    }

    /// Check if a cursor is present.
    pub fn has_cursor(&self) -> bool {
        self.params.cursor.is_some()
    }

    /// Get the comparison operator for keyset pagination.
    ///
    /// For descending order (most common), returns `<` for forward pagination.
    /// For ascending order, returns `>` for forward pagination.
    ///
    /// # Arguments
    /// * `descending` - Whether the primary sort is descending (true for most recent first)
    ///
    /// # Example
    /// ```rust,ignore
    /// let op = builder.keyset_operator(true); // "<" for forward, ">" for backward
    /// let sql = format!("WHERE (updated_at, id) {} ($1, $2)", op);
    /// ```
    pub fn keyset_operator(&self, descending: bool) -> &'static str {
        match (self.params.direction, descending) {
            (PaginationDirection::Forward, true) => "<",
            (PaginationDirection::Forward, false) => ">",
            (PaginationDirection::Backward, true) => ">",
            (PaginationDirection::Backward, false) => "<",
        }
    }

    /// Generate the keyset WHERE clause fragment for a two-column cursor.
    ///
    /// Returns a SQL fragment like `(col, id) < ($1, $2)` suitable for keyset pagination.
    ///
    /// # Arguments
    /// * `column` - The primary sort column name (e.g., "updated_at", "weight").
    ///   Must be a valid [`SqlIdentifier`](crate::identifiers::SqlIdentifier);
    ///   pass static strings only.
    /// * `param_offset` - Starting parameter number (1-indexed for PostgreSQL)
    /// * `descending` - Whether the primary sort is descending
    ///
    /// # Panics
    /// Panics if `column` is not a valid SQL identifier.
    ///
    /// # Example
    /// ```rust,ignore
    /// // For a query with existing WHERE clause
    /// let keyset = builder.keyset_condition("updated_at", 1, true);
    /// // Returns: "(updated_at, id) < ($1, $2)"
    ///
    /// let sql = format!(
    ///     "SELECT * FROM items WHERE deleted_at IS NULL AND {} LIMIT $3",
    ///     keyset
    /// );
    /// ```
    pub fn keyset_condition(&self, column: &str, param_offset: usize, descending: bool) -> String {
        let op = self.keyset_operator(descending);
        let column = crate::identifiers::SqlIdentifier::parse(column)
            .expect("keyset_condition column must be a valid SQL identifier");
        format!(
            "({}, id) {} (${}, ${})",
            column,
            op,
            param_offset,
            param_offset + 1
        )
    }

    /// Get the ORDER BY direction string.
    ///
    /// # Arguments
    /// * `descending` - Whether the base sort order is descending
    ///
    /// Returns "DESC" or "ASC" adjusted for pagination direction.
    pub fn order_direction(&self, descending: bool) -> &'static str {
        match (self.params.direction, descending) {
            (PaginationDirection::Forward, true) => "DESC",
            (PaginationDirection::Forward, false) => "ASC",
            (PaginationDirection::Backward, true) => "ASC",
            (PaginationDirection::Backward, false) => "DESC",
        }
    }

    /// Generate the ORDER BY clause for a two-column keyset.
    ///
    /// # Arguments
    /// * `column` - The primary sort column name. Must be a valid
    ///   [`SqlIdentifier`](crate::identifiers::SqlIdentifier); pass static
    ///   strings only.
    /// * `descending` - Whether the primary sort is descending
    ///
    /// # Panics
    /// Panics if `column` is not a valid SQL identifier.
    ///
    /// # Example
    /// ```rust,ignore
    /// let order = builder.keyset_order_by("updated_at", true);
    /// // Returns: "updated_at DESC, id DESC"
    /// ```
    pub fn keyset_order_by(&self, column: &str, descending: bool) -> String {
        let dir = self.order_direction(descending);
        let column = crate::identifiers::SqlIdentifier::parse(column)
            .expect("keyset_order_by column must be a valid SQL identifier");
        format!("{} {}, id {}", column, dir, dir)
    }

    /// Build response from fetched items.
    ///
    /// Items should be fetched with `query_limit()` (limit + 1).
    /// This method will trim to the actual page size and set has_more.
    pub fn build_response<T, F>(
        &self,
        mut items: Vec<T>,
        total: Option<i64>,
        cursor_fn: F,
    ) -> PaginatedResponse<T>
    where
        F: Fn(&T) -> Cursor,
    {
        let page_size = self.page_size() as usize;
        let has_more = items.len() > page_size;

        // Trim to actual page size
        if has_more {
            items.truncate(page_size);
        }

        // Build cursors
        let next_cursor = if has_more {
            items.last().map(|item| cursor_fn(item).encode())
        } else {
            None
        };

        let prev_cursor = if self.params.cursor.is_some() {
            items.first().map(|item| cursor_fn(item).encode())
        } else {
            None
        };

        PaginatedResponse {
            data: items,
            next_cursor,
            prev_cursor,
            has_more,
            total,
        }
    }
}
