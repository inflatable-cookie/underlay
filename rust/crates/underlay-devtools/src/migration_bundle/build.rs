use std::collections::BTreeMap;
use std::path::Path;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use underlay_migration_core::{
    OciBundleConfig, OciBundleLayout, OciLayerKind, OciSidecarDescriptor,
};

use super::package::{layer_descriptor, layer_descriptor_with_annotations};
use super::{
    media_shards, sha256_digest, validate_bundle_package, BundleBuildOptions, BundleBuildReport,
    BundlePackage, MigrationBundleError, DEFAULT_MEDIA_SHARD_MAX_BYTES, OCI_MANIFEST_MEDIA_TYPE,
};

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

fn ensure_parent_dir(path: &Path) -> Result<(), MigrationBundleError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}
