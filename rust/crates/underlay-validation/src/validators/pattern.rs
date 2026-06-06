use crate::FieldError;
use regex::Regex;

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
