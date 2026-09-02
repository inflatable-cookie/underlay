//! Unproven ownership refuses without disclosing the token.

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use super::owned_support::{
    assert_no_token_disclosure, authority, keys, token, FakeOwnedAdapter, PNG, TOKEN_A, TOKEN_B,
};
use crate::config::BlobUploadConfig;
use crate::error::BlobError;
use crate::owned::OwnedDestinationAuthority;
use crate::promotion::BlobAdapterPromotionExt;
use crate::types::BlobObjectKey;

#[tokio::test]
async fn pre_create_foreign_identical_incumbent_refuses_recovery_and_preserves_bytes() {
    let adapter = FakeOwnedAdapter::default();
    adapter.seed("media/a.png", PNG, "image/png");
    let token = token(TOKEN_A);
    let err = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_no_token_disclosure(&err, TOKEN_A);
    assert_eq!(adapter.stored_bytes("media/a.png"), Some(PNG.to_vec()));
    assert!(adapter.stored_metadata("media/a.png").unwrap().is_empty());
}

#[tokio::test]
async fn wrong_token_provider_bucket_and_key_refuse_without_disclosure() {
    let adapter = FakeOwnedAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    let (staging, destination) = keys();
    let owned_token = token(TOKEN_A);
    adapter
        .promote_verified_owned(
            &staging,
            &destination,
            "image/png",
            &BlobUploadConfig::default(),
            &owned_token,
        )
        .await
        .unwrap();

    let err = adapter
        .recover_owned_publication(&token(TOKEN_B), &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_no_token_disclosure(&err, TOKEN_B);

    let heads_before_authority_check = adapter.head_calls.load(Ordering::SeqCst);
    let err = adapter
        .recover_owned_publication(
            &owned_token,
            &OwnedDestinationAuthority::new(
                "s3",
                "fake-bucket",
                BlobObjectKey::parse("media/a.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));
    assert_eq!(
        adapter.head_calls.load(Ordering::SeqCst),
        heads_before_authority_check,
        "wrong provider must not inspect destination head"
    );

    let err = adapter
        .recover_owned_publication(
            &owned_token,
            &OwnedDestinationAuthority::new(
                "fake",
                "other-bucket",
                BlobObjectKey::parse("media/a.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));

    let err = adapter
        .recover_owned_publication(
            &owned_token,
            &OwnedDestinationAuthority::new(
                "fake",
                "fake-bucket",
                BlobObjectKey::parse("media/other.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::NotFound(_)));
}

#[tokio::test]
async fn missing_malformed_and_inconsistent_metadata_refuse() {
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

    let original = adapter.stored_metadata("media/a.png").unwrap();

    adapter.plant_metadata("media/a.png", HashMap::new());
    let err = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let mut malformed = original.clone();
    malformed.insert(
        crate::owned::OWNED_META_SHA256.to_string(),
        "zzzz".to_string(),
    );
    adapter.plant_metadata("media/a.png", malformed);
    let err = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let mut bad_size = original.clone();
    bad_size.insert(crate::owned::OWNED_META_SIZE.to_string(), "999".to_string());
    adapter.plant_metadata("media/a.png", bad_size);
    let err = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));

    let mut bad_mime = original;
    bad_mime.insert(
        crate::owned::OWNED_META_MIME.to_string(),
        "image/png\n".to_string(),
    );
    adapter.plant_metadata("media/a.png", bad_mime);
    let err = adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_eq!(adapter.stored_bytes("media/a.png"), Some(PNG.to_vec()));
}

#[tokio::test]
async fn copied_metadata_on_another_key_is_unproven() {
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

    let copied = adapter.stored_metadata("media/a.png").unwrap();
    adapter.seed("media/hostile.png", PNG, "image/png");
    adapter.plant_metadata("media/hostile.png", copied);

    let err = adapter
        .recover_owned_publication(
            &token,
            &OwnedDestinationAuthority::new(
                "fake",
                "fake-bucket",
                BlobObjectKey::parse("media/hostile.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_no_token_disclosure(&err, TOKEN_A);
    adapter
        .recover_owned_publication(&token, &authority())
        .await
        .unwrap();
}
