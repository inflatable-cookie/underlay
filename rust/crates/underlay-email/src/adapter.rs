use async_trait::async_trait;

use crate::error::EmailResult;
use crate::types::{Email, SendResult};

/// Trait for email sending backends.
///
/// Implementations provide different ways to send emails:
/// - SMTP (via lettre)
/// - AWS SES
/// - Development capture (saves to database)
/// - etc.
#[async_trait]
pub trait EmailAdapter: Send + Sync {
    /// Send an email.
    ///
    /// Returns a `SendResult` indicating success or failure.
    /// The implementation should not panic on failure; instead,
    /// it should return an error via `EmailResult`.
    async fn send(&self, email: &Email) -> EmailResult<SendResult>;

    /// Get the name of this adapter (for logging/debugging).
    fn name(&self) -> &'static str;

    /// Check if the adapter is healthy/connected.
    ///
    /// Default implementation returns true. Adapters that maintain
    /// connections should override this to perform actual health checks.
    async fn health_check(&self) -> EmailResult<()> {
        Ok(())
    }
}

/// A no-op adapter that does nothing (useful for testing).
#[derive(Debug, Default)]
pub struct NoopAdapter;

impl NoopAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EmailAdapter for NoopAdapter {
    async fn send(&self, email: &Email) -> EmailResult<SendResult> {
        Ok(SendResult::success(email.id, Some("noop".to_string())))
    }

    fn name(&self) -> &'static str {
        "noop"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EmailAddress;

    #[tokio::test]
    async fn test_noop_adapter() {
        let adapter = NoopAdapter::new();
        assert_eq!(adapter.name(), "noop");

        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test")
            .text_body("Hello")
            .build()
            .unwrap();

        let result = adapter.send(&email).await.unwrap();
        assert!(result.success);
        assert_eq!(result.message_id, Some("noop".to_string()));
    }

    #[tokio::test]
    async fn test_noop_adapter_health_check() {
        let adapter = NoopAdapter::new();
        assert!(adapter.health_check().await.is_ok());
    }
}
