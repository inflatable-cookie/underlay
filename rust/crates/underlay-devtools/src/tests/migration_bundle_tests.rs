use super::{
    migration_bundle_build, migration_bundle_publish, migration_bundle_pull, migration_run,
    sha256_digest, write_pulled_outputs, BundleBuildOptions, BundlePackage, BundlePublishOptions,
    BundlePullOptions, BundleRunOptions, MigrationBundleRef,
};
use base64::Engine as _;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use underlay_migration_core::{OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind};

fn temp_dir(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
    std::fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

#[test]
fn migration_bundle_build_writes_layout_json_with_real_digests() {
    let dir = temp_dir("underlay_bundle_build");
    let output = dir.join("bundle.json");

    let report = migration_bundle_build(&BundleBuildOptions {
        output_file: output.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: None,
        media_shard_max_bytes: None,
    })
    .expect("bundle build should succeed");

    assert_eq!(report.output_file, output);
    assert!(report.layer_count >= 3);
    assert!(report.bundle_digest.starts_with("sha256:"));
    assert_eq!(report.media_asset_count, 0);

    let contents = std::fs::read_to_string(&output).expect("bundle layout should be written");
    assert!(contents.contains("application/vnd.underlay.migration.bundle.v1"));
    assert!(contents.contains("\"digest\": \"sha256:"));
}

#[test]
fn migration_bundle_publish_requires_existing_bundle() {
    let dir = temp_dir("underlay_bundle_publish_missing");
    let bundle = dir.join("missing.json");

    let err = migration_bundle_publish(&BundlePublishOptions {
        bundle_file: bundle,
        oci_ref: "registry.example.com/org/bundle:demo".to_string(),
        local_store_dir: Some(dir.join("store")),
    })
    .expect_err("publish should fail for missing bundle");

    assert!(err.to_string().contains("does not exist"));
}

#[test]
fn migration_bundle_publish_and_pull_round_trip_from_local_store() {
    let dir = temp_dir("underlay_bundle_round_trip");
    let bundle_file = dir.join("bundle.json");
    let store_dir = dir.join("store");
    let pull_dir = dir.join("pull");

    let build = migration_bundle_build(&BundleBuildOptions {
        output_file: bundle_file.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: None,
        media_shard_max_bytes: None,
    })
    .expect("bundle build should succeed");

    let publish = migration_bundle_publish(&BundlePublishOptions {
        bundle_file: bundle_file.clone(),
        oci_ref: "registry.example.com/org/bundle:demo".to_string(),
        local_store_dir: Some(store_dir.clone()),
    })
    .expect("publish should succeed");

    assert_eq!(publish.digest, build.bundle_digest);
    assert_eq!(publish.status, "published-local");

    let pull = migration_bundle_pull(&BundlePullOptions {
        oci_ref: "registry.example.com/org/bundle:demo".to_string(),
        output_dir: pull_dir.clone(),
        local_store_dir: Some(store_dir),
    })
    .expect("pull should succeed");

    assert_eq!(pull.digest, publish.digest);
    assert_eq!(pull.status, "pulled-local");
    assert!(pull.output_file.exists());
    assert!(pull_dir.join("media-shards").exists());
}

#[test]
fn migration_bundle_publish_rejects_digest_mismatch_in_ref() {
    let dir = temp_dir("underlay_bundle_publish_mismatch");
    let bundle_file = dir.join("bundle.json");

    migration_bundle_build(&BundleBuildOptions {
        output_file: bundle_file.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: None,
        media_shard_max_bytes: None,
    })
    .expect("bundle build should succeed");

    let err = migration_bundle_publish(&BundlePublishOptions {
        bundle_file,
        oci_ref: "registry.example.com/org/bundle@sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
        local_store_dir: Some(dir.join("store")),
    })
    .expect_err("publish should reject mismatched digest ref");

    assert!(err.to_string().contains("digest mismatch"));
}

#[test]
fn migration_run_requires_digest_pinned_bundle_ref() {
    let dir = temp_dir("underlay_bundle_run_requires_digest");
    let err = migration_run(&BundleRunOptions {
        bundle_ref: "registry.example.com/org/bundle:demo".to_string(),
        output_dir: dir.join("run"),
        local_store_dir: Some(dir.join("store")),
    })
    .expect_err("run should require digest");

    assert!(err.to_string().contains("digest-pinned"));
}

#[test]
fn migration_bundle_ref_parses_digest_pinned_ref() {
    let bundle_ref = MigrationBundleRef::parse_digest_pinned(
        "registry.example.com/org/bundle@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    )
    .unwrap();

    assert_eq!(
        bundle_ref.as_str(),
        "registry.example.com/org/bundle@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
    assert_eq!(
        bundle_ref.digest(),
        "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    );
}

#[test]
fn migration_bundle_ref_rejects_tag_only_ref() {
    let err = MigrationBundleRef::parse_digest_pinned("registry.example.com/org/bundle:demo")
        .expect_err("tag-only ref should be rejected");

    assert!(err.to_string().contains("digest-pinned"));
}

#[test]
fn migration_bundle_ref_rejects_malformed_sha256_digest() {
    let err = MigrationBundleRef::parse_digest_pinned(
        "registry.example.com/org/bundle@sha256:not-a-real-digest",
    )
    .expect_err("malformed digest should be rejected");

    assert!(err.to_string().contains("64 hex"));
}

#[test]
fn bundle_run_options_accept_typed_bundle_ref() {
    let bundle_ref = MigrationBundleRef::parse_digest_pinned(
        "registry.example.com/org/bundle@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    )
    .unwrap();

    let options = BundleRunOptions::from_bundle_ref(
        bundle_ref.clone(),
        PathBuf::from("out"),
        Some(PathBuf::from("store")),
    );

    assert_eq!(options.bundle_ref().unwrap(), bundle_ref);
    assert_eq!(options.output_dir, PathBuf::from("out"));
    assert_eq!(options.local_store_dir, Some(PathBuf::from("store")));
}

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
fn migration_run_replays_digest_pinned_bundle_from_local_store() {
    let dir = temp_dir("underlay_bundle_run_digest");
    let bundle_file = dir.join("bundle.json");
    let store_dir = dir.join("store");
    let run_dir = dir.join("run");

    migration_bundle_build(&BundleBuildOptions {
        output_file: bundle_file.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: None,
        media_shard_max_bytes: None,
    })
    .expect("bundle build should succeed");

    let publish = migration_bundle_publish(&BundlePublishOptions {
        bundle_file,
        oci_ref: "registry.example.com/org/bundle:demo".to_string(),
        local_store_dir: Some(store_dir.clone()),
    })
    .expect("publish should succeed");

    let report = migration_run(&BundleRunOptions {
        bundle_ref: format!("registry.example.com/org/bundle@{}", publish.digest),
        output_dir: run_dir,
        local_store_dir: Some(store_dir),
    })
    .expect("run should succeed");

    assert_eq!(report.digest, publish.digest);
    assert_eq!(report.status, "prepared");
    assert!(report.output_file.exists());
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

#[test]
#[ignore = "requires Docker and a local registry container"]
fn migration_bundle_remote_registry_round_trip() {
    let dir = temp_dir("underlay_bundle_remote_round_trip");
    let bundle_file = dir.join("bundle.json");

    let port = pick_free_port();
    let registry_name = format!("underlay_test_registry_{}_{}", std::process::id(), port);
    let registry_ref = format!("http://127.0.0.1:{port}/underlay/test-bundle:demo");

    let run = Command::new("docker")
        .args([
            "run",
            "-d",
            "--rm",
            "--name",
            &registry_name,
            "-p",
            &format!("{port}:5000"),
            "registry:2",
        ])
        .output()
        .expect("docker should be available");
    if !run.status.success() {
        eprintln!(
            "skipping remote registry test because registry container failed to start: {}",
            String::from_utf8_lossy(&run.stderr)
        );
        return;
    }

    let _guard = DockerRegistryGuard {
        name: registry_name.clone(),
    };

    wait_for_registry(port);

    migration_bundle_build(&BundleBuildOptions {
        output_file: bundle_file.clone(),
        source_system: "legacy_system".to_string(),
        target_schema_version: "schema_v1".to_string(),
        media_dir: None,
        media_shard_max_bytes: None,
    })
    .expect("bundle build should succeed");

    let publish = migration_bundle_publish(&BundlePublishOptions {
        bundle_file,
        oci_ref: registry_ref.clone(),
        local_store_dir: None,
    })
    .expect("remote publish should succeed");

    assert!(publish.status.starts_with("published-remote"));
    assert!(
        publish.digest.starts_with("sha256:"),
        "expected digest, got {}",
        publish.digest
    );

    let pull = migration_bundle_pull(&BundlePullOptions {
        oci_ref: format!(
            "http://127.0.0.1:{port}/underlay/test-bundle@{}",
            publish.digest
        ),
        output_dir: dir.join("pull"),
        local_store_dir: None,
    })
    .expect("remote pull should succeed");

    assert_eq!(pull.status, "pulled-remote");
    assert!(pull.output_file.exists());
}

struct DockerRegistryGuard {
    name: String,
}

impl Drop for DockerRegistryGuard {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .args(["rm", "-f", &self.name])
            .status();
    }
}

fn pick_free_port() -> u16 {
    std::net::TcpListener::bind(("127.0.0.1", 0))
        .expect("bind ephemeral port")
        .local_addr()
        .expect("local addr should exist")
        .port()
}

fn wait_for_registry(port: u16) {
    let url = format!("http://127.0.0.1:{port}/v2/");
    for _ in 0..40 {
        if let Ok(response) = reqwest::blocking::get(&url) {
            if response.status().is_success() {
                return;
            }
        }
        sleep(Duration::from_millis(250));
    }
    panic!("registry did not become ready on port {port}");
}
