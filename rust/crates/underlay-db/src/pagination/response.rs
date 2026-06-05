use serde::{Deserialize, Serialize};

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
