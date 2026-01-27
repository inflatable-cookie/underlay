//! Email-based One-Time Password (TOTP) verification for Underlay.
//!
//! This crate provides a reusable email verification system that can be integrated
//! into any Underlay-based application. It handles:
//!
//! - Generating and sending verification codes via email
//! - Rate limiting code requests
//! - Verifying codes with attempt limiting
//! - Creating and consuming verification sessions
//!
//! # Architecture
//!
//! The crate uses trait-based abstractions for storage and email sending,
//! allowing applications to integrate with their existing infrastructure:
//!
//! - [`EmailTotpCodeRepository`] - Storage for verification codes
//! - [`VerificationSessionRepository`] - Storage for verification sessions
//! - [`EmailTotpSender`] - Email delivery
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_auth_email_totp::{
//!     EmailTotpService, EmailTotpConfig,
//!     EmailTotpCodeRepository, VerificationSessionRepository, EmailTotpSender,
//! };
//!
//! // Implement the repository traits for your storage backend
//! struct MyCodeRepository { /* ... */ }
//! struct MySessionRepository { /* ... */ }
//! struct MyEmailSender { /* ... */ }
//!
//! // Create the service
//! let service = EmailTotpService::new(
//!     MyCodeRepository::new(),
//!     MySessionRepository::new(),
//!     MyEmailSender::new(),
//!     EmailTotpConfig::default(),
//! );
//!
//! // Request a verification code
//! let expiry = service.request_code("user-123", "user@example.com", "password_change").await?;
//!
//! // User receives code via email...
//!
//! // Verify the code
//! let session = service.verify_code("user-123", "password_change", "123456").await?;
//!
//! // Consume the session when performing the sensitive action
//! let consumed = service.consume_session(&session.id, "user-123", "password_change").await?;
//! ```
//!
//! # Purposes
//!
//! The `purpose` parameter identifies why verification is being performed.
//! Common purposes include:
//!
//! - `"login"` - Email fallback for login when TOTP is unavailable
//! - `"password_change"` - Verifying identity before changing password
//! - `"password_reset"` - Forgot password flow
//! - `"sensitive_action"` - Generic verification for sensitive operations
//!
//! Applications can define their own purposes as needed.

mod code;
mod config;
mod error;
mod repository;
mod service;

pub use crate::code::generate_code;
pub use crate::config::EmailTotpConfig;
pub use crate::error::{EmailTotpError, EmailTotpResult};
pub use crate::repository::{
    EmailTotpCodeRepository, EmailTotpRepository, RateLimitStatus, StoredCode,
    VerificationSession, VerificationSessionRepository,
};
pub use crate::service::{EmailTotpSender, EmailTotpService, VERIFICATION_METHOD_EMAIL_TOTP};

/// Common purpose values for email TOTP verification.
pub mod purposes {
    /// Login verification (email fallback).
    pub const LOGIN: &str = "login";
    /// Password change verification.
    pub const PASSWORD_CHANGE: &str = "password_change";
    /// Password reset (forgot password).
    pub const PASSWORD_RESET: &str = "password_reset";
    /// Generic sensitive action verification.
    pub const SENSITIVE_ACTION: &str = "sensitive_action";
}
