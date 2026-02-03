//! Development email capture adapter.
//!
//! This adapter captures all emails to a storage backend (typically a database)
//! for inspection during development. It can optionally forward emails to
//! whitelisted addresses via a fallback adapter.

use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::adapter::EmailAdapter;
use crate::error::EmailResult;
use crate::manager::DevCaptureConfig;
use crate::types::{Email, SendResult};

/// A captured email record for storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedEmail {
    /// Unique identifier.
    pub id: Uuid,
    /// Original email ID.
    pub email_id: Uuid,
    /// Recipient addresses (To field).
    pub to_addresses: Vec<String>,
    /// Sender address.
    pub from_address: String,
    /// Reply-to address (if any).
    pub reply_to: Option<String>,
    /// CC addresses.
    pub cc_addresses: Vec<String>,
    /// BCC addresses.
    pub bcc_addresses: Vec<String>,
    /// Email subject.
    pub subject: String,
    /// Plain text body.
    pub text_body: Option<String>,
    /// HTML body.
    pub html_body: Option<String>,
    /// Custom headers as JSON.
    pub headers_json: Option<String>,
    /// When the email was captured.
    pub captured_at: DateTime<Utc>,
    /// Whether the email was also delivered (via fallback).
    pub was_delivered: bool,
    /// Error message if delivery failed.
    pub delivery_error: Option<String>,
}

impl CapturedEmail {
    /// Create a new captured email from an Email.
    pub fn from_email(email: &Email) -> Self {
        let headers_json = if email.headers.is_empty() {
            None
        } else {
            serde_json::to_string(&email.headers).ok()
        };

        Self {
            id: Uuid::now_v7(),
            email_id: email.id,
            to_addresses: email.to.iter().map(|a| a.email.clone()).collect(),
            from_address: email.from.email.clone(),
            reply_to: email.reply_to.as_ref().map(|a| a.email.clone()),
            cc_addresses: email.cc.iter().map(|a| a.email.clone()).collect(),
            bcc_addresses: email.bcc.iter().map(|a| a.email.clone()).collect(),
            subject: email.subject.clone(),
            text_body: email.text_body.clone(),
            html_body: email.html_body.clone(),
            headers_json,
            captured_at: Utc::now(),
            was_delivered: false,
            delivery_error: None,
        }
    }

    /// Mark this email as delivered.
    pub fn mark_delivered(&mut self) {
        self.was_delivered = true;
    }

    /// Mark this email as failed to deliver.
    pub fn mark_delivery_failed(&mut self, error: impl Into<String>) {
        self.was_delivered = false;
        self.delivery_error = Some(error.into());
    }
}

/// Trait for storing captured emails.
///
/// Implement this trait to provide storage for the development capture adapter.
/// Typically this would be backed by a database, but could be any storage mechanism.
#[async_trait]
pub trait EmailStore: Send + Sync {
    /// Store a captured email.
    async fn store(&self, email: CapturedEmail) -> EmailResult<()>;
}

/// Internal whitelist lookup for efficient address checking.
#[derive(Debug, Clone, Default)]
struct WhitelistLookup {
    addresses: HashSet<String>,
}

impl WhitelistLookup {
    fn from_config(config: &DevCaptureConfig) -> Self {
        Self {
            addresses: config.whitelist.iter().cloned().collect(),
        }
    }

    fn is_whitelisted(&self, email: &str) -> bool {
        self.addresses.contains(email)
    }

    fn any_whitelisted<'a>(&self, addresses: impl IntoIterator<Item = &'a str>) -> bool {
        addresses.into_iter().any(|addr| self.is_whitelisted(addr))
    }
}

/// Development email capture adapter.
///
/// This adapter captures all emails to a storage backend for inspection during
/// development. It can optionally forward emails to whitelisted addresses via
/// a fallback adapter (e.g., SMTP or SES).
///
/// # Example
///
/// ```rust,ignore
/// use std::sync::Arc;
/// use underlay_email::{DevCaptureAdapter, DevCaptureConfig, SmtpAdapter};
///
/// // Create a store (implement EmailStore trait)
/// let store = Arc::new(MyDatabaseStore::new(pool));
///
/// // Create config with whitelist
/// let config = DevCaptureConfig {
///     whitelist: vec!["dev@example.com".to_string()],
///     use_fallback: true,
/// };
///
/// // Create adapter with optional fallback for whitelisted addresses
/// let smtp = Arc::new(SmtpAdapter::new(&smtp_config)?);
/// let adapter = DevCaptureAdapter::new(store, config, Some(smtp));
/// ```
pub struct DevCaptureAdapter<S: EmailStore> {
    store: Arc<S>,
    whitelist: WhitelistLookup,
    use_fallback: bool,
    fallback: Option<Arc<dyn EmailAdapter>>,
}

impl<S: EmailStore> DevCaptureAdapter<S> {
    /// Create a new development capture adapter.
    ///
    /// - `store`: Storage backend for captured emails
    /// - `config`: Configuration including whitelist
    /// - `fallback`: Optional adapter for forwarding whitelisted emails
    pub fn new(
        store: Arc<S>,
        config: DevCaptureConfig,
        fallback: Option<Arc<dyn EmailAdapter>>,
    ) -> Self {
        Self {
            store,
            whitelist: WhitelistLookup::from_config(&config),
            use_fallback: config.use_fallback,
            fallback,
        }
    }

    /// Create a new adapter without a fallback (capture only).
    pub fn capture_only(store: Arc<S>) -> Self {
        Self {
            store,
            whitelist: WhitelistLookup::default(),
            use_fallback: false,
            fallback: None,
        }
    }

    /// Check if any recipients are whitelisted.
    fn has_whitelisted_recipients(&self, email: &Email) -> bool {
        let all_recipients = email
            .to
            .iter()
            .chain(email.cc.iter())
            .chain(email.bcc.iter())
            .map(|a| a.email.as_str());

        self.whitelist.any_whitelisted(all_recipients)
    }
}

#[async_trait]
impl<S: EmailStore + 'static> EmailAdapter for DevCaptureAdapter<S> {
    async fn send(&self, email: &Email) -> EmailResult<SendResult> {
        let mut captured = CapturedEmail::from_email(email);

        // Check if we should forward to fallback
        let should_forward =
            self.use_fallback && self.fallback.is_some() && self.has_whitelisted_recipients(email);

        if should_forward {
            if let Some(fallback) = &self.fallback {
                match fallback.send(email).await {
                    Ok(_) => {
                        captured.mark_delivered();
                    }
                    Err(e) => {
                        captured.mark_delivery_failed(e.to_string());
                    }
                }
            }
        }

        // Always capture to storage
        self.store.store(captured).await?;

        Ok(SendResult::success(
            email.id,
            Some("dev_capture".to_string()),
        ))
    }

    fn name(&self) -> &'static str {
        "dev_capture"
    }

    async fn health_check(&self) -> EmailResult<()> {
        // If we have a fallback, check its health too
        if let Some(fallback) = &self.fallback {
            fallback.health_check().await?;
        }
        Ok(())
    }
}

impl<S: EmailStore> std::fmt::Debug for DevCaptureAdapter<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevCaptureAdapter")
            .field("whitelist", &self.whitelist)
            .field("use_fallback", &self.use_fallback)
            .field("has_fallback", &self.fallback.is_some())
            .finish()
    }
}

/// In-memory store for testing purposes.
#[derive(Debug, Default)]
pub struct InMemoryEmailStore {
    emails: std::sync::Mutex<Vec<CapturedEmail>>,
}

impl InMemoryEmailStore {
    /// Create a new in-memory store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all captured emails.
    pub fn emails(&self) -> Vec<CapturedEmail> {
        self.emails.lock().unwrap().clone()
    }

    /// Clear all captured emails.
    pub fn clear(&self) {
        self.emails.lock().unwrap().clear();
    }

    /// Get the count of captured emails.
    pub fn count(&self) -> usize {
        self.emails.lock().unwrap().len()
    }
}

#[async_trait]
impl EmailStore for InMemoryEmailStore {
    async fn store(&self, email: CapturedEmail) -> EmailResult<()> {
        self.emails.lock().unwrap().push(email);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EmailAddress;

    fn create_test_email(to: &str) -> Email {
        Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new(to).unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .build()
            .unwrap()
    }

    #[test]
    fn test_whitelist_lookup() {
        let config = DevCaptureConfig {
            whitelist: vec![
                "dev@example.com".to_string(),
                "test@example.com".to_string(),
            ],
            use_fallback: true,
        };
        let lookup = WhitelistLookup::from_config(&config);

        assert!(lookup.is_whitelisted("dev@example.com"));
        assert!(lookup.is_whitelisted("test@example.com"));
        assert!(!lookup.is_whitelisted("other@example.com"));
    }

    #[test]
    fn test_whitelist_any_whitelisted() {
        let config = DevCaptureConfig {
            whitelist: vec!["dev@example.com".to_string()],
            use_fallback: true,
        };
        let lookup = WhitelistLookup::from_config(&config);

        assert!(lookup.any_whitelisted(["other@example.com", "dev@example.com"]));
        assert!(!lookup.any_whitelisted(["other@example.com", "another@example.com"]));
    }

    #[test]
    fn test_captured_email_from_email() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .cc(EmailAddress::new("cc@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello")
            .html_body("<p>Hello</p>")
            .build()
            .unwrap();

        let captured = CapturedEmail::from_email(&email);

        assert_eq!(captured.email_id, email.id);
        assert_eq!(captured.from_address, "sender@example.com");
        assert_eq!(captured.to_addresses, vec!["recipient@example.com"]);
        assert_eq!(captured.cc_addresses, vec!["cc@example.com"]);
        assert_eq!(captured.subject, "Test Subject");
        assert_eq!(captured.text_body, Some("Hello".to_string()));
        assert_eq!(captured.html_body, Some("<p>Hello</p>".to_string()));
        assert!(!captured.was_delivered);
        assert!(captured.delivery_error.is_none());
    }

    #[tokio::test]
    async fn test_dev_capture_adapter_capture_only() {
        let store = Arc::new(InMemoryEmailStore::new());
        let adapter = DevCaptureAdapter::capture_only(store.clone());

        let email = create_test_email("recipient@example.com");
        let result = adapter.send(&email).await.unwrap();

        assert!(result.success);
        assert_eq!(result.message_id, Some("dev_capture".to_string()));
        assert_eq!(store.count(), 1);

        let captured = &store.emails()[0];
        assert!(!captured.was_delivered);
    }

    #[tokio::test]
    async fn test_dev_capture_adapter_with_whitelist_no_match() {
        let store = Arc::new(InMemoryEmailStore::new());
        let config = DevCaptureConfig {
            whitelist: vec!["dev@example.com".to_string()],
            use_fallback: true,
        };
        let adapter = DevCaptureAdapter::new(store.clone(), config, None);

        let email = create_test_email("other@example.com");
        let result = adapter.send(&email).await.unwrap();

        assert!(result.success);
        assert_eq!(store.count(), 1);

        let captured = &store.emails()[0];
        assert!(!captured.was_delivered);
    }

    #[tokio::test]
    async fn test_in_memory_store() {
        let store = InMemoryEmailStore::new();
        assert_eq!(store.count(), 0);

        let email = create_test_email("test@example.com");
        let captured = CapturedEmail::from_email(&email);
        store.store(captured).await.unwrap();

        assert_eq!(store.count(), 1);

        store.clear();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_captured_email_mark_delivered() {
        let email = create_test_email("test@example.com");
        let mut captured = CapturedEmail::from_email(&email);

        assert!(!captured.was_delivered);
        captured.mark_delivered();
        assert!(captured.was_delivered);
    }

    #[test]
    fn test_captured_email_mark_delivery_failed() {
        let email = create_test_email("test@example.com");
        let mut captured = CapturedEmail::from_email(&email);

        captured.mark_delivery_failed("Connection refused");
        assert!(!captured.was_delivered);
        assert_eq!(
            captured.delivery_error,
            Some("Connection refused".to_string())
        );
    }
}
