use super::*;

fn token(bytes: &[u8]) -> OwnershipToken {
    OwnershipToken::from_bytes(bytes.to_vec()).unwrap()
}

fn authority(provider: &str, bucket: &str, key: &str) -> OwnedDestinationAuthority {
    OwnedDestinationAuthority::new(provider, bucket, BlobObjectKey::parse(key).unwrap()).unwrap()
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
fn facts_round_trip_through_object_metadata_without_the_token() {
    let token = token(b"tokensecret-disclosure-probe!!!!");
    let dest = authority("s3", "bucket", "media/a.png");
    let data = b"payload-bytes";
    let facts = OwnedPublicationFacts::from_token_and_bytes(&token, &dest, data, "image/png");
    let pairs = facts.metadata_pairs();
    let rendered = format!("{facts:?}{pairs:?}");
    assert!(!rendered.contains("tokensecret"));
    assert_eq!(pairs[0].0, OWNED_META_VERIFIER);
    assert_eq!(facts.size(), data.len() as u64);
    assert_eq!(facts.mime(), "image/png");
    assert_eq!(facts.sha256(), hex::encode(Sha256::digest(data)));

    let map: HashMap<_, _> = pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    let parsed = OwnedPublicationFacts::from_object_metadata(&map).unwrap();
    assert!(parsed.matches_token(&token, &dest));
    assert!(!parsed.matches_token(&OwnershipToken::from_bytes(vec![0x33; 32]).unwrap(), &dest));
}

#[test]
fn metadata_parse_refuses_malformed_digest_size_and_mime() {
    let token = token(&[0x11; 32]);
    let dest = authority("s3", "bucket", "media/a.png");
    let facts = OwnedPublicationFacts::from_token_and_bytes(&token, &dest, b"abc", "image/png");
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

#[test]
fn copied_metadata_does_not_recover_under_a_new_destination_authority() {
    let token = token(&[0x11; 32]);
    let original = authority("s3", "bucket", "media/a.png");
    let copied = authority("s3", "bucket", "media/hostile.png");
    let facts =
        OwnedPublicationFacts::from_token_and_bytes(&token, &original, b"payload", "image/png");
    assert!(facts.matches_token(&token, &original));
    assert!(
        !facts.matches_token(&token, &copied),
        "copied object metadata must not verify under a different key"
    );
    assert!(!facts.matches_token(&token, &authority("s3", "other-bucket", "media/a.png")));
    assert!(!facts.matches_token(&token, &authority("local", "bucket", "media/a.png")));
}

#[test]
fn token_reuse_across_two_destinations_produces_distinct_verifiers() {
    let token = token(&[0x11; 32]);
    let dest_a = authority("s3", "bucket", "media/a.png");
    let dest_b = authority("s3", "bucket", "media/b.png");
    let facts_a =
        OwnedPublicationFacts::from_token_and_bytes(&token, &dest_a, b"payload", "image/png");
    let facts_b =
        OwnedPublicationFacts::from_token_and_bytes(&token, &dest_b, b"payload", "image/png");
    assert_ne!(facts_a.metadata_pairs()[0].1, facts_b.metadata_pairs()[0].1);
    assert!(facts_a.matches_token(&token, &dest_a));
    assert!(facts_b.matches_token(&token, &dest_b));
    assert!(!facts_a.matches_token(&token, &dest_b));
    assert!(!facts_b.matches_token(&token, &dest_a));
}

#[test]
fn length_prefixed_encoding_rejects_concatenation_ambiguity() {
    let token = token(&[0x11; 32]);
    let key = "media/a.png";
    let ambiguous_a = authority("ab", "c", key);
    let ambiguous_b = authority("a", "bc", key);
    let facts_a =
        OwnedPublicationFacts::from_token_and_bytes(&token, &ambiguous_a, b"payload", "image/png");
    let facts_b =
        OwnedPublicationFacts::from_token_and_bytes(&token, &ambiguous_b, b"payload", "image/png");
    assert_ne!(
        facts_a.metadata_pairs()[0].1,
        facts_b.metadata_pairs()[0].1,
        "provider/bucket concatenation must not be ambiguous"
    );
    assert!(!facts_a.matches_token(&token, &ambiguous_b));
    assert!(!facts_b.matches_token(&token, &ambiguous_a));
}
