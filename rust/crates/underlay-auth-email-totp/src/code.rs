//! Code generation utilities for email TOTP.

use rand::RngExt;

/// Generate a secure random numeric code of the specified length.
///
/// # Example
///
/// ```
/// use underlay_auth_email_totp::generate_code;
///
/// let code = generate_code(6);
/// assert_eq!(code.len(), 6);
/// assert!(code.chars().all(|c| c.is_ascii_digit()));
/// ```
pub fn generate_code(length: usize) -> String {
    let mut rng = rand::rng();
    let max = 10_u64.pow(length as u32);
    let code: u64 = rng.random_range(0..max);
    format!("{:0width$}", code, width = length)
}

#[cfg(test)]
#[path = "tests/code_tests.rs"]
mod tests;
