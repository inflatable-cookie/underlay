use std::collections::BTreeMap;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_migration_core::{
    validate_oci_bundle_layout, OciBundleLayout, OciLayerDescriptor, OciLayerKind,
};

use super::{media_shards, MigrationBundleError, SHA256_PREFIX};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) struct BundlePackage {
    pub schema_version: String,
    pub layout: OciBundleLayout,
    pub payloads: BTreeMap<String, String>,
}

pub(crate) fn validate_bundle_package(package: &BundlePackage) -> Result<(), MigrationBundleError> {
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

pub(crate) fn decode_package(bytes: &[u8]) -> Result<BundlePackage, MigrationBundleError> {
    serde_json::from_slice(bytes).map_err(|err| {
        MigrationBundleError::Validation(format!("invalid bundle package JSON: {err}"))
    })
}

pub(super) fn decode_payload(
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

pub(super) fn layer_descriptor(
    kind: OciLayerKind,
    media_type: &str,
    payload: &[u8],
) -> OciLayerDescriptor {
    layer_descriptor_with_annotations(kind, media_type, payload, BTreeMap::new())
}

pub(super) fn layer_descriptor_with_annotations(
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

pub(crate) fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{SHA256_PREFIX}{}", hex::encode(digest))
}
