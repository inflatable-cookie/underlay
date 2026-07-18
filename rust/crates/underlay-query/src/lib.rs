//! Shared query model and SQL generation for Underlay.
//!
//! Holds the filter/sort vocabulary and the SQL `WHERE`/field-mapping builders
//! used by both the HTTP edge (which parses `filter[...]` / `sort=` wire
//! formats into these types) and the database layer (which turns them into
//! SQL). Splitting this out of `underlay-http` removes the inversion where
//! db-layer code depended on the HTTP crate for SQL construction.

pub mod field_mapping;
pub mod filter;
pub mod sort;
pub mod where_builder;

pub use field_mapping::FieldMapping;
pub use filter::{FilterField, FilterOperator};
pub use sort::{parse_sort_string, SortDirection, SortField};
pub use where_builder::{SqlValue, WhereBuilder};

/// Create a [`FieldMapping`] with a concise syntax.
///
/// All fields are mapped for both sorting and filtering by default.
///
/// # Example
///
/// ```
/// use underlay_query::field_mapping;
///
/// let mapping = field_mapping! {
///     "title" => "m.title",
///     "slug" => "m.slug",
///     "isLive" => "m.is_live",
/// };
/// ```
#[macro_export]
macro_rules! field_mapping {
    ($($api:expr => $db:expr),* $(,)?) => {{
        let mut mapping = $crate::FieldMapping::new();
        $(
            mapping = mapping.map($api, $db);
        )*
        mapping
    }};
}
