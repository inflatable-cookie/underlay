use std::collections::BTreeMap;

use crate::{
    validate_oci_bundle_layout, OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind,
    OciSidecarDescriptor,
};

fn valid_layout() -> OciBundleLayout {
    OciBundleLayout {
        artifact_type: "application/vnd.underlay.migration.bundle.v1".to_string(),
        media_type: "application/vnd.oci.image.manifest.v1+json".to_string(),
        config: OciBundleConfig::new("1", "0195-example", "v1", "legacy_demo", "schema_v1"),
        layers: vec![
            OciLayerDescriptor {
                kind: OciLayerKind::Manifest,
                media_type: "application/vnd.underlay.bundle.manifest.v1+json".to_string(),
                digest: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
                size_bytes: 1024,
                annotations: BTreeMap::new(),
            },
            OciLayerDescriptor {
                kind: OciLayerKind::DataChunk,
                media_type: "application/vnd.underlay.bundle.data.chunk.v1+zstd".to_string(),
                digest: "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                    .to_string(),
                size_bytes: 2048,
                annotations: BTreeMap::new(),
            },
        ],
        sidecars: vec![OciSidecarDescriptor {
            role: "decision_index".to_string(),
            artifact_type: "application/vnd.underlay.migration.decision-index.v1".to_string(),
            digest: "sha256:3333333333333333333333333333333333333333333333333333333333333333"
                .to_string(),
            media_type: "application/json".to_string(),
        }],
    }
}

#[test]
fn validate_oci_bundle_layout_accepts_valid_layout() {
    let layout = valid_layout();
    validate_oci_bundle_layout(&layout).expect("valid layout should pass");
}

#[test]
fn validate_oci_bundle_layout_requires_manifest_layer() {
    let mut layout = valid_layout();
    layout
        .layers
        .retain(|layer| layer.kind != OciLayerKind::Manifest);

    let err = validate_oci_bundle_layout(&layout).expect_err("missing manifest should fail");
    assert!(err.to_string().contains("manifest layer"));
}

#[test]
fn validate_oci_bundle_layout_rejects_non_sha256_digest() {
    let mut layout = valid_layout();
    layout.layers[0].digest = "md5:abcd".to_string();

    let err =
        validate_oci_bundle_layout(&layout).expect_err("invalid layer digest prefix should fail");
    assert!(err.to_string().contains("sha256"));
}
