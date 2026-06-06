use super::super::{
    migration_bundle_build, migration_bundle_publish, migration_bundle_pull, migration_run,
    BundleBuildOptions, BundlePublishOptions, BundlePullOptions, BundleRunOptions,
};
use super::support::temp_dir;

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
    let err = BundleRunOptions::parse_bundle_ref(
        "registry.example.com/org/bundle:demo",
        dir.join("run"),
        Some(dir.join("store")),
    )
    .expect_err("run options should require digest");

    assert!(err.to_string().contains("digest-pinned"));
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

    let options = BundleRunOptions::parse_bundle_ref(
        format!("registry.example.com/org/bundle@{}", publish.digest),
        run_dir,
        Some(store_dir),
    )
    .expect("run options should accept digest-pinned ref");

    let report = migration_run(&options).expect("run should succeed");

    assert_eq!(report.digest, publish.digest);
    assert_eq!(report.status, "prepared");
    assert!(report.output_file.exists());
}
