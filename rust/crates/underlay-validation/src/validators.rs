//! Built-in validators for common validation patterns.
//!
//! Each validator returns `Ok(())` if the value is valid, or `Err(FieldError)` if invalid.

use crate::FieldError;
use once_cell::sync::Lazy;
use regex::Regex;

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
    static EMAIL_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

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
    static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^https?://[a-zA-Z0-9][-a-zA-Z0-9]*(\.[a-zA-Z0-9][-a-zA-Z0-9]*)+(/.*)?$")
            .unwrap()
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
    static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"^[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{4}-?[0-9a-fA-F]{12}$",
        )
        .unwrap()
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

/// Validate that a numeric value is within a range.
///
/// # Arguments
///
/// * `value` - The number to validate
/// * `min` - Minimum value (inclusive), or None for no minimum
/// * `max` - Maximum value (inclusive), or None for no maximum
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::range(25, Some(18), Some(120)).is_ok());
/// assert!(validators::range(10, Some(18), Some(120)).is_err());
/// ```
pub fn range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: Option<T>,
    max: Option<T>,
) -> Result<(), FieldError> {
    if let Some(ref min_val) = min {
        if value < *min_val {
            return Err(FieldError::with_code(
                format!("Must be at least {}", min_val),
                "range.min",
            ));
        }
    }

    if let Some(ref max_val) = max {
        if value > *max_val {
            return Err(FieldError::with_code(
                format!("Must be at most {}", max_val),
                "range.max",
            ));
        }
    }

    Ok(())
}

/// Validate that a value is positive (greater than zero).
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::positive(5).is_ok());
/// assert!(validators::positive(0).is_err());
/// assert!(validators::positive(-1).is_err());
/// ```
pub fn positive<T: PartialOrd + Default>(value: T) -> Result<(), FieldError> {
    if value > T::default() {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must be a positive number",
            "positive",
        ))
    }
}

/// Validate that a value is non-negative (zero or greater).
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::non_negative(0).is_ok());
/// assert!(validators::non_negative(5).is_ok());
/// assert!(validators::non_negative(-1).is_err());
/// ```
pub fn non_negative<T: PartialOrd + Default>(value: T) -> Result<(), FieldError> {
    if value >= T::default() {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must be zero or greater",
            "non_negative",
        ))
    }
}

/// Validate that a string matches a regex pattern.
///
/// # Arguments
///
/// * `value` - The string to validate
/// * `pattern` - The regex pattern to match
/// * `message` - Error message if validation fails
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::pattern("ABC123", r"^[A-Z]+\d+$", "Must be letters followed by numbers").is_ok());
/// assert!(validators::pattern("123ABC", r"^[A-Z]+\d+$", "Must be letters followed by numbers").is_err());
/// ```
pub fn pattern(value: &str, pattern: &str, message: &str) -> Result<(), FieldError> {
    let regex = Regex::new(pattern).map_err(|_| FieldError::new("Invalid regex pattern"))?;

    if regex.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code(message, "pattern.invalid"))
    }
}

/// Validate that a value is one of the allowed options.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::one_of("active", &["active", "inactive"]).is_ok());
/// assert!(validators::one_of("unknown", &["active", "inactive"]).is_err());
/// ```
pub fn one_of<T: PartialEq + std::fmt::Debug>(value: T, options: &[T]) -> Result<(), FieldError> {
    if options.contains(&value) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            format!("Must be one of: {:?}", options),
            "one_of.invalid",
        ))
    }
}

/// Validate that a collection is not empty.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::not_empty(&vec![1, 2, 3]).is_ok());
/// assert!(validators::not_empty(&Vec::<i32>::new()).is_err());
/// ```
pub fn not_empty<T>(value: &[T]) -> Result<(), FieldError> {
    if value.is_empty() {
        Err(FieldError::with_code(
            "Must contain at least one item",
            "not_empty",
        ))
    } else {
        Ok(())
    }
}

/// Validate collection length is within bounds.
///
/// # Arguments
///
/// * `value` - The collection to validate
/// * `min` - Minimum length (inclusive), or None for no minimum
/// * `max` - Maximum length (inclusive), or None for no maximum
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::collection_length(&vec![1, 2, 3], Some(1), Some(5)).is_ok());
/// assert!(validators::collection_length(&vec![1], Some(2), None).is_err());
/// ```
pub fn collection_length<T>(
    value: &[T],
    min: Option<usize>,
    max: Option<usize>,
) -> Result<(), FieldError> {
    let len = value.len();

    if let Some(min_len) = min {
        if len < min_len {
            return Err(FieldError::with_code(
                format!("Must contain at least {} items", min_len),
                "collection.min",
            ));
        }
    }

    if let Some(max_len) = max {
        if len > max_len {
            return Err(FieldError::with_code(
                format!("Must contain at most {} items", max_len),
                "collection.max",
            ));
        }
    }

    Ok(())
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
    static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

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
    static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap());

    if SLUG_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code(
            "Must be a valid slug (lowercase letters, numbers, and hyphens)",
            "slug.invalid",
        ))
    }
}

/// Validate that all items in a collection are unique.
///
/// Useful for reorder payloads where duplicate IDs are invalid.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// assert!(validators::unique_items(&vec!["a", "b", "c"]).is_ok());
/// assert!(validators::unique_items(&vec!["a", "b", "a"]).is_err());
/// ```
pub fn unique_items<T: std::hash::Hash + Eq>(value: &[T]) -> Result<(), FieldError> {
    let mut seen = std::collections::HashSet::new();
    for item in value {
        if !seen.insert(item) {
            return Err(FieldError::with_code(
                "All items must be unique",
                "unique_items.duplicate",
            ));
        }
    }
    Ok(())
}

/// Validate that all items in a collection are unique, returning the count of duplicates.
///
/// This variant provides more detail for error messages.
///
/// # Example
///
/// ```rust
/// use underlay_validation::validators;
///
/// let result = validators::unique_items_detailed(&vec!["a", "b", "a", "c", "a"]);
/// assert!(result.is_err());
/// ```
pub fn unique_items_detailed<T: std::hash::Hash + Eq>(value: &[T]) -> Result<(), FieldError> {
    let mut seen = std::collections::HashSet::new();
    let mut duplicate_count = 0;
    for item in value {
        if !seen.insert(item) {
            duplicate_count += 1;
        }
    }
    if duplicate_count > 0 {
        Err(FieldError::with_code(
            format!(
                "Found {} duplicate item(s) - all items must be unique",
                duplicate_count
            ),
            "unique_items.duplicate",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "validators_tests.rs"]
mod tests;
