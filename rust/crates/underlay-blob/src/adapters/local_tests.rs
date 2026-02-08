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
