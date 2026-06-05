use base64::Engine as _;

use crate::keys::URL_SAFE_NO_PAD;
use crate::KeyPair;

use super::support::*;

#[test]
fn generates_unique_key_pairs() {
    let key1 = KeyPair::generate().unwrap();
    let key2 = KeyPair::generate().unwrap();

    assert_ne!(
        key1.private_key_pkcs8_der_b64,
        key2.private_key_pkcs8_der_b64
    );
    assert_ne!(key1.public_key_raw_b64, key2.public_key_raw_b64);
}

#[test]
fn generated_keys_are_valid_base64() {
    let key = KeyPair::generate().unwrap();

    let priv_decoded = STANDARD.decode(&key.private_key_pkcs8_der_b64).unwrap();
    assert!(!priv_decoded.is_empty());

    let pub_decoded = URL_SAFE_NO_PAD.decode(&key.public_key_raw_b64).unwrap();
    assert_eq!(pub_decoded.len(), 32);
}

#[test]
fn decode_private_key_returns_bytes() {
    let key = KeyPair::generate().unwrap();
    let bytes = key.decode_private_key_der().unwrap();
    assert!(!bytes.is_empty());
}

#[test]
fn decode_invalid_base64_fails() {
    let key = KeyPair::generate().unwrap();
    let invalid_key = KeyPair {
        private_key_pkcs8_der_b64: "not-valid-base64!!!".to_string(),
        public_key_raw_b64: key.public_key_raw_b64.clone(),
    };
    assert!(invalid_key.decode_private_key_der().is_err());
}
