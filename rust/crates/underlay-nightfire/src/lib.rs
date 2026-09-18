//! Compatibility facade for the standalone [`nightfire`] crate.
//!
//! New code should depend on `nightfire` directly. This crate preserves the
//! historical `underlay_nightfire` import path while consumers migrate.

pub use nightfire::*;
