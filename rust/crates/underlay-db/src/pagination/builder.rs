use super::cursor::Cursor;
use super::errors::CursorError;
use super::params::{CursorPaginationParams, PaginationDirection};
use super::response::PaginatedResponse;
use super::typed_cursors::{TimestampCursor, WeightCursor};
use crate::identifiers::{IdentifierError, SqlIdentifier};

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
    /// Returns a SQL fragment like `("col", id) < ($1, $2)` suitable for keyset
    /// pagination. The sort column is validated and quoted as a
    /// [`SqlIdentifier`], so an invalid column name is an error rather than a
    /// panic or an injection path.
    ///
    /// # Arguments
    /// * `column` - The primary sort column name (e.g., "updated_at", "weight").
    ///   Must be a valid [`SqlIdentifier`](crate::identifiers::SqlIdentifier).
    /// * `param_offset` - Starting parameter number (1-indexed for PostgreSQL)
    /// * `descending` - Whether the primary sort is descending
    ///
    /// # Errors
    /// Returns [`IdentifierError`] when `column` is not a valid SQL identifier.
    ///
    /// # Example
    /// ```rust,ignore
    /// // For a query with existing WHERE clause
    /// let keyset = builder.keyset_condition("updated_at", 1, true)?;
    /// // Returns: `("updated_at", id) < ($1, $2)`
    ///
    /// let sql = format!(
    ///     "SELECT * FROM items WHERE deleted_at IS NULL AND {} LIMIT $3",
    ///     keyset
    /// );
    /// ```
    pub fn keyset_condition(
        &self,
        column: &str,
        param_offset: usize,
        descending: bool,
    ) -> Result<String, IdentifierError> {
        let op = self.keyset_operator(descending);
        let column = SqlIdentifier::parse(column)?;
        Ok(format!(
            "({}, id) {} (${}, ${})",
            column.quoted(),
            op,
            param_offset,
            param_offset + 1
        ))
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
    /// The sort column is validated and quoted as a [`SqlIdentifier`], so an
    /// invalid column name is an error rather than a panic or an injection
    /// path.
    ///
    /// # Arguments
    /// * `column` - The primary sort column name. Must be a valid
    ///   [`SqlIdentifier`](crate::identifiers::SqlIdentifier).
    /// * `descending` - Whether the primary sort is descending
    ///
    /// # Errors
    /// Returns [`IdentifierError`] when `column` is not a valid SQL identifier.
    ///
    /// # Example
    /// ```rust,ignore
    /// let order = builder.keyset_order_by("updated_at", true)?;
    /// // Returns: `"updated_at" DESC, id DESC`
    /// ```
    pub fn keyset_order_by(
        &self,
        column: &str,
        descending: bool,
    ) -> Result<String, IdentifierError> {
        let dir = self.order_direction(descending);
        let column = SqlIdentifier::parse(column)?;
        Ok(format!("{} {}, id {}", column.quoted(), dir, dir))
    }

    /// Build response from fetched items.
    ///
    /// Items should be fetched with `query_limit()` (`limit + 1`).
    /// This method trims to the actual page size and sets `has_more` to
    /// indicate whether more items exist in the requested traversal direction
    /// (`next_cursor` for `Forward`, `prev_cursor` for `Backward`).
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

        // Backward queries flip ORDER BY in SQL, so restore forward display order.
        let is_backward = self.params.direction == PaginationDirection::Backward;
        if is_backward {
            items.reverse();
        }

        // Build cursors
        let (next_cursor, prev_cursor) = if is_backward {
            let next = if self.params.cursor.is_some() {
                items.last().map(|item| cursor_fn(item).encode())
            } else {
                None
            };
            let prev = if has_more {
                items.first().map(|item| cursor_fn(item).encode())
            } else {
                None
            };
            (next, prev)
        } else {
            let next = if has_more {
                items.last().map(|item| cursor_fn(item).encode())
            } else {
                None
            };
            let prev = if self.params.cursor.is_some() {
                items.first().map(|item| cursor_fn(item).encode())
            } else {
                None
            };
            (next, prev)
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
