use super::*;

#[test]
fn test_rendition_config_defaults() {
    let config = RenditionConfig::default();
    assert_eq!(config.thumbnail_max_dimension(), 400);
    assert_eq!(config.preview_max_dimension(), 1200);
    assert_eq!(config.jpeg_quality(), 85);
    assert!(!config.square_thumbnails());
    assert!(config.generate_thumbnails());
    assert!(!config.generate_previews());
    assert_eq!(config.thumbnail_name_ref(), "thumb");
    assert_eq!(config.preview_name_ref(), "preview");
}

#[test]
fn test_rendition_config_builder() {
    let config = RenditionConfig::with_previews()
        .thumbnail_size(256)
        .preview_size(1024)
        .quality(90)
        .square()
        .thumbnail_name("thumb_256")
        .preview_name("large");

    assert_eq!(config.thumbnail_max_dimension(), 256);
    assert_eq!(config.preview_max_dimension(), 1024);
    assert_eq!(config.jpeg_quality(), 90);
    assert!(config.square_thumbnails());
    assert!(config.generate_thumbnails());
    assert!(config.generate_previews());
    assert_eq!(config.thumbnail_name_ref(), "thumb_256");
    assert_eq!(config.preview_name_ref(), "large");
}

#[test]
fn test_farmyard_compat_config() {
    let config = RenditionConfig::farmyard_compat();
    assert_eq!(config.thumbnail_max_dimension(), 128);
    assert_eq!(config.jpeg_quality(), 80);
    assert!(config.square_thumbnails());
    assert!(config.generate_thumbnails());
    assert!(!config.generate_previews());
    assert_eq!(config.thumbnail_name_ref(), "thumb_128");
}

#[test]
fn test_quality_clamping() {
    let config = RenditionConfig::default().quality(150);
    assert_eq!(config.jpeg_quality(), 100);

    let config = RenditionConfig::default().quality(0);
    assert_eq!(config.jpeg_quality(), 1);
}

mod service_execution_tests {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use chrono::Utc;
    use image::codecs::png::{CompressionType, FilterType as PngFilterType, PngEncoder};
    use image::ImageEncoder;
    use underlay_blob::{
        BlobAdapter, BlobError, BlobResult, DownloadRequest, ObjectInfo, SignedUrl, StoredObject,
        UploadPlan, UploadRequest,
    };
    use uuid::Uuid;

    use super::*;
    use crate::domain::{
        CreateMediaInput, CreateRenditionInput, FinalizeUploadInput, ListMediaParams, Media,
        MediaId, MediaRendition, MediaRenditionId, MediaSummary, MediaVersion, MediaVersionId,
        UpdateMediaInput,
    };
    use crate::error::MediaResult;
    use crate::image::DEFAULT_MAX_IMAGE_HEIGHT;
    use crate::repository::MediaRepository;

    #[derive(Default)]
    struct CountingBlobAdapter {
        objects: Mutex<HashMap<String, Vec<u8>>>,
        bounded_reads: AtomicUsize,
    }

    impl CountingBlobAdapter {
        fn insert(&self, key: &str, bytes: Vec<u8>) {
            self.objects.lock().unwrap().insert(key.to_string(), bytes);
        }

        fn bounded_read_count(&self) -> usize {
            self.bounded_reads.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl BlobAdapter for CountingBlobAdapter {
        async fn initiate_upload(&self, _request: UploadRequest) -> BlobResult<UploadPlan> {
            Err(BlobError::Unsupported("not used in test".to_string()))
        }

        async fn finalise_upload(&self, _key: &str) -> BlobResult<StoredObject> {
            Err(BlobError::Unsupported("not used in test".to_string()))
        }

        fn public_url(&self, key: &str) -> String {
            format!("https://cdn.example.test/{key}")
        }

        async fn signed_download_url(&self, _request: DownloadRequest) -> BlobResult<SignedUrl> {
            Err(BlobError::Unsupported("not used in test".to_string()))
        }

        async fn delete(&self, key: &str) -> BlobResult<()> {
            self.objects.lock().unwrap().remove(key);
            Ok(())
        }

        async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
            let objects = self.objects.lock().unwrap();
            let bytes = objects
                .get(key)
                .ok_or_else(|| BlobError::NotFound(key.to_string()))?;
            Ok(ObjectInfo {
                key: key.to_string(),
                size: bytes.len() as u64,
                content_type: "image/jpeg".to_string(),
                etag: None,
                last_modified: None,
                metadata: HashMap::new(),
            })
        }

        async fn get_bytes(&self, key: &str) -> BlobResult<Vec<u8>> {
            self.objects
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .ok_or_else(|| BlobError::NotFound(key.to_string()))
        }

        async fn get_bytes_bounded(&self, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
            self.bounded_reads.fetch_add(1, Ordering::SeqCst);
            let bytes = self.get_bytes(key).await?;
            if bytes.len() as u64 > max_bytes {
                return Err(BlobError::TooLarge(bytes.len() as u64, max_bytes));
            }
            Ok(bytes)
        }

        async fn put_bytes(
            &self,
            key: &str,
            data: &[u8],
            content_type: &str,
        ) -> BlobResult<StoredObject> {
            self.objects
                .lock()
                .unwrap()
                .insert(key.to_string(), data.to_vec());
            Ok(StoredObject {
                provider: "memory".to_string(),
                bucket: "test-bucket".to_string(),
                key: key.to_string(),
                size: data.len() as u64,
                content_type: content_type.to_string(),
                etag: None,
            })
        }

        fn name(&self) -> &'static str {
            "memory"
        }

        fn bucket(&self) -> &str {
            "test-bucket"
        }

        async fn health_check(&self) -> BlobResult<()> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingMediaRepo {
        renditions: Mutex<Vec<MediaRendition>>,
    }

    #[async_trait]
    impl MediaRepository for RecordingMediaRepo {
        async fn create_media(
            &self,
            _input: CreateMediaInput,
            _created_by: Option<Uuid>,
        ) -> MediaResult<Media> {
            unreachable!("not used")
        }
        async fn get_media(&self, _id: MediaId) -> MediaResult<Option<Media>> {
            unreachable!("not used")
        }
        async fn update_media(
            &self,
            _id: MediaId,
            _input: UpdateMediaInput,
            _updated_by: Option<Uuid>,
        ) -> MediaResult<Media> {
            unreachable!("not used")
        }
        async fn soft_delete_media(
            &self,
            _id: MediaId,
            _deleted_by: Option<Uuid>,
        ) -> MediaResult<bool> {
            unreachable!("not used")
        }
        async fn restore_media(&self, _id: MediaId) -> MediaResult<bool> {
            unreachable!("not used")
        }
        async fn hard_delete_media(&self, _id: MediaId) -> MediaResult<bool> {
            unreachable!("not used")
        }
        async fn list_media(&self, _params: ListMediaParams) -> MediaResult<Vec<MediaSummary>> {
            unreachable!("not used")
        }
        async fn list_trash(&self) -> MediaResult<Vec<MediaSummary>> {
            unreachable!("not used")
        }
        async fn create_version(
            &self,
            _media_id: MediaId,
            _created_by: Option<Uuid>,
        ) -> MediaResult<MediaVersion> {
            unreachable!("not used")
        }
        async fn get_version(&self, _id: MediaVersionId) -> MediaResult<Option<MediaVersion>> {
            unreachable!("not used")
        }
        async fn finalize_version(
            &self,
            _id: MediaVersionId,
            _input: FinalizeUploadInput,
        ) -> MediaResult<MediaVersion> {
            unreachable!("not used")
        }
        async fn fail_version(&self, _id: MediaVersionId) -> MediaResult<bool> {
            unreachable!("not used")
        }
        async fn delete_version(&self, _id: MediaVersionId) -> MediaResult<bool> {
            unreachable!("not used")
        }
        async fn list_versions(&self, _media_id: MediaId) -> MediaResult<Vec<MediaVersion>> {
            unreachable!("not used")
        }
        async fn find_by_hash(&self, _sha256: &str) -> MediaResult<Option<Media>> {
            unreachable!("not used")
        }
        async fn set_current_version(
            &self,
            _media_id: MediaId,
            _version_id: MediaVersionId,
        ) -> MediaResult<()> {
            unreachable!("not used")
        }
        async fn create_rendition(
            &self,
            version_id: MediaVersionId,
            input: CreateRenditionInput,
        ) -> MediaResult<MediaRendition> {
            let rendition = MediaRendition {
                id: MediaRenditionId(Uuid::now_v7()),
                version_id,
                rendition_type: input.rendition_type,
                object_key: input.object_key,
                mime_type: input.mime_type,
                byte_size: input.byte_size,
                width: input.width,
                height: input.height,
                storage_provider: input.storage_provider,
                bucket: input.bucket,
                created_at: Utc::now(),
            };
            self.renditions.lock().unwrap().push(rendition.clone());
            Ok(rendition)
        }
        async fn get_rendition(
            &self,
            _id: MediaRenditionId,
        ) -> MediaResult<Option<MediaRendition>> {
            unreachable!("not used")
        }
        async fn list_renditions(
            &self,
            _version_id: MediaVersionId,
        ) -> MediaResult<Vec<MediaRendition>> {
            Ok(self.renditions.lock().unwrap().clone())
        }
        async fn delete_renditions(&self, _version_id: MediaVersionId) -> MediaResult<u64> {
            unreachable!("not used")
        }
        async fn list_unused_media(&self) -> MediaResult<Vec<Media>> {
            unreachable!("not used")
        }
    }

    fn encode_rgb_png(width: u32, height: u32) -> Vec<u8> {
        let mut png = Vec::new();
        let pixels = vec![64u8; (width as usize) * (height as usize) * 3];
        PngEncoder::new_with_quality(&mut png, CompressionType::Fast, PngFilterType::NoFilter)
            .write_image(&pixels, width, height, image::ExtendedColorType::Rgb8)
            .expect("encode test png");
        png
    }

    #[tokio::test]
    async fn rendition_service_honors_square_thumbnails_and_fetches_source_once() {
        let blob = Arc::new(CountingBlobAdapter::default());
        blob.insert("media/source.png", encode_rgb_png(200, 100));

        let repo = RecordingMediaRepo::default();
        let service = RenditionService::new(
            Arc::clone(&blob),
            RenditionConfig::with_previews()
                .thumbnail_size(64)
                .preview_size(120)
                .square(),
        );

        let renditions = service
            .generate_renditions_for_version(
                &repo,
                MediaId(Uuid::now_v7()),
                MediaVersionId(Uuid::now_v7()),
                "media/source.png",
            )
            .await
            .expect("generate renditions");

        assert_eq!(renditions.len(), 2);
        assert_eq!(
            blob.bounded_read_count(),
            1,
            "source bytes must be fetched once when both thumbnail and preview are enabled"
        );

        // Thumbnail must be square (64x64) because square_thumbnails is enabled.
        assert_eq!(renditions[0].width, Some(64));
        assert_eq!(renditions[0].height, Some(64));

        // Preview preserves aspect ratio (120x60).
        assert_eq!(renditions[1].width, Some(120));
        assert_eq!(renditions[1].height, Some(60));
    }

    #[tokio::test]
    async fn rendition_service_handles_extreme_1x16384_square_thumbnail() {
        let blob = Arc::new(CountingBlobAdapter::default());
        blob.insert(
            "media/tall.png",
            encode_rgb_png(1, DEFAULT_MAX_IMAGE_HEIGHT),
        );

        let service = RenditionService::new(
            Arc::clone(&blob),
            RenditionConfig::thumbnails_only()
                .thumbnail_size(64)
                .square(),
        );

        let result = service
            .generate_thumbnail("media/tall.png", "media/tall-thumb.jpg")
            .await
            .expect("1x16384 square thumbnail via RenditionService");

        assert_eq!(result.width, 64);
        assert_eq!(result.height, 64);
        assert_eq!(blob.bounded_read_count(), 1);
    }
}
