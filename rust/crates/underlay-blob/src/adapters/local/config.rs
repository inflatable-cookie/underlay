use std::path::PathBuf;

/// Configuration for the local filesystem adapter.
#[derive(Debug, Clone)]
pub struct LocalConfig {
    /// Base directory where files are stored.
    base_path: PathBuf,

    /// Base URL for serving files (e.g., "http://localhost:8080/uploads").
    /// This should point to a development-only endpoint that serves files from base_path.
    serve_url_base: String,

    /// Virtual bucket name (for compatibility with the adapter interface).
    bucket: String,

    /// Base URL for the upload endpoint (e.g., "http://localhost:8080/uploads").
    /// If not set, uses serve_url_base.
    upload_url_base: Option<String>,
}

impl LocalConfig {
    /// Create a new local configuration.
    pub fn new(base_path: impl Into<PathBuf>, serve_url_base: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            serve_url_base: serve_url_base.into(),
            bucket: "local".to_string(),
            upload_url_base: None,
        }
    }

    /// Set the virtual bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = bucket.into();
        self
    }

    /// Set a custom upload URL base.
    pub fn upload_url_base(mut self, url: impl Into<String>) -> Self {
        self.upload_url_base = Some(url.into());
        self
    }

    /// Return the base directory where files are stored.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Return the base URL used for serving files.
    pub fn serve_url_base(&self) -> &str {
        &self.serve_url_base
    }

    /// Return the virtual bucket name.
    pub fn bucket_name(&self) -> &str {
        &self.bucket
    }

    /// Return the optional upload URL base.
    pub fn upload_url_base_ref(&self) -> Option<&str> {
        self.upload_url_base.as_deref()
    }
}
