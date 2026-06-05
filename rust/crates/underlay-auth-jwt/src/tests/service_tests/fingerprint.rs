use base64::Engine as _;
use underlay_core::Uuid;

use crate::fingerprint::token_fingerprint;
use crate::keys::URL_SAFE_NO_PAD;

use super::support::*;

#[test]
fn fingerprint_is_consistent() {
    let token = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    let fp1 = token_fingerprint(token);
    let fp2 = token_fingerprint(token);

    assert_eq!(fp1, fp2);
}

#[test]
fn different_tokens_produce_different_fingerprints() {
    let (config, _) = JwtConfig::generate().unwrap();
    let jwt = JwtService::new(config).unwrap();

    let user_id = Uuid::new_v7();
    let session_id = Uuid::new_v7();

    let (token1, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
    let (token2, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();

    let fp1 = token_fingerprint(&token1);
    let fp2 = token_fingerprint(&token2);

    assert_ne!(fp1, fp2);
}

#[test]
fn fingerprint_is_base64url_encoded() {
    let (config, _) = JwtConfig::generate().unwrap();
    let jwt = JwtService::new(config).unwrap();

    let user_id = Uuid::new_v7();
    let session_id = Uuid::new_v7();

    let (token, _) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
    let fp = token_fingerprint(&token);

    let decoded = URL_SAFE_NO_PAD.decode(&fp).unwrap();
    assert_eq!(decoded.len(), 32);
}
