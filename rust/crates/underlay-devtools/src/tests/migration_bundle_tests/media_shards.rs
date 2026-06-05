use base64::Engine as _;
use std::collections::BTreeMap;

use super::super::{
    migration_bundle_build, sha256_digest, write_pulled_outputs, BundleBuildOptions, BundlePackage,
};
use super::support::temp_dir;
use underlay_migration_core::{OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind};

#[test]
fn pulled_media_shard_output_names_are_sanitized() {
    let dir = temp_dir("underlay_bundle_shard_output_escape");
    let output_dir = dir.join("pull");
    std::fs::create_dir_all(&output_dir).expect("output dir should exist");

    let media_id = uuid::Uuid::parse_str("018f0000-0000-7000-8000-000000000001").unwrap();
    let version_id = uuid::Uuid::parse_str("018f0000-0000-7000-8000-000000000002").unwrap();
    let payload_bytes = b"abc".to_vec();
    let payload_sha = sha256_digest(&payload_bytes);
    let object_key = underlay_media::storage::version_object_key(media_id, version_id, "a.bin")
        .expect("object key should be valid")
        .into_string();
    let shard_payload = serde_json::to_vec(&serde_json::json!({
        "schema_version": "1",
        "shard_id": "../../escape",
        "assets": [{
            "relative_path": "a.bin",
            "filename": "a.bin",
            "byte_size": payload_bytes.len(),
            "sha256": payload_sha,
            "content_base64": base64::engine::general_purpose::STANDARD.encode(&payload_bytes),
            "mapping": {
                "media_id": media_id.to_string(),
                "version_id": version_id.to_string(),
                "object_key": object_key,
            }
        }]
    }))
    .expect("shard payload should serialize");
    let layer_digest = sha256_digest(&shard_payload);

    let mut annotations = BTreeMap::new();
    annotations.insert("underlay.shard_id".to_string(), "../../escape".to_string());

    let package = BundlePackage {
        schema_version: "1".to_string(),
        layout: OciBundleLayout {
            artifact_type: "application/vnd.underlay.migration.bundle.v1".to_string(),
            media_type: "application/vnd.oci.image.manifest.v1+json".to_string(),
            config: OciBundleConfig {
                schema_version: "1".to_string(),
                bundle_id: underlay_core::Uuid::new_v7().to_string(),
                bundle_version: "v0-local".to_string(),
                source_system: "legacy_system".to_string(),
                target_schema_version: "schema_v1".to_string(),
            },
            layers: vec![OciLayerDescriptor {
                kind: OciLayerKind::MediaShard,
                media_type: "application/vnd.underlay.bundle.media.shard.v1+json".to_string(),
                digest: layer_digest.clone(),
                size_bytes: shard_payload.len() as u64,
                annotations,
            }],
            sidecars: Vec::new(),
        },
        payloads: BTreeMap::from([(
            layer_digest,
            base64::engine::general_purpose::STANDARD.encode(&shard_payload),
        )]),
    };

    write_pulled_outputs(&package, &output_dir).expect("pulled outputs should be written");

    assert!(!dir.join("escape.json").exists());
    assert!(output_dir
        .join("media-shards")
        .join(".._.._escape.json")
        .exists());
}

#[test]
fn migration_bundle_build_splits_media_into_deterministic_shards_with_mapping() {
    let dir = temp_dir("underlay_bundle_media_shards");
    let media_dir = dir.join("media");
    std::fs::create_dir_all(&media_dir).expect("media dir should exist");

    std::fs::write(media_dir.join("a.bin"), b"12345").expect("write a.bin");
    std::fs::write(media_dir.join("b.bin"), b"67890").expect("write b.bin");
    std::fs::write(media_dir.join("c.bin"), b"abcde").expect("write c.bin");

    let bundle_file = dir.join("bundle.json");
    let report = migration_bundle_build(&BundleBuildOptions {
        output_file: bundle_file.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: Some(media_dir),
        media_shard_max_bytes: Some(8),
    })
    .expect("bundle build with media should succeed");

    assert_eq!(report.media_asset_count, 3);
    assert!(report.media_shard_count >= 2);

    let package: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&bundle_file).expect("bundle json should exist"))
            .expect("bundle package json should parse");

    let layers = package
        .get("layout")
        .and_then(|v| v.get("layers"))
        .and_then(|v| v.as_array())
        .expect("layers array should exist");

    let media_layers: Vec<&serde_json::Value> = layers
        .iter()
        .filter(|layer| layer.get("kind").and_then(|v| v.as_str()) == Some("media_shard"))
        .collect();
    assert!(media_layers.len() >= 2);

    for layer in &media_layers {
        let annotations = layer
            .get("annotations")
            .and_then(|v| v.as_object())
            .expect("annotations should exist");
        assert!(annotations.contains_key("underlay.shard_id"));
        assert!(annotations.contains_key("underlay.media_asset_count"));
    }

    let payloads = package
        .get("payloads")
        .and_then(|v| v.as_object())
        .expect("payload map should exist");

    let first_media_digest = media_layers[0]
        .get("digest")
        .and_then(|v| v.as_str())
        .expect("media digest should exist");
    let encoded = payloads
        .get(first_media_digest)
        .and_then(|v| v.as_str())
        .expect("payload for media shard should exist");

    let shard_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .expect("media shard payload base64 should decode");
    let shard: serde_json::Value =
        serde_json::from_slice(&shard_bytes).expect("media shard payload should parse");
    let first_asset = shard
        .get("assets")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .expect("first media asset should exist");

    let mapping = first_asset
        .get("mapping")
        .and_then(|v| v.as_object())
        .expect("mapping should exist");
    let object_key = mapping
        .get("object_key")
        .and_then(|v| v.as_str())
        .expect("object_key should exist");
    assert!(object_key.starts_with("media/"));
    assert!(object_key.contains("/versions/"));
}
