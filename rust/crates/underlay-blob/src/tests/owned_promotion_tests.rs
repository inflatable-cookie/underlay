//! Owned promotion and staging-independent recovery against the in-memory double.

use std::sync::atomic::Ordering;

use sha2::{Digest, Sha256};

use super::owned_support::{authority, keys, token, FakeOwnedAdapter, PNG, TOKEN_A};
use crate::adapter::{BlobAdapter, NoopAdapter};
use crate::config::BlobUploadConfig;
use crate::error::BlobError;
use crate::owned::OwnedPublicationFacts;
use crate::promotion::BlobAdapterPromotionExt;

#[tokio::test]
async fn promote_verified_owned_publishes_facts_and_preserves_staging() {
    let adapter = FakeOwnedAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    let (staging, destination) = keys();
    let token = token(TOKEN_A);

    let result = adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &token,
        )
        .await
        .unwrap();

    assert_eq!(result.object.key, "media/a.png");
    assert_eq!(result.sha256, hex::encode(Sha256::digest(PNG)));
    assert_eq!(adapter.stored_bytes("staging/a.png"), Some(PNG.to_vec()));
    assert!(OwnedPublicationFacts::from_object_metadata(
        &adapter.stored_metadata("media/a.png").unwrap()
    )
    .unwrap()
    .matches_token(&token));
}

#[tokio::test]
async fn post_create_recovery_does_not_read_staging() {
    let adapter = FakeOwnedAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    let (staging, destination) = keys();
    let token = token(TOKEN_A);
    adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &token,
        )
        .await
        .unwrap();

    adapter.delete("staging/a.png").await.unwrap();
    adapter.bounded_reads.store(0, Ordering::SeqCst);
    let before_heads = adapter.head_calls.load(Ordering::SeqCst);

    let recovered = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap();

    assert_eq!(recovered.sha256, hex::encode(Sha256::digest(PNG)));
    assert_eq!(recovered.object.content_type, "image/png");
    assert_eq!(adapter.bounded_reads.load(Ordering::SeqCst), 0);
    assert_eq!(adapter.head_calls.load(Ordering::SeqCst), before_heads + 1);
    assert!(adapter.stored_bytes("staging/a.png").is_none());
}

#[tokio::test]
async fn ordinary_retry_collision_stays_destination_exists_then_matching_token_recovers() {
    let adapter = FakeOwnedAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    let (staging, destination) = keys();
    let token = token(TOKEN_A);
    adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &token,
        )
        .await
        .unwrap();

    let err = adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &token,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let recovered = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap();
    assert_eq!(recovered.object.size, PNG.len() as u64);
}

#[tokio::test]
async fn existing_adapters_refuse_owned_create() {
    let adapter = NoopAdapter::new();
    let (staging, destination) = keys();
    let err = adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &token(TOKEN_A),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::Unsupported(_)));
}
