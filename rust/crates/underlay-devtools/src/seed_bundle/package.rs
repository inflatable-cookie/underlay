use std::collections::BTreeMap;
use std::path::Path;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use underlay_migration_core::{OciBundleLayout, OciLayerDescriptor, OciLayerKind};

use crate::migration_bundle::MigrationBundleError;

const SHA256_PREFIX: &str = "sha256:";
pub(super) const OCI_MANIFEST_MEDIA_TYPE: &str = "application/vnd.oci.image.manifest.v1+json";
pub(super) const SEED_ARTIFACT_TYPE: &str = "application/vnd.underlay.seed.bundle.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) struct BundlePackage {
    pub schema_version: String,
    pub layout: OciBundleLayout,
    pub payloads: BTreeMap<String, String>,
}

pub(super) fn encode_payload(payload: &[u8]) -> String {
    BASE64.encode(payload)
}

pub(super) fn sha256_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{SHA256_PREFIX}{}", hex::encode(digest))
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

pub(super) fn ensure_parent_dir(path: &Path) -> Result<(), MigrationBundleError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
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
