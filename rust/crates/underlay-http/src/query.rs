//! Query parameters for API sorting and filtering.
//!
//! Sorting uses `sort=field:direction` pairs. Filtering uses bracket notation
//! such as `filter[field]=value` or `filter[weight][gte]=10`.
//!
//! The shared query model and SQL builders (`FilterField`, `FilterOperator`,
//! `SortField`, `SortDirection`, `WhereBuilder`, `FieldMapping`) live in the
//! `underlay-query` crate; this module owns the HTTP-side wire parsing
//! (`QueryParams`) and re-exports the model so existing
//! `underlay_http::query::*` paths keep working.

mod params;

pub use params::{ListQueryParams, QueryParams};

// Re-export the shared query model from underlay-query so callers importing
// from `underlay_http::query` are unaffected by the relocation.
pub use underlay_query::{
    field_mapping, parse_sort_string, FieldMapping, FilterField, FilterOperator, SortDirection,
    SortField, SqlValue, WhereBuilder,
};

#[cfg(test)]
#[path = "tests/query_tests.rs"]
mod tests;
