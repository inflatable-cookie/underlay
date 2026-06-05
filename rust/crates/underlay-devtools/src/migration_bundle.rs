use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_migration_core::{
    validate_oci_bundle_layout, OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind,
    OciSidecarDescriptor,
};

const SHA256_PREFIX: &str = "sha256:";
const DEFAULT_MEDIA_SHARD_MAX_BYTES: u64 = 16 * 1024 * 1024;
const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";

mod local_store;
mod media_shards;
mod remote_registry;

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

        let digest = local_store::extract_digest_from_ref(value).ok_or_else(|| {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) struct BundlePackage {
    pub schema_version: String,
    pub layout: OciBundleLayout,
    pub payloads: BTreeMap<String, String>,
}

pub fn migration_bundle_build(
    options: &BundleBuildOptions,
) -> Result<BundleBuildReport, MigrationBundleError> {
    if options.source_system.trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "source_system must not be empty".to_string(),
        ));
    }
    if options.target_schema_version.trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "target_schema_version must not be empty".to_string(),
        ));
    }

    ensure_parent_dir(&options.output_file)?;

    let config = OciBundleConfig {
        schema_version: "1".to_string(),
        bundle_id: underlay_core::Uuid::new_v7().to_string(),
        bundle_version: "v0-local".to_string(),
        source_system: options.source_system.clone(),
        target_schema_version: options.target_schema_version.clone(),
    };

    let manifest_payload = serde_json::to_vec(&serde_json::json!({
        "schema_version": config.schema_version,
        "bundle_id": config.bundle_id,
        "bundle_version": config.bundle_version,
        "source_system": config.source_system,
        "target_schema_version": config.target_schema_version,
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let data_chunk_payload = serde_json::to_vec(&serde_json::json!({
        "chunk_id": "chunk-0001",
        "record_count": 0,
        "source_system": options.source_system,
        "target_schema_version": options.target_schema_version,
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let media_entries = media_shards::collect_media_entries(options.media_dir.as_ref())?;
    let shard_max_bytes = options
        .media_shard_max_bytes
        .unwrap_or(DEFAULT_MEDIA_SHARD_MAX_BYTES)
        .max(1);
    let media_shards = media_shards::build_media_shards(&media_entries, shard_max_bytes)?;

    let sidecar_payload = serde_json::to_vec(&serde_json::json!({
        "schema_version": "1",
        "entries": {}
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let manifest_layer = layer_descriptor(
        OciLayerKind::Manifest,
        "application/vnd.underlay.bundle.manifest.v1+json",
        &manifest_payload,
    );
    let data_layer = layer_descriptor(
        OciLayerKind::DataChunk,
        "application/vnd.underlay.bundle.data.chunk.v1+zstd",
        &data_chunk_payload,
    );

    let sidecar_descriptor = OciSidecarDescriptor {
        role: "decision_index".to_string(),
        artifact_type: "application/vnd.underlay.migration.decision-index.v1".to_string(),
        digest: sha256_digest(&sidecar_payload),
        media_type: "application/json".to_string(),
    };

    let mut layers = vec![manifest_layer.clone(), data_layer.clone()];
    let mut payloads = BTreeMap::new();
    payloads.insert(
        manifest_layer.digest.clone(),
        BASE64.encode(manifest_payload),
    );
    payloads.insert(data_layer.digest.clone(), BASE64.encode(data_chunk_payload));

    for shard in &media_shards {
        let shard_payload = serde_json::to_vec(shard)
            .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

        let mut annotations = BTreeMap::new();
        annotations.insert(
            "underlay.shard_id".to_string(),
            shard.shard_id().to_string(),
        );
        annotations.insert(
            "underlay.media_asset_count".to_string(),
            shard.asset_count().to_string(),
        );

        let layer = layer_descriptor_with_annotations(
            OciLayerKind::MediaShard,
            "application/vnd.underlay.bundle.media.shard.v1+json",
            &shard_payload,
            annotations,
        );

        payloads.insert(layer.digest.clone(), BASE64.encode(shard_payload));
        layers.push(layer);
    }

    payloads.insert(
        sidecar_descriptor.digest.clone(),
        BASE64.encode(sidecar_payload),
    );

    let layout = OciBundleLayout {
        artifact_type: "application/vnd.underlay.migration.bundle.v1".to_string(),
        media_type: OCI_MANIFEST_MEDIA_TYPE.to_string(),
        config,
        layers,
        sidecars: vec![sidecar_descriptor],
    };

    let package = BundlePackage {
        schema_version: "1".to_string(),
        layout,
        payloads,
    };

    validate_bundle_package(&package)?;

    let encoded = serde_json::to_vec_pretty(&package)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(&options.output_file, &encoded)?;

    let bundle_digest = sha256_digest(&encoded);

    Ok(BundleBuildReport {
        output_file: options.output_file.clone(),
        artifact_type: package.layout.artifact_type,
        layer_count: package.layout.layers.len(),
        sidecar_count: package.layout.sidecars.len(),
        bundle_digest,
        media_asset_count: media_entries.len(),
        media_shard_count: media_shards.len(),
    })
}

pub fn migration_bundle_publish(
    options: &BundlePublishOptions,
) -> Result<BundlePublishReport, MigrationBundleError> {
    if options.oci_ref.trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "oci_ref must not be empty".to_string(),
        ));
    }

    if !options.bundle_file.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "bundle file does not exist: {}",
            options.bundle_file.display()
        )));
    }

    let bytes = std::fs::read(&options.bundle_file)?;
    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;

    let digest = sha256_digest(&bytes);
    if let Some(ref_digest) = local_store::extract_digest_from_ref(&options.oci_ref) {
        if ref_digest != digest {
            return Err(MigrationBundleError::Validation(format!(
                "oci_ref digest mismatch: ref={}, actual={}",
                ref_digest, digest
            )));
        }
    }

    if options.local_store_dir.is_none() && remote_registry::is_remote_ref(&options.oci_ref) {
        return remote_registry::remote_publish(options, &bytes);
    }

    local_store::publish_local_bundle(options, &bytes, digest)
}

pub fn migration_bundle_pull(
    options: &BundlePullOptions,
) -> Result<BundlePullReport, MigrationBundleError> {
    if options.oci_ref.trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "oci_ref must not be empty".to_string(),
        ));
    }

    std::fs::create_dir_all(&options.output_dir)?;

    if options.local_store_dir.is_none() && remote_registry::is_remote_ref(&options.oci_ref) {
        return remote_registry::remote_pull(options);
    }

    local_store::pull_local_bundle(options)
}

pub fn migration_run(options: &BundleRunOptions) -> Result<BundleRunReport, MigrationBundleError> {
    let bundle_ref = options.bundle_ref()?;
    let requested_digest = bundle_ref.digest().to_string();

    let pull = migration_bundle_pull(&BundlePullOptions {
        oci_ref: bundle_ref.to_string(),
        output_dir: options.output_dir.clone(),
        local_store_dir: options.local_store_dir.clone(),
    })?;

    if pull.digest != requested_digest {
        return Err(MigrationBundleError::Validation(format!(
            "pulled digest mismatch for run: requested {}, resolved {}",
            requested_digest, pull.digest
        )));
    }

    Ok(BundleRunReport {
        bundle_ref: bundle_ref.to_string(),
        output_file: pull.output_file,
        digest: pull.digest,
        run_id: underlay_core::Uuid::new_v7(),
        status: "prepared".to_string(),
    })
}

pub(super) fn write_pulled_outputs(
    package: &BundlePackage,
    output_dir: &Path,
) -> Result<PathBuf, MigrationBundleError> {
    let output_file = output_dir.join("bundle.json");
    let layout_json = serde_json::to_vec_pretty(&package.layout)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(&output_file, layout_json)?;

    let media_dir = output_dir.join("media-shards");
    std::fs::create_dir_all(&media_dir)?;

    for layer in package
        .layout
        .layers
        .iter()
        .filter(|layer| layer.kind == OciLayerKind::MediaShard)
    {
        let payload = decode_payload(&package.payloads, &layer.digest)?;
        let shard_id = layer
            .annotations
            .get("underlay.shard_id")
            .cloned()
            .unwrap_or_else(|| local_store::sanitize_ref(&layer.digest));
        let media_out = media_dir.join(format!("{shard_id}.json"));
        std::fs::write(media_out, payload)?;
    }

    Ok(output_file)
}

pub(super) fn validate_bundle_package(package: &BundlePackage) -> Result<(), MigrationBundleError> {
    validate_oci_bundle_layout(&package.layout)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    for layer in &package.layout.layers {
        let payload = decode_payload(&package.payloads, &layer.digest)?;
        let computed = sha256_digest(&payload);
        if computed != layer.digest {
            return Err(MigrationBundleError::Validation(format!(
                "layer digest mismatch for {}: expected {}, found {}",
                layer.media_type, layer.digest, computed
            )));
        }

        if layer.kind == OciLayerKind::MediaShard {
            media_shards::validate_media_shard_payload(&payload)?;
        }
    }

    for sidecar in &package.layout.sidecars {
        let payload = decode_payload(&package.payloads, &sidecar.digest)?;
        let computed = sha256_digest(&payload);
        if computed != sidecar.digest {
            return Err(MigrationBundleError::Validation(format!(
                "sidecar digest mismatch for {}: expected {}, found {}",
                sidecar.role, sidecar.digest, computed
            )));
        }
    }

    Ok(())
}

pub(super) fn decode_package(bytes: &[u8]) -> Result<BundlePackage, MigrationBundleError> {
    serde_json::from_slice(bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid bundle package JSON: {err}"))
    })
}

fn decode_payload(
    payloads: &BTreeMap<String, String>,
    digest: &str,
) -> Result<Vec<u8>, MigrationBundleError> {
    let encoded = payloads.get(digest).ok_or_else(|| {
        MigrationBundleError::Validation(format!("missing payload for digest {}", digest))
    })?;

    BASE64.decode(encoded).map_err(|err| {
        MigrationBundleError::Validation(format!(
            "invalid payload base64 for digest {}: {err}",
            digest
        ))
    })
}

fn layer_descriptor(kind: OciLayerKind, media_type: &str, payload: &[u8]) -> OciLayerDescriptor {
    layer_descriptor_with_annotations(kind, media_type, payload, BTreeMap::new())
}

fn layer_descriptor_with_annotations(
    kind: OciLayerKind,
    media_type: &str,
    payload: &[u8],
    annotations: BTreeMap<String, String>,
) -> OciLayerDescriptor {
    OciLayerDescriptor {
        kind,
        media_type: media_type.to_string(),
        digest: sha256_digest(payload),
        size_bytes: payload.len() as u64,
        annotations,
    }
}

pub(super) fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{SHA256_PREFIX}{:x}", digest)
}

fn ensure_parent_dir(path: &Path) -> Result<(), MigrationBundleError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/migration_bundle_tests.rs"]
mod tests;
