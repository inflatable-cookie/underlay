use std::path::{Path, PathBuf};

use underlay_migration_core::OciLayerKind;

use super::package::decode_payload;
use super::{local_store, BundlePackage, MigrationBundleError};

pub(crate) fn write_pulled_outputs(
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
        let media_out = media_dir.join(format!("{}.json", local_store::sanitize_ref(&shard_id)));
        std::fs::write(media_out, payload)?;
    }

    Ok(output_file)
}
