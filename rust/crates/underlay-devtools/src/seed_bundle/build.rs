use std::collections::BTreeMap;
use std::path::PathBuf;

use chrono::Utc;
use underlay_migration_core::{OciBundleConfig, OciBundleLayout, OciLayerKind};

use super::model::{SeedBundleBuildOptions, SeedBundleBuildReport, SeedManifest};
use super::package::{
    encode_payload, ensure_parent_dir, layer_descriptor, layer_descriptor_with_annotations,
    sha256_digest, BundlePackage, OCI_MANIFEST_MEDIA_TYPE, SEED_ARTIFACT_TYPE,
};
use crate::migration_bundle::MigrationBundleError;

pub fn seed_bundle_build(
    options: &SeedBundleBuildOptions,
) -> Result<SeedBundleBuildReport, MigrationBundleError> {
    if !options.source_dir().exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "source directory does not exist: {}",
            options.source_dir().display()
        )));
    }

    let manifest_path = options.source_dir().join("manifest.json");
    if !manifest_path.exists() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "no manifest.json in seed-bundle directory: {}",
            options.source_dir().display()
        )));
    }
    let manifest_bytes = std::fs::read(&manifest_path)?;
    let seed_manifest: SeedManifest = serde_json::from_slice(&manifest_bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid seed manifest.json: {err}"))
    })?;

    let sql_files = collect_sql_files(options.source_dir())?;
    if sql_files.is_empty() {
        return Err(MigrationBundleError::InvalidInput(format!(
            "no SQL files found in seed-bundle directory: {}",
            options.source_dir().display()
        )));
    }

    let config = OciBundleConfig {
        schema_version: "1".to_string(),
        bundle_id: underlay_core::Uuid::new_v7().to_string(),
        bundle_version: "v0-local".to_string(),
        source_system: "syllabus_generator".to_string(),
        target_schema_version: format!("farmyard-seed-{}", seed_manifest.name),
    };

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
        encode_payload(&manifest_layer_payload),
    );

    let mut total_sql_bytes: u64 = 0;
    let sql_file_count = sql_files.len();

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

        payloads.insert(layer.digest.clone(), encode_payload(&sql_content));
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

    ensure_parent_dir(options.output_file())?;
    let encoded = serde_json::to_vec_pretty(&package)
        .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
    std::fs::write(options.output_file(), &encoded)?;

    let bundle_digest = sha256_digest(&encoded);

    Ok(SeedBundleBuildReport {
        output_file: options.output_file().clone(),
        bundle_name: seed_manifest.name,
        artifact_type: SEED_ARTIFACT_TYPE.to_string(),
        layer_count: package.layout.layers.len(),
        bundle_digest,
        sql_file_count,
        total_sql_bytes,
    })
}

fn collect_sql_files(source_dir: &std::path::Path) -> Result<Vec<PathBuf>, MigrationBundleError> {
    let mut sql_files: Vec<PathBuf> = std::fs::read_dir(source_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
        .collect();
    sql_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(sql_files)
}
