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
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_code_length() {
        for length in [4, 6, 8] {
            for _ in 0..100 {
                let code = generate_code(length);
                assert_eq!(code.len(), length);
                assert!(code.chars().all(|c| c.is_ascii_digit()));
            }
        }
    }

    #[test]
    fn test_generate_code_randomness() {
        // Generate multiple codes and ensure diversity
        let codes: Vec<String> = (0..20).map(|_| generate_code(6)).collect();
        let unique: HashSet<_> = codes.iter().collect();
        // With 20 codes from 1M possibilities, we should have high diversity
        assert!(unique.len() > 15);
    }

    #[test]
    fn test_generate_code_zero_padded() {
        // Generate many codes to ensure zero-padding works
        for _ in 0..1000 {
            let code = generate_code(6);
            assert_eq!(code.len(), 6);
        }
    }
}
