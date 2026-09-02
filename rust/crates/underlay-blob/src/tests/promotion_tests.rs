//! `promote_verified` proof against a minimal in-memory adapter double.
//!
//! These tests are feature-independent (no `s3`/`local` storage backend
//! needed) so the core composition logic — capture once, validate,
//! exclusive-create — is always exercised, and to give an adapter-agnostic
//! home for the "mutable staging swap" oracle: the swap is simulated
//! deterministically rather than raced against real filesystem timing.

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use sha2::{Digest, Sha256};

use crate::adapter::{BlobAdapter, NoopAdapter};
use crate::config::BlobUploadConfig;
use crate::error::{BlobError, BlobResult};
use crate::promotion::BlobAdapterPromotionExt;
use crate::types::{
    BlobObjectKey, DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];

/// Minimal in-memory adapter double. Only `get_bytes_bounded` and
/// `put_bytes_create_only` are genuinely implemented; every other method
/// panics so these tests fail loudly if `promote_verified` ever regresses
/// to calling the unbounded `get_bytes` or the unconditional `put_bytes`.
#[derive(Default)]
struct FakeAdapter {
    objects: Mutex<HashMap<String, (Vec<u8>, String)>>,
    /// Bytes to splice into storage the moment a bounded read for the given
    /// key returns, simulating an attacker swap that lands after capture.
    swap_after_read: Mutex<Option<(String, Vec<u8>)>>,
}

impl FakeAdapter {
    fn seed(&self, key: &str, bytes: impl Into<Vec<u8>>, content_type: &str) {
        self.objects
            .lock()
            .unwrap()
            .insert(key.to_string(), (bytes.into(), content_type.to_string()));
    }

    fn stored(&self, key: &str) -> Option<Vec<u8>> {
        self.objects
            .lock()
            .unwrap()
            .get(key)
            .map(|(b, _)| b.clone())
    }
}

#[async_trait]
impl BlobAdapter for FakeAdapter {
    async fn initiate_upload(&self, _request: UploadRequest) -> BlobResult<UploadPlan> {
        unimplemented!("not used by promotion tests")
    }

    async fn finalise_upload(&self, _key: &str) -> BlobResult<StoredObject> {
        unimplemented!("not used by promotion tests")
    }

    fn public_url(&self, key: &str) -> String {
        key.to_string()
    }

    async fn signed_download_url(&self, _request: DownloadRequest) -> BlobResult<SignedUrl> {
        unimplemented!("not used by promotion tests")
    }

    async fn delete(&self, _key: &str) -> BlobResult<()> {
        Ok(())
    }

    async fn head(&self, _key: &str) -> BlobResult<ObjectInfo> {
        unimplemented!("not used by promotion tests")
    }

    async fn get_bytes(&self, _key: &str) -> BlobResult<Vec<u8>> {
        unimplemented!("promote_verified must not call the unbounded get_bytes")
    }

    async fn put_bytes(
        &self,
        _key: &str,
        _data: &[u8],
        _content_type: &str,
    ) -> BlobResult<StoredObject> {
        unimplemented!("promote_verified must not call the unconditional put_bytes")
    }

    fn name(&self) -> &'static str {
        "fake"
    }

    fn bucket(&self) -> &str {
        "fake-bucket"
    }

    async fn get_bytes_bounded(&self, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
        let bytes = {
            let objects = self.objects.lock().unwrap();
            objects.get(key).cloned().map(|(bytes, _)| bytes)
        }
        .ok_or_else(|| BlobError::NotFound(key.to_string()))?;

        if bytes.len() as u64 > max_bytes {
            return Err(BlobError::TooLarge(bytes.len() as u64, max_bytes));
        }

        // Land the simulated swap only after the read above has already
        // captured its own copy of the original bytes.
        let swap = self.swap_after_read.lock().unwrap().take();
        if let Some((swap_key, swap_bytes)) = swap {
            if swap_key == key {
                self.objects.lock().unwrap().insert(
                    swap_key,
                    (swap_bytes, "application/octet-stream".to_string()),
                );
            }
        }

        Ok(bytes)
    }

    async fn put_bytes_create_only(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        let mut objects = self.objects.lock().unwrap();
        if objects.contains_key(key) {
            return Err(BlobError::DestinationExists(key.to_string()));
        }
        objects.insert(key.to_string(), (data.to_vec(), content_type.to_string()));
        Ok(StoredObject::new(
            "fake",
            "fake-bucket",
            key,
            data.len() as u64,
            content_type,
        ))
    }
}

#[tokio::test]
async fn promote_verified_rejects_equal_staging_and_destination_keys() {
    let adapter = FakeAdapter::default();
    let key = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&key, &key, "image/png", &config)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::InvalidKey(_)));
}

#[tokio::test]
async fn promote_verified_publishes_captured_bytes_and_derives_sha256() {
    let adapter = FakeAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let result = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap();

    assert_eq!(result.object.provider, "fake");
    assert_eq!(result.object.key, "media/a.png");
    assert_eq!(result.object.content_type, "image/png");
    assert_eq!(result.object.size, PNG.len() as u64);
    assert_eq!(result.sha256.len(), 64);
    assert_eq!(result.sha256, hex::encode(Sha256::digest(PNG)));

    // Staging is preserved for the caller's own cleanup/recovery policy.
    assert_eq!(adapter.stored("staging/a.png"), Some(PNG.to_vec()));
    assert_eq!(adapter.stored("media/a.png"), Some(PNG.to_vec()));
}

#[tokio::test]
async fn promote_verified_publishes_the_captured_vector_even_if_staging_is_swapped_right_after_capture(
) {
    let adapter = FakeAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");

    // Same size, same declared type — the exact swap that would be
    // indistinguishable from a mutable size/MIME check alone.
    let swapped: Vec<u8> = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0xFF, 0xFF];
    assert_eq!(swapped.len(), PNG.len());
    *adapter.swap_after_read.lock().unwrap() = Some(("staging/a.png".to_string(), swapped));

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let result = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap();

    // Published bytes and hash come from the captured vector, not from
    // whatever staging holds by the time promotion finishes.
    assert_eq!(result.sha256, hex::encode(Sha256::digest(PNG)));
    assert_eq!(adapter.stored("media/a.png"), Some(PNG.to_vec()));
    assert_ne!(adapter.stored("staging/a.png"), Some(PNG.to_vec()));
}

#[tokio::test]
async fn promote_verified_refuses_a_destination_collision_and_preserves_it() {
    let adapter = FakeAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");
    adapter.seed("media/a.png", b"already here".as_slice(), "image/png");

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::DestinationExists(_)));
    assert_eq!(
        adapter.stored("media/a.png"),
        Some(b"already here".to_vec())
    );
}

#[tokio::test]
async fn promote_verified_rejects_a_declared_mime_not_matching_magic_bytes() {
    let adapter = FakeAdapter::default();
    adapter.seed(
        "staging/evil.png",
        b"<html><script>alert(1)</script>".as_slice(),
        "image/png",
    );

    let staging = BlobObjectKey::parse("staging/evil.png").unwrap();
    let destination = BlobObjectKey::parse("media/evil.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::InvalidContentType(_)));
    assert!(adapter.stored("media/evil.png").is_none());
}

#[tokio::test]
async fn promote_verified_rejects_a_disallowed_declared_content_type() {
    let adapter = FakeAdapter::default();
    adapter.seed(
        "staging/page.html",
        b"<html></html>".as_slice(),
        "text/html",
    );

    let staging = BlobObjectKey::parse("staging/page.html").unwrap();
    let destination = BlobObjectKey::parse("media/page.html").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "text/html", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::InvalidContentType(_)));
}

#[tokio::test]
async fn promote_verified_rejects_a_staging_source_over_the_configured_bound() {
    let adapter = FakeAdapter::default();
    adapter.seed("staging/a.png", PNG, "image/png");

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default().max_file_size_bytes((PNG.len() - 1) as u64);

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::TooLarge(_, _)));
    assert!(adapter.stored("media/a.png").is_none());
}

#[tokio::test]
async fn promote_verified_refuses_on_an_adapter_without_bounded_capture_or_exclusive_create() {
    // NoopAdapter is this crate's own minimal "custom adapter" example: it
    // does not override the new methods and must compile and refuse via
    // their fail-closed defaults.
    let adapter = NoopAdapter::new();
    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::Unsupported(_)));
}

/// An adapter that genuinely, exclusively writes the requested destination
/// bytes but reports a different key or size back. `promote_verified` must
/// not trust that echoed identity — otherwise `VerifiedPromotionResult`
/// could describe an object the caller never actually asked to publish.
#[derive(Default)]
struct LyingAdapter {
    objects: Mutex<HashMap<String, Vec<u8>>>,
    lie_about_key: Option<&'static str>,
    lie_about_size: Option<u64>,
}

#[async_trait]
impl BlobAdapter for LyingAdapter {
    async fn initiate_upload(&self, _request: UploadRequest) -> BlobResult<UploadPlan> {
        unimplemented!("not used by promotion tests")
    }
    async fn finalise_upload(&self, _key: &str) -> BlobResult<StoredObject> {
        unimplemented!("not used by promotion tests")
    }
    fn public_url(&self, key: &str) -> String {
        key.to_string()
    }
    async fn signed_download_url(&self, _request: DownloadRequest) -> BlobResult<SignedUrl> {
        unimplemented!("not used by promotion tests")
    }
    async fn delete(&self, _key: &str) -> BlobResult<()> {
        Ok(())
    }
    async fn head(&self, _key: &str) -> BlobResult<ObjectInfo> {
        unimplemented!("not used by promotion tests")
    }
    async fn get_bytes(&self, _key: &str) -> BlobResult<Vec<u8>> {
        unimplemented!("not used by promotion tests")
    }
    async fn put_bytes(
        &self,
        _key: &str,
        _data: &[u8],
        _content_type: &str,
    ) -> BlobResult<StoredObject> {
        unimplemented!("not used by promotion tests")
    }
    fn name(&self) -> &'static str {
        "lying"
    }
    fn bucket(&self) -> &str {
        "lying-bucket"
    }

    async fn get_bytes_bounded(&self, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
        let bytes = self
            .objects
            .lock()
            .unwrap()
            .get(key)
            .cloned()
            .ok_or_else(|| BlobError::NotFound(key.to_string()))?;
        if bytes.len() as u64 > max_bytes {
            return Err(BlobError::TooLarge(bytes.len() as u64, max_bytes));
        }
        Ok(bytes)
    }

    async fn put_bytes_create_only(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        // Genuinely, exclusively write the real bytes at the real key...
        let mut objects = self.objects.lock().unwrap();
        if objects.contains_key(key) {
            return Err(BlobError::DestinationExists(key.to_string()));
        }
        objects.insert(key.to_string(), data.to_vec());
        drop(objects);

        // ...but report a different identity for it.
        let reported_key = self.lie_about_key.unwrap_or(key);
        let reported_size = self.lie_about_size.unwrap_or(data.len() as u64);
        Ok(StoredObject::new(
            "lying",
            "lying-bucket",
            reported_key,
            reported_size,
            content_type,
        ))
    }
}

#[tokio::test]
async fn promote_verified_rejects_an_adapter_that_reports_a_different_destination_key() {
    let adapter = LyingAdapter {
        lie_about_key: Some("media/somewhere-else.png"),
        ..Default::default()
    };
    adapter
        .objects
        .lock()
        .unwrap()
        .insert("staging/a.png".to_string(), PNG.to_vec());

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::Internal(_)));
}

#[tokio::test]
async fn promote_verified_rejects_an_adapter_that_reports_a_different_size() {
    let adapter = LyingAdapter {
        lie_about_size: Some(999),
        ..Default::default()
    };
    adapter
        .objects
        .lock()
        .unwrap()
        .insert("staging/a.png".to_string(), PNG.to_vec());

    let staging = BlobObjectKey::parse("staging/a.png").unwrap();
    let destination = BlobObjectKey::parse("media/a.png").unwrap();
    let config = BlobUploadConfig::default();

    let err = adapter
        .promote_verified(&staging, &destination, "image/png", &config)
        .await
        .unwrap_err();

    assert!(matches!(err, BlobError::Internal(_)));
}
