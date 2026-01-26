//! Email adapter implementations.
//!
//! This module contains implementations of the `EmailAdapter` trait
//! for various email backends.

mod dev_capture;

#[cfg(feature = "smtp")]
mod smtp;

#[cfg(feature = "ses")]
mod ses;

pub use dev_capture::{CapturedEmail, DevCaptureAdapter, EmailStore, InMemoryEmailStore};

#[cfg(feature = "smtp")]
pub use smtp::SmtpAdapter;

#[cfg(feature = "ses")]
pub use ses::SesAdapter;
