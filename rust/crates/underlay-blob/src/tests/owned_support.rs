//! In-memory adapter double shared by owned promotion and recovery tests.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use async_trait::async_trait;

use crate::adapter::BlobAdapter;
use crate::error::{BlobError, BlobResult};
use crate::owned::{OwnedDestinationAuthority, OwnedPublicationFacts, OwnershipToken};
use crate::types::{
    BlobObjectKey, DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

pub(super) const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x01];
pub(super) const TOKEN_A: &[u8] = b"tokensecret-disclosure-probe!!!!";
pub(super) const TOKEN_B: &[u8] = b"other-token-not-the-first-one!!!!";

struct Object {
    bytes: Vec<u8>,
    content_type: String,
    metadata: HashMap<String, String>,
}

pub(super) struct FakeOwnedAdapter {
    objects: Mutex<HashMap<String, Object>>,
    pub(super) head_calls: AtomicUsize,
    pub(super) bounded_reads: AtomicUsize,
}

impl Default for FakeOwnedAdapter {
    fn default() -> Self {
        Self {
            objects: Mutex::new(HashMap::new()),
            head_calls: AtomicUsize::new(0),
            bounded_reads: AtomicUsize::new(0),
        }
    }
}

impl FakeOwnedAdapter {
    pub(super) fn seed(&self, key: &str, bytes: impl Into<Vec<u8>>, content_type: &str) {
        self.objects.lock().unwrap().insert(
            key.to_string(),
            Object {
                bytes: bytes.into(),
                content_type: content_type.to_string(),
                metadata: HashMap::new(),
            },
        );
    }

    pub(super) fn stored_bytes(&self, key: &str) -> Option<Vec<u8>> {
        self.objects
            .lock()
            .unwrap()
            .get(key)
            .map(|object| object.bytes.clone())
    }

    pub(super) fn stored_metadata(&self, key: &str) -> Option<HashMap<String, String>> {
        self.objects
            .lock()
            .unwrap()
            .get(key)
            .map(|object| object.metadata.clone())
    }

    pub(super) fn plant_metadata(&self, key: &str, metadata: HashMap<String, String>) {
        self.objects.lock().unwrap().get_mut(key).unwrap().metadata = metadata;
    }
}

#[async_trait]
impl BlobAdapter for FakeOwnedAdapter {
    async fn initiate_upload(&self, _request: UploadRequest) -> BlobResult<UploadPlan> {
        unimplemented!("not used by owned promotion tests")
    }
    async fn finalise_upload(&self, _key: &str) -> BlobResult<StoredObject> {
        unimplemented!("not used by owned promotion tests")
    }
    fn public_url(&self, key: &str) -> String {
        key.to_string()
    }
    async fn signed_download_url(&self, _request: DownloadRequest) -> BlobResult<SignedUrl> {
        unimplemented!("not used by owned promotion tests")
    }
    async fn delete(&self, key: &str) -> BlobResult<()> {
        self.objects.lock().unwrap().remove(key);
        Ok(())
    }
    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        self.head_calls.fetch_add(1, Ordering::SeqCst);
        let objects = self.objects.lock().unwrap();
        let object = objects
            .get(key)
            .ok_or_else(|| BlobError::NotFound(key.to_string()))?;
        Ok(ObjectInfo {
            key: key.to_string(),
            size: object.bytes.len() as u64,
            content_type: object.content_type.clone(),
            etag: None,
            last_modified: None,
            metadata: object.metadata.clone(),
        })
    }
    async fn get_bytes(&self, _key: &str) -> BlobResult<Vec<u8>> {
        panic!("owned recovery must not reread object bytes")
    }
    async fn put_bytes(
        &self,
        _key: &str,
        _data: &[u8],
        _content_type: &str,
    ) -> BlobResult<StoredObject> {
        panic!("owned promotion must not call unconditional put_bytes")
    }
    fn name(&self) -> &'static str {
        "fake"
    }
    fn bucket(&self) -> &str {
        "fake-bucket"
    }
    async fn get_bytes_bounded(&self, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
        self.bounded_reads.fetch_add(1, Ordering::SeqCst);
        let bytes = self
            .objects
            .lock()
            .unwrap()
            .get(key)
            .map(|object| object.bytes.clone())
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
        let mut objects = self.objects.lock().unwrap();
        if objects.contains_key(key) {
            return Err(BlobError::DestinationExists(key.to_string()));
        }
        objects.insert(
            key.to_string(),
            Object {
                bytes: data.to_vec(),
                content_type: content_type.to_string(),
                metadata: HashMap::new(),
            },
        );
        Ok(StoredObject::new(
            "fake",
            "fake-bucket",
            key,
            data.len() as u64,
            content_type,
        ))
    }
    async fn put_bytes_create_only_owned(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
        facts: &OwnedPublicationFacts,
    ) -> BlobResult<StoredObject> {
        let mut objects = self.objects.lock().unwrap();
        if objects.contains_key(key) {
            return Err(BlobError::DestinationExists(key.to_string()));
        }
        objects.insert(
            key.to_string(),
            Object {
                bytes: data.to_vec(),
                content_type: content_type.to_string(),
                metadata: facts
                    .metadata_pairs()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            },
        );
        Ok(StoredObject::new(
            "fake",
            "fake-bucket",
            key,
            data.len() as u64,
            content_type,
        ))
    }
}

pub(super) fn token(bytes: &[u8]) -> OwnershipToken {
    OwnershipToken::from_bytes(bytes.to_vec()).unwrap()
}

pub(super) fn keys() -> (BlobObjectKey, BlobObjectKey) {
    (
        BlobObjectKey::parse("staging/a.png").unwrap(),
        BlobObjectKey::parse("media/a.png").unwrap(),
    )
}

pub(super) fn authority() -> OwnedDestinationAuthority {
    OwnedDestinationAuthority::new(
        "fake",
        "fake-bucket",
        BlobObjectKey::parse("media/a.png").unwrap(),
    )
    .unwrap()
}

pub(super) fn assert_no_token_disclosure(err: &BlobError, secret: &[u8]) {
    let rendered = format!("{err:?}{err}");
    let as_utf8 = String::from_utf8_lossy(secret);
    assert!(!rendered.contains(as_utf8.as_ref()));
    assert!(!rendered.contains(&hex::encode(secret)));
}
