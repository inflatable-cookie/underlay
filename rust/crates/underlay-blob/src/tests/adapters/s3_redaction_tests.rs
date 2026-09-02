//! Proof that no raw backend error or credential-shaped fixture reaches the
//! public error from the new S3 bounded-capture / exclusive-create paths.
//!
//! Split from `s3_tests.rs` to keep both files under the doctor god-files
//! threshold; this file duplicates the small replay-client harness rather
//! than reaching into that module.

use super::*;
use crate::adapter::BlobAdapter;
use crate::error::BlobError;

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use aws_types::region::Region;
use aws_types::sdk_config::SdkConfig;

fn s3_adapter_with_replay(events: Vec<ReplayEvent>) -> S3Adapter {
    let replay_client = StaticReplayClient::new(events);

    let sdk_config = SdkConfig::builder()
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::for_tests()))
        .http_client(replay_client)
        .build();

    let config = S3Config::new("fixture-bucket", "us-east-1");
    S3Adapter::from_aws_config(&sdk_config, config)
}

/// A hostile S3-shaped XML error body: real S3 error responses for
/// signature failures echo the canonical request, the access key id, and a
/// computed signature back to the caller. None of that text — nor the
/// generic `AWSAccessKeyId`/`StringToSign` markers themselves — may ever
/// reach the public `BlobError`.
fn hostile_error_body() -> SdkBody {
    SdkBody::from(
        concat!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n",
            "<Error>",
            "<Code>SignatureDoesNotMatch</Code>",
            "<Message>The request signature we calculated does not match the signature you provided.</Message>",
            "<AWSAccessKeyId>AKIAIOSFODNN7EXAMPLE</AWSAccessKeyId>",
            "<StringToSign>AWS4-HMAC-SHA256\n20260902T000000Z\nsecret-canonical-request-body</StringToSign>",
            "<SignatureProvided>deadbeefcafefeed1234567890abcdef1234567890abcdef1234567890abcd</SignatureProvided>",
            "<RequestId>ABCD1234EXAMPLE</RequestId>",
            "</Error>",
        ),
    )
}

const HOSTILE_MARKERS: &[&str] = &[
    "AKIAIOSFODNN7EXAMPLE",
    "StringToSign",
    "secret-canonical-request-body",
    "SignatureProvided",
    "deadbeefcafefeed1234567890abcdef1234567890abcdef1234567890abcd",
];

fn assert_no_leak(err: &BlobError) {
    let rendered = format!("{err} {err:?}");
    for marker in HOSTILE_MARKERS {
        assert!(
            !rendered.contains(marker),
            "public error leaked hostile provider content {marker:?}: {rendered}"
        );
    }
}

async fn hostile_put_response(status: u16) -> BlobError {
    let adapter = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("PUT")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(status)
            .body(hostile_error_body())
            .unwrap(),
    )]);

    adapter
        .put_bytes_create_only("media/a.png", b"bytes", "image/png")
        .await
        .expect_err("hostile response must not read as success")
}

#[tokio::test]
async fn put_bytes_create_only_redacts_a_hostile_412_response() {
    let err = hostile_put_response(412).await;
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_no_leak(&err);
}

#[tokio::test]
async fn put_bytes_create_only_redacts_a_hostile_409_response() {
    let err = hostile_put_response(409).await;
    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_no_leak(&err);
}

#[tokio::test]
async fn put_bytes_create_only_redacts_a_hostile_500_response() {
    let err = hostile_put_response(500).await;
    assert!(!matches!(err, BlobError::DestinationExists(_)));
    assert_no_leak(&err);
}

#[tokio::test]
async fn get_bytes_bounded_redacts_a_hostile_403_response() {
    let adapter = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("GET")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(403)
            .body(hostile_error_body())
            .unwrap(),
    )]);

    let err = adapter
        .get_bytes_bounded("media/a.png", 1024)
        .await
        .expect_err("hostile response must not read as success");
    assert_no_leak(&err);
}
