//! Email infrastructure for Underlay applications.
//!
//! The crate exposes an adapter-based email manager with no-op, SMTP, and SES
//! backends. All backends implement `EmailAdapter`; use `AdapterType` and the
//! adapter-specific config types to select one.

mod adapter;
pub mod adapters;
mod error;
mod manager;
#[cfg(feature = "templates")]
mod templates;
mod types;

pub use adapter::{EmailAdapter, NoopAdapter};
pub use error::{EmailError, EmailResult};
pub use manager::{
    AdapterType, EmailManager, EmailManagerConfig, SesConfig, SmtpConfig, TlsMode,
};
pub use types::{Email, EmailAddress, EmailBuilder, SendResult};

#[cfg(feature = "smtp")]
pub use adapters::SmtpAdapter;

#[cfg(feature = "ses")]
pub use adapters::SesAdapter;

#[cfg(feature = "templates")]
pub use templates::{EmailContext, EmailTemplateEngine};

#[cfg(feature = "templates")]
pub use tera;
