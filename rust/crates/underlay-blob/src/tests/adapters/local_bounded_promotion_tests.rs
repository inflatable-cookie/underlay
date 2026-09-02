//! `g11.001` bounded-capture, exclusive-create, and `promote_verified`
//! proof against the real local filesystem adapter.
//!
//! Split from `local_tests.rs` to keep both files under the doctor
//! god-files threshold; this file shares that module's imports via
//! `super::*`.

use super::*;

#[tokio::test]
async fn test_get_bytes_bounded_accepts_a_source_within_the_cap() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-bounded-ok-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    adapter.write_file("a.bin", &[1, 2, 3], "application/octet-stream").await.unwrap();

    let bytes = adapter.get_bytes_bounded("a.bin", 3).await.unwrap();
    assert_eq!(bytes, vec![1, 2, 3]);

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_get_bytes_bounded_refuses_an_over_limit_source_without_full_buffering() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-bounded-oversized-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    let data = vec![7u8; 10];
    adapter.write_file("big.bin", &data, "application/octet-stream").await.unwrap();

    // Cap at 3: the read must stop at 4 bytes (max + 1 sentinel), never
    // retain all 10.
    let err = adapter.get_bytes_bounded("big.bin", 3).await.unwrap_err();
    match err {
        BlobError::TooLarge(observed, max) => {
            assert_eq!(max, 3);
            assert_eq!(observed, 4, "must stop at max_bytes + 1, not read the full source");
        }
        other => panic!("expected TooLarge, got {other:?}"),
    }

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_get_bytes_bounded_refuses_a_missing_source() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-bounded-missing-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter.get_bytes_bounded("missing.bin", 10).await.unwrap_err();
    assert!(matches!(err, BlobError::NotFound(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_get_bytes_bounded_refuses_a_directory_source() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-bounded-dir-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(temp_dir.join("a-dir")).await.unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter.get_bytes_bounded("a-dir", 10).await.unwrap_err();
    assert!(matches!(err, BlobError::Unsupported(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_get_bytes_bounded_refuses_a_symlink_source_without_following_it() {
    // The symlink target stays inside the base directory: a symlink that
    // *escapes* the base is already rejected earlier by the shared
    // containment check (see `test_local_adapter_rejects_symlink_escape`).
    // This proves the narrower, additional rule bounded capture needs: even
    // an in-bounds symlink is refused rather than followed.
    let temp_dir = std::env::temp_dir().join("underlay-blob-bounded-symlink-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(temp_dir.join("real.bin"), b"real bytes")
        .await
        .unwrap();

    std::os::unix::fs::symlink(temp_dir.join("real.bin"), temp_dir.join("link.bin")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter.get_bytes_bounded("link.bin", 1024).await.unwrap_err();
    assert!(matches!(err, BlobError::Unsupported(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_creates_a_fresh_destination() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-create-only-fresh-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let stored = adapter
        .put_bytes_create_only("media/a.png", b"published bytes", "image/png")
        .await
        .unwrap();
    assert_eq!(stored.key, "media/a.png");
    assert_eq!(stored.size, b"published bytes".len() as u64);

    let read_back = adapter.read_file("media/a.png").await.unwrap();
    assert_eq!(read_back, b"published bytes");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_refuses_an_occupied_destination_without_overwriting() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-create-only-occupied-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    adapter
        .write_file("media/a.png", b"original bytes", "image/png")
        .await
        .unwrap();

    let err = adapter
        .put_bytes_create_only("media/a.png", b"hostile replacement", "image/png")
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let unchanged = adapter.read_file("media/a.png").await.unwrap();
    assert_eq!(unchanged, b"original bytes");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_refuses_a_directory_destination() {
    let temp_dir = std::env::temp_dir().join("underlay-blob-create-only-dir-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(temp_dir.join("media")).await.unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .put_bytes_create_only("media", b"bytes", "image/png")
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_refuses_and_never_follows_a_symlink_destination() {
    // The symlink target stays inside the base directory, isolating the
    // no-follow-on-create property from the shared containment check
    // (already covered by `test_local_adapter_rejects_symlink_escape`).
    let temp_dir = std::env::temp_dir().join("underlay-blob-create-only-symlink-test");
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(temp_dir.join("target.png"), b"do not touch")
        .await
        .unwrap();

    std::os::unix::fs::symlink(temp_dir.join("target.png"), temp_dir.join("media.png")).unwrap();

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();

    let err = adapter
        .put_bytes_create_only("media.png", b"hostile", "image/png")
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let untouched = fs::read(temp_dir.join("target.png")).await.unwrap();
    assert_eq!(untouched, b"do not touch");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_put_bytes_create_only_yields_exactly_one_winner_under_concurrent_writers() {
    use std::sync::Arc;

    let temp_dir = std::env::temp_dir().join("underlay-blob-create-only-race-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = Arc::new(LocalAdapter::new(config).await.unwrap());

    let a = {
        let adapter = adapter.clone();
        tokio::spawn(
            async move { adapter.put_bytes_create_only("media/race.bin", b"writer-a", "application/octet-stream").await },
        )
    };
    let b = {
        let adapter = adapter.clone();
        tokio::spawn(
            async move { adapter.put_bytes_create_only("media/race.bin", b"writer-b", "application/octet-stream").await },
        )
    };

    let (a, b) = (a.await.unwrap(), b.await.unwrap());
    let outcomes = [a.is_ok(), b.is_ok()];
    assert_eq!(
        outcomes.iter().filter(|ok| **ok).count(),
        1,
        "exactly one writer must create the destination"
    );
    let loser = if a.is_ok() { b } else { a };
    assert!(matches!(loser.unwrap_err(), BlobError::DestinationExists(_)));

    // The winner's exact bytes persist, uncorrupted by the loser.
    let stored = adapter.read_file("media/race.bin").await.unwrap();
    assert!(stored == b"writer-a" || stored == b"writer-b");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_promote_verified_round_trip_preserves_staging_and_derives_sha256() {
    use crate::config::BlobUploadConfig;
    use crate::promotion::BlobAdapterPromotionExt;
    use crate::types::BlobObjectKey;
    use sha2::{Digest, Sha256};

    let temp_dir = std::env::temp_dir().join("underlay-blob-promote-round-trip-test");
    let _ = fs::remove_dir_all(&temp_dir).await;

    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    let upload_config = BlobUploadConfig::default();

    let png: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];
    adapter.write_file("staging/upload-1.png", png, "image/png").await.unwrap();

    let staging = BlobObjectKey::parse("staging/upload-1.png").unwrap();
    let destination = BlobObjectKey::parse("media/versions/1.png").unwrap();

    let result = adapter
        .promote_verified(&staging, &destination, "image/png", &upload_config)
        .await
        .unwrap();

    assert_eq!(result.object.key, "media/versions/1.png");
    assert_eq!(result.object.content_type, "image/png");
    assert_eq!(result.object.size, png.len() as u64);
    assert_eq!(result.sha256, hex::encode(Sha256::digest(png)));

    // Destination holds exactly the captured bytes.
    let published = adapter.read_file("media/versions/1.png").await.unwrap();
    assert_eq!(published, png);

    // Staging is preserved for the caller's own cleanup/recovery policy.
    let staged = adapter.read_file("staging/upload-1.png").await.unwrap();
    assert_eq!(staged, png);

    // Re-promoting to the same destination is a collision, not an
    // overwrite.
    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &upload_config)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let _ = fs::remove_dir_all(&temp_dir).await;
}
