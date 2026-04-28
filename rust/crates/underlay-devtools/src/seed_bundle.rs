//! Seed-data OCI bundle build/publish/pull.
//!
//! Seed bundles package SQL files from a seed-bundle directory into OCI bundles
//! for portable distribution. Each SQL file becomes a DataChunk layer with ordering
//! annotations. Unlike migration bundles, seed bundles have no media shards or
//! decision indexes — just SQL.
//!
//! Reuses the same `BundlePackage` JSON envelope and local OCI store infrastructure
//! as migration bundles, with a distinct artifact type.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_migration_core::{OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind};

use crate::migration_bundle::{
    migration_bundle_pull, BundlePublishOptions, BundlePublishReport, BundlePullOptions,
    MigrationBundleError,
};

const SHA256_PREFIX: &str = "sha256:";
const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";
const SEED_ARTIFACT_TYPE: &str = "application/vnd.underlay.seed.bundle.v1";

// ── Build options & report ─────────────────────────────────────────────

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

// ── Pull options for seed-specific extraction ──────────────────────────

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

// ── Internal types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct BundlePackage {
    pub schema_version: String,
    pub layout: OciBundleLayout,
    pub payloads: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SeedManifest {
    pub schema_version: String,
    pub name: String,
    pub priority: u32,
    pub tables: Vec<String>,
    pub idempotent_strategy: String,
    #[serde(default)]
    pub generated_at: String,
}

// ── Build ──────────────────────────────────────────────────────────────

pub fn seed_bundle_build(
    options: &SeedBundleBuildOptions,
) -> Result<SeedBundleBuildReport, MigrationBundleError> {
    if !options.source_dir.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "source directory does not exist: {}",
            options.source_dir.display()
        )));
    }

    // Read manifest.json
    let manifest_path = options.source_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "no manifest.json in seed-bundle directory: {}",
            options.source_dir.display()
        )));
    }
    let manifest_bytes = std::fs::read(&manifest_path)?;
    let seed_manifest: SeedManifest = serde_json::from_slice(&manifest_bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid seed manifest.json: {err}"))
    })?;

    // Collect SQL files in filename order
    let mut sql_files: Vec<PathBuf> = std::fs::read_dir(&options.source_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
        .collect();
    sql_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    if sql_files.is_empty() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "no SQL files found in seed-bundle directory: {}",
            options.source_dir.display()
        )));
    }

    // Build OCI config
    let config = OciBundleConfig {
        schema_version: "1".to_string(),
        bundle_id: underlay_core::Uuid::new_v7().to_string(),
        bundle_version: "v0-local".to_string(),
        source_system: "syllabus_generator".to_string(),
        target_schema_version: format!("farmyard-seed-{}", seed_manifest.name),
    };

    // Create manifest layer (the seed manifest itself)
    let manifest_layer_payload = serde_json::to_vec(&serde_json::json!({
        "schema_version": config.schema_version,
        "bundle_id": config.bundle_id,
        "seed_bundle_name": seed_manifest.name,
        "seed_bundle_priority": seed_manifest.priority,
        "tables": seed_manifest.tables,
        "idempotent_strategy": seed_manifest.idempotent_strategy,
        "generated_at": seed_manifest.generated_at,
        "packaged_at": Utc::now().to_rfc3339(),
    }))
    .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;

    let manifest_layer = layer_descriptor(
        OciLayerKind::Manifest,
        "application/vnd.underlay.seed.manifest.v1+json",
        &manifest_layer_payload,
    );

    let mut layers = vec![manifest_layer.clone()];
    let mut payloads = BTreeMap::new();
    payloads.insert(
        manifest_layer.digest.clone(),
        BASE64.encode(&manifest_layer_payload),
    );

    let mut total_sql_bytes: u64 = 0;
    let sql_file_count = sql_files.len();

    // Create a DataChunk layer for each SQL file
    for (order, sql_path) in sql_files.iter().enumerate() {
        let sql_content = std::fs::read(sql_path)?;
        total_sql_bytes += sql_content.len() as u64;

        let filename = sql_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown.sql")
            .to_string();

        let mut annotations = BTreeMap::new();
        annotations.insert("underlay.seed.file_name".to_string(), filename);
        annotations.insert("underlay.seed.apply_order".to_string(), order.to_string());

        let layer = layer_descriptor_with_annotations(
            OciLayerKind::DataChunk,
            "application/sql",
            &sql_content,
            annotations,
        );

        payloads.insert(layer.digest.clone(), BASE64.encode(&sql_content));
        layers.push(layer);
    }

    let layout = OciBundleLayout {
        artifact_type: SEED_ARTIFACT_TYPE.to_string(),
        media_type: OCI_MANIFEST_MEDIA_TYPE.to_string(),
        config,
        layers,
        sidecars: Vec::new(),
    };

    let package = BundlePackage {
        schema_version: "1".to_string(),
        layout,
        payloads,
    };

    // Write bundle file
    ensure_parent_dir(&options.output_file)?;
    let encoded = serde_json::to_vec_pretty(&package)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(&options.output_file, &encoded)?;

    let bundle_digest = sha256_digest(&encoded);

    Ok(SeedBundleBuildReport {
        output_file: options.output_file.clone(),
        bundle_name: seed_manifest.name,
        artifact_type: SEED_ARTIFACT_TYPE.to_string(),
        layer_count: package.layout.layers.len(),
        bundle_digest,
        sql_file_count,
        total_sql_bytes,
    })
}

// ── Publish (delegates to migration_bundle publish) ────────────────────

/// Publish a seed bundle to the local OCI store or remote registry.
///
/// This reuses the migration bundle publish path since the `BundlePackage`
/// format is identical.
pub fn seed_bundle_publish(
    options: &BundlePublishOptions,
) -> Result<BundlePublishReport, MigrationBundleError> {
    crate::migration_bundle::migration_bundle_publish(options)
}

// ── Pull (extracts SQL files to directory) ─────────────────────────────

/// Pull a seed bundle and extract SQL files to the output directory.
///
/// Unlike migration pull (which writes bundle.json + media-shards), seed pull
/// reconstructs the seed-bundle directory structure: manifest.json + SQL files.
pub fn seed_bundle_pull(
    options: &SeedBundlePullOptions,
) -> Result<SeedBundlePullReport, MigrationBundleError> {
    if options.oci_ref.trim().is_empty() {
        return Err(MigrationBundleError::InvalidInput(
            "oci_ref must not be empty".to_string(),
        ));
    }

    // Use migration pull to get the bundle package
    let pull_report = migration_bundle_pull(&BundlePullOptions {
        oci_ref: options.oci_ref.clone(),
        output_dir: options.output_dir.clone(),
        local_store_dir: options.local_store_dir.clone(),
    })?;

    // Re-read the pulled bundle to extract SQL files
    let bundle_json_path = options.output_dir.join("bundle.json");
    if !bundle_json_path.exists() {
        return Err(MigrationBundleError::Validation(
            "pulled bundle.json not found".to_string(),
        ));
    }

    // Read the original blob to get payloads (bundle.json only has the layout)
    let store = resolve_local_store_dir(options.local_store_dir.as_ref());
    let digest = resolve_digest(&store, &options.oci_ref)?;
    let blob_path = blob_path_for_digest(&store, &digest);
    let blob_bytes = std::fs::read(&blob_path)?;
    let package: BundlePackage = serde_json::from_slice(&blob_bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid bundle package JSON: {err}"))
    })?;

    // Extract manifest layer to manifest.json
    for layer in &package.layout.layers {
        if layer.kind == OciLayerKind::Manifest {
            let payload = decode_payload(&package.payloads, &layer.digest)?;
            let manifest_data: serde_json::Value =
                serde_json::from_slice(&payload).map_err(|err| {
                    MigrationBundleError::Validation(format!("invalid seed manifest: {err}"))
                })?;

            // Reconstruct manifest.json from the manifest layer
            let name = manifest_data["seed_bundle_name"]
                .as_str()
                .unwrap_or("unknown");
            let priority = manifest_data["seed_bundle_priority"]
                .as_u64()
                .unwrap_or(999);
            let tables = manifest_data["tables"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let strategy = manifest_data["idempotent_strategy"]
                .as_str()
                .unwrap_or("delete_insert");
            let generated_at = manifest_data["generated_at"].as_str().unwrap_or("");

            let seed_manifest = SeedManifest {
                schema_version: "1".to_string(),
                name: name.to_string(),
                priority: priority as u32,
                tables,
                idempotent_strategy: strategy.to_string(),
                generated_at: generated_at.to_string(),
            };

            let manifest_json = serde_json::to_vec_pretty(&seed_manifest)
                .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
            std::fs::write(options.output_dir.join("manifest.json"), manifest_json)?;
        }
    }

    // Extract SQL files from DataChunk layers
    let mut sql_file_count = 0;
    for layer in &package.layout.layers {
        if layer.kind == OciLayerKind::DataChunk {
            let payload = decode_payload(&package.payloads, &layer.digest)?;
            let filename = layer
                .annotations
                .get("underlay.seed.file_name")
                .cloned()
                .unwrap_or_else(|| format!("chunk_{}.sql", sql_file_count));

            std::fs::write(options.output_dir.join(&filename), &payload)?;
            sql_file_count += 1;
        }
    }

    // Clean up the intermediate bundle.json and media-shards from migration pull
    let _ = std::fs::remove_file(options.output_dir.join("bundle.json"));
    let _ = std::fs::remove_dir_all(options.output_dir.join("media-shards"));

    Ok(SeedBundlePullReport {
        oci_ref: options.oci_ref.clone(),
        output_dir: options.output_dir.clone(),
        digest: pull_report.digest,
        sql_file_count,
        status: pull_report.status,
    })
}

// ── Helpers ────────────────────────────────────────────────────────────

fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{SHA256_PREFIX}{:x}", digest)
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

fn ensure_parent_dir(path: &Path) -> Result<(), MigrationBundleError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
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

fn resolve_local_store_dir(local_store_dir: Option<&PathBuf>) -> PathBuf {
    if let Some(dir) = local_store_dir {
        return dir.clone();
    }
    if let Ok(env_dir) = std::env::var("UNDERLAY_LOCAL_OCI_DIR") {
        return PathBuf::from(env_dir);
    }
    PathBuf::from(".underlay-local-oci")
}

fn resolve_digest(store: &Path, oci_ref: &str) -> Result<String, MigrationBundleError> {
    // Check for digest-pinned ref first
    if let Some(digest) = extract_digest_from_ref(oci_ref) {
        return Ok(digest);
    }

    // Look up tag in refs/
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

    payload
        .get("digest")
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| {
            MigrationBundleError::Validation(format!(
                "ref mapping missing digest field: {}",
                ref_path.display()
            ))
        })
}

fn extract_digest_from_ref(oci_ref: &str) -> Option<String> {
    let (_, digest) = oci_ref.split_once('@')?;
    if !digest.starts_with(SHA256_PREFIX) {
        return None;
    }
    Some(digest.to_string())
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
