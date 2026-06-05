use underlay_migration_core::OciLayerKind;

use super::model::{SeedBundlePullOptions, SeedBundlePullReport, SeedManifest};
use super::package::{decode_payload, BundlePackage};
use crate::migration_bundle::{
    local_store, migration_bundle_pull, BundlePullOptions, MigrationBundleError,
};

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

    let pull_report = migration_bundle_pull(&BundlePullOptions {
        oci_ref: options.oci_ref.clone(),
        output_dir: options.output_dir.clone(),
        local_store_dir: options.local_store_dir.clone(),
    })?;

    let bundle_json_path = options.output_dir.join("bundle.json");
    if !bundle_json_path.exists() {
        return Err(MigrationBundleError::Validation(
            "pulled bundle.json not found".to_string(),
        ));
    }

    let store = local_store::resolve_local_store_dir(options.local_store_dir.as_ref())?;
    let digest = local_store::resolve_digest(&store, &options.oci_ref)?;
    let blob_path = store.blob_path_for_digest(&digest)?;
    let blob_bytes = std::fs::read(&blob_path)?;
    let package: BundlePackage = serde_json::from_slice(&blob_bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid bundle package JSON: {err}"))
    })?;

    extract_manifest(options, &package)?;
    let sql_file_count = extract_sql_files(options, &package)?;

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

fn extract_manifest(
    options: &SeedBundlePullOptions,
    package: &BundlePackage,
) -> Result<(), MigrationBundleError> {
    for layer in &package.layout.layers {
        if layer.kind == OciLayerKind::Manifest {
            let payload = decode_payload(&package.payloads, &layer.digest)?;
            let manifest_data: serde_json::Value =
                serde_json::from_slice(&payload).map_err(|err| {
                    MigrationBundleError::Validation(format!("invalid seed manifest: {err}"))
                })?;

            let seed_manifest = SeedManifest {
                schema_version: "1".to_string(),
                name: manifest_data["seed_bundle_name"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string(),
                priority: manifest_data["seed_bundle_priority"]
                    .as_u64()
                    .unwrap_or(999) as u32,
                tables: manifest_data["tables"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                idempotent_strategy: manifest_data["idempotent_strategy"]
                    .as_str()
                    .unwrap_or("delete_insert")
                    .to_string(),
                generated_at: manifest_data["generated_at"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            };

            let manifest_json = serde_json::to_vec_pretty(&seed_manifest)
                .map_err(|err| MigrationBundleError::Validation(err.to_string()))?;
            std::fs::write(options.output_dir.join("manifest.json"), manifest_json)?;
        }
    }
    Ok(())
}

fn extract_sql_files(
    options: &SeedBundlePullOptions,
    package: &BundlePackage,
) -> Result<usize, MigrationBundleError> {
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
    Ok(sql_file_count)
}
