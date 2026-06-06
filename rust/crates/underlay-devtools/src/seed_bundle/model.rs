use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SeedBundleBuildOptions {
    /// Path to the seed-bundle directory (contains manifest.json + SQL files).
    source_dir: PathBuf,
    /// Output `.oci` bundle file path.
    output_file: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SeedBundleBuildReport {
    pub output_file: PathBuf,
    pub bundle_name: String,
    pub artifact_type: String,
    pub layer_count: usize,
    pub bundle_digest: String,
    pub sql_file_count: usize,
    pub total_sql_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct SeedBundlePullOptions {
    /// OCI reference (tag or digest) to pull.
    oci_ref: String,
    /// Directory to extract SQL files into (e.g., `dist/seed-bundles/<name>/`).
    output_dir: PathBuf,
    /// Optional local OCI store directory override.
    local_store_dir: Option<PathBuf>,
}

impl SeedBundleBuildOptions {
    pub fn new(source_dir: impl Into<PathBuf>, output_file: impl Into<PathBuf>) -> Self {
        Self {
            source_dir: source_dir.into(),
            output_file: output_file.into(),
        }
    }

    pub fn source_dir(&self) -> &PathBuf {
        &self.source_dir
    }

    pub fn output_file(&self) -> &PathBuf {
        &self.output_file
    }
}

impl SeedBundlePullOptions {
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
pub struct SeedBundlePullReport {
    pub oci_ref: String,
    pub output_dir: PathBuf,
    pub digest: String,
    pub sql_file_count: usize,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct SeedManifest {
    pub schema_version: String,
    pub name: String,
    pub priority: u32,
    pub tables: Vec<String>,
    pub idempotent_strategy: String,
    #[serde(default)]
    pub generated_at: String,
}
