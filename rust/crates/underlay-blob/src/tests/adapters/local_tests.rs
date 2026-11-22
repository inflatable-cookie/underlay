use super::*;

#[tokio::test]
async fn test_local_adapter_write_and_read() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-test");
    let _ = fs::remove_dir_all(&temp_dir).await; // Clean up from previous runs

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Write a file
    let key = "test/hello.txt";
    let data = b"Hello, World!";
    let stored = adapter.write_file(key, data, "text/plain").await.unwrap();

    assert_eq!(stored.key, key);
    assert_eq!(stored.size, data.len() as u64);

    // Read it back
    let read_data = adapter.read_file(key).await.unwrap();
    assert_eq!(read_data, data);

    // Head
    let info = adapter.head(key).await.unwrap();
    assert_eq!(info.size, data.len() as u64);
    assert_eq!(info.content_type, "text/plain");

    // Delete
    adapter.delete(key).await.unwrap();
    assert!(!adapter.exists(key).await.unwrap());

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_local_adapter_rejects_unsafe_keys() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-key-security-test");
    let outside_file = temp_dir
        .parent()
        .expect("temp dir should have parent")
        .join("underlay-blob-outside.txt");
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_file(&outside_file).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    for key in [
        "../underlay-blob-outside.txt",
        "/tmp/outside.txt",
        "a/../b.txt",
        "a\\b.txt",
    ] {
        let err = adapter
            .write_file(key, b"blocked", "text/plain")
            .await
            .expect_err("unsafe key should be rejected");
        assert!(matches!(err, BlobError::InvalidKey(_)));
    }

    assert!(!outside_file.exists());
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_local_adapter_rejects_symlink_escape() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-symlink-test");
    let outside_dir = std::env::temp_dir().join("underlay-blob-symlink-outside");
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::create_dir_all(&outside_dir).await.unwrap();

    // Plant a symlink inside the base directory pointing outside it.
    std::os::unix::fs::symlink(&outside_dir, temp_dir.join("escape")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .write_file("escape/pwned.txt", b"blocked", "text/plain")
        .await
        .expect_err("symlink escape should be rejected");
    assert!(matches!(err, BlobError::InvalidKey(_)));

    assert!(!outside_dir.join("pwned.txt").exists());
    let _ = fs::remove_dir_all(&temp_dir).await;
    let _ = fs::remove_dir_all(&outside_dir).await;
}

#[tokio::test]
async fn test_local_adapter_public_url() {
    let config = LocalConfig::new("/tmp/test", "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let url = adapter.public_url("media/123/photo.jpg");
    assert_eq!(url, "http://localhost:8080/uploads/media/123/photo.jpg");
}

#[test]
fn test_guess_content_type() {
    assert_eq!(guess_content_type("photo.jpg"), "image/jpeg");
    assert_eq!(guess_content_type("photo.JPEG"), "image/jpeg");
    assert_eq!(guess_content_type("doc.pdf"), "application/pdf");
    assert_eq!(
        guess_content_type("unknown.xyz"),
        "application/octet-stream"
    );
}

#[tokio::test]
async fn test_is_path_within_base_rejects_traversal() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-security-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Valid path within base should succeed
    let valid_path = temp_dir.join("subdir");
    fs::create_dir_all(&valid_path).await.unwrap();
    assert!(adapter.is_path_within_base(&valid_path).is_some());

    // Path traversal with .. should be rejected (resolves outside base)
    let traversal_path = temp_dir.join("..").join("other");
    assert!(adapter.is_path_within_base(&traversal_path).is_none());

    // Base path itself should be rejected (we never delete the base)
    assert!(adapter.is_path_within_base(&temp_dir).is_none());

    // Non-existent path should be rejected (can't canonicalize)
    let nonexistent = temp_dir.join("does-not-exist");
    assert!(adapter.is_path_within_base(&nonexistent).is_none());

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_cleanup_preserves_base_directory() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-cleanup-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Create a nested structure and file
    let nested_path = temp_dir.join("a").join("b").join("c");
    fs::create_dir_all(&nested_path).await.unwrap();
    let file_path = nested_path.join("file.txt");
    fs::write(&file_path, b"test").await.unwrap();

    // Delete the file and trigger cleanup
    adapter.delete("a/b/c/file.txt").await.unwrap();

    // The nested empty directories should be cleaned up
    assert!(!nested_path.exists());
    assert!(!temp_dir.join("a").join("b").exists());
    assert!(!temp_dir.join("a").exists());

    // But the base directory MUST still exist
    assert!(temp_dir.exists(), "Base directory was incorrectly deleted!");

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_cleanup_stops_at_non_empty_directory() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-cleanup-nonempty-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Create nested structure with a sibling file
    let nested_path = temp_dir.join("media").join("123").join("versions");
    fs::create_dir_all(&nested_path).await.unwrap();

    // File to delete
    let file_to_delete = nested_path.join("v1.jpg");
    fs::write(&file_to_delete, b"image").await.unwrap();

    // Sibling file that should prevent parent deletion
    let sibling_file = temp_dir.join("media").join("123").join("metadata.json");
    fs::write(&sibling_file, b"{}").await.unwrap();

    // Delete the file
    adapter.delete("media/123/versions/v1.jpg").await.unwrap();

    // versions/ should be deleted (was empty after file removal)
    assert!(!nested_path.exists());

    // But media/123/ should still exist (has sibling file)
    assert!(temp_dir.join("media").join("123").exists());
    assert!(sibling_file.exists());

    // Clean up
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_finalise_upload_verified_rejects_mismatched_bytes() {
    use crate::adapter::BlobAdapterUploadExt;
    use crate::config::BlobUploadConfig;
    use crate::error::BlobError;

    let temp_dir = std::env::temp_dir().join("underlay-blob-verify-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    let upload_config = BlobUploadConfig::default();

    // A real PNG passes verification.
    let png: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];
    adapter
        .write_file("img/ok.png", png, "image/png")
        .await
        .unwrap();
    let stored = adapter
        .finalise_upload_verified("img/ok.png", "image/png", &upload_config)
        .await
        .unwrap();
    assert_eq!(stored.content_type, "image/png");

    // An HTML payload declared as PNG is rejected at finalise.
    adapter
        .write_file(
            "img/evil.png",
            b"<html><script>alert(1)</script>",
            "image/png",
        )
        .await
        .unwrap();
    let err = adapter
        .finalise_upload_verified("img/evil.png", "image/png", &upload_config)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidContentType(_)));

    // A disallowed declared type is rejected even if bytes are consistent.
    adapter
        .write_file("page/index.html", b"<html></html>", "text/html")
        .await
        .unwrap();
    let err = adapter
        .finalise_upload_verified("page/index.html", "text/html", &upload_config)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidContentType(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_initiate_upload_validated_enforces_config() {
    use crate::adapter::BlobAdapterUploadExt;
    use crate::config::BlobUploadConfig;
    use crate::error::BlobError;
    use crate::types::UploadRequest;

    let temp_dir = std::env::temp_dir().join("underlay-blob-initiate-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    let upload_config = BlobUploadConfig::default().max_file_size_mb(1);

    let ok = UploadRequest::parse_key("img/a.png", "image/png", 1024).unwrap();
    assert!(adapter
        .initiate_upload_validated(ok, &upload_config)
        .await
        .is_ok());

    let oversized = UploadRequest::parse_key("img/b.png", "image/png", 5 * 1024 * 1024).unwrap();
    assert!(matches!(
        adapter
            .initiate_upload_validated(oversized, &upload_config)
            .await,
        Err(BlobError::TooLarge(_, _))
    ));

    let scriptable = UploadRequest::parse_key("img/c.svg", "image/svg+xml", 1024).unwrap();
    assert!(matches!(
        adapter
            .initiate_upload_validated(scriptable, &upload_config)
            .await,
        Err(BlobError::InvalidContentType(_))
    ));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

fn create_unique_test_dir(prefix: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let base = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    for _ in 0..32 {
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let candidate = base.join(format!("{prefix}-{pid}-{nanos}-{seq}"));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return candidate,
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(err) => panic!("failed to create unique test directory {candidate:?}: {err}"),
        }
    }
    panic!("failed to allocate unique test directory for {prefix}");
}

#[cfg(unix)]
#[tokio::test]
async fn test_local_adapter_head_refuses_symlinks() {
    let root_dir = create_unique_test_dir("underlay-blob-head-symlink-test");
    let temp_dir = root_dir.join("storage");
    let outside_dir = root_dir.join("outside");
    fs::create_dir(&temp_dir).await.unwrap();
    fs::create_dir(&outside_dir).await.unwrap();

    fs::write(temp_dir.join("real.txt"), b"real").await.unwrap();
    fs::write(outside_dir.join("secret.txt"), b"secret")
        .await
        .unwrap();

    // In-bounds leaf symlink and out-of-bounds parent symlink.
    std::os::unix::fs::symlink(temp_dir.join("real.txt"), temp_dir.join("leaf-link.txt")).unwrap();
    std::os::unix::fs::symlink(&outside_dir, temp_dir.join("escape")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let leaf_err = adapter
        .head("leaf-link.txt")
        .await
        .expect_err("head must refuse leaf symlinks");
    assert!(
        matches!(leaf_err, BlobError::Unsupported(_)),
        "expected Unsupported for leaf symlink, got {leaf_err:?}"
    );

    let parent_err = adapter
        .head("escape/secret.txt")
        .await
        .expect_err("head must refuse symlinked parent components");
    assert!(
        matches!(parent_err, BlobError::InvalidKey(_)),
        "expected InvalidKey for symlinked parent, got {parent_err:?}"
    );

    let _ = fs::remove_dir_all(&root_dir).await;
}

#[tokio::test]
async fn test_local_adapter_initiate_upload_uses_join_public_url() {
    let root_dir = create_unique_test_dir("underlay-blob-upload-url-test");
    let temp_dir = root_dir.join("storage");

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads/");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let req =
        crate::types::UploadRequest::parse_key("media/my photo.jpg", "image/jpeg", 42).unwrap();
    let plan = adapter.initiate_upload(req).await.unwrap();
    assert_eq!(
        plan.upload_url,
        adapter.public_url("media/my photo.jpg"),
        "initiate_upload should join and encode URLs consistently with public_url"
    );

    let _ = fs::remove_dir_all(&root_dir).await;
}
