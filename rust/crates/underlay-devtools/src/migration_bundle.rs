use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, LOCATION};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_media::version_key;
use underlay_migration_core::{
    validate_oci_bundle_layout, OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind,
    OciSidecarDescriptor,
};

const SHA256_PREFIX: &str = "sha256:";
const DEFAULT_MEDIA_SHARD_MAX_BYTES: u64 = 16 * 1024 * 1024;
const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";
const OCI_CONFIG_MEDIA_TYPE: &str = "application/vnd.underlay.migration.bundle.config.v1+json";
const OCI_PACKAGE_LAYER_MEDIA_TYPE: &str = "application/vnd.underlay.bundle.package.v1+json";

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

#[derive(Debug, Clone)]
pub struct BundleRunReport {
    pub bundle_ref: String,
    pub output_file: PathBuf,
    pub digest: String,
    pub run_id: underlay_core::Uuid,
    pub status: String,
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
struct BundlePackage {
    pub schema_version: String,
    pub layout: OciBundleLayout,
    pub payloads: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MediaKeyMapping {
    pub media_id: String,
    pub version_id: String,
    pub object_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MediaAssetPayload {
    pub relative_path: String,
    pub filename: String,
    pub byte_size: u64,
    pub sha256: String,
    pub content_base64: String,
    pub mapping: MediaKeyMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MediaShardPayload {
    pub schema_version: String,
    pub shard_id: String,
    pub assets: Vec<MediaAssetPayload>,
}

#[derive(Debug, Clone)]
struct RawMediaEntry {
    relative_path: String,
    filename: String,
    bytes: Vec<u8>,
    sha256: String,
}

#[derive(Debug, Clone)]
struct RemoteRegistryRef {
    registry: String,
    repository: String,
    reference: String,
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

    let media_entries = collect_media_entries(options.media_dir.as_ref())?;
    let shard_max_bytes = options
        .media_shard_max_bytes
        .unwrap_or(DEFAULT_MEDIA_SHARD_MAX_BYTES)
        .max(1);
    let media_shards = build_media_shards(&media_entries, shard_max_bytes)?;

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
        annotations.insert("underlay.shard_id".to_string(), shard.shard_id.clone());
        annotations.insert(
            "underlay.media_asset_count".to_string(),
            shard.assets.len().to_string(),
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
    if let Some(ref_digest) = extract_digest_from_ref(&options.oci_ref) {
        if ref_digest != digest {
            return Err(MigrationBundleError::Validation(format!(
                "oci_ref digest mismatch: ref={}, actual={}",
                ref_digest, digest
            )));
        }
    }

    if options.local_store_dir.is_none() && is_remote_ref(&options.oci_ref) {
        return remote_publish(options, &bytes);
    }

    let store = resolve_local_store_dir(options.local_store_dir.as_ref());
    let blob_path = blob_path_for_digest(&store, &digest);
    if let Some(parent) = blob_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&blob_path, &bytes)?;

    let ref_dir = store.join("refs");
    std::fs::create_dir_all(&ref_dir)?;
    let ref_path = ref_dir.join(format!("{}.json", sanitize_ref(&options.oci_ref)));
    let ref_payload = serde_json::to_vec_pretty(&serde_json::json!({
        "oci_ref": options.oci_ref,
        "digest": digest,
        "blob_path": blob_path,
        "published_at": Utc::now(),
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(ref_path, ref_payload)?;

    Ok(BundlePublishReport {
        bundle_file: options.bundle_file.clone(),
        oci_ref: options.oci_ref.clone(),
        digest,
        status: "published-local".to_string(),
    })
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

    if options.local_store_dir.is_none() && is_remote_ref(&options.oci_ref) {
        return remote_pull(options);
    }

    let store = resolve_local_store_dir(options.local_store_dir.as_ref());
    let digest = match extract_digest_from_ref(&options.oci_ref) {
        Some(digest) => digest,
        None => resolve_ref_digest(&store, &options.oci_ref)?,
    };

    let blob_path = blob_path_for_digest(&store, &digest);
    if !blob_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "bundle blob not found for digest {}",
            digest
        )));
    }

    let bytes = std::fs::read(&blob_path)?;
    let actual_digest = sha256_digest(&bytes);
    if actual_digest != digest {
        return Err(MigrationBundleError::Validation(format!(
            "bundle blob digest mismatch: expected {}, found {}",
            digest, actual_digest
        )));
    }

    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;

    let output_file = write_pulled_outputs(&package, &options.output_dir)?;

    Ok(BundlePullReport {
        oci_ref: options.oci_ref.clone(),
        output_file,
        digest,
        status: "pulled-local".to_string(),
    })
}

pub fn migration_run(options: &BundleRunOptions) -> Result<BundleRunReport, MigrationBundleError> {
    let requested_digest = extract_digest_from_ref(&options.bundle_ref).ok_or_else(|| {
        MigrationBundleError::InvalidInput(
            "migration run requires digest-pinned --bundle <ref@sha256:...>".to_string(),
        )
    })?;

    let pull = migration_bundle_pull(&BundlePullOptions {
        oci_ref: options.bundle_ref.clone(),
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
        bundle_ref: options.bundle_ref.clone(),
        output_file: pull.output_file,
        digest: pull.digest,
        run_id: underlay_core::Uuid::new_v7(),
        status: "prepared".to_string(),
    })
}

fn is_remote_ref(oci_ref: &str) -> bool {
    oci_ref.starts_with("http://") || oci_ref.starts_with("https://")
}

fn remote_publish(
    options: &BundlePublishOptions,
    package_bytes: &[u8],
) -> Result<BundlePublishReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(&options.oci_ref)?;
    if remote_ref.reference.starts_with(SHA256_PREFIX) {
        return Err(MigrationBundleError::InvalidInput(
            "remote publish requires a tag reference, not digest".to_string(),
        ));
    }

    let package_digest = sha256_digest(package_bytes);
    let package = decode_package(package_bytes)?;
    let config_bytes = serde_json::to_vec(&package.layout.config)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    let config_digest = sha256_digest(&config_bytes);

    let client = Client::builder()
        .build()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    ping_registry(&client, &remote_ref.registry)?;
    upload_blob(
        &client,
        &remote_ref.registry,
        &remote_ref.repository,
        &config_digest,
        &config_bytes,
    )?;
    upload_blob(
        &client,
        &remote_ref.registry,
        &remote_ref.repository,
        &package_digest,
        package_bytes,
    )?;

    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "mediaType": OCI_MANIFEST_MEDIA_TYPE,
        "config": {
            "mediaType": OCI_CONFIG_MEDIA_TYPE,
            "digest": config_digest,
            "size": config_bytes.len()
        },
        "layers": [{
            "mediaType": OCI_PACKAGE_LAYER_MEDIA_TYPE,
            "digest": package_digest,
            "size": package_bytes.len()
        }]
    });
    let manifest_bytes = serde_json::to_vec(&manifest)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let manifest_url = format!(
        "{}/v2/{}/manifests/{}",
        remote_ref.registry, remote_ref.repository, remote_ref.reference
    );

    let response = client
        .put(&manifest_url)
        .header(CONTENT_TYPE, OCI_MANIFEST_MEDIA_TYPE)
        .body(manifest_bytes)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry manifest publish failed: status={}",
            response.status()
        )));
    }

    let manifest_digest = response
        .headers()
        .get("docker-content-digest")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or_default();

    let digest = if manifest_digest.starts_with(SHA256_PREFIX) {
        manifest_digest
    } else {
        package_digest
    };

    Ok(BundlePublishReport {
        bundle_file: options.bundle_file.clone(),
        oci_ref: options.oci_ref.clone(),
        digest,
        status: "published-remote".to_string(),
    })
}

fn remote_pull(options: &BundlePullOptions) -> Result<BundlePullReport, MigrationBundleError> {
    let remote_ref = parse_remote_ref(&options.oci_ref)?;
    let client = Client::builder()
        .build()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    ping_registry(&client, &remote_ref.registry)?;

    let manifest_url = format!(
        "{}/v2/{}/manifests/{}",
        remote_ref.registry, remote_ref.repository, remote_ref.reference
    );
    let response = client
        .get(&manifest_url)
        .header(
            ACCEPT,
            "application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json",
        )
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry manifest fetch failed: status={}",
            response.status()
        )));
    }

    let manifest_digest = response
        .headers()
        .get("docker-content-digest")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or_default();

    let manifest: serde_json::Value = response
        .json()
        .map_err(|err| MigrationBundleError::Validation(format!("invalid manifest JSON: {err}")))?;

    let layer_digest = manifest
        .get("layers")
        .and_then(|v| v.as_array())
        .and_then(|layers| layers.first())
        .and_then(|layer| layer.get("digest"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            MigrationBundleError::Validation("manifest missing primary package layer".to_string())
        })?
        .to_string();

    let blob_url = format!(
        "{}/v2/{}/blobs/{}",
        remote_ref.registry, remote_ref.repository, layer_digest
    );
    let blob_response = client
        .get(&blob_url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    if !blob_response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry blob fetch failed: status={}",
            blob_response.status()
        )));
    }
    let bytes = blob_response
        .bytes()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
        .to_vec();

    let actual_layer_digest = sha256_digest(&bytes);
    if actual_layer_digest != layer_digest {
        return Err(MigrationBundleError::Validation(format!(
            "remote blob digest mismatch: expected {}, found {}",
            layer_digest, actual_layer_digest
        )));
    }

    let package = decode_package(&bytes)?;
    validate_bundle_package(&package)?;
    let output_file = write_pulled_outputs(&package, &options.output_dir)?;

    let digest = if manifest_digest.starts_with(SHA256_PREFIX) {
        manifest_digest
    } else {
        layer_digest
    };

    Ok(BundlePullReport {
        oci_ref: options.oci_ref.clone(),
        output_file,
        digest,
        status: "pulled-remote".to_string(),
    })
}

fn parse_remote_ref(input: &str) -> Result<RemoteRegistryRef, MigrationBundleError> {
    let url = reqwest::Url::parse(input).map_err(|err| {
        MigrationBundleError::InvalidInput(format!("invalid remote oci_ref URL: {err}"))
    })?;
    let registry = format!(
        "{}://{}",
        url.scheme(),
        url.host_str()
            .ok_or_else(|| MigrationBundleError::InvalidInput(
                "remote oci_ref missing host".to_string()
            ))?
    );
    let registry = if let Some(port) = url.port() {
        format!("{registry}:{port}")
    } else {
        registry
    };

    let path = url.path().trim_start_matches('/');
    if path.is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "remote oci_ref must include repository and reference".to_string(),
        ));
    }

    if let Some((repository, digest)) = path.split_once('@') {
        if repository.is_empty() || digest.is_empty() {
            return Err(MigrationBundleError::InvalidInput(
                "remote oci_ref digest form must be <repo>@sha256:...".to_string(),
            ));
        }
        return Ok(RemoteRegistryRef {
            registry,
            repository: repository.to_string(),
            reference: digest.to_string(),
        });
    }

    let slash = path.rfind('/').unwrap_or(0);
    let colon = path.rfind(':').ok_or_else(|| {
        MigrationBundleError::InvalidInput(
            "remote oci_ref tag form must be <repo>:<tag>".to_string(),
        )
    })?;
    if colon <= slash || colon == path.len() - 1 {
        return Err(MigrationBundleError::InvalidInput(
            "remote oci_ref tag form must be <repo>:<tag>".to_string(),
        ));
    }

    Ok(RemoteRegistryRef {
        registry,
        repository: path[..colon].to_string(),
        reference: path[colon + 1..].to_string(),
    })
}

fn ping_registry(client: &Client, registry: &str) -> Result<(), MigrationBundleError> {
    let url = format!("{registry}/v2/");
    let response = client
        .get(url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    if !response.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "registry ping failed: status={}",
            response.status()
        )));
    }
    Ok(())
}

fn upload_blob(
    client: &Client,
    registry: &str,
    repository: &str,
    digest: &str,
    bytes: &[u8],
) -> Result<(), MigrationBundleError> {
    let start_url = format!("{registry}/v2/{repository}/blobs/uploads/");
    let start = client
        .post(&start_url)
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if start.status() != reqwest::StatusCode::ACCEPTED {
        return Err(MigrationBundleError::Validation(format!(
            "blob upload start failed: status={}",
            start.status()
        )));
    }

    let location = start
        .headers()
        .get(LOCATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            MigrationBundleError::Validation("registry missing upload location".to_string())
        })?;

    let mut upload_url = if location.starts_with("http://") || location.starts_with("https://") {
        reqwest::Url::parse(location)
            .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
    } else {
        reqwest::Url::parse(&format!("{registry}{location}"))
            .map_err(|err| MigrationBundleError::Validation(err.to_string()))?
    };
    upload_url.query_pairs_mut().append_pair("digest", digest);

    let finish = client
        .put(upload_url)
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(bytes.to_vec())
        .send()
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    if !finish.status().is_success() {
        return Err(MigrationBundleError::Validation(format!(
            "blob upload finish failed for {}: status={}",
            digest,
            finish.status()
        )));
    }

    Ok(())
}

fn write_pulled_outputs(
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
            .unwrap_or_else(|| sanitize_ref(&layer.digest));
        let media_out = media_dir.join(format!("{shard_id}.json"));
        std::fs::write(media_out, payload)?;
    }

    Ok(output_file)
}

fn validate_bundle_package(package: &BundlePackage) -> Result<(), MigrationBundleError> {
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
            validate_media_shard_payload(&payload)?;
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

fn validate_media_shard_payload(bytes: &[u8]) -> Result<(), MigrationBundleError> {
    let shard: MediaShardPayload = serde_json::from_slice(bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid media shard payload JSON: {err}"))
    })?;

    for asset in shard.assets {
        let content = BASE64.decode(asset.content_base64).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid base64 content for media asset {}: {err}",
                asset.relative_path
            ))
        })?;

        if content.len() as u64 != asset.byte_size {
            return Err(MigrationBundleError::Validation(format!(
                "media asset size mismatch for {}: expected {}, found {}",
                asset.relative_path,
                asset.byte_size,
                content.len()
            )));
        }

        let actual = sha256_digest(&content);
        if actual != asset.sha256 {
            return Err(MigrationBundleError::Validation(format!(
                "media asset digest mismatch for {}: expected {}, found {}",
                asset.relative_path, asset.sha256, actual
            )));
        }

        let media_id = uuid::Uuid::parse_str(&asset.mapping.media_id).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid mapping media_id for {}: {err}",
                asset.relative_path
            ))
        })?;
        let version_id = uuid::Uuid::parse_str(&asset.mapping.version_id).map_err(|err| {
            MigrationBundleError::Validation(format!(
                "invalid mapping version_id for {}: {err}",
                asset.relative_path
            ))
        })?;

        let expected_key = version_key(media_id, version_id, &asset.filename);
        if expected_key != asset.mapping.object_key {
            return Err(MigrationBundleError::Validation(format!(
                "mapping object_key mismatch for {}: expected {}, found {}",
                asset.relative_path, expected_key, asset.mapping.object_key
            )));
        }
    }

    Ok(())
}

fn build_media_shards(
    entries: &[RawMediaEntry],
    shard_max_bytes: u64,
) -> Result<Vec<MediaShardPayload>, MigrationBundleError> {
    let mut shards = Vec::new();

    if entries.is_empty() {
        shards.push(MediaShardPayload {
            schema_version: "1".to_string(),
            shard_id: "media-0001".to_string(),
            assets: Vec::new(),
        });
        return Ok(shards);
    }

    let mut current_assets: Vec<MediaAssetPayload> = Vec::new();
    let mut current_bytes: u64 = 0;
    let mut shard_index: u64 = 1;

    for entry in entries {
        let byte_size = entry.bytes.len() as u64;
        let should_rotate =
            !current_assets.is_empty() && current_bytes + byte_size > shard_max_bytes;

        if should_rotate {
            shards.push(MediaShardPayload {
                schema_version: "1".to_string(),
                shard_id: format!("media-{shard_index:04}"),
                assets: current_assets,
            });
            shard_index += 1;
            current_assets = Vec::new();
            current_bytes = 0;
        }

        let media_uuid = deterministic_uuid_from_seed(&format!("media:{}", entry.sha256));
        let version_uuid = deterministic_uuid_from_seed(&format!("version:{}", entry.sha256));

        current_assets.push(MediaAssetPayload {
            relative_path: entry.relative_path.clone(),
            filename: entry.filename.clone(),
            byte_size,
            sha256: entry.sha256.clone(),
            content_base64: BASE64.encode(&entry.bytes),
            mapping: MediaKeyMapping {
                media_id: media_uuid.to_string(),
                version_id: version_uuid.to_string(),
                object_key: version_key(media_uuid, version_uuid, &entry.filename),
            },
        });
        current_bytes += byte_size;
    }

    shards.push(MediaShardPayload {
        schema_version: "1".to_string(),
        shard_id: format!("media-{shard_index:04}"),
        assets: current_assets,
    });

    Ok(shards)
}

fn collect_media_entries(
    media_dir: Option<&PathBuf>,
) -> Result<Vec<RawMediaEntry>, MigrationBundleError> {
    let mut entries = Vec::new();

    if let Some(media_dir) = media_dir {
        if !media_dir.exists() {
            return Err(MigrationBundleError::InvalidInput(format!(
                "media_dir does not exist: {}",
                media_dir.display()
            )));
        }

        for file in collect_files_recursive(media_dir)? {
            let rel = file.strip_prefix(media_dir).map_err(|err| {
                MigrationBundleError::Validation(format!(
                    "failed to strip media_dir prefix for {}: {err}",
                    file.display()
                ))
            })?;
            let bytes = std::fs::read(&file)?;
            let filename = file
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| {
                    MigrationBundleError::Validation(format!(
                        "media filename is not valid UTF-8: {}",
                        file.display()
                    ))
                })?
                .to_string();

            entries.push(RawMediaEntry {
                relative_path: rel.to_string_lossy().to_string(),
                filename,
                sha256: sha256_digest(&bytes),
                bytes,
            });
        }
    }

    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(entries)
}

fn deterministic_uuid_from_seed(seed: &str) -> uuid::Uuid {
    let hash = Sha256::digest(seed.as_bytes());
    let mut bytes: [u8; 16] = hash[..16]
        .try_into()
        .expect("sha256 prefix should have 16 bytes");

    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    uuid::Uuid::from_bytes(bytes)
}

fn collect_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, MigrationBundleError> {
    let mut files = Vec::new();
    visit_dir(dir, &mut files)?;
    files.sort();
    Ok(files)
}

fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), MigrationBundleError> {
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dir(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn decode_package(bytes: &[u8]) -> Result<BundlePackage, MigrationBundleError> {
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

fn resolve_local_store_dir(local_store_dir: Option<&PathBuf>) -> PathBuf {
    if let Some(dir) = local_store_dir {
        return dir.clone();
    }

    if let Ok(env_dir) = std::env::var("UNDERLAY_LOCAL_OCI_DIR") {
        return PathBuf::from(env_dir);
    }

    PathBuf::from(".underlay-local-oci")
}

fn resolve_ref_digest(store: &Path, oci_ref: &str) -> Result<String, MigrationBundleError> {
    let ref_path = store
        .join("refs")
        .join(format!("{}.json", sanitize_ref(oci_ref)));
    if !ref_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "oci_ref not found in local store: {}",
            oci_ref
        )));
    }

    let bytes = std::fs::read(&ref_path)?;
    let payload: serde_json::Value = serde_json::from_slice(&bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid ref mapping JSON: {err}"))
    })?;

    let digest = payload
        .get("digest")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            MigrationBundleError::Validation(format!(
                "ref mapping missing digest field: {}",
                ref_path.display()
            ))
        })?;

    Ok(digest.to_string())
}

fn blob_path_for_digest(store: &Path, digest: &str) -> PathBuf {
    let digest_hex = digest.strip_prefix(SHA256_PREFIX).unwrap_or(digest);
    store
        .join("blobs")
        .join("sha256")
        .join(format!("{digest_hex}.json"))
}

fn sanitize_ref(oci_ref: &str) -> String {
    oci_ref
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn extract_digest_from_ref(oci_ref: &str) -> Option<String> {
    let (_, digest) = oci_ref.split_once('@')?;
    if !digest.starts_with(SHA256_PREFIX) {
        return None;
    }
    Some(digest.to_string())
}

fn sha256_digest(bytes: &[u8]) -> String {
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
