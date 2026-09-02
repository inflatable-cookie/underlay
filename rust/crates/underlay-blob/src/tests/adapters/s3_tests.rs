use super::*;
use crate::adapter::BlobAdapter;
use crate::error::BlobError;
use crate::types::DownloadRequest;

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use aws_types::region::Region;
use aws_types::sdk_config::SdkConfig;

/// Build an `S3Adapter` whose HTTP traffic is served entirely by a
/// `StaticReplayClient`, so conditional-write and bounded-read behavior can
/// be proved against a request/response fixture without live storage.
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

fn json_response(status: u16) -> http::Response<SdkBody> {
    http::Response::builder()
        .status(status)
        .body(SdkBody::empty())
        .unwrap()
}

#[tokio::test]
async fn put_bytes_create_only_sends_one_conditional_put_with_if_none_match() {
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

    let stored = adapter
        .put_bytes_create_only("media/a.png", b"published bytes", "image/png")
        .await
        .expect("first create should succeed");

    assert_eq!(stored.key, "media/a.png");
    assert_eq!(stored.size, b"published bytes".len() as u64);
    assert_eq!(stored.etag.as_deref(), Some("abc123"));

    let requests: Vec<_> = replay.actual_requests().collect();
    assert_eq!(requests.len(), 1, "must send exactly one PUT request");
    assert_eq!(requests[0].method(), "PUT");
    assert_eq!(
        requests[0]
            .headers()
            .get("if-none-match")
            .expect("PUT must carry If-None-Match"),
        "*"
    );
}

#[tokio::test]
async fn put_bytes_create_only_maps_a_412_precondition_failed_to_a_typed_collision() {
    let (adapter, _replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("PUT")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        json_response(412),
    )]);

    let err = adapter
        .put_bytes_create_only("media/a.png", b"bytes", "image/png")
        .await
        .expect_err("412 must map to a collision, not success");

    assert!(matches!(err, BlobError::DestinationExists(_)));
}

#[tokio::test]
async fn put_bytes_create_only_maps_a_409_conditional_conflict_to_a_typed_collision() {
    let (adapter, _replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("PUT")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        json_response(409),
    )]);

    let err = adapter
        .put_bytes_create_only("media/a.png", b"bytes", "image/png")
        .await
        .expect_err("409 must map to a collision, not success");

    assert!(matches!(err, BlobError::DestinationExists(_)));
}

#[tokio::test]
async fn put_bytes_create_only_maps_a_500_to_transport_failure_not_a_collision() {
    let (adapter, _replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("PUT")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/a.png")
            .body(SdkBody::empty())
            .unwrap(),
        json_response(500),
    )]);

    let err = adapter
        .put_bytes_create_only("media/a.png", b"bytes", "image/png")
        .await
        .expect_err("a real service failure must not read as success");

    assert!(!matches!(err, BlobError::DestinationExists(_)));
}

#[tokio::test]
async fn get_bytes_bounded_stops_at_max_plus_one_even_when_the_body_is_larger() {
    let oversized_body = vec![0x41u8; 1024];
    let (adapter, _replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("GET")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/big.bin")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(200)
            .body(SdkBody::from(oversized_body))
            .unwrap(),
    )]);

    let err = adapter
        .get_bytes_bounded("media/big.bin", 16)
        .await
        .expect_err("an over-limit source must be refused");

    match err {
        BlobError::TooLarge(observed, max) => {
            assert_eq!(max, 16);
            assert_eq!(
                observed, 17,
                "must stop at max_bytes + 1, not the full body"
            );
        }
        other => panic!("expected TooLarge, got {other:?}"),
    }
}

#[tokio::test]
async fn get_bytes_bounded_accepts_a_source_within_the_cap() {
    let (adapter, _replay) = s3_adapter_with_replay(vec![ReplayEvent::new(
        http::Request::builder()
            .method("GET")
            .uri("https://fixture-bucket.s3.us-east-1.amazonaws.com/media/small.bin")
            .body(SdkBody::empty())
            .unwrap(),
        http::Response::builder()
            .status(200)
            .body(SdkBody::from(vec![9u8; 5]))
            .unwrap(),
    )]);

    let bytes = adapter
        .get_bytes_bounded("media/small.bin", 16)
        .await
        .expect("a within-cap source must be accepted");

    assert_eq!(bytes, vec![9u8; 5]);
}

#[test]
fn minio_dev_config_uses_path_style_endpoint_and_bucket_public_base() {
    let config = S3Config::minio_dev("acme-media", "http://s3.acme.test:9000/");

    assert_eq!(config.bucket(), "acme-media");
    assert_eq!(config.region(), "us-east-1");
    assert_eq!(config.endpoint_url_ref(), Some("http://s3.acme.test:9000"));
    assert_eq!(
        config.public_url_base_ref(),
        Some("http://s3.acme.test:9000/acme-media")
    );
    assert!(config.path_style_enabled());
    assert!(config.public_read_enabled());
    assert_eq!(config.presign_url_base_ref(), None);
}

fn test_s3_config() -> Option<S3Config> {
    let bucket = std::env::var("UNDERLAY_BLOB_S3_TEST_BUCKET").ok()?;
    let region =
        std::env::var("UNDERLAY_BLOB_S3_TEST_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let endpoint_url = std::env::var("UNDERLAY_BLOB_S3_TEST_ENDPOINT").ok();
    let path_style = std::env::var("UNDERLAY_BLOB_S3_TEST_PATH_STYLE")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(endpoint_url.is_some());

    let mut config = S3Config::new(bucket, region).path_style(path_style);
    if let Some(endpoint) = endpoint_url {
        config = config.endpoint_url(endpoint);
    }

    Some(config)
}

fn unique_key() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("underlay-blob-integration/{nanos}.txt")
}

#[tokio::test]
async fn s3_adapter_round_trip_put_head_get_delete() {
    let Some(config) = test_s3_config() else {
        return;
    };

    let adapter = S3Adapter::new(config)
        .await
        .expect("failed to create S3 adapter for integration test");

    let key = unique_key();
    let bytes = b"underlay s3 integration test";
    let content_type = "text/plain";

    let stored = adapter
        .put_bytes(&key, bytes, content_type)
        .await
        .expect("put_bytes should upload object");
    assert_eq!(stored.key, key);
    assert_eq!(stored.size, bytes.len() as u64);

    let info = adapter
        .head(&key)
        .await
        .expect("head should find uploaded object");
    assert_eq!(info.size, bytes.len() as u64);
    assert_eq!(info.content_type, content_type);

    let downloaded = adapter
        .get_bytes(&key)
        .await
        .expect("get_bytes should download uploaded object");
    assert_eq!(downloaded, bytes);

    let signed = adapter
        .signed_download_url(DownloadRequest::parse_key(&key).unwrap())
        .await
        .expect("signed_download_url should return URL");
    assert!(!signed.url.is_empty());

    adapter
        .delete(&key)
        .await
        .expect("delete should remove uploaded object");

    let head_after_delete = adapter.head(&key).await;
    assert!(matches!(head_after_delete, Err(BlobError::NotFound(_))));
}
