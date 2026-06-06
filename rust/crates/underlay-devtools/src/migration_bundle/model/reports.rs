use std::path::PathBuf;

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
pub struct BundleRunReport {
    pub bundle_ref: String,
    pub output_file: PathBuf,
    pub digest: String,
    pub run_id: underlay_core::Uuid,
    pub status: String,
}
