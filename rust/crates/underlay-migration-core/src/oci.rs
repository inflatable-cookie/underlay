use serde::{Deserialize, Serialize};

use crate::{MigrationError, MigrationResult};

const DIGEST_PREFIX: &str = "sha256:";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OciBundleLayout {
    pub artifact_type: String,
    pub media_type: String,
    pub config: OciBundleConfig,
    pub layers: Vec<OciLayerDescriptor>,
    pub sidecars: Vec<OciSidecarDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OciBundleConfig {
    schema_version: String,
    bundle_id: String,
    bundle_version: String,
    source_system: String,
    target_schema_version: String,
}

impl OciBundleConfig {
    pub fn new(
        schema_version: impl Into<String>,
        bundle_id: impl Into<String>,
        bundle_version: impl Into<String>,
        source_system: impl Into<String>,
        target_schema_version: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: schema_version.into(),
            bundle_id: bundle_id.into(),
            bundle_version: bundle_version.into(),
            source_system: source_system.into(),
            target_schema_version: target_schema_version.into(),
        }
    }

    pub fn local_v1(
        source_system: impl Into<String>,
        target_schema_version: impl Into<String>,
    ) -> Self {
        Self::new(
            "1",
            underlay_core::Uuid::new_v7().to_string(),
            "v0-local",
            source_system,
            target_schema_version,
        )
    }

    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn bundle_id(&self) -> &str {
        &self.bundle_id
    }

    pub fn bundle_version(&self) -> &str {
        &self.bundle_version
    }

    pub fn source_system(&self) -> &str {
        &self.source_system
    }

    pub fn target_schema_version(&self) -> &str {
        &self.target_schema_version
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OciLayerKind {
    Manifest,
    DataChunk,
    MediaShard,
    AuxiliaryIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OciLayerDescriptor {
    pub kind: OciLayerKind,
    pub media_type: String,
    pub digest: String,
    pub size_bytes: u64,
    pub annotations: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OciSidecarDescriptor {
    pub role: String,
    pub artifact_type: String,
    pub digest: String,
    pub media_type: String,
}

pub fn validate_oci_bundle_layout(layout: &OciBundleLayout) -> MigrationResult<()> {
    if layout.artifact_type.is_empty() {
        return Err(MigrationError::DeterminismViolation(
            "oci bundle artifact_type must not be empty".to_string(),
        ));
    }

    if layout.layers.is_empty() {
        return Err(MigrationError::DeterminismViolation(
            "oci bundle must include at least one layer".to_string(),
        ));
    }

    if !layout
        .layers
        .iter()
        .any(|layer| layer.kind == OciLayerKind::Manifest)
    {
        return Err(MigrationError::DeterminismViolation(
            "oci bundle must include a manifest layer".to_string(),
        ));
    }

    for layer in &layout.layers {
        if !layer.digest.starts_with(DIGEST_PREFIX) {
            return Err(MigrationError::DeterminismViolation(format!(
                "oci layer digest must start with {DIGEST_PREFIX}: {}",
                layer.digest
            )));
        }
    }

    for sidecar in &layout.sidecars {
        if !sidecar.digest.starts_with(DIGEST_PREFIX) {
            return Err(MigrationError::DeterminismViolation(format!(
                "oci sidecar digest must start with {DIGEST_PREFIX}: {}",
                sidecar.digest
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/oci_tests.rs"]
mod tests;
