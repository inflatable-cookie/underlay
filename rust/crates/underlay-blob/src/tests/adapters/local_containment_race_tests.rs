//! Deterministic proof that local bounded-capture and exclusive-create
//! containment comes from the traversal syscalls themselves, not from a
//! check performed before them, plus the non-blocking FIFO guarantee.
//!
//! Split out from `local_bounded_promotion_tests.rs` to keep both files
//! under the doctor god-files threshold.

use super::*;

/// A symlinked *parent* component (not the leaf) that resolves outside the
/// base must refuse before ever touching the outside target — for both the
/// bounded read and the exclusive create path. The old `create_dir_all`
/// implementation would happily walk through such a symlink and create
/// directories/files at the real, uncontrolled location it points to; the
/// `openat(O_NOFOLLOW)` traversal refuses at the first symlinked component
/// instead.
#[tokio::test]
async fn test_get_bytes_bounded_refuses_a_symlinked_parent_component() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-read-test");
    let outside_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-read-outside");
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();
    fs::write(outside_dir.join("leaf.bin"), b"outside secret")
        .await
        .unwrap();

    // "escape" is a symlink planted inside the base pointing outside it.
    std::os::unix::fs::symlink(&outside_dir, temp_dir.join("escape")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .get_bytes_bounded("escape/leaf.bin", 1024)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_refuses_a_symlinked_parent_component_and_writes_nothing_outside(
) {
    let temp_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-create-test");
    let outside_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-create-outside");
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();

    std::os::unix::fs::symlink(&outside_dir, temp_dir.join("escape")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .put_bytes_create_only("escape/pwned.bin", b"hostile", "application/octet-stream")
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));

    // Nothing was written into the real directory the symlink points to.
    let mut entries = fs::read_dir(&outside_dir).await.unwrap();
    assert!(entries.next_entry().await.unwrap().is_none());

    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}

/// A deeper case: the missing intermediate component ("sub") would only be
/// reached by first descending through the symlinked "escape" component.
/// The old `create_dir_all(parent)` call would `mkdir` "sub" inside the
/// real outside directory; the fix refuses before that traversal step is
/// even attempted.
#[tokio::test]
async fn test_put_bytes_create_only_refuses_before_creating_missing_dirs_through_a_symlinked_parent(
) {
    let temp_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-deep-test");
    let outside_dir = std::env::temp_dir().join("underlay-blob-parent-symlink-deep-outside");
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();

    std::os::unix::fs::symlink(&outside_dir, temp_dir.join("escape")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .put_bytes_create_only(
            "escape/sub/pwned.bin",
            b"hostile",
            "application/octet-stream",
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));

    assert!(!outside_dir.join("sub").exists());

    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}

/// A source swapped to a FIFO must never block the caller: the descriptor
/// is opened `O_NOFOLLOW | O_NONBLOCK` and `fstat`-checked before any read
/// is attempted, so opening a FIFO with no writer present returns
/// immediately instead of hanging. Bound the call with a short timeout so a
/// regression (a blocking open) fails the test instead of hanging the
/// suite.
#[tokio::test]
async fn test_get_bytes_bounded_refuses_a_fifo_source_without_blocking() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-fifo-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();

    let fifo_path = temp_dir.join("pipe.bin");
    let c_path = std::ffi::CString::new(fifo_path.to_str().unwrap()).unwrap();
    let mkfifo_result = unsafe { libc::mkfifo(c_path.as_ptr(), 0o600) };
    assert_eq!(
        mkfifo_result, 0,
        "mkfifo should succeed in a fresh temp dir"
    );

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let outcome = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        adapter.get_bytes_bounded("pipe.bin", 1024),
    )
    .await
    .expect("get_bytes_bounded must not block on a FIFO with no writer");

    assert!(matches!(outcome.unwrap_err(), BlobError::Unsupported(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}
