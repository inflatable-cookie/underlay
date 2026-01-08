//! JWT claims types for Underlay authentication.

use serde::{Deserialize, Serialize};
use underlay_core::Uuid;

/// How this token is intended to be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenUse {
    /// Access token for API authentication.
    Access,
    /// Refresh token for obtaining new access tokens.
    Refresh,
}

/// Common JWT claims shared by access and refresh tokens.
///
/// Time-based claims (`iat`, `exp`, `nbf`) are represented as NumericDate
/// (seconds since Unix epoch) for JWT interoperability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonClaims {
    /// Token issuer.
    #[serde(rename = "iss")]
    pub issuer: String,

    /// Token subject (user ID).
    #[serde(rename = "sub")]
    pub subject: Uuid,

    /// Audience (optional).
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,

    /// Issued at (NumericDate).
    #[serde(rename = "iat")]
    pub issued_at: u64,

    /// Expiration time (NumericDate).
    #[serde(rename = "exp")]
    pub expires_at: u64,

    /// Not before time (NumericDate).
    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<u64>,

    /// Token ID.
    #[serde(rename = "jti")]
    pub token_id: Uuid,
}

/// Access token claims.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    /// Common claims.
    #[serde(flatten)]
    pub common: CommonClaims,

    /// Token use.
    #[serde(rename = "tuse")]
    pub token_use: TokenUse,

    /// Session ID.
    #[serde(rename = "sid")]
    pub session_id: Uuid,

    /// User roles/permissions.
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
}

/// Refresh token claims.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    /// Common claims.
    #[serde(flatten)]
    pub common: CommonClaims,

    /// Token use.
    #[serde(rename = "tuse")]
    pub token_use: TokenUse,

    /// Session ID.
    #[serde(rename = "sid")]
    pub session_id: Uuid,

    /// Previous refresh token ID (rotation tracking).
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub previous_token_id: Option<Uuid>,

    /// Refresh token version (incremented on refresh).
    #[serde(rename = "ver")]
    pub version: u32,
}
