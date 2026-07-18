use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::str::FromStr;
use underlay_query::{parse_sort_string, FilterField, FilterOperator, SortField};

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
    pub(crate) filter_raw: HashMap<String, String>,
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
    /// - `filter[field]=value` -> equality filter
    /// - `filter[field][op]=value` -> filter with operator
    pub fn filter_fields(&self) -> Vec<FilterField> {
        let mut filters = Vec::new();

        for (key, value) in &self.filter_raw {
            if let Some(rest) = key.strip_prefix("filter[") {
                if let Some((field_part, remainder)) = rest.split_once(']') {
                    let field = field_part.to_string();

                    if remainder.is_empty() {
                        filters.push(FilterField::new(field, FilterOperator::Eq, value.clone()));
                    } else if let Some(op_part) = remainder.strip_prefix('[') {
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
