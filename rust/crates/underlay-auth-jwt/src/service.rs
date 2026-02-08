//! JWT issuance and token verification.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use underlay_core::Uuid;

use crate::claims::{AccessTokenClaims, CommonClaims, RefreshTokenClaims, TokenUse};
use crate::config::JwtConfig;
use crate::error::{JwtError, JwtResult};

#[derive(Clone)]
pub struct JwtService {
    pub(crate) encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    pub(crate) config: JwtConfig,
}

impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("encoding_key", &"[REDACTED]")
            .field("decoding_key", &"[REDACTED]")
            .field("config", &self.config)
            .finish()
    }
}

impl JwtService {
    pub fn new(config: JwtConfig) -> JwtResult<Self> {
        let private_key_der = STANDARD
            .decode(&config.private_key_b64)
            .map_err(|_| JwtError::Key("Invalid base64 private key".to_string()))?;
        let encoding_key = EncodingKey::from_ed_der(&private_key_der);

        let public_key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&config.public_key_b64)
            .or_else(|_| base64::engine::general_purpose::STANDARD.decode(&config.public_key_b64))
            .map_err(|e| JwtError::Key(format!("Base64 error: {}", e)))?;
        let public_key_b64url =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(public_key_bytes);

        let decoding_key = DecodingKey::from_ed_components(&public_key_b64url)
            .map_err(|e| JwtError::Key(e.to_string()))?;

        let service = Self {
            encoding_key,
            decoding_key,
            config,
        };

        // Fail fast if the keypair is incompatible (startup-time cost only).
        service.validate_keypair_compatibility()?;

        Ok(service)
    }

    pub fn config(&self) -> &JwtConfig {
        &self.config
    }

    fn validate_keypair_compatibility(&self) -> JwtResult<()> {
        // Issue and verify a short-lived token using the currently configured
        // issuer/audience. This is startup-only and prevents silent key mismatch.
        let session_id = Uuid::new_v7();
        let user_id = Uuid::new_v7();

        let (token, _claims) = self.issue_access_token(user_id, session_id, vec![])?;

        // If this fails, either the public key doesn't match the private key,
        // or the validation config is inconsistent.
        self.verify_access_token(&token).map(|_| ())
    }

    pub fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        roles: Vec<String>,
    ) -> JwtResult<(String, AccessTokenClaims)> {
        let now = Utc::now().timestamp() as u64;
        let exp = (Utc::now() + self.config.access_token_lifetime()).timestamp() as u64;

        let claims = AccessTokenClaims {
            common: CommonClaims {
                issuer: self.config.issuer.clone(),
                subject: user_id,
                audience: self.config.audience.clone(),
                issued_at: now,
                expires_at: exp,
                not_before: Some(now),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Access,
            session_id,
            roles,
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| JwtError::Internal(e.to_string()))?;

        Ok((token, claims))
    }

    pub fn issue_refresh_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        previous_token_id: Option<Uuid>,
        version: u32,
    ) -> JwtResult<(String, RefreshTokenClaims)> {
        let now = Utc::now().timestamp() as u64;
        let exp = (Utc::now() + self.config.refresh_token_lifetime()).timestamp() as u64;

        let claims = RefreshTokenClaims {
            common: CommonClaims {
                issuer: self.config.issuer.clone(),
                subject: user_id,
                audience: self.config.audience.clone(),
                issued_at: now,
                expires_at: exp,
                not_before: Some(now),
                token_id: Uuid::new_v7(),
            },
            token_use: TokenUse::Refresh,
            session_id,
            previous_token_id,
            version,
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.typ = Some("JWT".to_string());

        let token = encode(&header, &claims, &self.encoding_key)
            .map_err(|e| JwtError::Internal(e.to_string()))?;

        Ok((token, claims))
    }

    pub fn verify_access_token(&self, token: &str) -> JwtResult<AccessTokenClaims> {
        let value = self.decode_and_validate_value(token)?;

        let token_use = value
            .get("tuse")
            .and_then(|v| v.as_str())
            .ok_or(JwtError::MalformedToken)?;

        if token_use != "access" {
            return Err(JwtError::UnsupportedTokenType);
        }

        serde_json::from_value::<AccessTokenClaims>(value).map_err(|_| JwtError::MalformedToken)
    }

    pub fn verify_refresh_token(&self, token: &str) -> JwtResult<RefreshTokenClaims> {
        let value = self.decode_and_validate_value(token)?;

        let token_use = value
            .get("tuse")
            .and_then(|v| v.as_str())
            .ok_or(JwtError::MalformedToken)?;

        if token_use != "refresh" {
            return Err(JwtError::UnsupportedTokenType);
        }

        serde_json::from_value::<RefreshTokenClaims>(value).map_err(|_| JwtError::MalformedToken)
    }

    fn decode_and_validate_value(&self, token: &str) -> JwtResult<serde_json::Value> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.leeway = self.config.leeway_seconds;
        validation.validate_nbf = true;

        // Require core JWT claims we rely on.
        if self.config.audience.is_some() {
            validation.set_required_spec_claims(&["exp", "iss", "sub", "aud"]);
        } else {
            validation.set_required_spec_claims(&["exp", "iss", "sub"]);
        }

        validation.set_issuer(&[self.config.issuer.clone()]);
        if let Some(aud) = &self.config.audience {
            validation.set_audience(&[aud.clone()]);
        }

        match decode::<serde_json::Value>(token, &self.decoding_key, &validation) {
            Ok(data) => Ok(data.claims),
            Err(err) => {
                use jsonwebtoken::errors::ErrorKind;
                match err.kind() {
                    ErrorKind::ExpiredSignature => Err(JwtError::Expired),
                    ErrorKind::ImmatureSignature => Err(JwtError::NotYetValid),
                    ErrorKind::InvalidToken => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidSignature => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidIssuer => Err(JwtError::InvalidToken),
                    ErrorKind::InvalidAudience => Err(JwtError::InvalidToken),
                    ErrorKind::Json(_) => Err(JwtError::MalformedToken),
                    _ => Err(JwtError::InvalidToken),
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "service_tests.rs"]
mod tests;
