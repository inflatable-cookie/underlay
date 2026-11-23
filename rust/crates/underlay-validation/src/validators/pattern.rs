use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use regex::Regex;

use crate::FieldError;

/// Upper bound on the number of distinct patterns kept in the process-wide
/// compiled-regex cache.
///
/// Patterns are normally compile-time literals from `#[validate(pattern = ...)]`
/// or a small set of hand-written literals, so the cache stays tiny. The bound
/// only exists so a caller that feeds dynamic patterns cannot grow it without
/// limit.
const PATTERN_CACHE_CAPACITY: usize = 128;

/// A cached regex compilation outcome.
///
/// Invalid patterns are remembered too, so a failing pattern does not
/// recompile on every validation call either.
#[derive(Clone)]
enum CachedPattern {
    Compiled(Regex),
    Invalid,
}

fn pattern_cache() -> &'static Mutex<HashMap<String, CachedPattern>> {
    static CACHE: OnceLock<Mutex<HashMap<String, CachedPattern>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Return the cached compilation outcome for `pattern`, compiling and caching
/// it on first use.
fn cached_pattern(pattern: &str) -> CachedPattern {
    let cache_mutex = pattern_cache();
    {
        // A panic elsewhere while holding the lock must not turn every later
        // validation call into a panic; recover the inner map instead.
        let cache = cache_mutex
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(cached) = cache.get(pattern) {
            return cached.clone();
        }
    }

    // Compile outside the global cache mutex so regex compilation does not block other threads.
    let compiled = match Regex::new(pattern) {
        Ok(regex) => CachedPattern::Compiled(regex),
        Err(_) => CachedPattern::Invalid,
    };

    let mut cache = cache_mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(cached) = cache.get(pattern) {
        return cached.clone();
    }

    if cache.len() >= PATTERN_CACHE_CAPACITY {
        if let Some(evicted) = cache.keys().next().cloned() {
            cache.remove(&evicted);
        }
    }

    cache.insert(pattern.to_string(), compiled.clone());
    compiled
}

/// Validate that a string matches a regex pattern.
///
/// Compiled patterns are cached per process, so repeated validation with the
/// same literal pattern does not recompile the regex.
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
    match cached_pattern(pattern) {
        CachedPattern::Compiled(regex) if regex.is_match(value) => Ok(()),
        CachedPattern::Compiled(_) => Err(FieldError::with_code(message, "pattern.invalid")),
        CachedPattern::Invalid => Err(FieldError::with_code(
            "Invalid regex pattern",
            "pattern.invalid_regex",
        )),
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

#[cfg(test)]
pub(crate) fn cached_pattern_count() -> usize {
    pattern_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .len()
}

#[cfg(test)]
pub(crate) fn pattern_cache_capacity() -> usize {
    PATTERN_CACHE_CAPACITY
}
