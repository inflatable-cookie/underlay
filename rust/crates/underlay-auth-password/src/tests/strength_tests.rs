use super::*;

#[test]
fn very_short_password_rejected() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.analyze("abc");

    assert_eq!(result.strength, PasswordStrength::VeryWeak);
    assert!(result.feedback.iter().any(|f| f.contains("at least")));
}

#[test]
fn common_password_rejected() {
    let analyzer = PasswordStrengthAnalyzer::new();

    for common in ["password", "123456", "admin", "qwerty"] {
        let result = analyzer.analyze(common);
        assert!(result.is_common);
        assert_eq!(result.strength, PasswordStrength::VeryWeak);
    }
}

#[test]
fn is_common_password_case_insensitive() {
    let analyzer = PasswordStrengthAnalyzer::new();

    assert!(analyzer.is_common_password("PASSWORD"));
    assert!(analyzer.is_common_password("Password"));
    assert!(analyzer.is_common_password("PaSsWoRd"));
}

#[test]
fn strength_levels_order_correctly() {
    assert!(PasswordStrength::VeryWeak < PasswordStrength::Weak);
    assert!(PasswordStrength::Weak < PasswordStrength::Fair);
    assert!(PasswordStrength::Fair < PasswordStrength::Good);
    assert!(PasswordStrength::Good < PasswordStrength::Strong);
}

#[test]
fn strong_password_has_all_features() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.analyze("S3cur3P@ssw0rd!");

    assert!(result.has_lowercase);
    assert!(result.has_uppercase);
    assert!(result.has_digits);
    assert!(result.has_special);
    assert!(result.length >= 8);
    assert!(!result.is_common);
    assert!(result.strength >= PasswordStrength::Fair);
}

#[test]
fn password_only_lowercase_is_weak() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.analyze("password123");

    assert!(result.has_lowercase);
    assert!(!result.has_uppercase);
    assert!(result.has_digits);
    assert!(!result.has_special);
    assert!(result.strength < PasswordStrength::Good);
}

#[test]
fn custom_min_length_is_respected() {
    let analyzer = PasswordStrengthAnalyzer::new().with_min_length(12);
    let result = analyzer.analyze("Short1!");

    assert!(result.feedback.iter().any(|f| f.contains("at least 12")));
    assert_eq!(result.strength, PasswordStrength::VeryWeak);
}

#[test]
fn add_common_passwords_extends_blocklist() {
    let analyzer =
        PasswordStrengthAnalyzer::new().add_common_passwords(&["custom_pass", "test1234"]);

    assert!(analyzer.is_common_password("custom_pass"));
    assert!(analyzer.is_common_password("test1234"));
    assert!(!analyzer.is_common_password("randompassword"));
}

#[test]
fn validate_returns_ok_for_strong_password() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.validate("S3cur3P@ss!");

    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert!(analysis.strength >= PasswordStrength::Fair);
}

#[test]
fn validate_returns_err_for_weak_password() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.validate("weak");

    assert!(result.is_err());
}

#[test]
fn unique_chars_count_is_correct() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let result = analyzer.analyze("aabbcc");

    assert_eq!(result.unique_chars, 3);
}

#[test]
fn entropy_bits_calculated() {
    let analyzer = PasswordStrengthAnalyzer::new();
    let weak = analyzer.analyze("abc");
    let strong = analyzer.analyze("S3cur3P@ssw0rd!");

    assert!(strong.entropy_bits > weak.entropy_bits);
}
