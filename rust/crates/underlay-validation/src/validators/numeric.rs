use crate::FieldError;

/// A numeric value that can be checked against range bounds.
///
/// Implemented for the primitive integer and floating-point types. The trait
/// exists so [`range`] can reject values that are not ordered at all — `NaN`
/// compares false against both bounds and would otherwise fail open.
pub trait RangeNumeric: PartialOrd + std::fmt::Display {
    /// Return `true` when the value cannot be ordered, which is only possible
    /// for floating-point `NaN`.
    fn is_unordered(&self) -> bool {
        false
    }
}

macro_rules! impl_integral_range_numeric {
    ($($ty:ty),+ $(,)?) => {
        $(impl RangeNumeric for $ty {})+
    };
}

macro_rules! impl_float_range_numeric {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl RangeNumeric for $ty {
                fn is_unordered(&self) -> bool {
                    self.is_nan()
                }
            }
        )+
    };
}

impl_integral_range_numeric!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
impl_float_range_numeric!(f32, f64);

/// Validate that a numeric value is within a range.
///
/// Unordered values (`NaN`) are rejected rather than failing open.
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
/// assert!(validators::range(f64::NAN, Some(0.0), Some(10.0)).is_err());
/// ```
pub fn range<T: RangeNumeric>(value: T, min: Option<T>, max: Option<T>) -> Result<(), FieldError> {
    if value.is_unordered() {
        return Err(FieldError::with_code("Must be a number", "range.nan"));
    }

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
