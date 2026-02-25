use super::*;

fn service() -> WebAuthnService {
    WebAuthnService::new(WebAuthnConfig {
        rp_id: "example.com".to_string(),
        rp_origin: "https://example.com".to_string(),
        rp_name: "Example".to_string(),
    })
    .unwrap()
}

#[test]
fn start_registration_produces_challenge_and_state() {
    let svc = service();
    let user_id = Uuid::new_v7();

    let (ccr, state) = svc
        .start_passkey_registration(user_id, "claire", "Claire", None)
        .unwrap();

    // Basic sanity: challenge options must contain rp + user.
    let json = serde_json::to_value(&ccr).unwrap();
    assert!(json.get("publicKey").is_some());
    assert!(format!("{state:?}").contains("PasskeyRegistration"));
}

#[test]
fn start_authentication_allows_discoverable_credentials() {
    let svc = service();
    let (rcr, _state) = svc.start_passkey_authentication(vec![]).unwrap();

    let json = serde_json::to_value(&rcr).unwrap();
    assert!(json.get("publicKey").is_some());

    let allow_credentials = json
        .get("publicKey")
        .and_then(|v| v.get("allowCredentials"))
        .and_then(|v| v.as_array())
        .unwrap();
    assert!(allow_credentials.is_empty());
}

#[test]
fn http_start_registration_returns_state_id_and_options() {
    let svc = service();

    let stored = std::cell::RefCell::new(None);

    let resp = svc
        .start_passkey_registration_http(
            StartPasskeyRegistrationRequest {
                user_id: Uuid::new_v7(),
                user_name: "claire".to_string(),
                display_name: "Claire".to_string(),
                exclude_credential_ids: None,
            },
            |state| {
                *stored.borrow_mut() = Some(state);
                Ok("state-1".to_string())
            },
        )
        .unwrap();

    assert_eq!(resp.state_id, "state-1");
    assert!(stored.borrow().is_some());

    let json = serde_json::to_value(&resp.options).unwrap();
    assert!(json.get("publicKey").is_some());
}

#[test]
fn credential_id_to_base64url_round_trips() {
    let credential_id: CredentialId = serde_json::from_value(serde_json::json!("AQID")).unwrap();
    let encoded = WebAuthnService::credential_id_to_base64url(&credential_id).unwrap();
    assert_eq!(encoded, "AQID");
}

#[test]
fn passkey_counter_from_json_extracts_counter() {
    assert_eq!(WebAuthnService::passkey_counter_from_json("not json"), None);
    assert_eq!(
        WebAuthnService::passkey_counter_from_json(r#"{"cred":{"counter":5}}"#),
        Some(5)
    );
}

#[test]
fn passkey_transports_from_json_extracts_transports() {
    assert_eq!(
        WebAuthnService::passkey_transports_from_json(
            r#"{"cred":{"transports":["internal","hybrid"]}}"#
        ),
        vec!["internal".to_string(), "hybrid".to_string()]
    );
}

#[test]
fn passkey_sync_info_from_json_extracts_backup_flags() {
    let info = WebAuthnService::passkey_sync_info_from_json(
        r#"{"cred":{"transports":["internal"],"backup_eligible":true,"backup_state":false,"user_verified":true}}"#,
    );

    assert_eq!(info.transports, vec!["internal".to_string()]);
    assert!(info.backup_eligible);
    assert!(!info.backup_state);
    assert!(info.user_verified);
}

#[test]
fn credential_metadata_from_stored_passkey_uses_transports_and_counter() {
    let stored = StoredPasskey {
        credential_id: "AQID".to_string(),
        passkey_json: r#"{"cred":{"transports":["internal"],"counter":9}}"#.to_string(),
        counter: Some(7),
    };

    let meta = WebAuthnService::credential_metadata_from_stored_passkey(&stored);
    match meta {
        CredentialMetadata::Passkey {
            credential_id,
            transports,
            last_counter,
        } => {
            assert_eq!(credential_id, "AQID");
            assert_eq!(transports, vec!["internal".to_string()]);
            assert_eq!(last_counter, 7);
        }
        _ => panic!("expected passkey metadata"),
    }
}

#[test]
fn invalid_stored_passkey_rejects_decode() {
    let svc = service();

    let stored = StoredPasskey {
        credential_id: "AQID".to_string(),
        passkey_json: "not json".to_string(),
        counter: None,
    };

    let err = svc.passkey_from_stored_passkey(&stored).unwrap_err();
    assert!(matches!(err, AuthError::BadRequest(_)));
}

#[test]
fn http_start_authentication_rejects_invalid_allowed_credentials() {
    let svc = service();

    let err = svc
        .start_passkey_authentication_http(
            StartPasskeyAuthenticationRequest {
                allowed_credentials: vec!["not json".to_string()],
            },
            |_state| Ok("state-1".to_string()),
        )
        .unwrap_err();

    assert!(matches!(err, AuthError::BadRequest(_)));
}

#[test]
fn invalid_finish_registration_fails_gracefully() {
    let svc = service();
    let user_id = Uuid::new_v7();
    let (_ccr, state) = svc
        .start_passkey_registration(user_id, "claire", "Claire", None)
        .unwrap();

    // Must be valid base64url to deserialize, but still invalid WebAuthn data.
    let bogus: RegisterPublicKeyCredential = serde_json::from_value(serde_json::json!({
        "id": "AQID",
        "rawId": "AQID",
        "type": "public-key",
        "response": {
            "attestationObject": "AQID",
            "clientDataJSON": "AQID"
        }
    }))
    .unwrap();

    let err = svc.finish_passkey_registration(&state, &bogus).unwrap_err();
    assert!(matches!(err, AuthError::PassKeyRegistrationFailed));
}

#[test]
fn passkey_encoding_rejects_invalid_json() {
    let svc = service();
    let err = svc.decode_passkey("not json").unwrap_err();
    assert!(matches!(err, AuthError::BadRequest(_)));
}

#[test]
fn auth_error_mapping_is_stable() {
    assert_eq!(
        AuthError::PassKeyRegistrationFailed.code(),
        "auth.passkey_registration_failed"
    );
    assert_eq!(
        AuthError::PassKeyAuthenticationFailed.code(),
        "auth.passkey_authentication_failed"
    );
    assert_eq!(
        AuthError::PassKeyCounterRegression.code(),
        "auth.passkey_counter_regression"
    );
}
