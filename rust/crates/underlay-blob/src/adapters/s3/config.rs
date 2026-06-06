use std::time::Duration;

/// Configuration for the S3 adapter.
#[derive(Debug, Clone)]
pub struct S3Config {
    /// The S3 bucket name.
    bucket: String,

    /// The AWS region.
    region: String,

    /// Optional custom endpoint URL (for S3-compatible services like MinIO, R2).
    endpoint_url: Option<String>,

    /// Optional base URL for public access (e.g., CDN URL).
    /// If not set, the standard S3 URL format is used.
    public_url_base: Option<String>,

    /// Optional base URL used when returning pre-signed browser URLs.
    ///
    /// This is useful when the AWS SDK should talk to an internal endpoint
    /// such as `http://minio:9000`, but browsers must receive an externally
    /// reachable HTTPS route such as `https://s3.acme.test`.
    presign_url_base: Option<String>,

    /// Default expiration for upload URLs.
    upload_url_expiry: Duration,

    /// Default expiration for download URLs.
    download_url_expiry: Duration,

    /// Whether to use path-style URLs (required for some S3-compatible services).
    path_style: bool,

    /// Whether the bucket should allow anonymous object reads.
    ///
    /// This is intended for local/dev buckets that are exposed through a
    /// public URL base. Production buckets should normally manage policy
    /// outside the adapter.
    public_read: bool,
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
            public_read: false,
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

    /// Enable or disable anonymous object reads for the configured bucket.
    pub fn public_read(mut self, enabled: bool) -> Self {
        self.public_read = enabled;
        self
    }

    /// Return the configured bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Return the configured AWS region.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return the optional S3-compatible endpoint URL.
    pub fn endpoint_url_ref(&self) -> Option<&str> {
        self.endpoint_url.as_deref()
    }

    /// Return the optional public URL base.
    pub fn public_url_base_ref(&self) -> Option<&str> {
        self.public_url_base.as_deref()
    }

    /// Return the optional browser-facing pre-sign URL base.
    pub fn presign_url_base_ref(&self) -> Option<&str> {
        self.presign_url_base.as_deref()
    }

    /// Return the default upload URL expiry duration.
    pub fn upload_url_expiry_duration(&self) -> Duration {
        self.upload_url_expiry
    }

    /// Return the default download URL expiry duration.
    pub fn download_url_expiry_duration(&self) -> Duration {
        self.download_url_expiry
    }

    /// Return whether path-style URLs are enabled.
    pub fn path_style_enabled(&self) -> bool {
        self.path_style
    }

    /// Return whether adapter-managed public reads are enabled.
    pub fn public_read_enabled(&self) -> bool {
        self.public_read
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
            .public_read(true)
            .path_style(true)
    }
}
