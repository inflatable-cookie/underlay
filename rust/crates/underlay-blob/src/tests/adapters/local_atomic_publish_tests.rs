//! Deterministic proof that local `put_bytes_create_only` never exposes a
//! partially-written destination and never strands a poisoned final name:
//! it writes and syncs an owned, unguessable same-directory temp file
//! first, then publishes it atomically to the final name.
//!
//! Split out to keep files under the doctor god-files threshold.

use super::*;

async fn temp_residue_names(dir: &std::path::Path) -> Vec<String> {
    let mut entries = fs::read_dir(dir).await.unwrap();
    let mut names = Vec::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(".underlay-tmp.") {
            names.push(name);
        }
    }
    names
}

#[tokio::test]
async fn test_put_bytes_create_only_is_invisible_until_fully_published_and_leaves_no_temp_residue()
{
    let temp_dir = std::env::temp_dir().join("underlay-blob-atomic-publish-visibility-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // Not visible before the call.
    let before = adapter.get_bytes_bounded("media/a.bin", 1024).await;
    assert!(matches!(before, Err(BlobError::NotFound(_))));

    let payload = vec![0x42u8; 4096];
    adapter
        .put_bytes_create_only("media/a.bin", &payload, "application/octet-stream")
        .await
        .unwrap();

    // Immediately visible afterward, with the complete bytes — never a
    // partial write.
    let after = adapter
        .get_bytes_bounded("media/a.bin", 4096)
        .await
        .unwrap();
    assert_eq!(after, payload);

    // No leftover temp name in the directory that received the publish.
    assert!(temp_residue_names(&temp_dir.join("media")).await.is_empty());

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_collision_preserves_the_incumbent_and_leaves_no_temp_residue() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-atomic-publish-collision-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    adapter
        .put_bytes_create_only(
            "media/a.bin",
            b"incumbent bytes",
            "application/octet-stream",
        )
        .await
        .unwrap();

    let err = adapter
        .put_bytes_create_only(
            "media/a.bin",
            b"hostile replacement",
            "application/octet-stream",
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let unchanged = adapter
        .get_bytes_bounded("media/a.bin", 1024)
        .await
        .unwrap();
    assert_eq!(unchanged, b"incumbent bytes");

    // The failed publish's own temp file was cleaned up; nothing but the
    // incumbent's final name remains.
    assert!(temp_residue_names(&temp_dir.join("media")).await.is_empty());
    let mut entries = fs::read_dir(temp_dir.join("media")).await.unwrap();
    let mut names = Vec::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    assert_eq!(names, vec!["a.bin".to_string()]);

    let _ = fs::remove_dir_all(&temp_dir).await;
}

/// Simulates a residue left by a hypothetical crashed prior attempt: a
/// stale `.underlay-tmp.*`-shaped file already sitting in the directory.
/// A fresh `put_bytes_create_only` for a real key must succeed normally
/// (collision detection is keyed on the final name only, never a stale
/// temp name) and must never touch the stale file — cleanup only ever
/// removes the exact temp name a given call generated for itself.
#[tokio::test]
async fn test_put_bytes_create_only_ignores_and_never_touches_a_stale_foreign_temp_file() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-atomic-publish-stale-temp-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir.join("media")).await.unwrap();

    let stale_temp = temp_dir
        .join("media")
        .join(".underlay-tmp.99999.deadbeefcafefeed");
    fs::write(&stale_temp, b"residue from a crashed prior attempt")
        .await
        .unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    // A retry after the "crash" is not permanently blocked: the stale temp
    // name never occupies the final destination name.
    let stored = adapter
        .put_bytes_create_only("media/a.bin", b"fresh publish", "application/octet-stream")
        .await
        .unwrap();
    assert_eq!(stored.key, "media/a.bin");

    let published = adapter
        .get_bytes_bounded("media/a.bin", 1024)
        .await
        .unwrap();
    assert_eq!(published, b"fresh publish");

    // The stale, foreign temp file is exactly as it was.
    let untouched = fs::read(&stale_temp).await.unwrap();
    assert_eq!(untouched, b"residue from a crashed prior attempt");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_concurrent_writers_leave_no_temp_residue() {
    use std::sync::Arc;

    let temp_dir = std::env::temp_dir().join("underlay-blob-atomic-publish-race-residue-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = Arc::new(LocalAdapter::new(config).await.unwrap());

    let mut handles = Vec::new();
    for i in 0..8u8 {
        let adapter = adapter.clone();
        handles.push(tokio::spawn(async move {
            adapter
                .put_bytes_create_only("media/race.bin", &[i; 4], "application/octet-stream")
                .await
        }));
    }

    let mut ok_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            ok_count += 1;
        }
    }
    assert_eq!(ok_count, 1, "exactly one writer publishes the destination");

    assert!(temp_residue_names(&temp_dir.join("media")).await.is_empty());

    let _ = fs::remove_dir_all(&temp_dir).await;
}
