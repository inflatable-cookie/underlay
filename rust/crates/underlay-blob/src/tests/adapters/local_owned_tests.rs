use super::*;
use crate::owned::{OwnedDestinationAuthority, OwnershipToken};
use crate::promotion::BlobAdapterPromotionExt;
use crate::types::BlobObjectKey;
use sha2::{Digest, Sha256};

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];
const TOKEN: &[u8] = b"tokensecret-disclosure-probe!!!!";

async fn adapter_in(label: &str) -> (LocalAdapter, std::path::PathBuf) {
    let temp_dir = std::env::temp_dir().join(format!("underlay-blob-local-owned-{label}"));
    let _ = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
    let adapter = LocalAdapter::new(config).await.unwrap();
    (adapter, temp_dir)
}

fn token() -> OwnershipToken {
    OwnershipToken::from_bytes(TOKEN.to_vec()).unwrap()
}

fn dest_authority(adapter: &LocalAdapter, key: &str) -> OwnedDestinationAuthority {
    OwnedDestinationAuthority::new(
        adapter.name(),
        adapter.bucket(),
        BlobObjectKey::parse(key).unwrap(),
    )
    .unwrap()
}

fn facts_for(
    adapter: &LocalAdapter,
    token: &OwnershipToken,
    key: &str,
) -> crate::OwnedPublicationFacts {
    crate::OwnedPublicationFacts::from_token_and_bytes(
        token,
        &dest_authority(adapter, key),
        PNG,
        "image/png",
    )
}

#[tokio::test]
async fn owned_create_returns_complete_facts_from_head_together_with_the_object() {
    let (adapter, temp_dir) = adapter_in("head-facts").await;
    let token = token();
    let facts = facts_for(&adapter, &token, "media/a.png");

    adapter
        .put_bytes_create_only_owned("media/a.png", PNG, "image/png", &facts)
        .await
        .unwrap();

    let info = adapter.head("media/a.png").await.unwrap();
    let parsed = crate::OwnedPublicationFacts::from_object_metadata(&info.metadata).unwrap();
    assert!(parsed.matches_token(&token, &dest_authority(&adapter, "media/a.png")));
    assert_eq!(parsed.size(), PNG.len() as u64);
    assert_eq!(parsed.mime(), "image/png");
    assert_eq!(
        adapter
            .get_bytes_bounded("media/a.png", 1024)
            .await
            .unwrap(),
        PNG
    );

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn unowned_create_has_no_reserved_metadata_and_recovery_refuses_identical_bytes() {
    let (adapter, temp_dir) = adapter_in("foreign-identical").await;
    adapter
        .put_bytes_create_only("media/a.png", PNG, "image/png")
        .await
        .unwrap();

    let info = adapter.head("media/a.png").await.unwrap();
    assert!(info.metadata.is_empty());

    let err = adapter
        .recover_owned_publication(
            &token(),
            &OwnedDestinationAuthority::new(
                "local",
                adapter.bucket(),
                BlobObjectKey::parse("media/a.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_eq!(
        adapter
            .get_bytes_bounded("media/a.png", 1024)
            .await
            .unwrap(),
        PNG
    );

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn owned_promotion_recovers_after_staging_is_removed_or_replaced() {
    let (adapter, temp_dir) = adapter_in("staging-gone").await;
    adapter
        .put_bytes("staging/a.png", PNG, "image/png")
        .await
        .unwrap();

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let token = token();
    adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &crate::BlobUploadConfig::default(),
            &token,
        )
        .await
        .unwrap();

    adapter.delete("staging/a.png").await.unwrap();
    let recovered = adapter
        .recover_owned_publication(
            &token,
            &OwnedDestinationAuthority::new("local", adapter.bucket(), destination.clone())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(recovered.sha256, hex::encode(Sha256::digest(PNG)));

    fs::create_dir_all(temp_dir.join("outside")).await.unwrap();
    fs::create_dir_all(temp_dir.join("staging")).await.unwrap();
    fs::write(temp_dir.join("outside/hostile.bin"), b"nope")
        .await
        .unwrap();
    std::os::unix::fs::symlink(
        temp_dir.join("outside/hostile.bin"),
        temp_dir.join("staging/a.png"),
    )
    .unwrap();

    let recovered_again = adapter
        .recover_owned_publication(
            &token,
            &OwnedDestinationAuthority::new("local", adapter.bucket(), destination).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(recovered_again.sha256, recovered.sha256);
    assert_eq!(
        adapter
            .get_bytes_bounded("media/a.png", 1024)
            .await
            .unwrap(),
        PNG
    );

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn concurrent_owned_writers_yield_one_creator() {
    use std::sync::Arc;

    let (adapter, temp_dir) = adapter_in("race").await;
    let adapter = Arc::new(adapter);
    let mut handles = Vec::new();
    for i in 0..8u8 {
        let adapter = adapter.clone();
        handles.push(tokio::spawn(async move {
            let token = OwnershipToken::from_bytes(vec![i; 32]).unwrap();
            let facts = facts_for(&adapter, &token, "media/race.png");
            adapter
                .put_bytes_create_only_owned("media/race.png", PNG, "image/png", &facts)
                .await
        }));
    }

    let mut ok = 0;
    let mut collisions = 0;
    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => ok += 1,
            Err(BlobError::DestinationExists(_)) => collisions += 1,
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }
    assert_eq!(ok, 1);
    assert_eq!(collisions, 7);

    let info = adapter.head("media/race.png").await.unwrap();
    assert!(crate::OwnedPublicationFacts::from_object_metadata(&info.metadata).is_some());

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn copied_owned_object_does_not_recover_under_the_new_key() {
    let (adapter, temp_dir) = adapter_in("copied-metadata").await;
    let token = token();
    adapter
        .put_bytes_create_only_owned(
            "media/a.png",
            PNG,
            "image/png",
            &facts_for(&adapter, &token, "media/a.png"),
        )
        .await
        .unwrap();

    let original = adapter.head("media/a.png").await.unwrap();
    adapter
        .put_bytes("media/hostile.png", PNG, "image/png")
        .await
        .unwrap();
    let hostile_path = adapter.path_for_key("media/hostile.png").unwrap();
    for (key, value) in &original.metadata {
        super::xattr::plant_raw_reserved_xattr(&hostile_path, key, value.as_bytes()).unwrap();
    }

    let err = adapter
        .recover_owned_publication(&token, &dest_authority(&adapter, "media/hostile.png"))
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    let rendered = format!("{err:?}{err}");
    assert!(!rendered.contains("tokensecret"));

    let recovered = adapter
        .recover_owned_publication(&token, &dest_authority(&adapter, "media/a.png"))
        .await
        .unwrap();
    assert_eq!(recovered.object.key, "media/a.png");

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn same_token_on_two_destinations_recovers_only_at_each_bound_key() {
    let (adapter, temp_dir) = adapter_in("token-reuse").await;
    let token = token();
    adapter
        .put_bytes_create_only_owned(
            "media/a.png",
            PNG,
            "image/png",
            &facts_for(&adapter, &token, "media/a.png"),
        )
        .await
        .unwrap();
    adapter
        .put_bytes_create_only_owned(
            "media/b.png",
            PNG,
            "image/png",
            &facts_for(&adapter, &token, "media/b.png"),
        )
        .await
        .unwrap();

    let head_a = adapter.head("media/a.png").await.unwrap();
    let head_b = adapter.head("media/b.png").await.unwrap();
    assert_ne!(
        head_a.metadata.get(crate::owned::OWNED_META_VERIFIER),
        head_b.metadata.get(crate::owned::OWNED_META_VERIFIER)
    );

    adapter
        .recover_owned_publication(&token, &dest_authority(&adapter, "media/a.png"))
        .await
        .unwrap();
    adapter
        .recover_owned_publication(&token, &dest_authority(&adapter, "media/b.png"))
        .await
        .unwrap();

    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn oversized_reserved_xattr_keeps_v096_head_and_refuses_owned_recovery() {
    let (adapter, temp_dir) = adapter_in("oversized-xattr").await;
    adapter
        .put_bytes("media/legacy.bin", PNG, "application/octet-stream")
        .await
        .unwrap();
    let path = adapter.path_for_key("media/legacy.bin").unwrap();
    let oversized = vec![0x41u8; 300];
    super::xattr::plant_raw_reserved_xattr(&path, crate::owned::OWNED_META_VERIFIER, &oversized)
        .unwrap();

    let info = adapter
        .head("media/legacy.bin")
        .await
        .expect("head must not become IoError for hostile reserved xattr");
    assert!(!info
        .metadata
        .contains_key(crate::owned::OWNED_META_VERIFIER));
    assert!(adapter.exists("media/legacy.bin").await.unwrap());

    let err = adapter
        .recover_owned_publication(&token(), &dest_authority(&adapter, "media/legacy.bin"))
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let err = adapter
        .put_bytes_create_only_owned(
            "media/legacy.bin",
            PNG,
            "image/png",
            &facts_for(&adapter, &token(), "media/legacy.bin"),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_eq!(
        adapter
            .get_bytes_bounded("media/legacy.bin", 1024)
            .await
            .unwrap(),
        PNG
    );

    let _ = fs::remove_dir_all(&temp_dir).await;
}
