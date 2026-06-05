//! Media library domain types.
//!
//! Core types for media items, versions, renditions, and usage tracking.
//! These types are database-agnostic and can be used across different
//! storage implementations.

mod entities;
mod identifiers;
mod inputs;
mod kinds;
mod migrated_attachments;
mod rendition_types;
mod usage_edges;

pub use entities::*;
pub use identifiers::*;
pub use inputs::*;
pub use kinds::*;
pub use migrated_attachments::*;
pub use rendition_types::*;
pub use usage_edges::*;

#[cfg(test)]
#[path = "tests/domain_tests.rs"]
mod tests;
