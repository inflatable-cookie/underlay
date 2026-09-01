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
#[derive(Clone)]
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

#[derive(Clone)]
pub struct AwsStaticCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
}

impl std::fmt::Debug for AwsStaticCredentials {
    /// Redacts the secret access key and session token. The access key id is a
    /// public identifier and stays visible so credential sources remain
    /// diagnosable.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsStaticCredentials")
            .field("access_key_id", &self.access_key_id)
            .field("secret_access_key", &"[REDACTED]")
            .field(
                "session_token",
                &self.session_token.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl std::fmt::Debug for AwsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsConfig")
            .field("region", &self.region)
            .field("endpoint_url", &self.endpoint_url)
            .field("static_credentials", &self.static_credentials)
            .finish()
    }
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

#[cfg(test)]
mod debug_redaction_tests {
    use super::{AwsConfig, AwsStaticCredentials};

    fn credentials() -> AwsStaticCredentials {
        AwsStaticCredentials {
            access_key_id: "AKIAEXAMPLEID".to_string(),
            secret_access_key: "super-secret-access-key".to_string(),
            session_token: Some("super-secret-session-token".to_string()),
        }
    }

    #[test]
    fn debug_redacts_secret_access_key_and_session_token() {
        let rendered = format!("{:?}", credentials());

        assert!(!rendered.contains("super-secret-access-key"));
        assert!(!rendered.contains("super-secret-session-token"));
        assert!(rendered.contains("AKIAEXAMPLEID"));
        assert_eq!(rendered.matches("[REDACTED]").count(), 2);
    }

    #[test]
    fn debug_omits_a_session_token_that_is_not_set() {
        let mut credentials = credentials();
        credentials.session_token = None;

        let rendered = format!("{credentials:?}");

        assert!(rendered.contains("session_token: None"));
        assert_eq!(rendered.matches("[REDACTED]").count(), 1);
    }

    #[test]
    fn config_debug_does_not_leak_embedded_credentials() {
        let config = AwsConfig::new("eu-west-2")
            .with_endpoint("http://localhost:4566")
            .with_static_credentials("AKIAEXAMPLEID", "super-secret-access-key");

        let rendered = format!("{config:?}");

        assert!(!rendered.contains("super-secret-access-key"));
        assert!(rendered.contains("eu-west-2"));
        assert!(rendered.contains("http://localhost:4566"));
    }
}
