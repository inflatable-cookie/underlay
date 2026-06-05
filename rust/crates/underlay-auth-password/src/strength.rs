//! Password strength analysis and validation.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use zxcvbn::zxcvbn;

/// Password requirements configuration.
///
/// This can be serialized and exposed via an API endpoint so that
/// frontend UIs can display accurate requirements without hardcoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasswordRequirements {
    /// Minimum password length.
    pub min_length: usize,
    /// Whether mixed case (upper + lower) is recommended.
    pub require_mixed_case: bool,
    /// Whether at least one digit is recommended.
    pub require_digit: bool,
    /// Whether at least one special character is recommended.
    pub require_special: bool,
    /// Minimum zxcvbn score required (0-4, where 3 = "Good").
    pub min_strength_score: u8,
    /// Human-readable description of requirements.
    pub description: String,
}

/// Password strength levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasswordStrength {
    /// Very weak - fails most requirements.
    VeryWeak,
    /// Weak - meets minimum length but low complexity.
    Weak,
    /// Fair - meets basic complexity requirements.
    Fair,
    /// Good - meets all standard requirements.
    Good,
    /// Strong - exceeds requirements with high entropy.
    Strong,
}

/// Analysis of password strength.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PasswordAnalysis {
    /// Overall strength rating.
    pub strength: PasswordStrength,
    /// Minimum required length (8).
    pub min_length: usize,
    /// Actual password length.
    pub length: usize,
    /// Whether password has lowercase letters.
    pub has_lowercase: bool,
    /// Whether password has uppercase letters.
    pub has_uppercase: bool,
    /// Whether password has digits.
    pub has_digits: bool,
    /// Whether password has special characters.
    pub has_special: bool,
    /// Number of unique characters.
    pub unique_chars: usize,
    /// Whether password is in the common passwords list.
    pub is_common: bool,
    /// Estimated entropy in bits.
    pub entropy_bits: f64,
    /// Feedback messages.
    pub feedback: Vec<String>,
}

/// Analyzer for password strength.
#[derive(Debug, Clone)]
pub struct PasswordStrengthAnalyzer {
    /// Minimum password length.
    min_length: usize,
    /// Common passwords to reject.
    common_passwords: HashSet<&'static str>,
}

impl Default for PasswordStrengthAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordStrengthAnalyzer {
    /// Check if a password is in the local common-passwords blocklist.
    pub fn is_common_password(&self, password: &str) -> bool {
        let normalized = password.trim().to_ascii_lowercase();
        self.common_passwords.contains(normalized.as_str())
    }

    /// Create a new analyzer with default settings.
    pub fn new() -> Self {
        let mut common_passwords = HashSet::new();
        let common_list = [
            "password",
            "123456",
            "123456789",
            "12345678",
            "12345",
            "1234567",
            "qwerty",
            "abc123",
            "monkey",
            "1234567890",
            "letmein",
            "trustno1",
            "dragon",
            "baseball",
            "iloveyou",
            "master",
            "sunshine",
            "ashley",
            "bailey",
            "passw0rd",
            "shadow",
            "123123",
            "654321",
            "superman",
            "qazwsx",
            "michael",
            "football",
            "password1",
            "password123",
            "batman",
            "login",
            "admin",
            "welcome",
            "hello",
            "charlie",
            "donald",
            "qwerty123",
            "password!",
            "admin123",
            "root",
            "toor",
            "jesus",
            "password1!",
            "secret",
            "access",
            "mustang",
            "george",
            "thomas",
            "hockey",
            "ranger",
            "daniel",
            "jordan",
            "ginger",
            "buster",
            "summer",
            "tigger",
            "robert",
            "soccer",
            "harley",
            "liverpool",
            "manutd",
            "arsenal",
            "chelsea",
        ];
        common_passwords.extend(common_list);

        Self {
            min_length: 8,
            common_passwords,
        }
    }

    /// Create an analyzer with custom minimum length.
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    /// Add common passwords to the blocklist.
    pub fn add_common_passwords(mut self, passwords: &[&'static str]) -> Self {
        self.common_passwords.extend(passwords);
        self
    }

    /// Get the password requirements configuration.
    ///
    /// This can be exposed via an API endpoint so frontends can display
    /// accurate requirements without hardcoding values.
    pub fn requirements(&self) -> PasswordRequirements {
        PasswordRequirements {
            min_length: self.min_length,
            // These are recommendations for achieving a good zxcvbn score
            require_mixed_case: true,
            require_digit: true,
            require_special: true,
            min_strength_score: 3, // "Good" strength
            description: format!(
                "Password must be at least {} characters with a mix of letters, numbers, and symbols. Avoid common words and patterns.",
                self.min_length
            ),
        }
    }

    /// Analyze a password and return its strength.
    pub fn analyze(&self, password: &str) -> PasswordAnalysis {
        let length = password.len();
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_digits = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password
            .chars()
            .any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());
        let unique_chars = password.chars().collect::<HashSet<_>>().len();
        let is_common = self.is_common_password(password);

        // Prefer zxcvbn score for overall strength.
        // It is more robust than composition rules (and is still fully offline).
        let entropy = zxcvbn(password, &[]);
        let (zxcvbn_score, zxcvbn_guesses, zxcvbn_feedback) = (
            entropy.score() as u8,
            entropy.guesses(),
            match entropy.feedback() {
                Some(f) => {
                    let mut msgs = Vec::new();
                    if let Some(w) = f.warning() {
                        msgs.push(w.to_string());
                    }
                    for s in f.suggestions() {
                        msgs.push(s.to_string());
                    }
                    msgs
                }
                None => Vec::new(),
            },
        );

        // Rough entropy estimate from guesses; not perfect but useful as an informational metric.
        let entropy_bits = if zxcvbn_guesses > 0 {
            (zxcvbn_guesses as f64).log2()
        } else {
            0.0
        };

        let mut feedback = Vec::new();
        if length < self.min_length {
            feedback.push(format!(
                "Password must be at least {} characters",
                self.min_length
            ));
        }
        feedback.extend(zxcvbn_feedback);

        let strength = if is_common || length < self.min_length {
            PasswordStrength::VeryWeak
        } else {
            match zxcvbn_score {
                0 => PasswordStrength::VeryWeak,
                1 => PasswordStrength::Weak,
                2 => PasswordStrength::Fair,
                3 => PasswordStrength::Good,
                _ => PasswordStrength::Strong,
            }
        };

        PasswordAnalysis {
            strength,
            min_length: self.min_length,
            length,
            has_lowercase,
            has_uppercase,
            has_digits,
            has_special,
            unique_chars,
            is_common,
            entropy_bits,
            feedback,
        }
    }

    /// Validate a password and return Ok if valid, or an error message.
    pub fn validate(&self, password: &str) -> Result<PasswordAnalysis, String> {
        let analysis = self.analyze(password);

        if analysis.is_common {
            return Err("Password is too common".to_string());
        }

        if analysis.strength < PasswordStrength::Good {
            if let Some(msg) = analysis.feedback.first() {
                return Err(msg.clone());
            }
            return Err("Password is too weak".to_string());
        }

        Ok(analysis)
    }
}

#[cfg(test)]
#[path = "tests/strength_tests.rs"]
mod tests;
