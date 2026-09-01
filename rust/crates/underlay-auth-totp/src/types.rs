use underlay_auth::CredentialMetadata;

#[derive(Clone, PartialEq, Eq)]
pub struct TotpSecret {
    /// Base32 (RFC4648, no padding) secret string.
    pub base32: String,
    /// Raw secret bytes.
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
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

impl std::fmt::Debug for TotpSecret {
    /// Both fields are the same shared secret in different encodings, so
    /// neither is rendered.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TotpSecret")
            .field("base32", &"[REDACTED]")
            .field(
                "bytes",
                &format_args!("[REDACTED; {} bytes]", self.bytes.len()),
            )
            .finish()
    }
}

impl std::fmt::Debug for TotpSetup {
    /// `otpauth_uri` and `qr_svg` both embed the shared secret, and
    /// `backup_codes` are single-use credentials, so all three are redacted.
    /// `backup_code_hashes` are stored digests and stay visible.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TotpSetup")
            .field("secret", &self.secret)
            .field("otpauth_uri", &"[REDACTED]")
            .field("qr_svg", &"[REDACTED]")
            .field(
                "backup_codes",
                &format_args!("[REDACTED; {} codes]", self.backup_codes.len()),
            )
            .field("backup_code_hashes", &self.backup_code_hashes)
            .field("metadata", &self.metadata)
            .finish()
    }
}

#[cfg(test)]
mod debug_redaction_tests {
    use super::{TotpSecret, TotpSetup};
    use underlay_auth::CredentialMetadata;

    fn setup() -> TotpSetup {
        TotpSetup {
            secret: TotpSecret {
                base32: "JBSWY3DPEHPK3PXP".to_string(),
                bytes: vec![1, 2, 3, 4, 5],
            },
            otpauth_uri: "otpauth://totp/Example?secret=JBSWY3DPEHPK3PXP".to_string(),
            qr_svg: "<svg>JBSWY3DPEHPK3PXP</svg>".to_string(),
            backup_codes: vec!["code-one".to_string(), "code-two".to_string()],
            backup_code_hashes: vec!["hash-one".to_string(), "hash-two".to_string()],
            metadata: CredentialMetadata::Totp {
                issuer: "Example".to_string(),
                algorithm: "SHA1".to_string(),
                digits: 6,
                period: 30,
            },
        }
    }

    #[test]
    fn secret_debug_redacts_both_encodings() {
        let rendered = format!("{:?}", setup().secret);

        assert!(!rendered.contains("JBSWY3DPEHPK3PXP"));
        assert!(rendered.contains("[REDACTED]"));
        assert!(rendered.contains("[REDACTED; 5 bytes]"));
    }

    #[test]
    fn setup_debug_redacts_every_secret_carrier() {
        let rendered = format!("{:?}", setup());

        assert!(!rendered.contains("JBSWY3DPEHPK3PXP"));
        assert!(!rendered.contains("code-one"));
        assert!(!rendered.contains("code-two"));
        assert!(rendered.contains("[REDACTED; 2 codes]"));
    }

    #[test]
    fn setup_debug_keeps_stored_backup_code_hashes_visible() {
        let rendered = format!("{:?}", setup());

        assert!(rendered.contains("hash-one"));
        assert!(rendered.contains("hash-two"));
    }
}
