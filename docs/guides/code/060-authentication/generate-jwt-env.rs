// Generates Ed25519 / EdDSA JWT env vars compatible with `underlay-auth-jwt`.
//
// This is meant to be copied into your app repo as a small binary, e.g.:
// `apps/api/src/bin/generate-jwt-env.rs` (or any crate bin target).
//
// It prints:
// - AUTH_JWT_PRIVATE_KEY: base64 PKCS#8 DER Ed25519 private key
// - AUTH_JWT_PUBLIC_KEY: base64url (no padding) raw Ed25519 public key (32 bytes)
// - default issuer/audience/leeway settings

fn main() {
    let (config, _keypair) =
        underlay_auth_jwt::JwtConfig::generate().expect("failed to generate jwt config");

    println!("# Paste these into apps/api/.env\n");

    println!("AUTH_JWT_PRIVATE_KEY={}", config.private_key_b64);
    println!("AUTH_JWT_PUBLIC_KEY={}", config.public_key_b64);
    println!("AUTH_JWT_ISSUER={}", config.issuer);

    if let Some(aud) = config.audience {
        println!("AUTH_JWT_AUDIENCE={}", aud);
    } else {
        println!("# AUTH_JWT_AUDIENCE=your-audience (optional)");
    }

    println!(
        "AUTH_ACCESS_TOKEN_LIFETIME_MINUTES={}",
        config.access_token_lifetime_minutes
    );
    println!(
        "AUTH_REFRESH_TOKEN_LIFETIME_DAYS={}",
        config.refresh_token_lifetime_days
    );
    println!("AUTH_JWT_LEEWAY_SECONDS={}", config.leeway_seconds);
}
