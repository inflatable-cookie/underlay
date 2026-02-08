use super::*;

#[test]
fn rfc6238_sha1_test_vectors_8_digits() {
    // Secret "12345678901234567890" (ASCII) base32-encoded.
    let secret_base32 = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let service = TotpService::new(Some(TotpConfig {
        issuer: "Test".to_string(),
        algorithm: TotpAlgorithm::Sha1,
        digits: 8,
        period_seconds: 30,
        skew_steps: 0,
    }));

    let secret = service.decode_secret(secret_base32).unwrap();

    let cases = [
        (59_u64, 94287082_u32),
        (1111111109_u64, 7081804_u32),
        (1111111111_u64, 14050471_u32),
        (1234567890_u64, 89005924_u32),
        (2000000000_u64, 69279037_u32),
        (20000000000_u64, 65353130_u32),
    ];

    for (ts, expected) in cases {
        let counter = ts / 30;
        let code = totp_code(&secret, counter, 8, TotpAlgorithm::Sha1);
        assert_eq!(code, expected);
    }
}

#[test]
fn verify_totp_accepts_within_skew() {
    let service = TotpService::new(Some(TotpConfig {
        issuer: "Test".to_string(),
        algorithm: TotpAlgorithm::Sha1,
        digits: 6,
        period_seconds: 30,
        skew_steps: 1,
    }));

    let secret = service.generate_secret();

    let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
    let counter = 1_000_000 / 30;
    let code = totp_code(&secret.bytes, counter + 1, 6, TotpAlgorithm::Sha1);

    let ok = service.verify_totp_bytes_at(&secret.bytes, &format!("{:06}", code), now, None);
    assert!(ok.is_ok());
}

#[test]
fn verify_totp_rejects_replay() {
    let service = TotpService::new(Some(TotpConfig {
        issuer: "Test".to_string(),
        algorithm: TotpAlgorithm::Sha1,
        digits: 6,
        period_seconds: 30,
        skew_steps: 0,
    }));

    let secret = service.generate_secret();

    let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
    let counter = 1_000_000 / 30;
    let code = totp_code(&secret.bytes, counter, 6, TotpAlgorithm::Sha1);
    let formatted = format!("{:06}", code);

    let verified = service
        .verify_totp_with_replay_protection(&secret.base32, &formatted, now, counter)
        .unwrap_err();

    assert_eq!(verified, TotpError::Replay);
}

#[test]
fn provisioning_uri_contains_expected_fields() {
    let service = TotpService::new(Some(TotpConfig {
        issuer: "My App".to_string(),
        ..TotpConfig::default()
    }));

    let secret = service.generate_secret();
    let uri = service.provisioning_uri("user@example.com", &secret.base32);

    assert!(uri.starts_with("otpauth://totp/"));
    assert!(uri.contains("secret="));
    assert!(uri.contains("issuer=My%20App"));
    assert!(uri.contains("digits=6"));
    assert!(uri.contains("period=30"));
}

#[test]
fn qr_svg_is_generated() {
    let service = TotpService::new(None);
    let secret = service.generate_secret();
    let uri = service.provisioning_uri("user@example.com", &secret.base32);
    let svg = service.qr_svg(&uri).unwrap();

    assert!(svg.contains("<svg"));
}

#[test]
fn backup_codes_generate_and_verify() {
    let service = TotpService::new(None);
    let (codes, hashes) = service.generate_backup_codes(5);

    assert_eq!(codes.len(), 5);
    assert_eq!(hashes.len(), 5);

    let idx = service.verify_backup_code(&codes[2], &hashes).unwrap();
    assert_eq!(idx, 2);

    assert!(matches!(
        service.verify_backup_code("WRONG-CODE", &hashes),
        Err(AuthError::BackupCodeInvalid)
    ));
}

#[test]
fn verify_second_factor_accepts_backup_code() {
    let service = TotpService::new(None);
    let setup = service.setup("user@example.com", 3).unwrap();

    let now = UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);

    let result = service
        .verify_second_factor(
            &setup.secret.base32,
            None,
            TwoFactorCode::BackupCode(&setup.backup_codes[1]),
            &setup.backup_code_hashes,
            now,
        )
        .unwrap();

    assert_eq!(result, TwoFactorVerified::BackupCode { index: 1 });
}

#[test]
fn setup_includes_qr_and_backup_codes() {
    let service = TotpService::new(None);
    let setup = service.setup("user@example.com", 8).unwrap();

    assert!(!setup.secret.base32.is_empty());
    assert!(setup.otpauth_uri.starts_with("otpauth://"));
    assert!(setup.qr_svg.contains("<svg"));
    assert_eq!(setup.backup_codes.len(), 8);
    assert_eq!(setup.backup_code_hashes.len(), 8);
}
