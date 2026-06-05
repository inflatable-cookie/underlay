use serde::de::DeserializeOwned;

use super::MigrationReportError;

pub(crate) fn parse_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, MigrationReportError> {
    serde_json::from_slice(bytes).map_err(MigrationReportError::Json)
}
