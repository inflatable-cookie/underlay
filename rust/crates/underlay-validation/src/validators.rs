//! Built-in validators for common validation patterns.
//!
//! Each validator returns `Ok(())` if the value is valid, or `Err(FieldError)`
//! if invalid.

mod collection;
mod numeric;
mod pattern;
mod string;

pub use collection::{collection_length, not_empty, unique_items, unique_items_detailed};
pub use numeric::{non_negative, positive, range};
pub use pattern::{one_of, pattern};
pub use string::{alphanumeric, email, length, required, slug, url, username, uuid};

#[cfg(test)]
#[path = "tests/validators_tests.rs"]
mod tests;
