//! Time-based One-Time Password (TOTP) primitives.
//!
//! This crate provides app-agnostic helpers for:
//! - generating secrets
//! - generating otpauth provisioning URIs + QR SVG
//! - verifying TOTP codes with a time window
//! - generating and verifying backup codes (hashed)
//!
//! Storage, encryption, and credential association are owned by the app.

use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Digest;
use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};
use underlay_auth::{AuthError, AuthResult, CredentialMetadata};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
}

impl TotpAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            TotpAlgorithm::Sha1 => "SHA1",
        }
    }
}

/// Configuration for TOTP generation and verification.
///
/// # Example
///
/// ```
/// use underlay_auth_totp::{TotpConfig, TotpAlgorithm};
///
/// // Default config
/// let config = TotpConfig::default();
///
/// // Custom config with builder pattern
/// let config = TotpConfig::default()
///     .with_issuer("My App")
///     .with_digits(8)
///     .with_period_seconds(60)
///     .with_skew_steps(2);
/// ```
#[derive(Debug, Clone)]
pub struct TotpConfig {
    /// The issuer name displayed in authenticator apps.
    /// Default: "Underlay"
    pub issuer: String,
    /// The TOTP algorithm to use.
    /// Default: SHA1 (widely supported)
    pub algorithm: TotpAlgorithm,
    /// Number of digits in the TOTP code.
    /// Default: 6
    pub digits: u32,
    /// Time period for each code in seconds.
    /// Default: 30
    pub period_seconds: u64,
    /// How many time steps before/after current time are accepted.
    /// Default: 1 (accepts previous, current, and next code)
    pub skew_steps: i64,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            issuer: "Underlay".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period_seconds: 30,
            skew_steps: 1,
        }
    }
}

impl TotpConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the issuer name (shown in authenticator apps).
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = issuer.into();
        self
    }

    /// Set the TOTP algorithm.
    pub fn with_algorithm(mut self, algorithm: TotpAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Set the number of digits in TOTP codes.
    ///
    /// Common values: 6 (default, most compatible) or 8.
    pub fn with_digits(mut self, digits: u32) -> Self {
        self.digits = digits;
        self
    }

    /// Set the time period for each code in seconds.
    ///
    /// Default is 30 seconds. Some apps use 60 seconds.
    pub fn with_period_seconds(mut self, seconds: u64) -> Self {
        self.period_seconds = seconds;
        self
    }

    /// Set the time skew tolerance in steps.
    ///
    /// A value of 1 (default) accepts the previous, current, and next code.
    /// A value of 0 only accepts the current code (strict).
    /// A value of 2 provides more tolerance for clock drift.
    pub fn with_skew_steps(mut self, steps: i64) -> Self {
        self.skew_steps = steps;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TotpSecret {
    /// Base32 (RFC4648, no padding) secret string.
    pub base32: String,
    /// Raw secret bytes.
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TotpSetup {
    pub secret: TotpSecret,
    pub otpauth_uri: String,
    /// QR code as an SVG string.
    pub qr_svg: String,
    /// Backup codes to display once.
    pub backup_codes: Vec<String>,
    /// Hashes to store (hex-encoded SHA-256 of normalized code).
    pub backup_code_hashes: Vec<String>,
    pub metadata: CredentialMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedTotp {
    pub counter: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoFactorCode<'a> {
    Totp(&'a str),
    BackupCode(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoFactorVerified {
    Totp(VerifiedTotp),
    BackupCode { index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TotpError {
    #[error("invalid totp secret")]
    InvalidSecret,
    #[error("invalid totp code")]
    InvalidCode,
    #[error("replay detected")]
    Replay,
}

impl From<TotpError> for AuthError {
    fn from(value: TotpError) -> Self {
        match value {
            TotpError::InvalidSecret | TotpError::InvalidCode => AuthError::TwoFactorInvalid,
            TotpError::Replay => AuthError::TwoFactorInvalid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TotpService {
    config: TotpConfig,
}

impl TotpService {
    pub fn new(config: Option<TotpConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
        }
    }

    pub fn config(&self) -> &TotpConfig {
        &self.config
    }

    pub fn metadata(&self) -> CredentialMetadata {
        CredentialMetadata::Totp {
            issuer: self.config.issuer.clone(),
            algorithm: self.config.algorithm.as_str().to_string(),
            digits: self.config.digits as u8,
            period: self.config.period_seconds as u8,
        }
    }

    pub fn generate_secret(&self) -> TotpSecret {
        // 160-bit is the common baseline.
        let mut bytes = vec![0_u8; 20];
        OsRng.fill_bytes(&mut bytes);

        let base32 = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &bytes);
        TotpSecret { base32, bytes }
    }

    pub fn decode_secret(&self, base32_secret: &str) -> Result<Vec<u8>, TotpError> {
        base32::decode(base32::Alphabet::Rfc4648 { padding: false }, base32_secret)
            .ok_or(TotpError::InvalidSecret)
    }

    pub fn provisioning_uri(&self, account_name: &str, secret_base32: &str) -> String {
        let issuer = url_escape(&self.config.issuer);
        let account = url_escape(account_name);
        let label = format!("{}:{}", issuer, account);

        // RFC6238 provisioning URI format.
        // otpauth://totp/{issuer}:{account}?secret=...&issuer=...&algorithm=...&digits=...&period=...
        format!(
            "otpauth://totp/{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
            label,
            secret_base32,
            issuer,
            self.config.algorithm.as_str(),
            self.config.digits,
            self.config.period_seconds
        )
    }

    pub fn qr_svg(&self, otpauth_uri: &str) -> AuthResult<String> {
        let code = qrcode::QrCode::new(otpauth_uri.as_bytes())
            .map_err(|e| AuthError::Internal(format!("failed to build QR code: {e}")))?;

        let svg = code
            .render::<qrcode::render::svg::Color>()
            .min_dimensions(200, 200)
            .quiet_zone(true)
            .build();

        Ok(svg)
    }

    pub fn generate_backup_codes(&self, count: usize) -> (Vec<String>, Vec<String>) {
        let mut codes = Vec::with_capacity(count);
        let mut hashes = Vec::with_capacity(count);

        for _ in 0..count {
            let code = generate_backup_code();
            let hash = hash_backup_code(&code);

            codes.push(code);
            hashes.push(hash);
        }

        (codes, hashes)
    }

    pub fn setup(&self, account_name: &str, backup_code_count: usize) -> AuthResult<TotpSetup> {
        let secret = self.generate_secret();
        let otpauth_uri = self.provisioning_uri(account_name, &secret.base32);
        let qr_svg = self.qr_svg(&otpauth_uri)?;
        let (backup_codes, backup_code_hashes) = self.generate_backup_codes(backup_code_count);

        Ok(TotpSetup {
            secret,
            otpauth_uri,
            qr_svg,
            backup_codes,
            backup_code_hashes,
            metadata: self.metadata(),
        })
    }

    pub fn verify_totp(&self, secret_base32: &str, code: &str) -> bool {
        self.verify_totp_at(secret_base32, code, SystemTime::now())
            .is_ok()
    }

    pub fn verify_totp_at(
        &self,
        secret_base32: &str,
        code: &str,
        now: SystemTime,
    ) -> Result<VerifiedTotp, TotpError> {
        let bytes = self.decode_secret(secret_base32)?;
        self.verify_totp_bytes_at(&bytes, code, now, None)
    }

    pub fn verify_totp_with_replay_protection(
        &self,
        secret_base32: &str,
        code: &str,
        now: SystemTime,
        last_counter: u64,
    ) -> Result<VerifiedTotp, TotpError> {
        let bytes = self.decode_secret(secret_base32)?;
        self.verify_totp_bytes_at(&bytes, code, now, Some(last_counter))
    }

    pub fn verify_totp_bytes_at(
        &self,
        secret: &[u8],
        code: &str,
        now: SystemTime,
        last_counter: Option<u64>,
    ) -> Result<VerifiedTotp, TotpError> {
        let code = normalize_numeric_code(code)?;

        let now_secs = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TotpError::InvalidCode)?
            .as_secs();

        let current_counter = now_secs / self.config.period_seconds;

        // Accept current +/- skew.
        for step in -self.config.skew_steps..=self.config.skew_steps {
            let counter = if step.is_negative() {
                current_counter.saturating_sub(step.wrapping_abs() as u64)
            } else {
                current_counter.saturating_add(step as u64)
            };

            let expected = totp_code(secret, counter, self.config.digits, self.config.algorithm);
            if expected == code {
                if let Some(last) = last_counter {
                    if counter <= last {
                        return Err(TotpError::Replay);
                    }
                }

                return Ok(VerifiedTotp { counter });
            }
        }

        Err(TotpError::InvalidCode)
    }

    pub fn verify_backup_code(
        &self,
        input: &str,
        stored_hashes: &[String],
    ) -> Result<usize, AuthError> {
        let candidate = normalize_backup_code(input);
        if candidate.is_empty() {
            return Err(AuthError::BackupCodeInvalid);
        }

        let candidate_hash = hash_backup_code(&candidate);

        for (i, stored) in stored_hashes.iter().enumerate() {
            if constant_time_eq_hex(stored, &candidate_hash) {
                return Ok(i);
            }
        }

        Err(AuthError::BackupCodeInvalid)
    }

    /// Helper for app login flows: verify either a TOTP code or a backup code.
    ///
    /// Apps can call this after password verification when a user has 2FA enabled.
    pub fn verify_second_factor(
        &self,
        secret_base32: &str,
        last_counter: Option<u64>,
        input: TwoFactorCode<'_>,
        backup_code_hashes: &[String],
        now: SystemTime,
    ) -> AuthResult<TwoFactorVerified> {
        match input {
            TwoFactorCode::Totp(code) => {
                let verified = match last_counter {
                    Some(last) => self
                        .verify_totp_with_replay_protection(secret_base32, code, now, last)
                        .map_err(AuthError::from)?,
                    None => self
                        .verify_totp_at(secret_base32, code, now)
                        .map_err(AuthError::from)?,
                };
                Ok(TwoFactorVerified::Totp(verified))
            }
            TwoFactorCode::BackupCode(code) => {
                let index = self.verify_backup_code(code, backup_code_hashes)?;
                Ok(TwoFactorVerified::BackupCode { index })
            }
        }
    }
}

fn normalize_numeric_code(code: &str) -> Result<u32, TotpError> {
    let trimmed = code.trim();
    if trimmed.is_empty() {
        return Err(TotpError::InvalidCode);
    }

    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Err(TotpError::InvalidCode);
    }

    trimmed.parse::<u32>().map_err(|_| TotpError::InvalidCode)
}

fn url_escape(input: &str) -> Cow<'_, str> {
    // Minimal percent-encoding for otpauth label/query.
    // RFC requires URL encoding; we keep it small and dependency-free.
    if input
        .bytes()
        .all(|b| matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~'))
    {
        return Cow::Borrowed(input);
    }

    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }

    Cow::Owned(out)
}

fn totp_code(secret: &[u8], counter: u64, digits: u32, algorithm: TotpAlgorithm) -> u32 {
    let mac = match algorithm {
        TotpAlgorithm::Sha1 => {
            use hmac::Mac;
            type HmacSha1 = hmac::Hmac<sha1::Sha1>;

            let mut mac = HmacSha1::new_from_slice(secret).expect("HMAC accepts any key size");
            mac.update(&counter.to_be_bytes());
            mac.finalize().into_bytes().to_vec()
        }
    };

    // Dynamic truncation (RFC4226).
    let offset = (mac[mac.len() - 1] & 0x0f) as usize;
    let slice = &mac[offset..offset + 4];
    let binary = u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]) & 0x7fff_ffff;

    let modulo = 10_u32.pow(digits);
    binary % modulo
}

fn normalize_backup_code(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn generate_backup_code() -> String {
    // 10 chars, easy to type, avoid ambiguous chars.
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

    let mut raw = [0_u8; 10];
    OsRng.fill_bytes(&mut raw);

    let mut out = String::with_capacity(11);
    for (i, b) in raw.into_iter().enumerate() {
        if i == 5 {
            out.push('-');
        }
        out.push(ALPHABET[(b as usize) % ALPHABET.len()] as char);
    }

    out
}

fn hash_backup_code(code: &str) -> String {
    let normalized = normalize_backup_code(code);
    let digest = sha2::Sha256::digest(normalized.as_bytes());
    hex::encode(digest)
}

fn constant_time_eq_hex(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0_u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }

    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc6238_sha1_test_vectors_8_digits() {
        // Secret "12345678901234567890" (ASCII) base32-encoded.
        let secret_base32 = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
        let service = TotpService::new(Some(TotpConfig {
            issuer: "Test".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 8,
            period_seconds: 30,
            skew_steps: 0,
        }));

        let secret = service.decode_secret(secret_base32).unwrap();

        let cases = [
            (59_u64, 94287082_u32),
            (1111111109_u64, 7081804_u32),
            (1111111111_u64, 14050471_u32),
            (1234567890_u64, 89005924_u32),
            (2000000000_u64, 69279037_u32),
            (20000000000_u64, 65353130_u32),
        ];

        for (ts, expected) in cases {
            let counter = ts / 30;
            let code = totp_code(&secret, counter, 8, TotpAlgorithm::Sha1);
            assert_eq!(code, expected);
        }
    }

    #[test]
    fn verify_totp_accepts_within_skew() {
        let service = TotpService::new(Some(TotpConfig {
            issuer: "Test".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period_seconds: 30,
            skew_steps: 1,
        }));

        let secret = service.generate_secret();

        let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
        let counter = 1_000_000 / 30;
        let code = totp_code(&secret.bytes, counter + 1, 6, TotpAlgorithm::Sha1);

        let ok = service.verify_totp_bytes_at(&secret.bytes, &format!("{:06}", code), now, None);
        assert!(ok.is_ok());
    }

    #[test]
    fn verify_totp_rejects_replay() {
        let service = TotpService::new(Some(TotpConfig {
            issuer: "Test".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            period_seconds: 30,
            skew_steps: 0,
        }));

        let secret = service.generate_secret();

        let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
        let counter = 1_000_000 / 30;
        let code = totp_code(&secret.bytes, counter, 6, TotpAlgorithm::Sha1);
        let formatted = format!("{:06}", code);

        let verified = service
            .verify_totp_with_replay_protection(&secret.base32, &formatted, now, counter)
            .unwrap_err();

        assert_eq!(verified, TotpError::Replay);
    }

    #[test]
    fn provisioning_uri_contains_expected_fields() {
        let service = TotpService::new(Some(TotpConfig {
            issuer: "My App".to_string(),
            ..TotpConfig::default()
        }));

        let secret = service.generate_secret();
        let uri = service.provisioning_uri("user@example.com", &secret.base32);

        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("secret="));
        assert!(uri.contains("issuer=My%20App"));
        assert!(uri.contains("digits=6"));
        assert!(uri.contains("period=30"));
    }

    #[test]
    fn qr_svg_is_generated() {
        let service = TotpService::new(None);
        let secret = service.generate_secret();
        let uri = service.provisioning_uri("user@example.com", &secret.base32);
        let svg = service.qr_svg(&uri).unwrap();

        assert!(svg.contains("<svg"));
    }

    #[test]
    fn backup_codes_generate_and_verify() {
        let service = TotpService::new(None);
        let (codes, hashes) = service.generate_backup_codes(5);

        assert_eq!(codes.len(), 5);
        assert_eq!(hashes.len(), 5);

        let idx = service.verify_backup_code(&codes[2], &hashes).unwrap();
        assert_eq!(idx, 2);

        assert!(matches!(
            service.verify_backup_code("WRONG-CODE", &hashes),
            Err(AuthError::BackupCodeInvalid)
        ));
    }

    #[test]
    fn verify_second_factor_accepts_backup_code() {
        let service = TotpService::new(None);
        let setup = service.setup("user@example.com", 3).unwrap();

        let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);

        let result = service
            .verify_second_factor(
                &setup.secret.base32,
                None,
                TwoFactorCode::BackupCode(&setup.backup_codes[1]),
                &setup.backup_code_hashes,
                now,
            )
            .unwrap();

        assert_eq!(result, TwoFactorVerified::BackupCode { index: 1 });
    }

    #[test]
    fn setup_includes_qr_and_backup_codes() {
        let service = TotpService::new(None);
        let setup = service.setup("user@example.com", 8).unwrap();

        assert!(!setup.secret.base32.is_empty());
        assert!(setup.otpauth_uri.starts_with("otpauth://"));
        assert!(setup.qr_svg.contains("<svg"));
        assert_eq!(setup.backup_codes.len(), 8);
        assert_eq!(setup.backup_code_hashes.len(), 8);
    }
}
