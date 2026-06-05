use std::path::PathBuf;

use super::super::{BundleRunOptions, MigrationBundleRef};

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
