use serde::{Deserialize, Serialize};
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
