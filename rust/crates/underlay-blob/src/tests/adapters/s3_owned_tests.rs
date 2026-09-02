use super::*;
use crate::adapter::BlobAdapter;
use crate::error::BlobError;
use crate::owned::{OwnedDestinationAuthority, OwnedPublicationFacts, OwnershipToken};
use crate::promotion::BlobAdapterPromotionExt;
use crate::types::BlobObjectKey;

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use aws_types::region::Region;
use aws_types::sdk_config::SdkConfig;
use sha2::{Digest, Sha256};

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];
const TOKEN: &[u8] = b"tokensecret-disclosure-probe!!!!";

fn s3_adapter_with_replay(events: Vec<ReplayEvent>) -> (S3Adapter, StaticReplayClient) {
    let replay_client = StaticReplayClient::new(events);
    let sdk_config = SdkConfig::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .http_client(replay_client.clone())
        .build();
    let config = S3Config::new("fixture-bucket", "us-east-1");
    let adapter = S3Adapter::from_aws_config(&sdk_config, config);
    (adapter, replay_client)
}

fn token() -> OwnershipToken {
    OwnershipToken::from_bytes(TOKEN.to_vec()).unwrap()
}

fn facts() -> OwnedPublicationFacts {
    OwnedPublicationFacts::from_token_and_bytes(&token(), PNG, "image/png")
}

fn verifier_hex(facts: &OwnedPublicationFacts) -> String {
    facts.metadata_pairs()[0].1.clone()
}

#[tokio::test]
async fn put_bytes_create_only_owned_sends_one_conditional_put_with_reserved_metadata() {
    let facts = facts();
    let (adapter, replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("PUT")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(200)
            .header("ETag", "\"abc123\"")
            .body(SdkBody::empty())
            .unwrap(),
    )]);

    adapter
        .put_bytes_create_only_owned("media/a.png", PNG, "image/png", &facts)
        .await
        .expect("owned create should succeed");

    let requests: Vec<_> = replay.actual_requests().collect();
    assert_eq!(requests.len(), 1, "must send exactly one PUT request");
    let headers = requests[0].headers();
    let verifier = verifier_hex(&facts);
    assert_eq!(headers.get("if-none-match").unwrap(), "*");
    assert_eq!(
        headers
            .get("x-amz-meta-underlay-owned-v1-verifier")
            .unwrap(),
        verifier.as_str()
    );
    assert_eq!(
        headers.get("x-amz-meta-underlay-owned-v1-sha256").unwrap(),
        facts.sha256()
    );
    assert_eq!(
        headers.get("x-amz-meta-underlay-owned-v1-size").unwrap(),
        PNG.len().to_string().as_str()
    );
    assert_eq!(
        headers.get("x-amz-meta-underlay-owned-v1-mime").unwrap(),
        "image/png"
    );
    let rendered = format!("{headers:?}");
    assert!(!rendered.contains("tokensecret"));
}

#[tokio::test]
async fn recover_owned_publication_uses_head_only() {
    let facts = facts();
    let verifier = verifier_hex(&facts);
    let (adapter, replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("HEAD")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(200)
            .header("Content-Length", PNG.len().to_string())
            .header("Content-Type", "image/png")
            .header("ETag", "\"abc123\"")
            .header("x-amz-meta-underlay-owned-v1-verifier", verifier.as_str())
            .header("x-amz-meta-underlay-owned-v1-sha256", facts.sha256())
            .header("x-amz-meta-underlay-owned-v1-size", PNG.len().to_string())
            .header("x-amz-meta-underlay-owned-v1-mime", "image/png")
            .body(SdkBody::empty())
            .unwrap(),
    )]);

    let recovered = adapter
        .recover_owned_publication(
            &token(),
            &OwnedDestinationAuthority::new(
                "s3",
                "fixture-bucket",
                BlobObjectKey::parse("media/a.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .expect("matching token must recover from head");

    assert_eq!(recovered.sha256, hex::encode(Sha256::digest(PNG)));
    assert_eq!(recovered.object.content_type, "image/png");
    assert_eq!(recovered.object.size, PNG.len() as u64);
    let requests: Vec<_> = replay.actual_requests().collect();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method(), "HEAD");
}

#[tokio::test]
async fn recover_owned_publication_wrong_token_is_a_typed_collision() {
    let facts = facts();
    let verifier = verifier_hex(&facts);
    let (adapter, replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("HEAD")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(200)
            .header("Content-Length", PNG.len().to_string())
            .header("Content-Type", "image/png")
            .header("x-amz-meta-underlay-owned-v1-verifier", verifier.as_str())
            .header("x-amz-meta-underlay-owned-v1-sha256", facts.sha256())
            .header("x-amz-meta-underlay-owned-v1-size", PNG.len().to_string())
            .header("x-amz-meta-underlay-owned-v1-mime", "image/png")
            .body(SdkBody::empty())
            .unwrap(),
    )]);

    let err = adapter
        .recover_owned_publication(
            &OwnershipToken::from_bytes(vec![0x22; 32]).unwrap(),
            &OwnedDestinationAuthority::new(
                "s3",
                "fixture-bucket",
                BlobObjectKey::parse("media/a.png").unwrap(),
            )
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::DestinationExists(_)));
    let rendered = format!("{err:?}{err}");
    assert!(!rendered.contains("tokensecret"));
    assert_eq!(replay.actual_requests().count(), 1);
}
