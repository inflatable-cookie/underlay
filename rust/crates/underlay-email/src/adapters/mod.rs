//! Email adapter implementations.
//!
//! This module contains implementations of the `EmailAdapter` trait
//! for various email backends.

#[cfg(feature = "smtp")]
mod smtp;

#[cfg(feature = "smtp")]
pub use smtp::SmtpAdapter;

// Future adapters:
// - AWS SES adapter (Phase 1.3)
// - Development capture adapter (Phase 1.4)
