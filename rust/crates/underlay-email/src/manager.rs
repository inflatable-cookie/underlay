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
    default_from: EmailAddress,
    /// Adapter type to use.
    adapter_type: AdapterType,
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
    host: String,
    /// SMTP server port.
    port: u16,
    /// Username for authentication (optional).
    username: Option<String>,
    /// Password for authentication (optional).
    password: Option<String>,
    /// TLS mode.
    tls_mode: TlsMode,
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
    region: String,
    /// Optional configuration set name.
    configuration_set: Option<String>,
}

/// Development capture adapter configuration.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DevCaptureConfig {
    /// Email addresses that should also be sent via a real adapter.
    whitelist: Vec<String>,
    /// Whether to use a fallback adapter for whitelisted addresses.
    use_fallback: bool,
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

impl EmailManagerConfig {
    /// Create email manager configuration with an explicit adapter type.
    pub fn new(default_from: EmailAddress, adapter_type: AdapterType) -> Self {
        Self {
            default_from,
            adapter_type,
        }
    }

    /// Create email manager configuration using the no-op adapter.
    pub fn noop(default_from: EmailAddress) -> Self {
        Self::new(default_from, AdapterType::Noop)
    }

    /// Return the default from address.
    pub fn default_from(&self) -> &EmailAddress {
        &self.default_from
    }

    /// Return the configured adapter type.
    pub fn adapter_type(&self) -> &AdapterType {
        &self.adapter_type
    }
}

impl SmtpConfig {
    /// Create SMTP configuration for a host with default port and TLS settings.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            ..Default::default()
        }
    }

    /// Set the SMTP server port.
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set SMTP credentials.
    pub fn with_credentials(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set the SMTP TLS mode.
    pub fn with_tls_mode(mut self, tls_mode: TlsMode) -> Self {
        self.tls_mode = tls_mode;
        self
    }

    /// Return the SMTP host.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Return the SMTP port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Return the configured SMTP username.
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    /// Return the configured SMTP password.
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Return the SMTP TLS mode.
    pub fn tls_mode(&self) -> TlsMode {
        self.tls_mode
    }
}

impl SesConfig {
    /// Create SES configuration for an AWS region.
    pub fn new(region: impl Into<String>) -> Self {
        Self {
            region: region.into(),
            configuration_set: None,
        }
    }

    /// Set the optional SES configuration set name.
    pub fn with_configuration_set(mut self, configuration_set: impl Into<String>) -> Self {
        self.configuration_set = Some(configuration_set.into());
        self
    }

    /// Return the AWS region.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Return the optional SES configuration set name.
    pub fn configuration_set(&self) -> Option<&str> {
        self.configuration_set.as_deref()
    }
}

impl DevCaptureConfig {
    /// Create development capture configuration with no whitelist and no fallback.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the whitelist.
    pub fn with_whitelist<I, S>(mut self, whitelist: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.whitelist = whitelist.into_iter().map(Into::into).collect();
        self
    }

    /// Add one address to the whitelist.
    pub fn with_whitelisted_address(mut self, address: impl Into<String>) -> Self {
        self.whitelist.push(address.into());
        self
    }

    /// Enable fallback delivery for whitelisted addresses.
    pub fn with_fallback(mut self) -> Self {
        self.use_fallback = true;
        self
    }

    /// Return the whitelist.
    pub fn whitelist(&self) -> &[String] {
        &self.whitelist
    }

    /// Return whether fallback delivery is enabled.
    pub fn use_fallback(&self) -> bool {
        self.use_fallback
    }
}

#[cfg(test)]
#[path = "tests/manager_tests.rs"]
mod tests;
