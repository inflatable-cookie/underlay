use std::time::{SystemTime, UNIX_EPOCH};

use rand::RngExt;

use crate::{TotpAlgorithm, TotpError, TotpSecret, TotpService, VerifiedTotp};

impl TotpService {
    pub fn generate_secret(&self) -> TotpSecret {
        let mut bytes = vec![0_u8; 20];
        rand::rng().fill(&mut bytes[..]);

        let base32 = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &bytes);
        TotpSecret { base32, bytes }
    }

    pub fn decode_secret(&self, base32_secret: &str) -> Result<Vec<u8>, TotpError> {
        base32::decode(base32::Alphabet::Rfc4648 { padding: false }, base32_secret)
            .ok_or(TotpError::InvalidSecret)
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

        let current_counter = now_secs / self.config.period_seconds();

        for step in -self.config.skew_steps()..=self.config.skew_steps() {
            let counter = if step.is_negative() {
                current_counter.saturating_sub(step.wrapping_abs() as u64)
            } else {
                current_counter.saturating_add(step as u64)
            };

            let expected = totp_code(
                secret,
                counter,
                self.config.digits(),
                self.config.algorithm(),
            );
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

pub(crate) fn totp_code(secret: &[u8], counter: u64, digits: u32, algorithm: TotpAlgorithm) -> u32 {
    let mac = match algorithm {
        TotpAlgorithm::Sha1 => {
            use hmac::Mac;
            type HmacSha1 = hmac::Hmac<sha1::Sha1>;

            let mut mac = HmacSha1::new_from_slice(secret).expect("HMAC accepts any key size");
            mac.update(&counter.to_be_bytes());
            mac.finalize().into_bytes().to_vec()
        }
    };

    let offset = (mac[mac.len() - 1] & 0x0f) as usize;
    let slice = &mac[offset..offset + 4];
    let binary = u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]) & 0x7fff_ffff;

    let modulo = 10_u32.pow(digits);
    binary % modulo
}
