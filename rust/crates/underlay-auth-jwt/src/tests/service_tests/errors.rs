use super::support::*;

#[test]
fn jwt_error_codes_are_correct() {
    assert_eq!(JwtError::Config("test".to_string()).code(), "auth.internal");
    assert_eq!(JwtError::Key("test".to_string()).code(), "auth.internal");
    assert_eq!(JwtError::Expired.code(), "auth.session_expired");
    assert_eq!(JwtError::NotYetValid.code(), "auth.token_not_yet_valid");
    assert_eq!(JwtError::InvalidToken.code(), "auth.token_invalid");
    assert_eq!(JwtError::MalformedToken.code(), "auth.token_malformed");
    assert_eq!(JwtError::SessionRevoked.code(), "auth.session_revoked");
    assert_eq!(
        JwtError::TokenFingerprintMismatch.code(),
        "auth.token_fingerprint_mismatch"
    );
    assert_eq!(JwtError::RefreshReplayDetected.code(), "auth.token_invalid");
    assert_eq!(JwtError::UnsupportedTokenType.code(), "auth.token_invalid");
    assert_eq!(
        JwtError::Internal("test".to_string()).code(),
        "auth.internal"
    );
}

#[test]
fn jwt_error_converts_to_auth_error() {
    use underlay_auth::AuthError;

    assert_eq!(AuthError::SessionExpired, JwtError::Expired.into());
    assert_eq!(AuthError::TokenNotYetValid, JwtError::NotYetValid.into());
    assert_eq!(AuthError::TokenInvalid, JwtError::InvalidToken.into());
    assert_eq!(AuthError::TokenMalformed, JwtError::MalformedToken.into());
    assert_eq!(AuthError::SessionRevoked, JwtError::SessionRevoked.into());
    assert_eq!(
        AuthError::TokenFingerprintMismatch,
        JwtError::TokenFingerprintMismatch.into()
    );
    assert_eq!(
        AuthError::TokenInvalid,
        JwtError::RefreshReplayDetected.into()
    );
    assert_eq!(
        AuthError::TokenInvalid,
        JwtError::UnsupportedTokenType.into()
    );
    assert!(matches!(
        JwtError::Config("error".into()).into(),
        AuthError::Internal(_)
    ));
    assert!(matches!(
        JwtError::Key("error".into()).into(),
        AuthError::Internal(_)
    ));
    assert!(matches!(
        JwtError::Internal("error".into()).into(),
        AuthError::Internal(_)
    ));
}
