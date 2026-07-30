use super::*;

fn test_cipher() -> SecretCipher {
    let key = vec![42u8; 32];
    let b64 = STANDARD.encode(&key);
    SecretCipher::from_key_string("TEST_KEY", &b64).unwrap()
}

#[test]
fn encrypt_is_nondeterministic_and_decrypts() {
    let cipher = test_cipher();

    let enc1 = cipher.encrypt("JBSWY3DPEHPK3PXP").unwrap();
    let enc2 = cipher.encrypt("JBSWY3DPEHPK3PXP").unwrap();
    assert_ne!(enc1, enc2);
    assert!(enc1.starts_with("enc:v1:"));

    assert_eq!(cipher.decrypt(&enc1).unwrap(), "JBSWY3DPEHPK3PXP");
}

#[test]
fn plain_prefix_rejected_by_default() {
    let cipher = test_cipher();

    assert!(matches!(
        cipher.decrypt("plain:abc"),
        Err(AuthError::Internal(_))
    ));
}

#[test]
fn plain_prefix_readable_only_under_explicit_migration() {
    let cipher = test_cipher().with_plain_migration(true);

    assert_eq!(cipher.decrypt("plain:abc").unwrap(), "abc");
}

#[test]
fn unknown_format_rejected() {
    let cipher = test_cipher();

    assert!(cipher.decrypt("JBSWY3DPEHPK3PXP").is_err());
    assert!(cipher.decrypt("enc:v2:foo:bar").is_err());
    assert!(cipher.decrypt("").is_err());
}

#[test]
fn tampered_ciphertext_rejected() {
    let cipher = test_cipher();
    let enc = cipher.encrypt("secret").unwrap();
    let tampered = format!("{}x", &enc[..enc.len() - 1]);

    assert!(cipher.decrypt(&tampered).is_err());
}

#[test]
fn wrong_key_rejected() {
    let cipher = test_cipher();
    let enc = cipher.encrypt("secret").unwrap();

    let other_key = vec![7u8; 32];
    let other = SecretCipher::from_key_string("OTHER", &STANDARD.encode(&other_key)).unwrap();
    assert!(other.decrypt(&enc).is_err());
}

#[test]
fn key_must_be_32_bytes() {
    assert!(SecretCipher::from_key_string("K", &STANDARD.encode([1u8; 16])).is_err());
    assert!(SecretCipher::from_key_string("K", "not-base64!!!").is_err());
}

#[test]
fn generated_key_round_trips() {
    let key = SecretCipher::generate_key();
    let cipher = SecretCipher::from_key_string("GEN", &key).unwrap();
    let enc = cipher.encrypt("secret").unwrap();
    assert_eq!(cipher.decrypt(&enc).unwrap(), "secret");
}

#[test]
fn is_encrypted_checks_prefix() {
    assert!(SecretCipher::is_encrypted("enc:v1:abc:def"));
    assert!(!SecretCipher::is_encrypted("plain:abc"));
    assert!(!SecretCipher::is_encrypted("JBSWY3DPEHPK3PXP"));
}

#[test]
fn debug_redacts_key_material() {
    let cipher = test_cipher();
    let debug = format!("{cipher:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("42"));
}
