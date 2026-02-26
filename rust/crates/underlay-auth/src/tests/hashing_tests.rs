use super::*;

#[test]
fn hash_is_nondeterministic_and_verifies() {
    let hasher = Argon2Hasher::new();
    let password = b"correct horse battery staple";

    let h1 = hasher.hash_password(password).unwrap();
    let h2 = hasher.hash_password(password).unwrap();

    assert_ne!(h1, h2);
    assert!(hasher.verify_password(password, &h1).unwrap());
    assert!(hasher.verify_password(password, &h2).unwrap());
    assert!(!hasher.verify_password(b"wrong", &h1).unwrap());

    // Verifying should use parameters from the PHC string, not the current hasher params.
    let weaker = Argon2Hasher::with_params(32768, 2, 2);
    assert!(weaker.verify_password(password, &h1).unwrap());
}

#[test]
fn needs_rehash_detects_algorithm_and_params() {
    let hasher = Argon2Hasher::new();
    let password = b"password";
    let hash = hasher.hash_password(password).unwrap();

    assert!(!hasher.needs_rehash(&hash));

    let mutated_alg = hash.replacen("$argon2id$", "$argon2i$", 1);
    assert!(hasher.needs_rehash(&mutated_alg));

    let weaker = Argon2Hasher::with_params(32768, 2, 4);
    assert!(weaker.needs_rehash(&hash));
}
