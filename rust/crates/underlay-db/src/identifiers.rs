use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierError {
    Empty,
    EmptyPart,
    TooManyParts,
    InvalidStart { value: String },
    InvalidChar { value: String, ch: char },
}

impl fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "identifier is empty"),
            Self::EmptyPart => write!(f, "qualified identifier contains an empty part"),
            Self::TooManyParts => write!(f, "qualified identifier has too many parts"),
            Self::InvalidStart { value } => {
                write!(
                    f,
                    "identifier must start with lowercase ascii or _: {value}"
                )
            }
            Self::InvalidChar { value, ch } => {
                write!(f, "identifier contains invalid character {ch:?}: {value}")
            }
        }
    }
}

impl std::error::Error for IdentifierError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SqlIdentifier(String);

impl SqlIdentifier {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, IdentifierError> {
        let value = value.as_ref().trim();
        validate_sql_identifier(value)?;
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn quoted(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl fmt::Display for SqlIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SqlIdentifier {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedTableName {
    schema: Option<SqlIdentifier>,
    table: SqlIdentifier,
}

impl QualifiedTableName {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, IdentifierError> {
        let value = value.as_ref().trim();
        validate_qualified_table_name(value)?;
        let parts = value.split('.').collect::<Vec<_>>();
        match parts.as_slice() {
            [table] => Ok(Self {
                schema: None,
                table: SqlIdentifier::parse(table)?,
            }),
            [schema, table] => Ok(Self {
                schema: Some(SqlIdentifier::parse(schema)?),
                table: SqlIdentifier::parse(table)?,
            }),
            _ => Err(IdentifierError::TooManyParts),
        }
    }

    pub fn from_schema_table(
        schema: impl AsRef<str>,
        table: impl AsRef<str>,
    ) -> Result<Self, IdentifierError> {
        Ok(Self {
            schema: Some(SqlIdentifier::parse(schema)?),
            table: SqlIdentifier::parse(table)?,
        })
    }

    pub fn schema(&self) -> Option<&SqlIdentifier> {
        self.schema.as_ref()
    }

    pub fn table(&self) -> &SqlIdentifier {
        &self.table
    }

    pub fn quoted(&self) -> String {
        match &self.schema {
            Some(schema) => format!("{}.{}", schema.quoted(), self.table.quoted()),
            None => self.table.quoted(),
        }
    }
}

impl fmt::Display for QualifiedTableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.schema {
            Some(schema) => write!(f, "{}.{}", schema.as_str(), self.table.as_str()),
            None => f.write_str(self.table.as_str()),
        }
    }
}

impl FromStr for QualifiedTableName {
    type Err = IdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

pub fn validate_sql_identifier(value: &str) -> Result<(), IdentifierError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }

    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(IdentifierError::Empty);
    };

    if !(first.is_ascii_lowercase() || first == '_') {
        return Err(IdentifierError::InvalidStart {
            value: value.to_string(),
        });
    }

    for ch in chars {
        if !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_') {
            return Err(IdentifierError::InvalidChar {
                value: value.to_string(),
                ch,
            });
        }
    }

    Ok(())
}

pub fn validate_qualified_table_name(value: &str) -> Result<(), IdentifierError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }

    let parts = value.split('.').collect::<Vec<_>>();
    if parts.len() > 2 {
        return Err(IdentifierError::TooManyParts);
    }

    for part in parts {
        if part.is_empty() {
            return Err(IdentifierError::EmptyPart);
        }
        validate_sql_identifier(part)?;
    }

    Ok(())
}

pub fn quote_sql_identifier(value: &str) -> Result<String, IdentifierError> {
    Ok(SqlIdentifier::parse(value)?.quoted())
}

pub fn format_qualified_table_name(value: &str) -> Result<String, IdentifierError> {
    Ok(QualifiedTableName::parse(value)?.quoted())
}

pub fn format_schema_table(schema: &str, table: &str) -> Result<String, IdentifierError> {
    Ok(QualifiedTableName::from_schema_table(schema, table)?.quoted())
}

#[cfg(test)]
#[path = "tests/identifiers_tests.rs"]
mod tests;
