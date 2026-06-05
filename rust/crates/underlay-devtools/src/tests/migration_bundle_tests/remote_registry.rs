use std::process::Command;

use super::super::{
    migration_bundle_build, migration_bundle_publish, migration_bundle_pull, BundleBuildOptions,
    BundlePublishOptions, BundlePullOptions,
};
use super::support::{pick_free_port, temp_dir, wait_for_registry, DockerRegistryGuard};

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
