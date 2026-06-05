use rand::RngExt;
use sha2::Digest;
use underlay_auth::AuthError;

use crate::TotpService;

impl TotpService {
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
}

fn normalize_backup_code(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn generate_backup_code() -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

    let mut raw = [0_u8; 10];
    rand::rng().fill(&mut raw);

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
