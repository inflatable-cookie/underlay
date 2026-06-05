use super::{parse_schema_name, validate_schema_name, DestructiveGuard};

#[test]
fn schema_validation_allows_simple_names() {
    assert!(validate_schema_name("learning"));
    assert!(validate_schema_name("infra"));
    assert!(validate_schema_name("content_library"));
    assert!(validate_schema_name("_private"));
}

#[test]
fn schema_validation_rejects_mixed_case_and_symbols() {
    assert!(!validate_schema_name("Learning"));
    assert!(!validate_schema_name("learning; drop schema public"));
    assert!(!validate_schema_name("learning-content"));
    assert!(!validate_schema_name("learning.content"));
    assert!(!validate_schema_name(""));
    assert!(!validate_schema_name(" "));
}

#[test]
fn destructive_guard_defaults_to_disallowed() {
    let guard = DestructiveGuard::disallow();
    assert!(!guard.is_allowed());
}

#[test]
fn destructive_guard_can_be_allowed() {
    let guard = DestructiveGuard::allow();
    assert!(guard.is_allowed());
}

#[test]
fn schema_validation_allows_underscore_prefix() {
    assert!(validate_schema_name("_test"));
    assert!(validate_schema_name("_my_schema"));
}

#[test]
fn schema_validation_allows_numbers_after_first_char() {
    assert!(validate_schema_name("schema1"));
    assert!(validate_schema_name("learning2content"));
    assert!(validate_schema_name("_private123"));
}

#[test]
fn schema_validation_rejects_uppercase() {
    assert!(!validate_schema_name("PUBLIC"));
    assert!(!validate_schema_name("Learning"));
    assert!(!validate_schema_name("My_Schema"));
}

#[test]
fn schema_validation_rejects_hyphen() {
    assert!(!validate_schema_name("learning-content"));
    assert!(!validate_schema_name("my-schema"));
}

#[test]
fn schema_validation_rejects_dot() {
    assert!(!validate_schema_name("learning.content"));
    assert!(!validate_schema_name("schema.table"));
}

#[test]
fn schema_validation_rejects_special_chars() {
    assert!(!validate_schema_name("learning@content"));
    assert!(!validate_schema_name("schema#table"));
    assert!(!validate_schema_name("test$schema"));
}

#[test]
fn schema_validation_rejects_whitespace() {
    assert!(!validate_schema_name("learning content"));
    assert!(!validate_schema_name("\t\n"));
    assert!(!validate_schema_name("a b c"));
}

#[test]
fn schema_validation_trims_leading_trailing() {
    assert!(validate_schema_name("  learning"));
    assert!(validate_schema_name("learning  "));
    assert!(validate_schema_name("  learning  "));
}

#[test]
fn schema_parse_returns_typed_identifier() {
    let schema = parse_schema_name(" content ").unwrap();
    assert_eq!(schema.as_str(), "content");
    assert_eq!(schema.quoted(), "\"content\"");
}

#[test]
fn schema_parse_rejects_qualified_names() {
    assert!(parse_schema_name("content.media").is_err());
}
