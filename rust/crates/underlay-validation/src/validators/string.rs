use crate::FieldError;
use regex::Regex;
use std::sync::LazyLock;

/// Validate that a string is a valid email address.
///
/// Uses a simple regex that covers most valid email formats.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::email("user@example.com").is_ok());
/// assert!(validators::email("not-an-email").is_err());
/// ```
pub fn email(value: &str) -> Result<(), FieldError> {
    static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("email regex should compile")
    });

    if EMAIL_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Invalid email address",
            "email.invalid",
        ))
    }
}

/// Validate that a string is a valid URL.
///
/// Checks for http:// or https:// prefix and basic URL structure.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::url("https://example.com").is_ok());
/// assert!(validators::url("not-a-url").is_err());
/// ```
pub fn url(value: &str) -> Result<(), FieldError> {
    static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^https?://[a-zA-Z0-9][-a-zA-Z0-9]*(\.[a-zA-Z0-9][-a-zA-Z0-9]*)+(/.*)?$")
            .expect("url regex should compile")
    });

    if URL_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code("Invalid URL", "url.invalid"))
    }
}

/// Validate that a string is a valid UUID.
///
/// Accepts both hyphenated and non-hyphenated formats.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
/// assert!(validators::uuid("not-a-uuid").is_err());
/// ```
pub fn uuid(value: &str) -> Result<(), FieldError> {
    static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12}$",
        )
        .expect("uuid regex should compile")
    });

    if UUID_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code("Invalid UUID", "uuid.invalid"))
    }
}

/// Validate string length is within bounds.
///
/// # Arguments
///
/// * `value` - The string to validate
/// * `min` - Minimum length (inclusive), or None for no minimum
/// * `max` - Maximum length (inclusive), or None for no maximum
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::length("short", Some(8), Some(100)).is_err());
/// assert!(validators::length("password123", Some(8), Some(100)).is_ok());
/// ```
pub fn length(value: &str, min: Option<usize>, max: Option<usize>) -> Result<(), FieldError> {
    let len = value.chars().count();

    if let Some(min_len) = min {
        if len < min_len {
            return Err(FieldError::with_code(
                format!("Must be at least {} characters", min_len),
                "length.min",
            ));
        }
    }

    if let Some(max_len) = max {
        if len > max_len {
            return Err(FieldError::with_code(
                format!("Must be at most {} characters", max_len),
                "length.max",
            ));
        }
    }

    Ok(())
}

/// Validate that a string is not empty.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::required("value").is_ok());
/// assert!(validators::required("").is_err());
/// assert!(validators::required("   ").is_err());
/// ```
pub fn required(value: &str) -> Result<(), FieldError> {
    if value.trim().is_empty() {
        Err(FieldError::with_code("This field is required", "required"))
    } else {
        Ok(())
    }
}

/// Validate that a string contains only alphanumeric characters.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::alphanumeric("abc123").is_ok());
/// assert!(validators::alphanumeric("abc-123").is_err());
/// ```
pub fn alphanumeric(value: &str) -> Result<(), FieldError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must contain only letters and numbers",
            "alphanumeric",
        ))
    }
}

/// Validate a username format (alphanumeric, underscore, hyphen).
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::username("john_doe-123").is_ok());
/// assert!(validators::username("john@doe").is_err());
/// ```
pub fn username(value: &str) -> Result<(), FieldError> {
    static USERNAME_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").expect("username regex should compile"));

    if USERNAME_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must contain only letters, numbers, underscores, and hyphens",
            "username.invalid",
        ))
    }
}

/// Validate a slug format (lowercase, alphanumeric, hyphen).
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::slug("my-article-title").is_ok());
/// assert!(validators::slug("My Article").is_err());
/// ```
pub fn slug(value: &str) -> Result<(), FieldError> {
    static SLUG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").expect("slug regex should compile")
    });

    if SLUG_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must be a valid slug (lowercase letters, numbers, and hyphens)",
            "slug.invalid",
        ))
    }
}
