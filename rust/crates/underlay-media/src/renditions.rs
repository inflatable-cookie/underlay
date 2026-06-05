//! Rendition generation service.
//!
//! This module provides functionality for generating thumbnails and previews
//! from source images, and managing their storage lifecycle.

mod config;
mod keys;
mod processing;
mod result;
mod service;

pub use config::RenditionConfig;
pub use result::RenditionResult;
pub use service::RenditionService;

#[cfg(test)]
#[path = "tests/renditions_tests.rs"]
mod tests;
