use super::*;
use crate::adapter::BlobAdapter;
use crate::error::BlobError;
use crate::types::DownloadRequest;

#[test]
fn minio_dev_config_uses_path_style_endpoint_and_bucket_public_base() {
    let config = S3Config::minio_dev("acme-media", "http://s3.acme.test:9000/");

    assert_eq!(config.bucket, "acme-media");
    assert_eq!(config.region, "us-east-1");
    assert_eq!(
        config.endpoint_url.as_deref(),
        Some("http://s3.acme.test:9000")
    );
    assert_eq!(
        config.public_url_base.as_deref(),
        Some("http://s3.acme.test:9000/acme-media")
    );
    assert!(config.path_style);
    assert!(config.public_read);
    assert_eq!(config.presign_url_base, None);
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
