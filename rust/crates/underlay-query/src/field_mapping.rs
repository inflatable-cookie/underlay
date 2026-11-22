use std::collections::HashMap;

/// Whether a mapped database column is safe to interpolate into SQL.
///
/// Accepts a plain identifier (`title`) or one `alias.column` pair (`m.title`),
/// where each part matches `[a-zA-Z_][a-zA-Z0-9_]*`. Quotes, spaces, semicolons,
/// comments, and extra dots are rejected.
pub(crate) fn is_safe_column_identifier(column: &str) -> bool {
    fn is_identifier(part: &str) -> bool {
        let mut chars = part.chars();
        matches!(chars.next(), Some(c) if c == '_' || c.is_ascii_alphabetic())
            && chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
    }

    match column.split_once('.') {
        None => is_identifier(column),
        Some((alias, name)) => is_identifier(alias) && is_identifier(name),
    }
}

/// Builder for creating field mappings between API names and database columns.
///
/// Simplifies the common pattern of creating HashMaps for sort and filter field mappings.
/// Can be reused across multiple query handlers for the same entity type.
///
/// # Example
///
/// ```
/// use underlay_query::FieldMapping;
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
    /// Entries whose column is not a plain SQL identifier are left out, so they
    /// behave as unmapped fields.
    ///
    /// Returns a HashMap suitable for use with `QueryParams::sql_order_by()`.
    pub fn sort_map(&self) -> HashMap<&str, &str> {
        safe_entries(&self.sort_fields).collect()
    }

    /// Get the filter field mapping as a HashMap with &str references.
    ///
    /// Entries whose column is not a plain SQL identifier are left out, so they
    /// behave as unmapped fields.
    ///
    /// Returns a HashMap suitable for use with `WhereBuilder::add_filter()`.
    pub fn filter_map(&self) -> HashMap<&str, &str> {
        safe_entries(&self.filter_fields).collect()
    }

    /// Look up a sort column by API field name.
    ///
    /// Returns `None` when the field is unmapped or its column is not a plain
    /// SQL identifier.
    pub fn get_sort(&self, api_name: &str) -> Option<&str> {
        self.sort_fields
            .get(api_name)
            .map(|s| s.as_str())
            .filter(|column| is_safe_column_identifier(column))
    }

    /// Look up a filter column by API field name.
    ///
    /// Returns `None` when the field is unmapped or its column is not a plain
    /// SQL identifier.
    pub fn get_filter(&self, api_name: &str) -> Option<&str> {
        self.filter_fields
            .get(api_name)
            .map(|s| s.as_str())
            .filter(|column| is_safe_column_identifier(column))
    }
}

fn safe_entries(fields: &HashMap<String, String>) -> impl Iterator<Item = (&str, &str)> {
    fields
        .iter()
        .filter(|(_, column)| is_safe_column_identifier(column))
        .map(|(api, column)| (api.as_str(), column.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_column_identifier_accepts_plain_and_qualified_names() {
        assert!(is_safe_column_identifier("title"));
        assert!(is_safe_column_identifier("_private1"));
        assert!(is_safe_column_identifier("m.title"));
        assert!(is_safe_column_identifier("m.is_live"));
    }

    #[test]
    fn safe_column_identifier_rejects_injection_shapes() {
        for column in [
            "",
            "m.",
            ".title",
            "1title",
            "m.1title",
            "m title",
            "title;",
            "title; DROP TABLE users",
            "title--",
            "title/*x*/",
            "\"title\"",
            "'title'",
            "title)",
            "m.title.extra",
            "a-b",
            "m.title OR 1=1",
        ] {
            assert!(!is_safe_column_identifier(column), "accepted {column:?}");
        }
    }

    #[test]
    fn unsafe_mapped_columns_are_treated_as_unmapped() {
        let mapping = FieldMapping::new()
            .map("title", "m.title")
            .map("evil", "m.title; DROP TABLE users")
            .sort_only("sortEvil", "m.x --")
            .filter_only("filterEvil", "\"quoted\"");

        assert_eq!(mapping.get_filter("title"), Some("m.title"));
        assert_eq!(mapping.get_filter("evil"), None);
        assert_eq!(mapping.get_sort("sortEvil"), None);
        assert_eq!(mapping.get_filter("filterEvil"), None);

        let sort = mapping.sort_map();
        assert_eq!(sort.get("title"), Some(&"m.title"));
        assert!(!sort.contains_key("evil"));
        assert!(!sort.contains_key("sortEvil"));

        let filter = mapping.filter_map();
        assert_eq!(filter.get("title"), Some(&"m.title"));
        assert!(!filter.contains_key("evil"));
        assert!(!filter.contains_key("filterEvil"));
    }
}
