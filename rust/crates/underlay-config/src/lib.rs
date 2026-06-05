//! Layered TOML configuration helpers for Underlay applications.
//!
//! The loader owns file stacking only. Apps still own their typed config
//! structs, validation, and explicit env override allowlists.

mod constants;
mod discovery;
mod error;
mod merge;
mod stack;

pub use constants::{DEFAULT_CONFIG_DIR, DEFAULT_ENVIRONMENT, DEFAULT_ENV_VAR};
pub use discovery::discover_config_dir;
pub use error::ConfigError;
pub use stack::ConfigStack;

#[cfg(test)]
mod tests;
