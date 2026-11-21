use serde_json::Value;
use webauthn_rs::prelude::{DiscoverableAuthentication, Passkey, PublicKeyCredential};

use super::*;

// ES256 assertion for example.com, signed once with a throwaway test key.
// The fixture contains the public key, assertion and its fixed challenge.
fn fixture() -> Value {
    serde_json::from_str(include_str!("passkey_assertion.json")).unwrap()
}

fn start_discoverable(service: &WebAuthnService) -> DiscoverableAuthentication {
    let mut state = None;
    let started = service
        .start_passkey_authentication_http(
            StartPasskeyAuthenticationRequest {
                allowed_credentials: Vec::new(),
            },
            |challenge_state| {
                state = Some(challenge_state);
                Ok("discoverable-state".to_owned())
            },
        )
        .unwrap();
    assert_eq!(started.state_id, "discoverable-state");
    assert!(started.options.public_key.allow_credentials.is_empty());
    assert_eq!(
        serde_json::to_value(started.options).unwrap()["mediation"],
        "conditional"
    );
    let PasskeyAuthenticationState::Discoverable(state) = state.unwrap() else {
        panic!("an empty allow list should persist conditional UI state");
    };
    state
}

#[test]
fn http_start_and_identify_discoverable_authentication() {
    let service = service();
    let state = start_discoverable(&service);
    let fixture = fixture();
    let assertion: PublicKeyCredential =
        serde_json::from_value(fixture["assertion"].clone()).unwrap();
    let (user_id, credential_id) = service
        .identify_discoverable_authentication(&assertion)
        .unwrap();
    assert_eq!(user_id.to_string(), fixture["user_id"].as_str().unwrap());
    assert_eq!(
        WebAuthnService::credential_id_to_base64url(&credential_id).unwrap(),
        fixture["assertion"]["id"].as_str().unwrap()
    );

    let passkey: Passkey = serde_json::from_value(fixture["passkey"].clone()).unwrap();
    assert!(matches!(
        service.finish_discoverable_authentication(&assertion, &state, vec![passkey]),
        Err(AuthError::PassKeyAuthenticationFailed)
    ));
}

#[test]
fn http_start_with_stored_credentials_preserves_the_allow_list() {
    let service = service();
    let fixture = fixture();
    let result = service
        .start_passkey_authentication_http(
            StartPasskeyAuthenticationRequest {
                allowed_credentials: vec![fixture["passkey"].to_string()],
            },
            |state| {
                assert!(matches!(state, PasskeyAuthenticationState::AllowListed(_)));
                Ok("allow-listed-state".into())
            },
        )
        .unwrap();
    assert_eq!(result.state_id, "allow-listed-state");
    let allowed_credentials = result.options.public_key.allow_credentials;
    assert_eq!(allowed_credentials.len(), 1);
    assert_eq!(
        serde_json::to_value(&allowed_credentials[0].id).unwrap(),
        fixture["assertion"]["id"]
    );
}

#[cfg(feature = "danger-allow-state-serialisation")]
#[test]
fn http_finish_discoverable_authentication_verifies_signature_and_updates_counter() {
    let service = service();
    let fixture = fixture();
    // Reconstruct the saved challenge for which the fixture assertion was signed.
    let mut saved_state = serde_json::to_value(start_discoverable(&service)).unwrap();
    saved_state["ast"]["challenge"] = fixture["challenge"].clone();
    let state = serde_json::from_value(saved_state).unwrap();
    let passkey = service
        .decode_passkey(&fixture["passkey"].to_string())
        .unwrap();
    let stored = service.stored_passkey_from_passkey(&passkey).unwrap();

    let (user_id, result) = service
        .finish_discoverable_authentication_http(
            FinishPasskeyAuthenticationRequest {
                state_id: "discoverable-state".into(),
                credential: serde_json::from_value(fixture["assertion"].clone()).unwrap(),
            },
            |state_id| {
                assert_eq!(state_id, "discoverable-state");
                Ok(state)
            },
            |user_id, credential_id| {
                assert_eq!(user_id.to_string(), fixture["user_id"].as_str().unwrap());
                assert_eq!(credential_id, passkey.cred_id());
                Ok(passkey.clone())
            },
        )
        .unwrap();
    assert_eq!(user_id.to_string(), fixture["user_id"].as_str().unwrap());
    assert!(result.user_verified());
    assert_eq!(result.counter(), 2);

    let update = service
        .update_stored_passkey_after_authentication(&stored, &result)
        .unwrap();
    assert!(update.changed);
    assert_eq!(update.stored_passkey.credential_id, stored.credential_id);
    assert_eq!(update.stored_passkey.counter, Some(2));
    assert_eq!(
        WebAuthnService::passkey_counter_from_json(&update.stored_passkey.passkey_json),
        Some(2)
    );

    for counter in [None, Some(1), Some(3)] {
        let mut updated = update.stored_passkey.clone();
        updated.counter = counter;
        assert!(matches!(
            service.update_stored_passkey_after_authentication(&updated, &result),
            Err(AuthError::PassKeyCounterRegression)
        ));
    }
}

#[cfg(feature = "attestation")]
#[test]
fn attested_registration_rejects_an_empty_trust_store_before_persisting_state() {
    use webauthn_rs::prelude::AttestationCaList;

    let result = service().start_attested_passkey_registration_http(
        StartAttestedPasskeyRegistrationRequest {
            user_id: Uuid::new_v7(),
            user_name: "alice".into(),
            display_name: "Alice".into(),
            exclude_credential_ids: None,
            ui_hint_authenticator_attachment: None,
        },
        AttestationCaList::default(),
        |_| panic!("must not persist an attestation challenge without a trust store"),
    );
    assert!(matches!(result, Err(AuthError::PassKeyRegistrationFailed)));
}

#[cfg(feature = "attestation")]
#[test]
fn attested_authentication_uses_the_stored_credential_and_checks_the_challenge() {
    let service = service();
    let fixture = fixture();
    let passkey = service
        .decode_attested_passkey(&fixture["passkey"].to_string())
        .unwrap();
    let stored = service
        .stored_attested_passkey_from_attested(&passkey)
        .unwrap();
    assert_eq!(
        stored.credential_id,
        fixture["assertion"]["id"].as_str().unwrap()
    );
    assert_eq!(stored.counter, Some(1));
    let read_back = service
        .attested_passkey_from_stored_attested_passkey(&stored)
        .unwrap();
    assert_eq!(read_back.cred_id(), passkey.cred_id());

    let mut state = None;
    let response = service
        .start_attested_passkey_authentication_http(
            StartAttestedPasskeyAuthenticationRequest {
                allowed_credentials: vec![stored.attested_passkey_json],
            },
            |authentication| {
                state = Some(authentication);
                Ok("attested-state".into())
            },
        )
        .unwrap();
    assert_eq!(response.options.public_key.allow_credentials.len(), 1);

    let finish = service.finish_attested_passkey_authentication_http(
        FinishAttestedPasskeyAuthenticationRequest {
            state_id: response.state_id,
            credential: serde_json::from_value(fixture["assertion"].clone()).unwrap(),
        },
        |id| {
            assert_eq!(id, "attested-state");
            Ok(state.unwrap())
        },
    );
    assert!(matches!(
        finish,
        Err(AuthError::PassKeyAuthenticationFailed)
    ));
}
