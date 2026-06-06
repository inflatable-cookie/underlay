use crate::FieldError;

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
