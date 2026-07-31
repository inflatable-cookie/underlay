//! Canonical session rotation state machine for Underlay apps.
//!
//! One implementation of refresh-token rotation with the security rules
//! built in and unskippable:
//!
//! - RFC 6819 reuse detection with session-family revocation
//! - atomic compare-and-swap rotation (concurrent refresh can't double-mint)
//! - absolute session lifetime cap
//! - account status re-check on every refresh (suspended/deleted accounts
//!   cannot keep a session alive)
//! - roles re-issued from the account provider on every rotation
//! - optional client fingerprint (IP/User-Agent) advisory or strict mode
//!
//! Consumers implement [`SessionRepository`] over their own schema and
//! [`AccountProvider`] over their user/role lookup; everything else lives
//! here, once.

mod config;
mod repository;
mod service;
mod types;

pub use config::SessionServiceConfig;
pub use repository::{AccountProvider, AccountState, AccountStatus, SessionRepository};
pub use service::{RefreshOutcome, SessionService};
pub use types::{SessionFingerprint, SessionRecord, Tokens};

#[cfg(test)]
mod tests;
