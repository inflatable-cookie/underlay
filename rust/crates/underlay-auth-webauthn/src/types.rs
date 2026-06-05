use serde::{Deserialize, Serialize};
use underlay_core::Uuid;
use webauthn_rs::prelude::{
    AuthenticationResult, CreationChallengeResponse, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse,
};

use crate::CredentialId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredPasskey {
    /// Base64url-encoded credential ID (safe for indexing).
    pub credential_id: String,

    /// JSON-encoded `webauthn_rs::prelude::Passkey`.
    pub passkey_json: String,

    /// The last known signature counter, if available.
    pub counter: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasskeySyncInfo {
    pub transports: Vec<String>,
    pub backup_eligible: bool,
    pub backup_state: bool,
    pub user_verified: bool,
}

#[derive(Debug, Clone)]
pub struct StoredPasskeyUpdate {
    pub stored_passkey: StoredPasskey,
    pub changed: bool,
}

#[cfg(feature = "attestation")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAttestedPasskey {
    /// Base64url-encoded credential ID (safe for indexing).
    pub credential_id: String,

    /// JSON-encoded `webauthn_rs::prelude::AttestedPasskey`.
    pub attested_passkey_json: String,

    /// The last known signature counter, if available.
    pub counter: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyRegistrationRequest {
    pub user_id: Uuid,
    pub user_name: String,
    pub display_name: String,
    pub exclude_credential_ids: Option<Vec<CredentialId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyRegistrationResponse {
    pub options: CreationChallengeResponse,

    /// Opaque identifier for server-side persistence between start/finish.
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyRegistrationRequest {
    pub state_id: String,
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyRegistrationResponse {
    pub stored_passkey: StoredPasskey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyAuthenticationRequest {
    /// JSON-encoded `Passkey` values that are allowed for this authentication.
    ///
    /// For discoverable credentials, pass an empty list.
    pub allowed_credentials: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartPasskeyAuthenticationResponse {
    pub options: RequestChallengeResponse,

    /// Opaque identifier for server-side persistence between start/finish.
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyAuthenticationRequest {
    pub state_id: String,
    pub credential: PublicKeyCredential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishPasskeyAuthenticationResponse {
    pub result: AuthenticationResult,
}

#[derive(Debug, Clone)]
pub struct WebAuthnConfig {
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: String,
}
