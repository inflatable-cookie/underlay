//! Query parameters for sorting and filtering
//!
//! This module provides types and utilities for handling sort and filter
//! query parameters in API endpoints.
//!
//! # Query Parameter Format
//!
//! ## Sorting
//!
//! Sort parameters use the format: `sort=field1:asc,field2:desc`
//!
//! - Comma-separated list of `field:direction` pairs
//! - Direction is `asc` or `desc` (defaults to `asc` if omitted)
//!
//! Example: `?sort=title:asc,createdAt:desc`
//!
//! ## Filtering
//!
//! Filter parameters use bracket notation: `filter[field]=value`
//!
//! - Simple equality: `filter[pathwayId]=abc123`
//! - With operator: `filter[weight][gte]=10`
//! - Supported operators: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `like`
//!
//! Example: `?filter[isLive]=true&filter[weight][gte]=10`

mod field_mapping;
mod filter;
mod params;
mod sort;
mod where_builder;

pub use field_mapping::FieldMapping;
pub use filter::{FilterField, FilterOperator};
pub use params::QueryParams;
pub use sort::{parse_sort_string, SortDirection, SortField};
pub use where_builder::WhereBuilder;

/// Create a FieldMapping with a concise syntax.
///
/// All fields are mapped for both sorting and filtering by default.
///
/// # Example
///
/// ```
/// use underlay_http::field_mapping;
///
/// let mapping = field_mapping! {
///     "title" => "m.title",
///     "slug" => "m.slug",
///     "isLive" => "m.is_live",
///     "createdAt" => "m.created_at",
/// };
/// ```
#[macro_export]
macro_rules! field_mapping {
    ($($api:expr => $db:expr),* $(,)?) => {{
        let mut mapping = $crate::query::FieldMapping::new();
        $(
            mapping = mapping.map($api, $db);
        )*
        mapping
    }};
}

#[cfg(test)]
#[path = "tests/query_tests.rs"]
mod tests;
