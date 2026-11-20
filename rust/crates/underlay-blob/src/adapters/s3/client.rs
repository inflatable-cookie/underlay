use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::Client;
use aws_smithy_runtime_api::client::{orchestrator::HttpResponse, result::SdkError};
use tracing::warn;

use crate::error::{BlobError, BlobResult};

use super::{S3Adapter, S3Config};

impl S3Adapter {
    /// Ensure the configured bucket exists and has any adapter-managed policy.
    /// Successful setup is cached for this adapter's lifetime.
    pub async fn ensure_bucket_ready(&self) -> BlobResult<()> {
        self.ensure_bucket_exists().await
    }

    pub(super) fn sdk_error_details<E, R>(
        err: &aws_smithy_runtime_api::client::result::SdkError<E, R>,
    ) -> String
    where
        E: ProvideErrorMetadata + std::fmt::Debug + std::fmt::Display,
        R: std::fmt::Debug,
    {
        let code = err
            .as_service_error()
            .and_then(ProvideErrorMetadata::code)
            .unwrap_or("<none>");
        let message = err
            .as_service_error()
            .and_then(ProvideErrorMetadata::message)
            .unwrap_or("<none>");

        format!("code={code}, message={message}, debug={err:?}")
    }

    /// Map a non-collision S3 `SdkError` to a redacted, stable public
    /// error.
    ///
    /// Full provider detail (code, message, and the SDK's `Debug` output —
    /// which can include request/response text a scanner would flag as
    /// credential-shaped, e.g. a `SignatureDoesNotMatch` canonical-request
    /// echo) is logged via `tracing` for operator diagnosis, but never
    /// returned to the caller. Only an HTTP status code, when available,
    /// and a fixed operation label cross the public boundary.
    /// `NoSuchKey`/`NotFound` still map to the typed `BlobError::NotFound`;
    /// that is a stable classification, not raw provider text.
    pub(super) fn redacted_transport_error<E>(
        err: &SdkError<E, HttpResponse>,
        key: &str,
        operation: &'static str,
    ) -> BlobError
    where
        E: ProvideErrorMetadata + std::fmt::Debug + std::fmt::Display,
    {
        let code = err.as_service_error().and_then(ProvideErrorMetadata::code);
        let message = Self::redacted_error_message(err, operation);
        if matches!(code, Some("NoSuchKey" | "NotFound")) {
            BlobError::NotFound(key.to_string())
        } else {
            BlobError::TransportError(message)
        }
    }

    pub(super) fn redacted_error_message<E>(
        err: &SdkError<E, HttpResponse>,
        operation: &'static str,
    ) -> String
    where
        E: ProvideErrorMetadata + std::fmt::Debug + std::fmt::Display,
    {
        let status = err.raw_response().map(|r| r.status().as_u16());
        let code = err.as_service_error().and_then(ProvideErrorMetadata::code);
        warn!(
            operation,
            status = ?status,
            code = code.unwrap_or("<none>"),
            detail = %Self::sdk_error_details(err),
            "S3 request failed"
        );

        match status {
            Some(status) => format!("s3 {operation} failed (status {status})"),
            None => format!("s3 {operation} failed (no response)"),
        }
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

        if config.path_style_enabled() {
            s3_config_builder = s3_config_builder.force_path_style(true);
        }

        Client::from_conf(s3_config_builder.build())
    }

    /// Create a new S3 adapter with the given configuration.
    ///
    /// This uses the default AWS credential chain (environment variables,
    /// IAM role, shared credentials file, etc.).
    pub async fn new(config: S3Config) -> BlobResult<Self> {
        let mut aws = underlay_aws::AwsConfig::new(config.region());

        if let Some(endpoint) = config.endpoint_url_ref() {
            aws = aws.with_endpoint(endpoint);
        }

        let aws_config = aws.load().await;

        let client = Self::build_client(&aws_config, &config, config.endpoint_url_ref());
        let presign_client = Self::build_client(
            &aws_config,
            &config,
            config.presign_url_base_ref().or(config.endpoint_url_ref()),
        );

        Ok(Self {
            client,
            presign_client,
            config,
            bucket_ready: Default::default(),
        })
    }

    /// Create a new S3 adapter from an existing AWS SDK config.
    pub fn from_aws_config(aws_config: &underlay_aws::SdkConfig, config: S3Config) -> Self {
        let client = Self::build_client(aws_config, &config, config.endpoint_url_ref());
        let presign_client = Self::build_client(
            aws_config,
            &config,
            config.presign_url_base_ref().or(config.endpoint_url_ref()),
        );

        Self {
            client,
            presign_client,
            config,
            bucket_ready: Default::default(),
        }
    }

    /// Generate the standard S3 public URL for an object.
    pub(super) fn default_public_url(&self, key: &str) -> String {
        let base = if self.config.path_style_enabled() {
            if let Some(endpoint) = self.config.endpoint_url_ref() {
                format!(
                    "{}/{}",
                    endpoint.trim_end_matches('/'),
                    self.config.bucket()
                )
            } else {
                format!(
                    "https://s3.{}.amazonaws.com/{}",
                    self.config.region(),
                    self.config.bucket()
                )
            }
        } else {
            format!(
                "https://{}.s3.{}.amazonaws.com",
                self.config.bucket(),
                self.config.region()
            )
        };
        crate::adapter::join_public_url(&base, key)
    }

    /// Classify ordinary S3 requests using structured provider errors.
    pub(super) fn map_s3_error<E>(
        err: &SdkError<E, HttpResponse>,
        key: &str,
        operation: &'static str,
    ) -> BlobError
    where
        E: ProvideErrorMetadata + std::fmt::Debug + std::fmt::Display,
    {
        let code = err.as_service_error().and_then(ProvideErrorMetadata::code);
        let status = err
            .raw_response()
            .map(|response| response.status().as_u16());
        let message = Self::redacted_error_message(err, operation);

        match (code, status) {
            (Some("NoSuchKey" | "NotFound"), _) => BlobError::NotFound(key.to_string()),
            (Some("NoSuchBucket"), _) => BlobError::BucketNotFound(message),
            (Some("InvalidAccessKeyId" | "SignatureDoesNotMatch" | "ExpiredToken"), _)
            | (_, Some(401)) => BlobError::AuthError(message),
            (Some("AccessDenied" | "Forbidden"), _) | (_, Some(403)) => {
                BlobError::AccessDenied(message)
            }
            (_, Some(404)) if !key.is_empty() => BlobError::NotFound(key.to_string()),
            _ => BlobError::TransportError(message),
        }
    }

    pub(super) async fn ensure_bucket_exists(&self) -> BlobResult<()> {
        self.bucket_ready
            .get_or_try_init(|| self.initialise_bucket())
            .await?;
        Ok(())
    }

    async fn initialise_bucket(&self) -> BlobResult<()> {
        match self
            .client
            .head_bucket()
            .bucket(self.config.bucket())
            .send()
            .await
        {
            Ok(_) => return self.ensure_bucket_policy().await,
            Err(err) => {
                let code = err.as_service_error().and_then(ProvideErrorMetadata::code);
                let missing_bucket = matches!(code, Some("NoSuchBucket" | "NotFound"))
                    || err
                        .raw_response()
                        .is_some_and(|response| response.status().as_u16() == 404);
                if !missing_bucket {
                    return Err(BlobError::BucketNotFound(Self::redacted_error_message(
                        &err,
                        "head_bucket",
                    )));
                }
            }
        }

        let mut request = self.client.create_bucket().bucket(self.config.bucket());
        if self.config.region() != "us-east-1" {
            request = request.create_bucket_configuration(
                CreateBucketConfiguration::builder()
                    .location_constraint(BucketLocationConstraint::from(self.config.region()))
                    .build(),
            );
        }

        match request.send().await {
            Ok(_) => self.ensure_bucket_policy().await,
            Err(err) => {
                let code = err.as_service_error().and_then(ProvideErrorMetadata::code);
                if matches!(
                    code,
                    Some("BucketAlreadyOwnedByYou" | "BucketAlreadyExists")
                ) {
                    self.ensure_bucket_policy().await
                } else {
                    Err(BlobError::BucketNotFound(Self::redacted_error_message(
                        &err,
                        "create_bucket",
                    )))
                }
            }
        }
    }

    async fn ensure_bucket_policy(&self) -> BlobResult<()> {
        if !self.config.public_read_enabled() {
            return Ok(());
        }

        let policy = format!(
            r#"{{"Version":"2012-10-17","Statement":[{{"Effect":"Allow","Principal":{{"AWS":["*"]}},"Action":["s3:GetObject"],"Resource":["arn:aws:s3:::{bucket}/*"]}}]}}"#,
            bucket = self.config.bucket()
        );

        self.client
            .put_bucket_policy()
            .bucket(self.config.bucket())
            .policy(policy)
            .send()
            .await
            .map_err(|e| {
                BlobError::TransportError(Self::redacted_error_message(&e, "put_bucket_policy"))
            })?;

        Ok(())
    }
}
