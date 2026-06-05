use underlay_core::Uuid;

use super::support::*;

#[test]
fn valid_access_token_verifies_successfully() {
    let (config, _) = JwtConfig::generate().unwrap();
    let jwt = JwtService::new(config).unwrap();

    let user_id = Uuid::new_v7();
    let session_id = Uuid::new_v7();

    let (token, _claims) = jwt.issue_access_token(user_id, session_id, vec![]).unwrap();
    let verified = jwt.verify_access_token(&token).unwrap();

    assert_eq!(verified.common.subject, user_id);
    assert_eq!(verified.session_id, session_id);
    assert_eq!(verified.token_use, TokenUse::Access);
}

#[test]
fn valid_refresh_token_verifies_successfully() {
    let (config, _) = JwtConfig::generate().unwrap();
    let jwt = JwtService::new(config).unwrap();

    let user_id = Uuid::new_v7();
    let session_id = Uuid::new_v7();

    let (token, _claims) = jwt
        .issue_refresh_token(user_id, session_id, None, 1)
        .unwrap();
    let verified = jwt.verify_refresh_token(&token).unwrap();

    assert_eq!(verified.common.subject, user_id);
    assert_eq!(verified.session_id, session_id);
    assert_eq!(verified.token_use, TokenUse::Refresh);
    assert_eq!(verified.version, 1);
}
