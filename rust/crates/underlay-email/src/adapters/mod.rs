//! Email adapter implementations.
//!
//! This module contains implementations of the `EmailAdapter` trait
//! for various email backends.

#[cfg(feature = "smtp")]
mod smtp;

#[cfg(feature = "ses")]
mod ses;

#[cfg(feature = "smtp")]
pub use smtp::SmtpAdapter;

#[cfg(feature = "ses")]
pub use ses::SesAdapter;

// Future adapters:
// - Development capture adapter (Phase 1.4)
