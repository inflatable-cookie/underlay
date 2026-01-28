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

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl SortDirection {
    /// Returns the SQL keyword for this direction
    pub fn sql(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
        }
    }
}

impl FromStr for SortDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" | "ascending" => Ok(SortDirection::Asc),
            "desc" | "descending" => Ok(SortDirection::Desc),
            _ => Err(()),
        }
    }
}

/// A single sort field with direction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortField {
    /// The field name to sort by
    pub field: String,
    /// The sort direction
    pub direction: SortDirection,
}

impl SortField {
    /// Create a new sort field
    pub fn new(field: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }

    /// Create an ascending sort field
    pub fn asc(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Asc)
    }

    /// Create a descending sort field
    pub fn desc(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Desc)
    }
}

/// Filter operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FilterOperator {
    /// Equal (default)
    #[default]
    Eq,
    /// Not equal
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Gte,
    /// Less than
    Lt,
    /// Less than or equal
    Lte,
    /// LIKE pattern match (use % for wildcards)
    Like,
}

impl FilterOperator {
    /// Returns the SQL operator for this filter
    ///
    /// Note: `Like` uses PostgreSQL's `ILIKE` for case-insensitive matching,
    /// which is the common expectation for user-facing text search.
    pub fn sql(&self) -> &'static str {
        match self {
            FilterOperator::Eq => "=",
            FilterOperator::Ne => "!=",
            FilterOperator::Gt => ">",
            FilterOperator::Gte => ">=",
            FilterOperator::Lt => "<",
            FilterOperator::Lte => "<=",
            FilterOperator::Like => "ILIKE",
        }
    }
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterOperator::Eq => write!(f, "eq"),
            FilterOperator::Ne => write!(f, "ne"),
            FilterOperator::Gt => write!(f, "gt"),
            FilterOperator::Gte => write!(f, "gte"),
            FilterOperator::Lt => write!(f, "lt"),
            FilterOperator::Lte => write!(f, "lte"),
            FilterOperator::Like => write!(f, "like"),
        }
    }
}

impl FromStr for FilterOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eq" | "=" => Ok(FilterOperator::Eq),
            "ne" | "!=" => Ok(FilterOperator::Ne),
            "gt" | ">" => Ok(FilterOperator::Gt),
            "gte" | ">=" => Ok(FilterOperator::Gte),
            "lt" | "<" => Ok(FilterOperator::Lt),
            "lte" | "<=" => Ok(FilterOperator::Lte),
            "like" => Ok(FilterOperator::Like),
            _ => Err(()),
        }
    }
}

/// A single filter condition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterField {
    /// The field name to filter on
    pub field: String,
    /// The filter operator
    pub operator: FilterOperator,
    /// The value to compare against
    pub value: String,
}

impl FilterField {
    /// Create a new filter field
    pub fn new(
        field: impl Into<String>,
        operator: FilterOperator,
        value: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            operator,
            value: value.into(),
        }
    }

    /// Create an equality filter
    pub fn eq(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(field, FilterOperator::Eq, value)
    }
}

/// Query parameters for sorting and filtering
///
/// Use with Axum's `Query` extractor:
///
/// ```ignore
/// use axum::extract::Query;
/// use underlay_http::query::QueryParams;
///
/// async fn list_items(Query(query): Query<QueryParams>) -> impl IntoResponse {
///     let sorts = query.sort_fields();
///     let filters = query.filter_fields();
///     // Apply to database query...
/// }
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryParams {
    /// Sort parameter: `sort=field1:asc,field2:desc`
    #[serde(default, deserialize_with = "deserialize_sort")]
    pub sort: Vec<SortField>,

    /// Filter parameters: collected from `filter[field]=value` or `filter[field][op]=value`
    #[serde(default, flatten)]
    filter_raw: HashMap<String, String>,
}

fn deserialize_sort<'de, D>(deserializer: D) -> Result<Vec<SortField>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(Vec::new()),
        Some(s) if s.is_empty() => Ok(Vec::new()),
        Some(s) => Ok(parse_sort_string(&s)),
    }
}

/// Parse a sort string like "field1:asc,field2:desc" into SortFields
pub fn parse_sort_string(s: &str) -> Vec<SortField> {
    s.split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }

            let (field, direction) = if let Some((f, d)) = part.split_once(':') {
                let dir = d.parse().unwrap_or(SortDirection::Asc);
                (f.trim().to_string(), dir)
            } else {
                (part.to_string(), SortDirection::Asc)
            };

            if field.is_empty() {
                return None;
            }

            Some(SortField { field, direction })
        })
        .collect()
}

impl QueryParams {
    /// Get the sort fields
    pub fn sort_fields(&self) -> &[SortField] {
        &self.sort
    }

    /// Check if any sort fields are specified
    pub fn has_sort(&self) -> bool {
        !self.sort.is_empty()
    }

    /// Parse filter fields from the raw filter parameters
    ///
    /// Handles both formats:
    /// - `filter[field]=value` → equality filter
    /// - `filter[field][op]=value` → filter with operator
    pub fn filter_fields(&self) -> Vec<FilterField> {
        let mut filters = Vec::new();

        for (key, value) in &self.filter_raw {
            // Match filter[field] or filter[field][op]
            if let Some(rest) = key.strip_prefix("filter[") {
                if let Some((field_part, remainder)) = rest.split_once(']') {
                    let field = field_part.to_string();

                    if remainder.is_empty() {
                        // Simple filter: filter[field]=value
                        filters.push(FilterField::new(field, FilterOperator::Eq, value.clone()));
                    } else if let Some(op_part) = remainder.strip_prefix('[') {
                        // Operator filter: filter[field][op]=value
                        if let Some(op_str) = op_part.strip_suffix(']') {
                            let operator = op_str.parse().unwrap_or(FilterOperator::Eq);
                            filters.push(FilterField::new(field, operator, value.clone()));
                        }
                    }
                }
            }
        }

        filters
    }

    /// Check if any filters are specified
    pub fn has_filters(&self) -> bool {
        self.filter_raw.keys().any(|k| k.starts_with("filter["))
    }

    /// Get a specific filter value by field name (equality filter only)
    pub fn get_filter(&self, field: &str) -> Option<&str> {
        let key = format!("filter[{}]", field);
        self.filter_raw.get(&key).map(|s| s.as_str())
    }

    /// Get a specific filter value as a parsed type
    pub fn get_filter_as<T: FromStr>(&self, field: &str) -> Option<T> {
        self.get_filter(field).and_then(|s| s.parse().ok())
    }

    /// Build SQL ORDER BY clause from sort fields
    ///
    /// # Arguments
    /// * `field_map` - Maps API field names to database column names
    ///
    /// # Returns
    /// SQL ORDER BY clause (without the "ORDER BY" keyword), or empty string if no sorts
    ///
    /// # Example
    /// ```
    /// use underlay_http::query::{QueryParams, parse_sort_string};
    /// use std::collections::HashMap;
    ///
    /// let mut params = QueryParams::default();
    /// params.sort = parse_sort_string("title:asc,createdAt:desc");
    ///
    /// let mut field_map = HashMap::new();
    /// field_map.insert("title", "m.title");
    /// field_map.insert("createdAt", "m.created_at");
    ///
    /// let clause = params.sql_order_by(&field_map);
    /// assert_eq!(clause, "m.title ASC, m.created_at DESC");
    /// ```
    pub fn sql_order_by(&self, field_map: &HashMap<&str, &str>) -> String {
        self.sort
            .iter()
            .filter_map(|sf| {
                field_map
                    .get(sf.field.as_str())
                    .map(|col| format!("{} {}", col, sf.direction.sql()))
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Build SQL ORDER BY clause with a default if no sorts specified
    pub fn sql_order_by_or(&self, field_map: &HashMap<&str, &str>, default: &str) -> String {
        let clause = self.sql_order_by(field_map);
        if clause.is_empty() {
            default.to_string()
        } else {
            clause
        }
    }
}

/// Builder for constructing SQL WHERE clauses from filters
///
/// Handles parameter binding safely to prevent SQL injection.
///
/// # Example
/// ```
/// use underlay_http::query::{FilterField, FilterOperator, WhereBuilder};
/// use std::collections::HashMap;
///
/// let filters = vec![
///     FilterField::eq("pathwayId", "abc123"),
///     FilterField::new("weight", FilterOperator::Gte, "10"),
/// ];
///
/// let mut field_map = HashMap::new();
/// field_map.insert("pathwayId", "m.pathway_id");
/// field_map.insert("weight", "m.weight");
///
/// let mut builder = WhereBuilder::new(1); // Start at $1
/// for filter in &filters {
///     builder.add_filter(filter, &field_map);
/// }
///
/// let (clause, values) = builder.build();
/// assert_eq!(clause, "m.pathway_id = $1 AND m.weight >= $2");
/// assert_eq!(values, vec!["abc123", "10"]);
/// ```
pub struct WhereBuilder {
    conditions: Vec<String>,
    values: Vec<String>,
    param_index: u32,
}

impl WhereBuilder {
    /// Create a new WHERE builder starting at the given parameter index
    pub fn new(start_index: u32) -> Self {
        Self {
            conditions: Vec::new(),
            values: Vec::new(),
            param_index: start_index,
        }
    }

    /// Add a filter condition
    ///
    /// Returns true if the filter was added (field exists in map)
    pub fn add_filter(&mut self, filter: &FilterField, field_map: &HashMap<&str, &str>) -> bool {
        if let Some(column) = field_map.get(filter.field.as_str()) {
            let condition = format!("{} {} ${}", column, filter.operator.sql(), self.param_index);
            self.conditions.push(condition);
            self.values.push(filter.value.clone());
            self.param_index += 1;
            true
        } else {
            false
        }
    }

    /// Add a raw condition with a value
    pub fn add_raw(&mut self, condition: &str, value: impl Into<String>) {
        let condition = condition.replace("{}", &format!("${}", self.param_index));
        self.conditions.push(condition);
        self.values.push(value.into());
        self.param_index += 1;
    }

    /// Add a condition without a parameter (e.g., for boolean checks)
    pub fn add_condition(&mut self, condition: impl Into<String>) {
        self.conditions.push(condition.into());
    }

    /// Get the current parameter index (for adding more parameters after build)
    pub fn next_param_index(&self) -> u32 {
        self.param_index
    }

    /// Check if any conditions have been added
    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    /// Build the WHERE clause and values
    ///
    /// Returns (clause, values) where clause is conditions joined by AND
    pub fn build(self) -> (String, Vec<String>) {
        (self.conditions.join(" AND "), self.values)
    }

    /// Build the WHERE clause with prefix
    ///
    /// Returns "WHERE ..." if there are conditions, empty string otherwise
    pub fn build_with_where(self) -> (String, Vec<String>) {
        if self.conditions.is_empty() {
            (String::new(), Vec::new())
        } else {
            let (clause, values) = self.build();
            (format!("WHERE {}", clause), values)
        }
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
mod tests {
    use super::*;

    #[test]
    fn test_sort_direction_sql() {
        assert_eq!(SortDirection::Asc.sql(), "ASC");
        assert_eq!(SortDirection::Desc.sql(), "DESC");
    }

    #[test]
    fn test_sort_direction_parse() {
        assert_eq!("asc".parse::<SortDirection>().unwrap(), SortDirection::Asc);
        assert_eq!(
            "desc".parse::<SortDirection>().unwrap(),
            SortDirection::Desc
        );
        assert_eq!("ASC".parse::<SortDirection>().unwrap(), SortDirection::Asc);
        assert_eq!(
            "DESC".parse::<SortDirection>().unwrap(),
            SortDirection::Desc
        );
        assert!("invalid".parse::<SortDirection>().is_err());
    }

    #[test]
    fn test_parse_sort_string() {
        let sorts = parse_sort_string("title:asc,createdAt:desc");
        assert_eq!(sorts.len(), 2);
        assert_eq!(sorts[0].field, "title");
        assert_eq!(sorts[0].direction, SortDirection::Asc);
        assert_eq!(sorts[1].field, "createdAt");
        assert_eq!(sorts[1].direction, SortDirection::Desc);
    }

    #[test]
    fn test_parse_sort_string_no_direction() {
        let sorts = parse_sort_string("title,name");
        assert_eq!(sorts.len(), 2);
        assert_eq!(sorts[0].direction, SortDirection::Asc);
        assert_eq!(sorts[1].direction, SortDirection::Asc);
    }

    #[test]
    fn test_parse_sort_string_empty() {
        let sorts = parse_sort_string("");
        assert!(sorts.is_empty());
    }

    #[test]
    fn test_filter_operator_sql() {
        assert_eq!(FilterOperator::Eq.sql(), "=");
        assert_eq!(FilterOperator::Ne.sql(), "!=");
        assert_eq!(FilterOperator::Gt.sql(), ">");
        assert_eq!(FilterOperator::Gte.sql(), ">=");
        assert_eq!(FilterOperator::Lt.sql(), "<");
        assert_eq!(FilterOperator::Lte.sql(), "<=");
        assert_eq!(FilterOperator::Like.sql(), "ILIKE");
    }

    #[test]
    fn test_filter_operator_parse() {
        assert_eq!("eq".parse::<FilterOperator>().unwrap(), FilterOperator::Eq);
        assert_eq!(
            "gte".parse::<FilterOperator>().unwrap(),
            FilterOperator::Gte
        );
        assert_eq!(
            "like".parse::<FilterOperator>().unwrap(),
            FilterOperator::Like
        );
    }

    #[test]
    fn test_sql_order_by() {
        let mut params = QueryParams::default();
        params.sort = parse_sort_string("title:asc,weight:desc");

        let mut field_map = HashMap::new();
        field_map.insert("title", "m.title");
        field_map.insert("weight", "m.weight");

        let clause = params.sql_order_by(&field_map);
        assert_eq!(clause, "m.title ASC, m.weight DESC");
    }

    #[test]
    fn test_sql_order_by_unknown_field() {
        let mut params = QueryParams::default();
        params.sort = parse_sort_string("title:asc,unknown:desc");

        let mut field_map = HashMap::new();
        field_map.insert("title", "m.title");

        let clause = params.sql_order_by(&field_map);
        assert_eq!(clause, "m.title ASC"); // unknown field is ignored
    }

    #[test]
    fn test_where_builder() {
        let filters = vec![
            FilterField::eq("pathwayId", "abc123"),
            FilterField::new("weight", FilterOperator::Gte, "10"),
        ];

        let mut field_map = HashMap::new();
        field_map.insert("pathwayId", "m.pathway_id");
        field_map.insert("weight", "m.weight");

        let mut builder = WhereBuilder::new(1);
        for filter in &filters {
            builder.add_filter(filter, &field_map);
        }

        let (clause, values) = builder.build();
        assert_eq!(clause, "m.pathway_id = $1 AND m.weight >= $2");
        assert_eq!(values, vec!["abc123", "10"]);
    }

    #[test]
    fn test_where_builder_with_where() {
        let filters = vec![FilterField::eq("status", "active")];

        let mut field_map = HashMap::new();
        field_map.insert("status", "m.status");

        let mut builder = WhereBuilder::new(1);
        for filter in &filters {
            builder.add_filter(filter, &field_map);
        }

        let (clause, _) = builder.build_with_where();
        assert_eq!(clause, "WHERE m.status = $1");
    }

    #[test]
    fn test_where_builder_empty() {
        let builder = WhereBuilder::new(1);
        let (clause, values) = builder.build_with_where();
        assert_eq!(clause, "");
        assert!(values.is_empty());
    }

    #[test]
    fn test_query_params_serde_urlencoded() {
        // Test that QueryParams can be deserialized from URL query string format
        let query_string = "sort=title%3Aasc%2Cweight%3Adesc";
        let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

        assert_eq!(params.sort.len(), 2);
        assert_eq!(params.sort[0].field, "title");
        assert_eq!(params.sort[0].direction, SortDirection::Asc);
        assert_eq!(params.sort[1].field, "weight");
        assert_eq!(params.sort[1].direction, SortDirection::Desc);
    }

    #[test]
    fn test_query_params_serde_urlencoded_single_sort() {
        let query_string = "sort=weight%3Adesc";
        let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

        assert_eq!(params.sort.len(), 1);
        assert_eq!(params.sort[0].field, "weight");
        assert_eq!(params.sort[0].direction, SortDirection::Desc);
    }

    #[test]
    fn test_query_params_serde_urlencoded_empty() {
        let query_string = "";
        let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

        assert!(params.sort.is_empty());
    }

    #[test]
    fn test_query_params_serde_urlencoded_with_filter() {
        let query_string = "sort=title%3Aasc&filter%5BpathwayId%5D=abc123";
        let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

        assert_eq!(params.sort.len(), 1);
        assert_eq!(params.sort[0].field, "title");

        let filters = params.filter_fields();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].field, "pathwayId");
        assert_eq!(filters[0].value, "abc123");
    }

    #[test]
    fn test_query_params_serde_urlencoded_with_like_filter() {
        // filter[name][like]=%test%
        let query_string = "filter%5Bname%5D%5Blike%5D=%25test%25";
        let params: QueryParams = serde_urlencoded::from_str(query_string).unwrap();

        let filters = params.filter_fields();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].field, "name");
        assert_eq!(filters[0].operator, FilterOperator::Like);
        assert_eq!(filters[0].value, "%test%");
    }

    #[test]
    fn test_field_mapping_map() {
        let mapping = FieldMapping::new()
            .map("title", "m.title")
            .map("isLive", "m.is_live");

        let sort = mapping.sort_map();
        assert_eq!(sort.get("title"), Some(&"m.title"));
        assert_eq!(sort.get("isLive"), Some(&"m.is_live"));

        let filter = mapping.filter_map();
        assert_eq!(filter.get("title"), Some(&"m.title"));
        assert_eq!(filter.get("isLive"), Some(&"m.is_live"));
    }

    #[test]
    fn test_field_mapping_sort_only() {
        let mapping = FieldMapping::new()
            .map("title", "m.title")
            .sort_only("weight", "m.weight");

        let sort = mapping.sort_map();
        assert_eq!(sort.get("weight"), Some(&"m.weight"));

        let filter = mapping.filter_map();
        assert_eq!(filter.get("weight"), None);
        assert_eq!(filter.get("title"), Some(&"m.title"));
    }

    #[test]
    fn test_field_mapping_filter_only() {
        let mapping = FieldMapping::new()
            .map("title", "m.title")
            .filter_only("pathwayId", "m.pathway_id");

        let sort = mapping.sort_map();
        assert_eq!(sort.get("pathwayId"), None);

        let filter = mapping.filter_map();
        assert_eq!(filter.get("pathwayId"), Some(&"m.pathway_id"));
    }

    #[test]
    fn test_field_mapping_get() {
        let mapping = FieldMapping::new()
            .map("title", "m.title")
            .sort_only("weight", "m.weight")
            .filter_only("pathwayId", "m.pathway_id");

        assert_eq!(mapping.get_sort("title"), Some("m.title"));
        assert_eq!(mapping.get_sort("weight"), Some("m.weight"));
        assert_eq!(mapping.get_sort("pathwayId"), None);

        assert_eq!(mapping.get_filter("title"), Some("m.title"));
        assert_eq!(mapping.get_filter("weight"), None);
        assert_eq!(mapping.get_filter("pathwayId"), Some("m.pathway_id"));
    }

    #[test]
    fn test_field_mapping_macro() {
        let mapping = crate::field_mapping! {
            "title" => "m.title",
            "slug" => "m.slug",
        };

        assert_eq!(mapping.get_sort("title"), Some("m.title"));
        assert_eq!(mapping.get_filter("slug"), Some("m.slug"));
    }
}
