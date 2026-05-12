//! AWS S3-compatible blob storage adapter.

use std::collections::HashMap;
use std::env;
use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::Client;
use chrono::{DateTime, Utc};
use tracing::warn;

use crate::adapter::BlobAdapter;
use crate::error::{BlobError, BlobResult};
use crate::types::{
    DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

/// Configuration for the S3 adapter.
#[derive(Debug, Clone)]
pub struct S3Config {
    /// The S3 bucket name.
    pub bucket: String,

    /// The AWS region.
    pub region: String,

    /// Optional custom endpoint URL (for S3-compatible services like MinIO, R2).
    pub endpoint_url: Option<String>,

    /// Optional base URL for public access (e.g., CDN URL).
    /// If not set, the standard S3 URL format is used.
    pub public_url_base: Option<String>,

    /// Optional base URL used when returning pre-signed browser URLs.
    ///
    /// This is useful when the AWS SDK should talk to an internal endpoint
    /// such as `http://minio:9000`, but browsers must receive an externally
    /// reachable HTTPS route such as `https://s3.acme.test`.
    pub presign_url_base: Option<String>,

    /// Default expiration for upload URLs.
    pub upload_url_expiry: Duration,

    /// Default expiration for download URLs.
    pub download_url_expiry: Duration,

    /// Whether to use path-style URLs (required for some S3-compatible services).
    pub path_style: bool,
}

impl S3Config {
    /// Create a new S3 configuration with default settings.
    pub fn new(bucket: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            region: region.into(),
            endpoint_url: None,
            public_url_base: None,
            presign_url_base: None,
            upload_url_expiry: Duration::from_secs(3600), // 1 hour
            download_url_expiry: Duration::from_secs(300), // 5 minutes
            path_style: false,
        }
    }

    /// Set a custom endpoint URL (for S3-compatible services).
    pub fn endpoint_url(mut self, url: impl Into<String>) -> Self {
        self.endpoint_url = Some(url.into());
        self
    }

    /// Set a custom public URL base (for CDN).
    pub fn public_url_base(mut self, url: impl Into<String>) -> Self {
        self.public_url_base = Some(url.into());
        self
    }

    /// Set a custom base URL for browser-facing pre-signed URLs.
    pub fn presign_url_base(mut self, url: impl Into<String>) -> Self {
        self.presign_url_base = Some(url.into());
        self
    }

    /// Set the upload URL expiry duration.
    pub fn upload_url_expiry(mut self, duration: Duration) -> Self {
        self.upload_url_expiry = duration;
        self
    }

    /// Set the download URL expiry duration.
    pub fn download_url_expiry(mut self, duration: Duration) -> Self {
        self.download_url_expiry = duration;
        self
    }

    /// Enable path-style URLs.
    pub fn path_style(mut self, enabled: bool) -> Self {
        self.path_style = enabled;
        self
    }

    /// Create a development-friendly MinIO config.
    ///
    /// This assumes:
    /// - an S3-compatible endpoint such as MinIO
    /// - path-style URLs
    /// - a public base at `{endpoint}/{bucket}`
    /// - the conventional local-dev region `us-east-1`
    pub fn minio_dev(bucket: impl Into<String>, endpoint_url: impl Into<String>) -> Self {
        let bucket = bucket.into();
        let endpoint_url = endpoint_url.into().trim_end_matches('/').to_string();
        let public_url_base = format!("{}/{}", endpoint_url, bucket);

        Self::new(bucket, "us-east-1")
            .endpoint_url(endpoint_url)
            .public_url_base(public_url_base)
            .path_style(true)
    }

    /// Build an S3 config from shared blob env vars, falling back to a MinIO
    /// development shape when env overrides are absent.
    ///
    /// Recognized env vars:
    /// - `BLOB_S3_BUCKET`
    /// - `BLOB_S3_REGION`
    /// - `BLOB_S3_ENDPOINT_URL`
    /// - `BLOB_S3_PUBLIC_URL_BASE`
    /// - `BLOB_S3_PRESIGN_URL_BASE`
    /// - `BLOB_S3_PATH_STYLE`
    pub fn from_env_or_minio_dev(
        default_bucket: impl Into<String>,
        default_endpoint_url: impl Into<String>,
    ) -> Self {
        let default_bucket = default_bucket.into();
        let default_endpoint_url = default_endpoint_url.into();

        let bucket = env::var("BLOB_S3_BUCKET").unwrap_or(default_bucket);
        let endpoint_url = env::var("BLOB_S3_ENDPOINT_URL").unwrap_or(default_endpoint_url);
        let region = env::var("BLOB_S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());
        let path_style = env::var("BLOB_S3_PATH_STYLE")
            .ok()
            .map(|value| matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(true);

        let endpoint_url = endpoint_url.trim_end_matches('/').to_string();
        let public_url_base = env::var("BLOB_S3_PUBLIC_URL_BASE")
            .unwrap_or_else(|_| format!("{}/{}", endpoint_url, bucket));
        let presign_url_base = env::var("BLOB_S3_PRESIGN_URL_BASE")
            .ok()
            .map(|value| value.trim_end_matches('/').to_string())
            .filter(|value| !value.is_empty());

        let mut config = Self::new(bucket, region)
            .endpoint_url(endpoint_url)
            .public_url_base(public_url_base)
            .path_style(path_style);

        if let Some(base) = presign_url_base {
            config = config.presign_url_base(base);
        }

        config
    }
}

/// AWS S3-compatible blob storage adapter.
pub struct S3Adapter {
    client: Client,
    presign_client: Client,
    config: S3Config,
}

impl S3Adapter {
    fn sdk_error_details<E, R>(
        err: &aws_smithy_runtime_api::client::result::SdkError<E, R>,
    ) -> String
    where
        E: ProvideErrorMetadata + std::fmt::Debug + std::fmt::Display,
        R: std::fmt::Debug,
    {
        let code = err
            .as_service_error()
            .map(ProvideErrorMetadata::code)
            .flatten()
            .unwrap_or("<none>");
        let message = err
            .as_service_error()
            .map(ProvideErrorMetadata::message)
            .flatten()
            .unwrap_or("<none>");

        format!("code={code}, message={message}, debug={err:?}")
    }

    fn build_client(
        aws_config: &underlay_aws::SdkConfig,
        config: &S3Config,
        endpoint: Option<&str>,
    ) -> Client {
        let mut s3_config_builder = aws_sdk_s3::config::Builder::from(aws_config);

        if let Some(endpoint) = endpoint {
            s3_config_builder = s3_config_builder.endpoint_url(endpoint);
        }

        if config.path_style {
            s3_config_builder = s3_config_builder.force_path_style(true);
        }

        Client::from_conf(s3_config_builder.build())
    }

    /// Create a new S3 adapter with the given configuration.
    ///
    /// This uses the default AWS credential chain (environment variables,
    /// IAM role, shared credentials file, etc.).
    pub async fn new(config: S3Config) -> BlobResult<Self> {
        let mut aws = underlay_aws::AwsConfig::new(&config.region);

        if let Some(endpoint) = &config.endpoint_url {
            aws = aws.with_endpoint(endpoint);
        }

        let aws_config = aws.load().await;

        let client = Self::build_client(&aws_config, &config, config.endpoint_url.as_deref());
        let presign_client = Self::build_client(
            &aws_config,
            &config,
            config
                .presign_url_base
                .as_deref()
                .or(config.endpoint_url.as_deref()),
        );

        Ok(Self {
            client,
            presign_client,
            config,
        })
    }

    /// Create a new S3 adapter from an existing AWS SDK config.
    pub fn from_aws_config(aws_config: &underlay_aws::SdkConfig, config: S3Config) -> Self {
        let client = Self::build_client(aws_config, &config, config.endpoint_url.as_deref());
        let presign_client = Self::build_client(
            aws_config,
            &config,
            config
                .presign_url_base
                .as_deref()
                .or(config.endpoint_url.as_deref()),
        );

        Self {
            client,
            presign_client,
            config,
        }
    }

    /// Generate the standard S3 public URL for an object.
    fn default_public_url(&self, key: &str) -> String {
        if self.config.path_style {
            if let Some(endpoint) = &self.config.endpoint_url {
                format!("{}/{}/{}", endpoint, self.config.bucket, key)
            } else {
                format!(
                    "https://s3.{}.amazonaws.com/{}/{}",
                    self.config.region, self.config.bucket, key
                )
            }
        } else {
            format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                self.config.bucket, self.config.region, key
            )
        }
    }

    /// Convert S3 SDK errors to BlobError.
    fn map_s3_error(err: aws_sdk_s3::Error, key: &str) -> BlobError {
        let err_str = err.to_string();

        if err_str.contains("NoSuchKey") || err_str.contains("NotFound") {
            BlobError::NotFound(key.to_string())
        } else if err_str.contains("NoSuchBucket") {
            BlobError::BucketNotFound(err_str)
        } else if err_str.contains("AccessDenied") || err_str.contains("Forbidden") {
            BlobError::AccessDenied(err_str)
        } else if err_str.contains("credentials") || err_str.contains("InvalidAccessKeyId") {
            BlobError::AuthError(err_str)
        } else {
            BlobError::TransportError(err_str)
        }
    }

    async fn ensure_bucket_exists(&self) -> BlobResult<()> {
        match self
            .client
            .head_bucket()
            .bucket(&self.config.bucket)
            .send()
            .await
        {
            Ok(_) => return Ok(()),
            Err(err) => {
                let err_str = Self::sdk_error_details(&err);
                let missing_bucket = err_str.contains("NoSuchBucket")
                    || err_str.contains("NotFound")
                    || err_str.contains("404");
                warn!(
                    bucket = %self.config.bucket,
                    endpoint_url = self.config.endpoint_url.as_deref().unwrap_or("<none>"),
                    missing_bucket,
                    error = %err_str,
                    "S3 head_bucket failed during ensure_bucket_exists"
                );
                if !missing_bucket {
                    return Err(BlobError::BucketNotFound(format!(
                        "head_bucket failed for '{}': {}",
                        self.config.bucket, err_str
                    )));
                }
            }
        }

        let mut request = self.client.create_bucket().bucket(&self.config.bucket);
        if self.config.region != "us-east-1" {
            request = request.create_bucket_configuration(
                CreateBucketConfiguration::builder()
                    .location_constraint(BucketLocationConstraint::from(
                        self.config.region.as_str(),
                    ))
                    .build(),
            );
        }

        match request.send().await {
            Ok(_) => Ok(()),
            Err(err) => {
                let err_str = Self::sdk_error_details(&err);
                warn!(
                    bucket = %self.config.bucket,
                    endpoint_url = self.config.endpoint_url.as_deref().unwrap_or("<none>"),
                    error = %err_str,
                    "S3 create_bucket failed during ensure_bucket_exists"
                );
                if err_str.contains("BucketAlreadyOwnedByYou")
                    || err_str.contains("BucketAlreadyExists")
                {
                    Ok(())
                } else {
                    Err(BlobError::BucketNotFound(format!(
                        "create_bucket failed for '{}': {}",
                        self.config.bucket, err_str
                    )))
                }
            }
        }
    }
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
            .bucket(&self.config.bucket)
            .key(&request.key)
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
            object_key: request.key,
        })
    }

    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject> {
        // Verify the object exists and get its metadata
        let info = self.head(key).await?;

        Ok(StoredObject {
            provider: "s3".to_string(),
            bucket: self.config.bucket.clone(),
            key: key.to_string(),
            size: info.size,
            content_type: info.content_type,
            etag: info.etag,
        })
    }

    fn public_url(&self, key: &str) -> String {
        if let Some(base) = &self.config.public_url_base {
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
            .bucket(&self.config.bucket)
            .key(&request.key);

        // Set content disposition if filename provided
        if let Some(filename) = &request.filename {
            get_request = get_request
                .response_content_disposition(format!("attachment; filename=\"{}\"", filename));
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
            .bucket(&self.config.bucket)
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
            .bucket(&self.config.bucket)
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
            .bucket(&self.config.bucket)
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
            .bucket(&self.config.bucket)
            .key(key)
            .content_type(content_type)
            .content_length(data.len() as i64)
            .body(body)
            .send()
            .await
            .map_err(|e| BlobError::UploadFailed(format!("Failed to upload object: {}", e)))?;

        Ok(StoredObject {
            provider: "s3".to_string(),
            bucket: self.config.bucket.clone(),
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
        &self.config.bucket
    }

    async fn health_check(&self) -> BlobResult<()> {
        // Try to list objects with max 1 to verify connectivity and permissions
        self.client
            .list_objects_v2()
            .bucket(&self.config.bucket)
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
            .field("bucket", &self.config.bucket)
            .field("region", &self.config.region)
            .field("endpoint_url", &self.config.endpoint_url)
            .finish()
    }
}

#[cfg(test)]
#[path = "../tests/adapters/s3_tests.rs"]
mod tests;
