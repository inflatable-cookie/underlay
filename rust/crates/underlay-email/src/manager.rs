use std::sync::Arc;

use crate::adapter::EmailAdapter;
use crate::error::EmailResult;
use crate::types::{Email, EmailAddress, EmailBuilder, SendResult};

/// Manager for sending emails through a configured adapter.
///
/// The `EmailManager` provides a high-level interface for sending emails,
/// handling common tasks like setting default from addresses and logging.
pub struct EmailManager {
    adapter: Arc<dyn EmailAdapter>,
    default_from: EmailAddress,
}

impl EmailManager {
    /// Create a new email manager with the given adapter and default from address.
    pub fn new(adapter: Arc<dyn EmailAdapter>, default_from: EmailAddress) -> Self {
        Self {
            adapter,
            default_from,
        }
    }

    /// Get the name of the underlying adapter.
    pub fn adapter_name(&self) -> &'static str {
        self.adapter.name()
    }

    /// Get the default from address.
    pub fn default_from(&self) -> &EmailAddress {
        &self.default_from
    }

    /// Send an email.
    pub async fn send(&self, email: &Email) -> EmailResult<SendResult> {
        self.adapter.send(email).await
    }

    /// Send an email using the builder pattern with the default from address.
    pub async fn send_email(
        &self,
        to: EmailAddress,
        subject: impl Into<String>,
        text_body: impl Into<String>,
        html_body: Option<String>,
    ) -> EmailResult<SendResult> {
        let mut builder = Email::builder()
            .from(self.default_from.clone())
            .to(to)
            .subject(subject)
            .text_body(text_body);

        if let Some(html) = html_body {
            builder = builder.html_body(html);
        }

        let email = builder.build()?;
        self.send(&email).await
    }

    /// Create an email builder pre-configured with the default from address.
    pub fn email_builder(&self) -> EmailBuilder {
        Email::builder().from(self.default_from.clone())
    }

    /// Check the health of the email adapter.
    pub async fn health_check(&self) -> EmailResult<()> {
        self.adapter.health_check().await
    }
}

/// Configuration for the email manager.
#[derive(Debug, Clone)]
pub struct EmailManagerConfig {
    /// Default from address for emails.
    pub default_from: EmailAddress,
    /// Adapter type to use.
    pub adapter_type: AdapterType,
}

/// Available email adapter types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterType {
    /// No-op adapter (does nothing, for testing).
    Noop,
    /// SMTP adapter using lettre.
    Smtp(SmtpConfig),
    /// AWS SES adapter.
    Ses(SesConfig),
    /// Development capture adapter (saves to database).
    DevCapture(DevCaptureConfig),
}

/// SMTP adapter configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmtpConfig {
    /// SMTP server hostname.
    pub host: String,
    /// SMTP server port.
    pub port: u16,
    /// Username for authentication (optional).
    pub username: Option<String>,
    /// Password for authentication (optional).
    pub password: Option<String>,
    /// TLS mode.
    pub tls_mode: TlsMode,
}

/// TLS mode for SMTP connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TlsMode {
    /// Require TLS (STARTTLS or implicit TLS).
    #[default]
    Required,
    /// Use TLS if available, but allow unencrypted.
    Opportunistic,
    /// Never use TLS (not recommended for production).
    None,
}

/// AWS SES adapter configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SesConfig {
    /// AWS region for SES.
    pub region: String,
    /// Optional configuration set name.
    pub configuration_set: Option<String>,
}

/// Development capture adapter configuration.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DevCaptureConfig {
    /// Email addresses that should also be sent via a real adapter.
    pub whitelist: Vec<String>,
    /// Whether to use a fallback adapter for whitelisted addresses.
    pub use_fallback: bool,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 587,
            username: None,
            password: None,
            tls_mode: TlsMode::Required,
        }
    }
}

impl Default for SesConfig {
    fn default() -> Self {
        Self {
            region: "us-east-1".to_string(),
            configuration_set: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::NoopAdapter;

    #[tokio::test]
    async fn test_email_manager_send() {
        let adapter = Arc::new(NoopAdapter::new());
        let from = EmailAddress::new("noreply@example.com").unwrap();
        let manager = EmailManager::new(adapter, from);

        let to = EmailAddress::new("user@example.com").unwrap();
        let result = manager
            .send_email(to, "Test Subject", "Hello, World!", None)
            .await
            .unwrap();

        assert!(result.success);
    }

    #[tokio::test]
    async fn test_email_manager_builder() {
        let adapter = Arc::new(NoopAdapter::new());
        let from = EmailAddress::with_name("noreply@example.com", "My App").unwrap();
        let manager = EmailManager::new(adapter, from.clone());

        let email = manager
            .email_builder()
            .to(EmailAddress::new("user@example.com").unwrap())
            .subject("Test")
            .text_body("Hello")
            .build()
            .unwrap();

        assert_eq!(email.from.email, "noreply@example.com");
        assert_eq!(email.from.name, Some("My App".to_string()));
    }

    #[test]
    fn test_config_defaults() {
        let smtp = SmtpConfig::default();
        assert_eq!(smtp.host, "localhost");
        assert_eq!(smtp.port, 587);
        assert_eq!(smtp.tls_mode, TlsMode::Required);

        let ses = SesConfig::default();
        assert_eq!(ses.region, "us-east-1");

        let dev = DevCaptureConfig::default();
        assert!(dev.whitelist.is_empty());
        assert!(!dev.use_fallback);
    }
}
