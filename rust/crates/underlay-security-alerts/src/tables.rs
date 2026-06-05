use std::str::FromStr;

use underlay_db::QualifiedTableName;

use crate::error::{SecurityAlertError, SecurityAlertResult};

macro_rules! define_table_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            name: QualifiedTableName,
        }

        impl $name {
            pub fn parse(value: impl AsRef<str>) -> SecurityAlertResult<Self> {
                let name = QualifiedTableName::parse(value)
                    .map_err(|_| SecurityAlertError::InvalidTableName)?;
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

        impl FromStr for $name {
            type Err = SecurityAlertError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }
    };
}

define_table_type!(
    LoginAttemptsTable,
    "Typed login-attempt table location used for security alert signal reads."
);

define_table_type!(
    SecurityAlertEventsTable,
    "Typed security-alert events table location used for cooldown checks and inserts."
);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecurityAlertTables {
    pub login_attempts: LoginAttemptsTable,
    pub alert_events: SecurityAlertEventsTable,
}

impl SecurityAlertTables {
    pub fn new(login_attempts: LoginAttemptsTable, alert_events: SecurityAlertEventsTable) -> Self {
        Self {
            login_attempts,
            alert_events,
        }
    }
}

#[cfg(test)]
#[path = "tests/tables_tests.rs"]
mod tests;
