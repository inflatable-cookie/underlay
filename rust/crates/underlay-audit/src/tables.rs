use std::str::FromStr;

use underlay_db::QualifiedTableName;

use crate::error::{AuditError, AuditResult};

/// Typed audit log table location.
///
/// Use this when table location comes from app config. The raw string audit
/// APIs are retained for compatibility, but this type keeps validation at the
/// boundary instead of every query call.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuditTable {
    name: QualifiedTableName,
}

impl AuditTable {
    pub fn parse(value: impl AsRef<str>) -> AuditResult<Self> {
        let name = QualifiedTableName::parse(value).map_err(|_| AuditError::InvalidTableName)?;
        Ok(Self { name })
    }

    pub fn from_qualified(name: QualifiedTableName) -> Self {
        Self { name }
    }

    pub fn as_qualified(&self) -> &QualifiedTableName {
        &self.name
    }

    pub fn quoted(&self) -> String {
        self.name.quoted()
    }
}

impl FromStr for AuditTable {
    type Err = AuditError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[cfg(test)]
#[path = "tests/tables_tests.rs"]
mod tests;
