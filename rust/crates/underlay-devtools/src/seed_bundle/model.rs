use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SeedBundleBuildOptions {
    /// Path to the seed-bundle directory (contains manifest.json + SQL files).
    pub source_dir: PathBuf,
    /// Output `.oci` bundle file path.
    pub output_file: PathBuf,
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
    pub oci_ref: String,
    /// Directory to extract SQL files into (e.g., `dist/seed-bundles/<name>/`).
    pub output_dir: PathBuf,
    /// Optional local OCI store directory override.
    pub local_store_dir: Option<PathBuf>,
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
