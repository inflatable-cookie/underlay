use underlay_auth::{AuthError, AuthResult, CredentialMetadata};
use webauthn_rs::prelude::*;

use super::WebAuthnService;
use crate::{CredentialId, PasskeySyncInfo, StoredPasskey, StoredPasskeyUpdate, WebAuthnError};

impl WebAuthnService {
    pub fn credential_id_to_base64url(credential_id: &CredentialId) -> AuthResult<String> {
        let value = serde_json::to_value(credential_id)
            .map_err(|_| WebAuthnError::InvalidPasskeyEncoding)?;

        match value {
            serde_json::Value::String(s) => Ok(s),
            _ => Err(WebAuthnError::InvalidPasskeyEncoding.into()),
        }
    }

    /// Attempts to read the signature counter from an encoded `Passkey` JSON string.
    ///
    /// This is intended for indexing/diagnostics only; rely on `AuthenticationResult` and
    /// `Passkey::update_credential` for counter updates.
    pub fn passkey_counter_from_json(passkey_json: &str) -> Option<u32> {
        let value: serde_json::Value = serde_json::from_str(passkey_json).ok()?;
        let counter = value.get("cred")?.get("counter")?.as_u64()?;
        u32::try_from(counter).ok()
    }

    pub fn stored_passkey_from_passkey(&self, passkey: &Passkey) -> AuthResult<StoredPasskey> {
        let passkey_json = self.encode_passkey(passkey)?;
        let credential_id =
            Self::credential_id_to_base64url(&Self::credential_id_from_passkey(passkey))?;
        let counter = Self::passkey_counter_from_json(&passkey_json);

        Ok(StoredPasskey {
            credential_id,
            passkey_json,
            counter,
        })
    }

    pub fn passkey_from_stored_passkey(&self, stored: &StoredPasskey) -> AuthResult<Passkey> {
        self.decode_passkey(&stored.passkey_json)
    }

    pub fn passkey_transports_from_json(passkey_json: &str) -> Vec<String> {
        let value: serde_json::Value = match serde_json::from_str(passkey_json) {
            Ok(v) => v,
            Err(_) => return vec![],
        };

        let Some(transports) = value.get("cred").and_then(|v| v.get("transports")) else {
            return vec![];
        };

        let Some(transports) = transports.as_array() else {
            return vec![];
        };

        transports
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    }

    pub fn passkey_sync_info_from_json(passkey_json: &str) -> PasskeySyncInfo {
        let value: serde_json::Value = match serde_json::from_str(passkey_json) {
            Ok(v) => v,
            Err(_) => {
                return PasskeySyncInfo {
                    transports: vec![],
                    backup_eligible: false,
                    backup_state: false,
                    user_verified: false,
                };
            }
        };

        let transports = Self::passkey_transports_from_json(passkey_json);

        let backup_eligible = value
            .get("cred")
            .and_then(|v| v.get("backup_eligible"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let backup_state = value
            .get("cred")
            .and_then(|v| v.get("backup_state"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let user_verified = value
            .get("cred")
            .and_then(|v| v.get("user_verified"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        PasskeySyncInfo {
            transports,
            backup_eligible,
            backup_state,
            user_verified,
        }
    }

    pub fn credential_metadata_from_stored_passkey(stored: &StoredPasskey) -> CredentialMetadata {
        let transports = Self::passkey_transports_from_json(&stored.passkey_json);
        let last_counter = stored
            .counter
            .or_else(|| Self::passkey_counter_from_json(&stored.passkey_json))
            .unwrap_or(0);

        CredentialMetadata::Passkey {
            credential_id: stored.credential_id.clone(),
            transports,
            last_counter,
        }
    }

    pub fn authentication_result_credential_id_base64url(
        result: &AuthenticationResult,
    ) -> AuthResult<String> {
        let value = serde_json::to_value(result.cred_id())
            .map_err(|_| WebAuthnError::InvalidPasskeyEncoding)?;

        match value {
            serde_json::Value::String(s) => Ok(s),
            _ => Err(WebAuthnError::InvalidPasskeyEncoding.into()),
        }
    }

    pub fn find_stored_passkey_by_credential_id<'a>(
        stored: &'a [StoredPasskey],
        credential_id: &str,
    ) -> Option<&'a StoredPasskey> {
        stored.iter().find(|pk| pk.credential_id == credential_id)
    }

    /// Applies an [`AuthenticationResult`] to a stored passkey, performing counter regression checks
    /// and returning an updated stored passkey when the credential properties changed.
    pub fn update_stored_passkey_after_authentication(
        &self,
        stored: &StoredPasskey,
        result: &AuthenticationResult,
    ) -> AuthResult<StoredPasskeyUpdate> {
        let result_credential_id = Self::authentication_result_credential_id_base64url(result)?;
        if result_credential_id != stored.credential_id {
            return Err(AuthError::PassKeyCredentialNotFound);
        }

        let stored_counter = stored
            .counter
            .or_else(|| Self::passkey_counter_from_json(&stored.passkey_json));

        let new_counter = result.counter();
        if new_counter > 0 {
            if let Some(old_counter) = stored_counter {
                if new_counter <= old_counter {
                    return Err(AuthError::PassKeyCounterRegression);
                }
            }
        }

        let mut passkey = self.passkey_from_stored_passkey(stored)?;
        let changed = passkey
            .update_credential(result)
            .ok_or(AuthError::PassKeyCredentialNotFound)?;

        if !changed {
            return Ok(StoredPasskeyUpdate {
                stored_passkey: stored.clone(),
                changed: false,
            });
        }

        let passkey_json = self.encode_passkey(&passkey)?;
        let counter = Self::passkey_counter_from_json(&passkey_json).or(stored.counter);

        Ok(StoredPasskeyUpdate {
            stored_passkey: StoredPasskey {
                credential_id: stored.credential_id.clone(),
                passkey_json,
                counter,
            },
            changed: true,
        })
    }
}
