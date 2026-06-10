//! Shared AWS SDK configuration for Underlay crates.
//!
//! Provides a unified way to build AWS SDK configurations with consistent
//! credential chain, region, and endpoint handling. Used by `underlay-blob`
//! (S3) and `underlay-email` (SES) to share AWS setup.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_aws::AwsConfig;
//!
//! let config = AwsConfig::new("eu-west-2")
//!     .with_endpoint("http://localhost:4566");
//!
//! let sdk_config = config.load().await;
//!
//! // Pass to any AWS service client
//! let s3_client = aws_sdk_s3::Client::new(&sdk_config);
//! let ses_client = aws_sdk_sesv2::Client::new(&sdk_config);
//! ```

pub use aws_config::SdkConfig;
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
pub use aws_types::region::Region;

/// Configuration for building an AWS SDK config.
///
/// Wraps `aws-config` to provide a consistent interface across all Underlay
/// crates that use AWS services.
#[derive(Debug, Clone)]
pub struct AwsConfig {
    /// AWS region (e.g., "eu-west-2", "us-east-1").
    region: String,

    /// Optional custom endpoint URL.
    ///
    /// Used for S3-compatible services (MinIO, R2, LocalStack) or local
    /// development with tools like localstack.
    endpoint_url: Option<String>,

    /// Optional static credentials used instead of the ambient provider chain.
    static_credentials: Option<AwsStaticCredentials>,
}

#[derive(Debug, Clone)]
pub struct AwsStaticCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
}

impl AwsConfig {
    /// Create a new AWS config for the given region.
    pub fn new(region: impl Into<String>) -> Self {
        Self {
            region: region.into(),
            endpoint_url: None,
            static_credentials: None,
        }
    }

    /// Set a custom endpoint URL (for S3-compatible services or local dev).
    pub fn with_endpoint(mut self, url: impl Into<String>) -> Self {
        self.endpoint_url = Some(url.into());
        self
    }

    /// Set explicit static credentials.
    pub fn with_static_credentials(
        mut self,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> Self {
        self.static_credentials = Some(AwsStaticCredentials {
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
            session_token: None,
        });
        self
    }

    /// Return the configured AWS region.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return the optional custom endpoint URL.
    pub fn endpoint_url(&self) -> Option<&str> {
        self.endpoint_url.as_deref()
    }

    /// Return the optional static credentials.
    pub fn static_credentials(&self) -> Option<&AwsStaticCredentials> {
        self.static_credentials.as_ref()
    }

    /// Load the AWS SDK configuration.
    ///
    /// Uses the default credential chain (environment variables, IAM role,
    /// shared credentials file, etc.).
    pub async fn load(&self) -> SdkConfig {
        let mut builder = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(Region::new(self.region.clone()));

        if let Some(endpoint) = &self.endpoint_url {
            builder = builder.endpoint_url(endpoint);
        }

        if let Some(credentials) = &self.static_credentials {
            let credentials = Credentials::new(
                credentials.access_key_id.clone(),
                credentials.secret_access_key.clone(),
                credentials.session_token.clone(),
                None,
                "underlay-static-config",
            );
            builder = builder.credentials_provider(SharedCredentialsProvider::new(credentials));
        }

        builder.load().await
    }
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
