//! Slug utilities for generating and validating URL-friendly identifiers.

use regex::Regex;
use std::sync::LazyLock;

/// List of reserved slugs that could conflict with routes.
pub const RESERVED_SLUGS: &[&str] = &[
    "new",
    "edit",
    "delete",
    "create",
    "update",
    "list",
    "admin",
    "api",
    "auth",
    "login",
    "logout",
    "register",
    "settings",
    "profile",
    "dashboard",
    "search",
];

/// Regex pattern for valid slugs.
static SLUG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());

/// Convert a string to a URL-friendly slug.
///
/// # Examples
/// ```
/// use underlay_core::slugify;
/// assert_eq!(slugify("Hello World!"), "hello-world");
/// assert_eq!(slugify("FA1 2024"), "fa1-2024");
/// assert_eq!(slugify("  Spaces  "), "spaces");
/// assert_eq!(slugify("Über Café"), "uber-cafe");
/// ```
pub fn slugify(input: &str) -> String {
    use deunicode::deunicode;

    let normalized = deunicode(input);

    let chars: String = normalized
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();

    chars
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Validate slug format without checking availability.
///
/// A valid slug:
/// - Contains only lowercase letters, numbers, and hyphens
/// - Starts and ends with a letter or number
/// - Is between 2 and 100 characters
/// - Has no consecutive hyphens
pub fn is_valid_slug_format(slug: &str) -> bool {
    SLUG_REGEX.is_match(slug) && slug.len() >= 2 && slug.len() <= 100
}

/// Check if a slug is reserved (would conflict with routes).
pub fn is_reserved_slug(slug: &str) -> bool {
    RESERVED_SLUGS.contains(&slug)
}

/// Validation error types for slugs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlugValidationError {
    TooShort,
    TooLong,
    InvalidFormat,
    Reserved,
}

impl std::fmt::Display for SlugValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlugValidationError::TooShort => write!(f, "too_short"),
            SlugValidationError::TooLong => write!(f, "too_long"),
            SlugValidationError::InvalidFormat => write!(f, "invalid_format"),
            SlugValidationError::Reserved => write!(f, "reserved"),
        }
    }
}

/// Validate a slug and return detailed error information.
pub fn validate_slug(slug: &str) -> Result<(), SlugValidationError> {
    if slug.len() < 2 {
        return Err(SlugValidationError::TooShort);
    }
    if slug.len() > 100 {
        return Err(SlugValidationError::TooLong);
    }
    if !is_valid_slug_format(slug) {
        return Err(SlugValidationError::InvalidFormat);
    }
    if is_reserved_slug(slug) {
        return Err(SlugValidationError::Reserved);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("  spaces  "), "spaces");
    }

    #[test]
    fn test_slugify_numbers() {
        assert_eq!(slugify("FA1 2024"), "fa1-2024");
        assert_eq!(slugify("Module 123"), "module-123");
    }

    #[test]
    fn test_slugify_unicode() {
        assert_eq!(slugify("Über Café"), "uber-cafe");
        assert_eq!(slugify("日本語"), "ri-ben-yu");
    }

    #[test]
    fn test_valid_slug_format() {
        assert!(is_valid_slug_format("hello-world"));
        assert!(is_valid_slug_format("fa1-2024"));
        assert!(is_valid_slug_format("ab"));
        assert!(!is_valid_slug_format("a")); // too short
        assert!(!is_valid_slug_format("Hello-World")); // uppercase
        assert!(!is_valid_slug_format("hello--world")); // double hyphen
        assert!(!is_valid_slug_format("-hello")); // leading hyphen
        assert!(!is_valid_slug_format("hello-")); // trailing hyphen
    }

    #[test]
    fn test_reserved_slugs() {
        assert!(is_reserved_slug("new"));
        assert!(is_reserved_slug("edit"));
        assert!(is_reserved_slug("admin"));
        assert!(!is_reserved_slug("hello-world"));
    }

    #[test]
    fn test_validate_slug() {
        assert!(validate_slug("hello-world").is_ok());
        assert_eq!(validate_slug("a"), Err(SlugValidationError::TooShort));
        assert_eq!(validate_slug("new"), Err(SlugValidationError::Reserved));
        assert_eq!(
            validate_slug("Hello-World"),
            Err(SlugValidationError::InvalidFormat)
        );
    }
}
