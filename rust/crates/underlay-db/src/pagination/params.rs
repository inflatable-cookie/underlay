use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::cursor::Cursor;
use super::errors::CursorError;

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
    /// Page number for offset-based list consumers.
    #[serde(default = "default_page")]
    pub page: i64,

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

fn default_page() -> i64 {
    1
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
            page: 1,
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

    /// Set the 1-indexed page number for offset-based consumers.
    pub fn with_page(mut self, page: i64) -> Self {
        self.page = page;
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

    /// Get the effective 1-indexed page number.
    pub fn effective_page(&self) -> i64 {
        self.page.max(1)
    }

    /// Get the SQL OFFSET for page-based consumers.
    pub fn offset(&self) -> i64 {
        (self.effective_page() - 1) * self.effective_limit()
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
