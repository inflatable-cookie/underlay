//! Password hashing using Argon2id.

use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rand_core::OsRng;
use std::io;

/// Trait for hashing passwords securely.
pub trait PasswordHasherExt: Send + Sync {
    /// Hash a password.
    fn hash_password(&self, password: &[u8]) -> Result<String, io::Error>;

    /// Check if a password needs rehashing (e.g., using old parameters).
    fn needs_rehash(&self, hash: &str) -> bool;
}

/// Trait for verifying passwords against hashes.
pub trait PasswordVerifierExt: Send + Sync {
    /// Verify a password against a hash.
    fn verify_password(&self, password: &[u8], hash: &str) -> Result<bool, io::Error>;
}

/// Argon2id password hasher with configurable parameters.
#[derive(Debug, Clone)]
pub struct Argon2Hasher {
    /// Memory cost in KiB (default: 64 MiB = 65536 KiB).
    memory_cost: u32,
    /// Number of iterations (default: 3).
    iterations: u32,
    /// Parallelism factor (default: 4).
    parallelism: u32,
}

impl Default for Argon2Hasher {
    fn default() -> Self {
        Self {
            memory_cost: 65536,
            iterations: 3,
            parallelism: 4,
        }
    }
}

impl Argon2Hasher {
    /// Create a new hasher with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a hasher with custom parameters.
    ///
    /// # Arguments
    /// * `memory_cost` - Memory cost in KiB (minimum 8, must be power of 2)
    /// * `iterations` - Number of iterations (minimum 1)
    /// * `parallelism` - Parallelism factor (minimum 1)
    pub fn with_params(memory_cost: u32, iterations: u32, parallelism: u32) -> Self {
        Self {
            memory_cost,
            iterations,
            parallelism,
        }
    }

    fn params(&self) -> Result<argon2::Params, io::Error> {
        argon2::Params::new(self.memory_cost, self.iterations, self.parallelism, None).map_err(
            |e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid Argon2 parameters: {e}"),
                )
            },
        )
    }
}

impl PasswordHasherExt for Argon2Hasher {
    fn hash_password(&self, password: &[u8]) -> Result<String, io::Error> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            self.params()?,
        );
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = argon2.hash_password(password, &salt).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to hash password: {}", e),
            )
        })?;

        Ok(password_hash.to_string())
    }

    fn needs_rehash(&self, hash: &str) -> bool {
        match PasswordHash::new(hash) {
            Ok(parsed) => {
                if parsed.algorithm.as_str() != "argon2id" {
                    return true;
                }

                // Argon2 v0x13 == decimal 19.
                if parsed.version != Some(0x13) {
                    return true;
                }

                if parsed.salt.is_none() || parsed.hash.is_none() {
                    return true;
                }

                let memory_cost_str = parsed.params.get("m").map(|v| v.as_str());
                let iterations_str = parsed.params.get("t").map(|v| v.as_str());
                let parallelism_str = parsed.params.get("p").map(|v| v.as_str());

                let memory_cost = memory_cost_str.and_then(|v| v.parse().ok()).unwrap_or(0);
                let iterations = iterations_str.and_then(|v| v.parse().ok()).unwrap_or(0);
                let parallelism = parallelism_str.and_then(|v| v.parse().ok()).unwrap_or(0);

                memory_cost != self.memory_cost
                    || iterations != self.iterations
                    || parallelism != self.parallelism
            }
            Err(_) => true,
        }
    }
}

impl PasswordVerifierExt for Argon2Hasher {
    fn verify_password(&self, password: &[u8], hash: &str) -> Result<bool, io::Error> {
        match PasswordHash::new(hash) {
            Ok(parsed_hash) => {
                let argon2 = Argon2::new(
                    argon2::Algorithm::Argon2id,
                    argon2::Version::V0x13,
                    self.params()?,
                );

                let result = argon2.verify_password(password, &parsed_hash);

                match result {
                    Ok(()) => Ok(true),
                    Err(argon2::password_hash::Error::Password) => Ok(false),
                    Err(e) => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to verify password: {}", e),
                    )),
                }
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to parse password hash: {}", e),
            )),
        }
    }
}

#[cfg(test)]
#[path = "tests/hashing_tests.rs"]
mod tests;
