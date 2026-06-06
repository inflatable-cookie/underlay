//! Derive macro for the `Validate` trait.
//!
//! This crate provides a `#[derive(Validate)]` macro that automatically generates
//! validation code based on field attributes.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_validation::Validate;
//!
//! #[derive(Validate)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(length(min = 8, max = 100))]
//!     password: String,
//!
//!     #[validate(range(min = 18, max = 120))]
//!     age: i32,
//!
//!     #[validate(required)]
//!     name: String,
//! }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;
mod field;
mod rules;

use derive::impl_validate;

/// Derive the `Validate` trait for a struct.
///
/// # Supported Attributes
///
/// - `#[validate(email)]` - Validate as email
/// - `#[validate(url)]` - Validate as URL
/// - `#[validate(uuid)]` - Validate as UUID
/// - `#[validate(required)]` - Non-empty string
/// - `#[validate(length(min = N, max = M))]` - String length bounds
/// - `#[validate(range(min = N, max = M))]` - Numeric range bounds
/// - `#[validate(pattern = "regex")]` - Custom regex
/// - `#[validate(custom = "function_name")]` - Custom validator function
/// - `#[validate(nested)]` - Validate nested struct
/// - `#[validate(skip)]` - Skip validation for this field
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match impl_validate(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
