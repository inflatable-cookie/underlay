use std::collections::HashMap;

/// Builder for creating field mappings between API names and database columns.
///
/// Simplifies the common pattern of creating HashMaps for sort and filter field mappings.
/// Can be reused across multiple query handlers for the same entity type.
///
/// # Example
///
/// ```
/// use underlay_http::query::FieldMapping;
///
/// let mapping = FieldMapping::new()
///     .map("title", "m.title")
///     .map("slug", "m.slug")
///     .map("isLive", "m.is_live")
///     .sort_only("weight", "m.weight")
///     .filter_only("pathwayId", "m.pathway_id");
///
/// // Use with QueryParams and WhereBuilder
/// // let order_by = query.sql_order_by(mapping.sort_map());
/// // builder.add_filter(&filter, mapping.filter_map());
/// ```
#[derive(Debug, Clone, Default)]
pub struct FieldMapping {
    sort_fields: HashMap<String, String>,
    filter_fields: HashMap<String, String>,
}

impl FieldMapping {
    /// Create a new empty field mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Map an API field name to a database column for both sorting and filtering.
    ///
    /// This is the most common case where a field can be used for both operations.
    pub fn map(mut self, api_name: &str, db_column: &str) -> Self {
        self.sort_fields
            .insert(api_name.to_string(), db_column.to_string());
        self.filter_fields
            .insert(api_name.to_string(), db_column.to_string());
        self
    }

    /// Map a field only for sorting (not filtering).
    ///
    /// Use this for fields that should be sortable but not filterable,
    /// like computed columns or aggregates.
    pub fn sort_only(mut self, api_name: &str, db_column: &str) -> Self {
        self.sort_fields
            .insert(api_name.to_string(), db_column.to_string());
        self
    }

    /// Map a field only for filtering (not sorting).
    ///
    /// Use this for fields that should be filterable but not sortable,
    /// like foreign keys or boolean flags that don't make sense to sort by.
    pub fn filter_only(mut self, api_name: &str, db_column: &str) -> Self {
        self.filter_fields
            .insert(api_name.to_string(), db_column.to_string());
        self
    }

    /// Get the sort field mapping as a HashMap with &str references.
    ///
    /// Returns a HashMap suitable for use with `QueryParams::sql_order_by()`.
    pub fn sort_map(&self) -> HashMap<&str, &str> {
        self.sort_fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    /// Get the filter field mapping as a HashMap with &str references.
    ///
    /// Returns a HashMap suitable for use with `WhereBuilder::add_filter()`.
    pub fn filter_map(&self) -> HashMap<&str, &str> {
        self.filter_fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    /// Look up a sort column by API field name.
    pub fn get_sort(&self, api_name: &str) -> Option<&str> {
        self.sort_fields.get(api_name).map(|s| s.as_str())
    }

    /// Look up a filter column by API field name.
    pub fn get_filter(&self, api_name: &str) -> Option<&str> {
        self.filter_fields.get(api_name).map(|s| s.as_str())
    }
}
