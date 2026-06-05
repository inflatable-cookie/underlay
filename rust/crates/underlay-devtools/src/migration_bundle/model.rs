use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use super::local_store;

#[derive(Debug, Clone)]
pub struct BundleBuildOptions {
    pub output_file: PathBuf,
    pub source_system: String,
    pub target_schema_version: String,
    pub media_dir: Option<PathBuf>,
    pub media_shard_max_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BundlePublishOptions {
    pub bundle_file: PathBuf,
    pub oci_ref: String,
    pub local_store_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BundlePullOptions {
    pub oci_ref: String,
    pub output_dir: PathBuf,
    pub local_store_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BundleBuildReport {
    pub output_file: PathBuf,
    pub artifact_type: String,
    pub layer_count: usize,
    pub sidecar_count: usize,
    pub bundle_digest: String,
    pub media_asset_count: usize,
    pub media_shard_count: usize,
}

#[derive(Debug, Clone)]
pub struct BundlePublishReport {
    pub bundle_file: PathBuf,
    pub oci_ref: String,
    pub digest: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct BundlePullReport {
    pub oci_ref: String,
    pub output_file: PathBuf,
    pub digest: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct BundleRunOptions {
    pub bundle_ref: String,
    pub output_dir: PathBuf,
    pub local_store_dir: Option<PathBuf>,
}

impl BundleRunOptions {
    pub fn from_bundle_ref(
        bundle_ref: MigrationBundleRef,
        output_dir: PathBuf,
        local_store_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            bundle_ref: bundle_ref.to_string(),
            output_dir,
            local_store_dir,
        }
    }

    pub fn bundle_ref(&self) -> Result<MigrationBundleRef, MigrationBundleError> {
        MigrationBundleRef::parse_digest_pinned(&self.bundle_ref)
    }
}

#[derive(Debug, Clone)]
pub struct BundleRunReport {
    pub bundle_ref: String,
    pub output_file: PathBuf,
    pub digest: String,
    pub run_id: underlay_core::Uuid,
    pub status: String,
}

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

#[derive(Debug)]
pub enum MigrationBundleError {
    Io(std::io::Error),
    Validation(String),
    InvalidInput(String),
}

impl std::fmt::Display for MigrationBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationBundleError::Io(err) => write!(f, "{err}"),
            MigrationBundleError::Validation(msg) => write!(f, "{msg}"),
            MigrationBundleError::InvalidInput(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for MigrationBundleError {}

impl From<std::io::Error> for MigrationBundleError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
