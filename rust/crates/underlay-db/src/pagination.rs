//! Cursor-based pagination utilities for database queries.
//!
//! This module provides types and helpers for implementing efficient keyset pagination
//! that scales to millions of rows with consistent performance.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_db::pagination::{PaginationParams, PaginatedResponse, Cursor};
//!
//! // Parse pagination params from query string
//! let params = PaginationParams::default();
//!
//! // Build cursor for keyset pagination
//! let cursor = Cursor::new()
//!     .with_value("weight", 5)
//!     .with_id(some_uuid);
//!
//! // Return paginated response
//! let response = PaginatedResponse {
//!     data: items,
//!     next_cursor: Some(cursor.encode()),
//!     prev_cursor: None,
//!     has_more: true,
//!     total: Some(1234),
//! };
//! ```

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Default number of items per page.
pub const DEFAULT_PAGE_SIZE: i64 = 30;

/// Maximum allowed page size.
pub const MAX_PAGE_SIZE: i64 = 100;

/// Direction of pagination traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaginationDirection {
    #[default]
    Forward,
    Backward,
}

/// Pagination parameters extracted from query string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginationParams {
    /// Number of items per page (default: 50, max: 100).
    #[serde(default = "default_limit")]
    pub limit: i64,

    /// Opaque cursor string for position.
    #[serde(default)]
    pub cursor: Option<String>,

    /// Direction of traversal (default: forward).
    #[serde(default)]
    pub direction: PaginationDirection,

    /// Whether to include total count (default: true).
    #[serde(default = "default_include_total")]
    pub include_total: bool,
}

fn default_limit() -> i64 {
    DEFAULT_PAGE_SIZE
}

fn default_include_total() -> bool {
    true
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: DEFAULT_PAGE_SIZE,
            cursor: None,
            direction: PaginationDirection::Forward,
            include_total: true,
        }
    }
}

impl PaginationParams {
    /// Create new pagination params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page size limit.
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    /// Set the cursor.
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Set the direction.
    pub fn with_direction(mut self, direction: PaginationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set whether to include total count.
    pub fn with_include_total(mut self, include: bool) -> Self {
        self.include_total = include;
        self
    }

    /// Get the effective limit, clamped to valid range.
    pub fn effective_limit(&self) -> i64 {
        self.limit.clamp(1, MAX_PAGE_SIZE)
    }

    /// Decode the cursor if present.
    pub fn decode_cursor<T: DeserializeOwned>(&self) -> Result<Option<T>, CursorError> {
        match &self.cursor {
            Some(encoded) => {
                let cursor = Cursor::decode(encoded)?;
                Ok(Some(cursor.into_value()?))
            }
            None => Ok(None),
        }
    }
}

/// A paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginatedResponse<T> {
    /// The items for the current page.
    pub data: Vec<T>,

    /// Cursor to fetch the next page, or null if no more pages.
    pub next_cursor: Option<String>,

    /// Cursor to fetch the previous page, or null if at the start.
    pub prev_cursor: Option<String>,

    /// Whether there are more items after this page.
    pub has_more: bool,

    /// Total count of items (null if not requested or unavailable).
    pub total: Option<i64>,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response.
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            next_cursor: None,
            prev_cursor: None,
            has_more: false,
            total: None,
        }
    }

    /// Set the next cursor.
    pub fn with_next_cursor(mut self, cursor: Option<String>) -> Self {
        self.next_cursor = cursor;
        self
    }

    /// Set the previous cursor.
    pub fn with_prev_cursor(mut self, cursor: Option<String>) -> Self {
        self.prev_cursor = cursor;
        self
    }

    /// Set has_more flag.
    pub fn with_has_more(mut self, has_more: bool) -> Self {
        self.has_more = has_more;
        self
    }

    /// Set total count.
    pub fn with_total(mut self, total: Option<i64>) -> Self {
        self.total = total;
        self
    }

    /// Map the data to a different type.
    pub fn map<U, F>(self, f: F) -> PaginatedResponse<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResponse {
            data: self.data.into_iter().map(f).collect(),
            next_cursor: self.next_cursor,
            prev_cursor: self.prev_cursor,
            has_more: self.has_more,
            total: self.total,
        }
    }
}

/// Error type for cursor operations.
#[derive(Debug, Clone)]
pub enum CursorError {
    /// Base64 decoding failed.
    DecodeError(String),
    /// JSON parsing failed.
    ParseError(String),
    /// Missing required field in cursor.
    MissingField(String),
    /// Invalid field type in cursor.
    InvalidType(String),
}

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorError::DecodeError(msg) => write!(f, "Cursor decode error: {}", msg),
            CursorError::ParseError(msg) => write!(f, "Cursor parse error: {}", msg),
            CursorError::MissingField(field) => write!(f, "Cursor missing field: {}", field),
            CursorError::InvalidType(msg) => write!(f, "Cursor invalid type: {}", msg),
        }
    }
}

impl std::error::Error for CursorError {}

/// A cursor for keyset pagination.
///
/// Cursors encode the sort column values and a tiebreaker ID to enable
/// efficient keyset pagination that works consistently at any depth.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// The cursor values as a JSON map.
    #[serde(flatten)]
    values: HashMap<String, serde_json::Value>,
}

impl Cursor {
    /// Create a new empty cursor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an integer value to the cursor.
    pub fn with_int(mut self, key: impl Into<String>, value: i64) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::Number(value.into()));
        self
    }

    /// Add a string value to the cursor.
    pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.into()));
        self
    }

    /// Add a UUID value to the cursor (stored as string).
    pub fn with_uuid(mut self, key: impl Into<String>, value: Uuid) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.to_string()));
        self
    }

    /// Add the tiebreaker ID (convenience for common pattern).
    pub fn with_id(self, id: Uuid) -> Self {
        self.with_uuid("id", id)
    }

    /// Add a weight value (convenience for common pattern).
    pub fn with_weight(self, weight: i32) -> Self {
        self.with_int("w", weight as i64)
    }

    /// Add a timestamp value (stored as ISO string).
    pub fn with_timestamp(
        mut self,
        key: impl Into<String>,
        value: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.to_rfc3339()));
        self
    }

    /// Get an integer value from the cursor.
    pub fn get_int(&self, key: &str) -> Result<i64, CursorError> {
        self.values
            .get(key)
            .ok_or_else(|| CursorError::MissingField(key.to_string()))?
            .as_i64()
            .ok_or_else(|| CursorError::InvalidType(format!("{} is not an integer", key)))
    }

    /// Get a string value from the cursor.
    pub fn get_string(&self, key: &str) -> Result<&str, CursorError> {
        self.values
            .get(key)
            .ok_or_else(|| CursorError::MissingField(key.to_string()))?
            .as_str()
            .ok_or_else(|| CursorError::InvalidType(format!("{} is not a string", key)))
    }

    /// Get a UUID value from the cursor.
    pub fn get_uuid(&self, key: &str) -> Result<Uuid, CursorError> {
        let s = self.get_string(key)?;
        Uuid::parse_str(s)
            .map_err(|e| CursorError::InvalidType(format!("{} is not a valid UUID: {}", key, e)))
    }

    /// Get the tiebreaker ID (convenience for common pattern).
    pub fn get_id(&self) -> Result<Uuid, CursorError> {
        self.get_uuid("id")
    }

    /// Get weight value (convenience for common pattern).
    pub fn get_weight(&self) -> Result<i32, CursorError> {
        self.get_int("w").map(|v| v as i32)
    }

    /// Get a timestamp value from the cursor.
    pub fn get_timestamp(&self, key: &str) -> Result<chrono::DateTime<chrono::Utc>, CursorError> {
        let s = self.get_string(key)?;
        chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .map_err(|e| {
                CursorError::InvalidType(format!("{} is not a valid timestamp: {}", key, e))
            })
    }

    /// Encode the cursor to a URL-safe string.
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(&self.values).unwrap_or_default();
        URL_SAFE_NO_PAD.encode(json.as_bytes())
    }

    /// Decode a cursor from a URL-safe string.
    pub fn decode(encoded: &str) -> Result<Self, CursorError> {
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|e| CursorError::DecodeError(e.to_string()))?;

        let json = String::from_utf8(bytes).map_err(|e| CursorError::DecodeError(e.to_string()))?;

        let values: HashMap<String, serde_json::Value> =
            serde_json::from_str(&json).map_err(|e| CursorError::ParseError(e.to_string()))?;

        Ok(Self { values })
    }

    /// Convert the cursor into a typed value.
    pub fn into_value<T: DeserializeOwned>(self) -> Result<T, CursorError> {
        let json = serde_json::Value::Object(self.values.into_iter().collect());
        serde_json::from_value(json).map_err(|e| CursorError::ParseError(e.to_string()))
    }
}

/// Helper for building paginated queries with a common pattern.
///
/// This is a convenience struct for the common case of paginating by
/// (weight, id) or (created_at, id).
#[derive(Debug, Clone)]
pub struct PaginationBuilder {
    params: PaginationParams,
}

impl PaginationBuilder {
    /// Create a new pagination builder from params.
    pub fn new(params: PaginationParams) -> Self {
        Self { params }
    }

    /// Get the effective LIMIT value (limit + 1 to detect has_more).
    pub fn query_limit(&self) -> i64 {
        self.params.effective_limit() + 1
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

    // ========================================================================
    // Query Builder Helpers
    // ========================================================================

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
    /// * `column` - The primary sort column name (e.g., "updated_at", "weight")
    /// * `param_offset` - Starting parameter number (1-indexed for PostgreSQL)
    /// * `descending` - Whether the primary sort is descending
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
    /// * `column` - The primary sort column name
    /// * `descending` - Whether the primary sort is descending
    ///
    /// # Example
    /// ```rust,ignore
    /// let order = builder.keyset_order_by("updated_at", true);
    /// // Returns: "updated_at DESC, id DESC"
    /// ```
    pub fn keyset_order_by(&self, column: &str, descending: bool) -> String {
        let dir = self.order_direction(descending);
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

/// Cursor for weight-based pagination (common pattern).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightCursor {
    /// Weight value.
    #[serde(rename = "w")]
    pub weight: i32,
    /// Tiebreaker ID.
    pub id: Uuid,
}

impl WeightCursor {
    /// Create a new weight cursor.
    pub fn new(weight: i32, id: Uuid) -> Self {
        Self { weight, id }
    }

    /// Encode to cursor string.
    pub fn encode(&self) -> String {
        Cursor::new()
            .with_weight(self.weight)
            .with_id(self.id)
            .encode()
    }
}

/// Cursor for timestamp-based pagination (common pattern).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampCursor {
    /// Timestamp value.
    #[serde(rename = "t")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Tiebreaker ID.
    pub id: Uuid,
}

impl TimestampCursor {
    /// Create a new timestamp cursor.
    pub fn new(timestamp: chrono::DateTime<chrono::Utc>, id: Uuid) -> Self {
        Self { timestamp, id }
    }

    /// Encode to cursor string.
    pub fn encode(&self) -> String {
        Cursor::new()
            .with_timestamp("t", self.timestamp)
            .with_id(self.id)
            .encode()
    }
}

#[cfg(test)]
#[path = "pagination_tests.rs"]
mod tests;
