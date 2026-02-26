use super::{parse_leading_version, sync_migrations, SyncMigrationsError};
use std::path::PathBuf;

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
fn parse_leading_version_accepts_numeric_prefix() {
    let version = parse_leading_version("0002_create_auth_tables.sql")
        .expect("version should parse from filename prefix");
    assert_eq!(version, 2);
}

#[test]
fn parse_leading_version_rejects_missing_prefix() {
    let err = parse_leading_version("create_auth_tables.sql")
        .expect_err("filename without leading digits should fail");
    match err {
        SyncMigrationsError::InvalidMigrationFilename(name) => {
            assert_eq!(name, "create_auth_tables.sql")
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn sync_migrations_errors_when_target_missing() {
    let path = temp_dir("underlay_sync_missing_target");
    std::fs::remove_dir_all(&path).expect("temp dir should be removed");

    let err = sync_migrations(&path, true).expect_err("missing target should fail");
    match err {
        SyncMigrationsError::TargetMissing(p) => assert_eq!(p, path),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn sync_migrations_writes_then_skips_when_unchanged() {
    let path = temp_dir("underlay_sync_write_skip");

    let first = sync_migrations(&path, false).expect("initial sync should succeed");
    assert!(
        !first.written.is_empty(),
        "expected migrations to be written"
    );
    assert!(first.skipped.is_empty(), "first sync should not skip");

    for written in &first.written {
        let file_name = written
            .file_name()
            .and_then(|n| n.to_str())
            .expect("file name should be valid utf-8");
        assert!(
            file_name.contains("__underlay_auth__"),
            "unexpected synced filename: {file_name}"
        );
    }

    let second = sync_migrations(&path, false).expect("second sync should succeed");
    assert!(second.written.is_empty(), "second sync should not write");
    assert_eq!(
        second.skipped.len(),
        first.written.len(),
        "all synced files should be skipped when unchanged"
    );
}

#[test]
fn sync_migrations_detects_content_mismatch() {
    let path = temp_dir("underlay_sync_mismatch");
    let first = sync_migrations(&path, false).expect("initial sync should succeed");

    let changed = first
        .written
        .first()
        .expect("at least one migration should be written");
    std::fs::write(changed, "-- modified by test\n").expect("file should be writable");

    let err = sync_migrations(&path, false).expect_err("mismatch should fail");
    match err {
        SyncMigrationsError::ContentMismatch {
            path: mismatch_path,
        } => {
            assert_eq!(mismatch_path, *changed)
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
