//! Query parameters for API sorting and filtering.
//!
//! Sorting uses `sort=field:direction` pairs. Filtering uses bracket notation
//! such as `filter[field]=value` or `filter[weight][gte]=10`.

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
