//! Email infrastructure for Underlay applications.
//!
//! This crate provides a flexible, adapter-based email system that supports
//! multiple backends (SMTP, AWS SES, etc.) with a development capture mode
//! for local testing.
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use underlay_email::{EmailManager, EmailAddress, NoopAdapter};
//!
//! // Create an email manager with a no-op adapter (for testing)
//! let adapter = Arc::new(NoopAdapter::new());
//! let from = EmailAddress::new("noreply@example.com").unwrap();
//! let manager = EmailManager::new(adapter, from);
//!
//! // Send an email
//! let to = EmailAddress::new("user@example.com").unwrap();
//! let result = manager.send_email(
//!     to,
//!     "Welcome!",
//!     "Thanks for signing up.",
//!     Some("<h1>Thanks for signing up.</h1>".to_string()),
//! ).await?;
//! ```
//!
//! # Adapter Pattern
//!
//! The crate uses the adapter pattern to support different email backends:
//!
//! - `NoopAdapter` - Does nothing (for testing)
//! - `SmtpAdapter` - Sends via SMTP (using lettre)
//! - `SesAdapter` - Sends via AWS SES
//! - `DevCaptureAdapter` - Captures to database (for development)
//!
//! All adapters implement the `EmailAdapter` trait.
//!
//! # Configuration
//!
//! Use `AdapterType` to specify which adapter to use, along with its
//! specific configuration (e.g., `SmtpConfig`, `SesConfig`).

mod adapter;
pub mod adapters;
mod error;
mod manager;
#[cfg(feature = "templates")]
mod templates;
mod types;

// Re-export main types
pub use adapter::{EmailAdapter, NoopAdapter};
pub use adapters::{CapturedEmail, DevCaptureAdapter, EmailStore, InMemoryEmailStore};
pub use error::{EmailError, EmailResult};
pub use manager::{
    AdapterType, DevCaptureConfig, EmailManager, EmailManagerConfig, SesConfig, SmtpConfig,
    TlsMode,
};
pub use types::{Email, EmailAddress, EmailBuilder, SendResult};

// Re-export adapters
#[cfg(feature = "smtp")]
pub use adapters::SmtpAdapter;

#[cfg(feature = "ses")]
pub use adapters::SesAdapter;

#[cfg(feature = "templates")]
pub use templates::{EmailContext, EmailTemplateEngine};
