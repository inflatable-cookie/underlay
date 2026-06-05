use underlay_blob::{BlobAdapter, BlobAdapterObjectKeyExt, BlobObjectKey};

use super::super::keys::parse_rendition_result_key;
use super::RenditionService;
use crate::domain::MediaVersionId;
use crate::error::{MediaError, MediaResult};
use crate::repository::MediaRepository;

impl<B: BlobAdapter> RenditionService<B> {
    /// Delete all rendition blobs for a version.
    ///
    /// This fetches renditions from the repository and deletes their blobs
    /// from storage. It returns the number of blobs deleted.
    pub async fn delete_version_renditions<R: MediaRepository>(
        &self,
        repo: &R,
        version_id: MediaVersionId,
    ) -> MediaResult<u64> {
        let renditions = repo.list_renditions(version_id).await?;
        let mut deleted = 0u64;

        for rendition in renditions {
            if let Err(e) = self
                .blob_adapter
                .delete_object_key(&rendition.object_key)
                .await
            {
                tracing::warn!(
                    version_id = %version_id,
                    rendition_id = %rendition.id,
                    object_key = %rendition.object_key,
                    error = %e,
                    "Failed to delete rendition blob"
                );
            } else {
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Delete a single rendition blob.
    pub async fn delete_rendition_blob(&self, object_key: &str) -> MediaResult<()> {
        let object_key = parse_rendition_result_key(object_key)?;
        self.delete_rendition_blob_object_key(&object_key).await
    }

    /// Delete a single typed rendition blob.
    pub async fn delete_rendition_blob_object_key(
        &self,
        object_key: &BlobObjectKey,
    ) -> MediaResult<()> {
        self.blob_adapter
            .delete_object_key(object_key)
            .await
            .map_err(|e| MediaError::storage(format!("Failed to delete blob: {}", e)))
    }
}
