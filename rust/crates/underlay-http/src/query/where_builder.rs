use super::FilterField;
use std::collections::HashMap;

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
