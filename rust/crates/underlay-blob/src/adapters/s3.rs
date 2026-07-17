//! AWS S3-compatible blob storage adapter.

mod client;
mod config;

use std::collections::HashMap;

use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use chrono::{DateTime, Utc};

use crate::adapter::BlobAdapter;
use crate::error::{BlobError, BlobResult};
use crate::types::{
    DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

pub use config::S3Config;

/// AWS S3-compatible blob storage adapter.
pub struct S3Adapter {
    client: Client,
    presign_client: Client,
    config: S3Config,
}

#[async_trait]
impl BlobAdapter for S3Adapter {
    async fn initiate_upload(&self, request: UploadRequest) -> BlobResult<UploadPlan> {
        self.ensure_bucket_exists().await?;

        let expires_in = request.expires_in;
        let presigning_config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| BlobError::PresignError(e.to_string()))?;

        let mut put_request = self
            .presign_client
            .put_object()
            .bucket(self.config.bucket())
            .key(request.key.as_str())
            .content_type(&request.content_type)
            .content_length(request.content_length as i64);

        // Add metadata
        for (k, v) in &request.metadata {
            put_request = put_request.metadata(k, v);
        }

        let presigned = put_request
            .presigned(presigning_config)
            .await
            .map_err(|e| BlobError::PresignError(e.to_string()))?;

        let expires_at = Utc::now()
            + chrono::Duration::from_std(expires_in).unwrap_or_else(|_| chrono::Duration::hours(1));

        // Build required headers from the presigned request
        let mut required_headers = HashMap::new();
        required_headers.insert("Content-Type".to_string(), request.content_type.clone());

        Ok(UploadPlan {
            upload_url: presigned.uri().to_string(),
            method: "PUT".to_string(),
            required_headers,
            max_bytes: request.content_length,
            allowed_content_types: vec![request.content_type],
            expires_at,
            object_key: request.key.into_string(),
        })
    }

    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject> {
        // Verify the object exists and get its metadata
        let info = self.head(key).await?;

        Ok(StoredObject {
            provider: "s3".to_string(),
            bucket: self.config.bucket().to_string(),
            key: key.to_string(),
            size: info.size,
            content_type: info.content_type,
            etag: info.etag,
        })
    }

    fn public_url(&self, key: &str) -> String {
        if let Some(base) = self.config.public_url_base_ref() {
            format!("{}/{}", base.trim_end_matches('/'), key)
        } else {
            self.default_public_url(key)
        }
    }

    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl> {
        let expires_in = request.expires_in;
        let presigning_config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| BlobError::PresignError(e.to_string()))?;

        let mut get_request = self
            .presign_client
            .get_object()
            .bucket(self.config.bucket())
            .key(request.key.as_str());

        // Set content disposition if filename provided. The filename is
        // untrusted input; escape per RFC 6266 to prevent header injection.
        if let Some(filename) = &request.filename {
            get_request = get_request
                .response_content_disposition(crate::types::content_disposition_attachment(
                    filename,
                ));
        }

        let presigned = get_request
            .presigned(presigning_config)
            .await
            .map_err(|e| BlobError::PresignError(e.to_string()))?;

        let expires_at = Utc::now()
            + chrono::Duration::from_std(expires_in)
                .unwrap_or_else(|_| chrono::Duration::minutes(5));

        Ok(SignedUrl {
            url: presigned.uri().to_string(),
            expires_at,
        })
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        self.client
            .delete_object()
            .bucket(self.config.bucket())
            .key(key)
            .send()
            .await
            .map_err(|e| Self::map_s3_error(e.into(), key))?;

        Ok(())
    }

    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        let response = self
            .client
            .head_object()
            .bucket(self.config.bucket())
            .key(key)
            .send()
            .await
            .map_err(|e| Self::map_s3_error(e.into(), key))?;

        let last_modified = response.last_modified().map(|t| {
            DateTime::<Utc>::from_timestamp(t.secs(), t.subsec_nanos()).unwrap_or_else(Utc::now)
        });

        let metadata = response
            .metadata()
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();

        Ok(ObjectInfo {
            key: key.to_string(),
            size: response.content_length().unwrap_or(0) as u64,
            content_type: response
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string(),
            etag: response.e_tag().map(|s| s.trim_matches('"').to_string()),
            last_modified,
            metadata,
        })
    }

    async fn get_bytes(&self, key: &str) -> BlobResult<Vec<u8>> {
        let response = self
            .client
            .get_object()
            .bucket(self.config.bucket())
            .key(key)
            .send()
            .await
            .map_err(|e| Self::map_s3_error(e.into(), key))?;

        let bytes = response
            .body
            .collect()
            .await
            .map_err(|e| BlobError::DownloadFailed(format!("Failed to read object body: {}", e)))?
            .into_bytes()
            .to_vec();

        Ok(bytes)
    }

    async fn put_bytes(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        self.ensure_bucket_exists().await?;

        let body = aws_sdk_s3::primitives::ByteStream::from(data.to_vec());

        let response = self
            .client
            .put_object()
            .bucket(self.config.bucket())
            .key(key)
            .content_type(content_type)
            .content_length(data.len() as i64)
            .body(body)
            .send()
            .await
            .map_err(|e| BlobError::UploadFailed(format!("Failed to upload object: {}", e)))?;

        Ok(StoredObject {
            provider: "s3".to_string(),
            bucket: self.config.bucket().to_string(),
            key: key.to_string(),
            size: data.len() as u64,
            content_type: content_type.to_string(),
            etag: response.e_tag().map(|s| s.trim_matches('"').to_string()),
        })
    }

    fn name(&self) -> &'static str {
        "s3"
    }

    fn bucket(&self) -> &str {
        self.config.bucket()
    }

    async fn health_check(&self) -> BlobResult<()> {
        // Try to list objects with max 1 to verify connectivity and permissions
        self.client
            .list_objects_v2()
            .bucket(self.config.bucket())
            .max_keys(1)
            .send()
            .await
            .map_err(|e| Self::map_s3_error(e.into(), ""))?;

        Ok(())
    }
}

impl std::fmt::Debug for S3Adapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S3Adapter")
            .field("bucket", &self.config.bucket())
            .field("region", &self.config.region())
            .field("endpoint_url", &self.config.endpoint_url_ref())
            .finish()
    }
}

#[cfg(test)]
#[path = "../tests/adapters/s3_tests.rs"]
mod tests;
