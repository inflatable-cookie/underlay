use std::fmt;
use std::str::FromStr;

use super::MigrationBundleError;
use crate::migration_bundle::local_store;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MigrationBundleRef {
    value: String,
    digest: String,
}

impl MigrationBundleRef {
    pub fn parse_digest_pinned(value: impl AsRef<str>) -> Result<Self, MigrationBundleError> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(MigrationBundleError::InvalidInput(
                "bundle_ref must not be empty".to_string(),
            ));
        }

        let digest = local_store::digest_from_ref(value)?.ok_or_else(|| {
            MigrationBundleError::InvalidInput(
                "migration run requires digest-pinned --bundle <ref@sha256:...>".to_string(),
            )
        })?;

        Ok(Self {
            value: value.to_string(),
            digest,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

impl fmt::Display for MigrationBundleRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for MigrationBundleRef {
    type Err = MigrationBundleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse_digest_pinned(value)
    }
}
