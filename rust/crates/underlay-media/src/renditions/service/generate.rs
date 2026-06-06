use underlay_blob::{BlobAdapter, BlobObjectKey};

use super::super::keys::parse_rendition_result_key;
use super::super::processing::{generate_and_store, read_generate_and_store};
use super::super::result::RenditionResult;
use super::RenditionService;
use crate::domain::RenditionType;
use crate::error::MediaResult;

impl<B: BlobAdapter> RenditionService<B> {
    /// Generate a thumbnail from a source image.
    ///
    /// Reads the source image from blob storage, generates a thumbnail,
    /// and writes it to the target location.
    pub async fn generate_thumbnail(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> MediaResult<RenditionResult> {
        let source_key = parse_rendition_result_key(source_key)?;
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_thumbnail_object_key(&source_key, &target_key)
            .await
    }

    /// Generate a thumbnail from typed source and target object keys.
    pub async fn generate_thumbnail_object_key(
        &self,
        source_key: &BlobObjectKey,
        target_key: &BlobObjectKey,
    ) -> MediaResult<RenditionResult> {
        read_generate_and_store(
            self.blob_adapter.as_ref(),
            source_key,
            target_key,
            self.config.thumbnail_max_dimension(),
            self.config.jpeg_quality(),
            "thumbnail",
        )
        .await
    }

    /// Generate a preview from a source image.
    ///
    /// Previews are larger than thumbnails, suitable for detail views.
    pub async fn generate_preview(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> MediaResult<RenditionResult> {
        let source_key = parse_rendition_result_key(source_key)?;
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_preview_object_key(&source_key, &target_key)
            .await
    }

    /// Generate a preview from typed source and target object keys.
    pub async fn generate_preview_object_key(
        &self,
        source_key: &BlobObjectKey,
        target_key: &BlobObjectKey,
    ) -> MediaResult<RenditionResult> {
        read_generate_and_store(
            self.blob_adapter.as_ref(),
            source_key,
            target_key,
            self.config.preview_max_dimension(),
            self.config.jpeg_quality(),
            "preview",
        )
        .await
    }

    /// Generate a rendition from raw bytes (without reading from storage).
    ///
    /// This is useful when you already have the source bytes in memory.
    pub async fn generate_from_bytes(
        &self,
        source_bytes: &[u8],
        target_key: &str,
        rendition_type: &RenditionType,
    ) -> MediaResult<RenditionResult> {
        let target_key = parse_rendition_result_key(target_key)?;
        self.generate_from_bytes_object_key(source_bytes, &target_key, rendition_type)
            .await
    }

    /// Generate a rendition from raw bytes into a typed target object key.
    pub async fn generate_from_bytes_object_key(
        &self,
        source_bytes: &[u8],
        target_key: &BlobObjectKey,
        rendition_type: &RenditionType,
    ) -> MediaResult<RenditionResult> {
        let max_dimension = match rendition_type {
            RenditionType::Thumbnail => self.config.thumbnail_max_dimension(),
            RenditionType::Preview => self.config.preview_max_dimension(),
            RenditionType::Custom(_) => self.config.thumbnail_max_dimension(),
        };

        generate_and_store(
            self.blob_adapter.as_ref(),
            source_bytes,
            target_key,
            max_dimension,
            self.config.jpeg_quality(),
            "rendition",
        )
        .await
    }
}
