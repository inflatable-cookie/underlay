//! Deterministic proof that `LocalAdapter` pins one owned descriptor to the
//! base directory at construction, so bounded/exclusive operations are
//! immune to the base directory being renamed and its old pathname
//! replaced with a symlink afterward.
//!
//! Split out to keep files under the doctor god-files threshold.

use super::*;

/// After construction, rename the base directory away and put a symlink at
/// its old pathname pointing at an attacker-controlled directory. A read
/// through the already-constructed adapter must still resolve inside the
/// real (moved) base directory via the pinned descriptor, never through the
/// replacement symlink — and the outside directory must stay untouched.
#[tokio::test]
async fn test_get_bytes_bounded_resolves_through_the_pinned_descriptor_after_base_rename() {
    let original_dir = std::env::temp_dir().join("underlay-blob-base-pin-read-original");
    let moved_dir = std::env::temp_dir().join("underlay-blob-base-pin-read-moved");
    let outside_dir = std::env::temp_dir().join("underlay-blob-base-pin-read-outside");
    let _ = fs::remove_dir_all(&original_dir).await;
    let _ = fs::remove_dir_all(&moved_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;

    fs::create_dir_all(&original_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();
    fs::write(outside_dir.join("probe.bin"), b"outside secret bytes")
        .await
        .unwrap();

    let config = LocalConfig::new(&original_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Seed the file the adapter will read, then move the whole base
    // directory elsewhere and replace its old pathname with a symlink to
    // the attacker's directory. The already-open pinned descriptor keeps
    // referring to the real (now-moved) directory inode regardless.
    fs::write(original_dir.join("probe.bin"), b"real pinned bytes")
        .await
        .unwrap();
    fs::rename(&original_dir, &moved_dir).await.unwrap();
    std::os::unix::fs::symlink(&outside_dir, &original_dir).unwrap();

    let bytes = adapter.get_bytes_bounded("probe.bin", 1024).await.unwrap();
    assert_eq!(bytes, b"real pinned bytes");

    let _ = fs::remove_dir_all(&moved_dir).await;
    let _ = fs::remove_file(&original_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}

/// Same rename/replace attack against `put_bytes_create_only`: the write
/// must land in the real (moved) base directory via the pinned descriptor,
/// and the outside directory the replacement symlink points to must never
/// receive anything.
#[tokio::test]
async fn test_put_bytes_create_only_publishes_through_the_pinned_descriptor_after_base_rename() {
    let original_dir = std::env::temp_dir().join("underlay-blob-base-pin-create-original");
    let moved_dir = std::env::temp_dir().join("underlay-blob-base-pin-create-moved");
    let outside_dir = std::env::temp_dir().join("underlay-blob-base-pin-create-outside");
    let _ = fs::remove_dir_all(&original_dir).await;
    let _ = fs::remove_dir_all(&moved_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;

    fs::create_dir_all(&original_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();

    let config = LocalConfig::new(&original_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    fs::rename(&original_dir, &moved_dir).await.unwrap();
    std::os::unix::fs::symlink(&outside_dir, &original_dir).unwrap();

    let stored = adapter
        .put_bytes_create_only(
            "media/probe.bin",
            b"pinned publish",
            "application/octet-stream",
        )
        .await
        .unwrap();
    assert_eq!(stored.key, "media/probe.bin");

    // Published into the real (moved) directory via the pinned descriptor...
    let published = fs::read(moved_dir.join("media/probe.bin")).await.unwrap();
    assert_eq!(published, b"pinned publish");

    // ...never into the outside directory the replacement symlink points to.
    let mut outside_entries = fs::read_dir(&outside_dir).await.unwrap();
    assert!(outside_entries.next_entry().await.unwrap().is_none());

    let _ = fs::remove_dir_all(&moved_dir).await;
    let _ = fs::remove_file(&original_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}
