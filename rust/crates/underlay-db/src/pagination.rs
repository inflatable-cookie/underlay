//! Cursor-based pagination utilities for database queries.
//!
//! This module provides types and helpers for implementing efficient keyset pagination
//! that scales to millions of rows with consistent performance.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_db::pagination::{CursorPaginationParams, PaginatedResponse, Cursor};
//!
//! // Parse pagination params from query string
//! let params = CursorPaginationParams::default();
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

mod builder;
mod cursor;
mod errors;
mod params;
mod response;
mod typed_cursors;

pub use builder::*;
pub use cursor::*;
pub use errors::*;
pub use params::*;
pub use response::*;
pub use typed_cursors::*;

#[cfg(test)]
#[path = "tests/pagination_tests.rs"]
mod tests;
