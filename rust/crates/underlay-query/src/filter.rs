use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

/// Error returned when parsing a [`FilterOperator`] from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFilterOperatorError {
    input: String,
}

impl ParseFilterOperatorError {
    /// Return the unrecognized operator string.
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for ParseFilterOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown filter operator: {:?}", self.input)
    }
}

impl std::error::Error for ParseFilterOperatorError {}

impl FromStr for FilterOperator {
    type Err = ParseFilterOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eq" | "=" => Ok(FilterOperator::Eq),
            "ne" | "!=" => Ok(FilterOperator::Ne),
            "gt" | ">" => Ok(FilterOperator::Gt),
            "gte" | ">=" => Ok(FilterOperator::Gte),
            "lt" | "<" => Ok(FilterOperator::Lt),
            "lte" | "<=" => Ok(FilterOperator::Lte),
            "like" => Ok(FilterOperator::Like),
            _ => Err(ParseFilterOperatorError {
                input: s.to_string(),
            }),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_returns_typed_error_with_input() {
        let err = "regex".parse::<FilterOperator>().unwrap_err();
        assert_eq!(err.input(), "regex");
        assert!(err.to_string().contains("regex"));
        let _: &dyn std::error::Error = &err;
    }
}
