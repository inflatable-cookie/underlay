use underlay_auth::CredentialMetadata;

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
