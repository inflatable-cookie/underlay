use super::*;

fn token(bytes: &[u8]) -> OwnershipToken {
    OwnershipToken::from_bytes(bytes.to_vec()).unwrap()
}

#[test]
fn ownership_token_rejects_short_input_without_echoing_it() {
    let secret = b"short-secret";
    let err = OwnershipToken::from_bytes(secret.to_vec()).unwrap_err();
    let rendered = format!("{err:?}{err}");
    assert!(matches!(err, BlobError::ConfigError(_)));
    assert!(!rendered.contains("short-secret"));
}

#[test]
fn ownership_token_debug_redacts_raw_bytes() {
    let secret = b"tokensecret-disclosure-probe!!!!";
    assert_eq!(secret.len(), OwnershipToken::MIN_LEN);
    let token = token(secret);
    let rendered = format!("{token:?}");
    assert_eq!(rendered, "OwnershipToken([redacted])");
    assert!(!rendered.contains("tokensecret"));
    assert!(!rendered.contains(&hex::encode(secret)));
}

#[test]
fn verifier_is_stable_and_not_the_raw_token() {
    let token = token(&[0x11; 32]);
    let digest = token.verifier_digest();
    assert_ne!(&digest, &[0x11; 32]);
    assert_eq!(digest, token.verifier_digest());
    assert_ne!(
        digest,
        OwnershipToken::from_bytes(vec![0x22; 32])
            .unwrap()
            .verifier_digest()
    );
}

#[test]
fn facts_round_trip_through_object_metadata_without_the_token() {
    let token = token(b"tokensecret-disclosure-probe!!!!");
    let data = b"payload-bytes";
    let facts = OwnedPublicationFacts::from_token_and_bytes(&token, data, "image/png");
    let pairs = facts.metadata_pairs();
    let rendered = format!("{facts:?}{pairs:?}");
    assert!(!rendered.contains("tokensecret"));
    assert_eq!(pairs[0].0, OWNED_META_VERIFIER);
    assert_eq!(facts.size(), data.len() as u64);
    assert_eq!(facts.mime(), "image/png");
    assert_eq!(facts.sha256(), hex::encode(Sha256::digest(data)));

    let map: HashMap<_, _> = pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    let parsed = OwnedPublicationFacts::from_object_metadata(&map).unwrap();
    assert!(parsed.matches_token(&token));
    assert!(!parsed.matches_token(&OwnershipToken::from_bytes(vec![0x33; 32]).unwrap()));
}

#[test]
fn metadata_parse_refuses_malformed_digest_size_and_mime() {
    let token = token(&[0x11; 32]);
    let facts = OwnedPublicationFacts::from_token_and_bytes(&token, b"abc", "image/png");
    let mut map: HashMap<_, _> = facts
        .metadata_pairs()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    map.insert(OWNED_META_SHA256.to_string(), "not-a-digest".to_string());
    assert!(OwnedPublicationFacts::from_object_metadata(&map).is_none());

    map.insert(OWNED_META_SHA256.to_string(), facts.sha256().to_string());
    map.insert(OWNED_META_SIZE.to_string(), "12x".to_string());
    assert!(OwnedPublicationFacts::from_object_metadata(&map).is_none());

    map.insert(OWNED_META_SIZE.to_string(), "3".to_string());
    map.insert(OWNED_META_MIME.to_string(), "image/png\n".to_string());
    assert!(OwnedPublicationFacts::from_object_metadata(&map).is_none());

    map.insert(OWNED_META_MIME.to_string(), String::new());
    assert!(OwnedPublicationFacts::from_object_metadata(&map).is_none());

    map.remove(OWNED_META_VERIFIER);
    assert!(OwnedPublicationFacts::from_object_metadata(&map).is_none());
}

#[test]
fn destination_authority_rejects_empty_provider_or_bucket() {
    let key = BlobObjectKey::parse("media/a.png").unwrap();
    assert!(OwnedDestinationAuthority::new("", "bucket", key.clone()).is_err());
    assert!(OwnedDestinationAuthority::new("s3", "", key).is_err());
}
