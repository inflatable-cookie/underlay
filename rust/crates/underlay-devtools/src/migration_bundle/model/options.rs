use std::path::PathBuf;

use super::{MigrationBundleError, MigrationBundleRef};

#[derive(Debug, Clone)]
pub struct BundleBuildOptions {
    output_file: PathBuf,
    source_system: String,
    target_schema_version: String,
    media_dir: Option<PathBuf>,
    media_shard_max_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BundlePublishOptions {
    bundle_file: PathBuf,
    oci_ref: String,
    local_store_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BundlePullOptions {
    oci_ref: String,
    output_dir: PathBuf,
    local_store_dir: Option<PathBuf>,
}

impl BundleBuildOptions {
    pub fn new(
        output_file: impl Into<PathBuf>,
        source_system: impl Into<String>,
        target_schema_version: impl Into<String>,
    ) -> Self {
        Self {
            output_file: output_file.into(),
            source_system: source_system.into(),
            target_schema_version: target_schema_version.into(),
            media_dir: None,
            media_shard_max_bytes: None,
        }
    }

    pub fn with_media_dir(mut self, media_dir: impl Into<PathBuf>) -> Self {
        self.media_dir = Some(media_dir.into());
        self
    }

    pub fn with_optional_media_dir(mut self, media_dir: Option<PathBuf>) -> Self {
        self.media_dir = media_dir;
        self
    }

    pub fn with_media_shard_max_bytes(mut self, max_bytes: u64) -> Self {
        self.media_shard_max_bytes = Some(max_bytes);
        self
    }

    pub fn with_optional_media_shard_max_bytes(mut self, max_bytes: Option<u64>) -> Self {
        self.media_shard_max_bytes = max_bytes;
        self
    }

    pub fn output_file(&self) -> &PathBuf {
        &self.output_file
    }

    pub fn source_system(&self) -> &str {
        &self.source_system
    }

    pub fn target_schema_version(&self) -> &str {
        &self.target_schema_version
    }

    pub fn media_dir(&self) -> Option<&PathBuf> {
        self.media_dir.as_ref()
    }

    pub fn media_shard_max_bytes(&self) -> Option<u64> {
        self.media_shard_max_bytes
    }
}

impl BundlePublishOptions {
    pub fn new(bundle_file: impl Into<PathBuf>, oci_ref: impl Into<String>) -> Self {
        Self {
            bundle_file: bundle_file.into(),
            oci_ref: oci_ref.into(),
            local_store_dir: None,
        }
    }

    pub fn with_local_store_dir(mut self, local_store_dir: impl Into<PathBuf>) -> Self {
        self.local_store_dir = Some(local_store_dir.into());
        self
    }

    pub fn with_optional_local_store_dir(mut self, local_store_dir: Option<PathBuf>) -> Self {
        self.local_store_dir = local_store_dir;
        self
    }

    pub fn bundle_file(&self) -> &PathBuf {
        &self.bundle_file
    }

    pub fn oci_ref(&self) -> &str {
        &self.oci_ref
    }

    pub fn local_store_dir(&self) -> Option<&PathBuf> {
        self.local_store_dir.as_ref()
    }
}

impl BundlePullOptions {
    pub fn new(oci_ref: impl Into<String>, output_dir: impl Into<PathBuf>) -> Self {
        Self {
            oci_ref: oci_ref.into(),
            output_dir: output_dir.into(),
            local_store_dir: None,
        }
    }

    pub fn with_local_store_dir(mut self, local_store_dir: impl Into<PathBuf>) -> Self {
        self.local_store_dir = Some(local_store_dir.into());
        self
    }

    pub fn with_optional_local_store_dir(mut self, local_store_dir: Option<PathBuf>) -> Self {
        self.local_store_dir = local_store_dir;
        self
    }

    pub fn oci_ref(&self) -> &str {
        &self.oci_ref
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    pub fn local_store_dir(&self) -> Option<&PathBuf> {
        self.local_store_dir.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct BundleRunOptions {
    bundle_ref: MigrationBundleRef,
    output_dir: PathBuf,
    local_store_dir: Option<PathBuf>,
}

impl BundleRunOptions {
    pub fn new(
        bundle_ref: MigrationBundleRef,
        output_dir: PathBuf,
        local_store_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            bundle_ref,
            output_dir,
            local_store_dir,
        }
    }

    pub fn from_bundle_ref(
        bundle_ref: MigrationBundleRef,
        output_dir: PathBuf,
        local_store_dir: Option<PathBuf>,
    ) -> Self {
        Self::new(bundle_ref, output_dir, local_store_dir)
    }

    pub fn parse_bundle_ref(
        bundle_ref: impl AsRef<str>,
        output_dir: PathBuf,
        local_store_dir: Option<PathBuf>,
    ) -> Result<Self, MigrationBundleError> {
        Ok(Self::new(
            MigrationBundleRef::parse_digest_pinned(bundle_ref)?,
            output_dir,
            local_store_dir,
        ))
    }

    pub fn bundle_ref(&self) -> &MigrationBundleRef {
        &self.bundle_ref
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    pub fn local_store_dir(&self) -> Option<&PathBuf> {
        self.local_store_dir.as_ref()
    }
}
