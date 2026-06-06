use crate::FieldError;

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

/// Validate that all items in a collection are unique, returning the count of
/// duplicates.
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
