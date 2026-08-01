//! Canonical admin list envelope and unified list query extractor.
//!
//! Wire shape (contract 115): `{ data, total, has_more }`. Every consumer
//! used to redefine this (`PageListResponseDto` etc.); it lives here once.

use serde::Serialize;

use super::pagination::PagePaginationParams;

/// Canonical paged list response for admin browse endpoints.
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PageList<T> {
    /// Items in this page.
    pub data: Vec<T>,
    /// Total items across all pages.
    pub total: u64,
    /// Whether more items exist beyond this page.
    pub has_more: bool,
}

impl<T> PageList<T> {
    /// Build from a page's items, the total, and the offset the page was
    /// fetched at.
    pub fn new(data: Vec<T>, total: u64, offset: u64) -> Self {
        let has_more = offset.saturating_add(data.len() as u64) < total;
        Self {
            data,
            total,
            has_more,
        }
    }

    /// Build a whole-set response (small bounded collections returned in
    /// full). `has_more` is always false.
    pub fn from_bounded(data: Vec<T>) -> Self {
        let total = data.len() as u64;
        Self {
            data,
            total,
            has_more: false,
        }
    }

    /// Map the item type, preserving pagination metadata.
    pub fn map<U>(self, f: impl FnMut(T) -> U) -> PageList<U> {
        PageList {
            data: self.data.into_iter().map(f).collect(),
            total: self.total,
            has_more: self.has_more,
        }
    }
}

impl PagePaginationParams {
    /// Wrap data and total into the canonical [`PageList`] envelope,
    /// computing `has_more` from this page's offset.
    pub fn wrap_page_list<T>(self, data: Vec<T>, total: u64) -> PageList<T> {
        PageList::new(data, total, self.offset() as u64)
    }
}

#[cfg(test)]
#[path = "tests/page_list_tests.rs"]
mod tests;
