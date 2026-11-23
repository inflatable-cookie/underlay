use super::*;

#[test]
fn test_email() {
    assert!(email("user@example.com").is_ok());
    assert!(email("user.name+tag@example.co.uk").is_ok());
    assert!(email("user@a-b.example.com").is_ok());
    assert!(email("user@sub.example-site.io").is_ok());
    assert!(email("not-an-email").is_err());
    assert!(email("@example.com").is_err());
    assert!(email("user@").is_err());
}

#[test]
fn test_email_rejects_malformed_domain_labels() {
    assert!(email("user@..com").is_err());
    assert!(email("user@example..com").is_err());
    assert!(email("user@.example.com").is_err());
    assert!(email("user@-example.com").is_err());
    assert!(email("user@example-.com").is_err());
    assert!(email("user@example.-com").is_err());
    assert!(email("user@example.com-").is_err());
    assert!(email("user@example.c").is_err());
    assert!(email("user@example.c0m").is_err());
}

#[test]
fn test_url() {
    assert!(url("https://example.com").is_ok());
    assert!(url("http://sub.example.com/path").is_ok());
    assert!(url("not-a-url").is_err());
    assert!(url("ftp://example.com").is_err());
}

#[test]
fn test_uuid() {
    assert!(uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    assert!(uuid("550e8400e29b41d4a716446655440000").is_ok());
    assert!(uuid("not-a-uuid").is_err());
}

#[test]
fn test_length() {
    assert!(length("hello", Some(1), Some(10)).is_ok());
    assert!(length("hi", Some(3), None).is_err());
    assert!(length("hello world", None, Some(5)).is_err());
    // Unicode support
    assert!(length("日本語", Some(1), Some(5)).is_ok());
}

#[test]
fn test_required() {
    assert!(required("value").is_ok());
    assert!(required("").is_err());
    assert!(required("   ").is_err());
}

#[test]
fn test_range() {
    assert!(range(5, Some(1), Some(10)).is_ok());
    assert!(range(0, Some(1), None).is_err());
    assert!(range(15, None, Some(10)).is_err());
    assert!(range(5.5, Some(1.0), Some(10.0)).is_ok());
}

#[test]
fn test_positive() {
    assert!(positive(5).is_ok());
    assert!(positive(0).is_err());
    assert!(positive(-1).is_err());
}

#[test]
fn test_non_negative() {
    assert!(non_negative(0).is_ok());
    assert!(non_negative(5).is_ok());
    assert!(non_negative(-1).is_err());
}

#[test]
fn test_pattern() {
    assert!(pattern("ABC123", r"^[A-Z]+\d+$", "Invalid").is_ok());
    assert!(pattern("abc123", r"^[A-Z]+\d+$", "Invalid").is_err());
}

#[test]
fn test_pattern_invalid_regex_carries_structured_code() {
    let err = pattern("anything", "(", "Invalid").expect_err("invalid regex should fail");
    assert_eq!(err.code.as_deref(), Some("pattern.invalid_regex"));

    let mismatch = pattern("abc", r"^\d+$", "Must be digits").expect_err("mismatch should fail");
    assert_eq!(mismatch.code.as_deref(), Some("pattern.invalid"));
}

#[test]
fn test_one_of() {
    assert!(one_of("active", &["active", "inactive"]).is_ok());
    assert!(one_of("unknown", &["active", "inactive"]).is_err());
    assert!(one_of(1, &[1, 2, 3]).is_ok());
}

#[test]
fn test_not_empty() {
    assert!(not_empty(&[1, 2, 3]).is_ok());
    assert!(not_empty(&Vec::<i32>::new()).is_err());
}

#[test]
fn test_collection_length() {
    assert!(collection_length(&[1, 2, 3], Some(1), Some(5)).is_ok());
    assert!(collection_length(&[1], Some(2), None).is_err());
    assert!(collection_length(&[1, 2, 3, 4, 5, 6], None, Some(5)).is_err());
}

#[test]
fn test_alphanumeric() {
    assert!(alphanumeric("abc123").is_ok());
    assert!(alphanumeric("ABC123").is_ok());
    assert!(alphanumeric("abc-123").is_err());
    assert!(alphanumeric("abc 123").is_err());
}

#[test]
fn test_username() {
    assert!(username("john_doe").is_ok());
    assert!(username("john-doe-123").is_ok());
    assert!(username("john@doe").is_err());
    assert!(username("john doe").is_err());
}

#[test]
fn test_slug() {
    assert!(slug("my-article").is_ok());
    assert!(slug("article123").is_ok());
    assert!(slug("ab").is_ok());
    assert!(slug("a").is_err());
    assert!(slug(&"a".repeat(101)).is_err());
    assert!(slug("My-Article").is_err());
    assert!(slug("my--article").is_err());
    assert!(slug("-article").is_err());
}

#[test]
fn test_unique_items() {
    assert!(unique_items(&["a", "b", "c"]).is_ok());
    assert!(unique_items(&[1, 2, 3]).is_ok());
    assert!(unique_items(&["a", "b", "a"]).is_err());
    assert!(unique_items(&[1, 2, 1]).is_err());
    assert!(unique_items(&Vec::<i32>::new()).is_ok());
}

#[test]
fn test_unique_items_detailed() {
    assert!(unique_items_detailed(&["a", "b", "c"]).is_ok());
    let err = unique_items_detailed(&["a", "b", "a", "c", "a"]).unwrap_err();
    assert!(err.message.contains("2 duplicate"));
}

#[test]
fn test_range_rejects_nan() {
    assert!(range(f64::NAN, Some(0.0), Some(10.0)).is_err());
    assert!(range(f64::NAN, Some(0.0), None).is_err());
    assert!(range(f64::NAN, None, Some(10.0)).is_err());
    assert!(range(f64::NAN, None, None).is_err());
    assert!(range(f32::NAN, Some(0.0_f32), Some(10.0_f32)).is_err());

    let err = range(f64::NAN, Some(0.0), Some(10.0)).expect_err("NaN must not pass range");
    assert_eq!(err.code.as_deref(), Some("range.nan"));
}

#[test]
fn test_pattern_reuses_cached_compilation() {
    assert!(pattern("ABC123", r"^[A-Z]+\d+$", "Invalid").is_ok());
    assert!(pattern("ABCDEF456", r"^[A-Z]+\d+$", "Invalid").is_ok());
    assert!(pattern("123ABC", r"^[A-Z]+\d+$", "Invalid").is_err());
    assert!(pattern("anything", "(", "Invalid").is_err());
    assert!(pattern("anything", "(", "Invalid").is_err());
}

#[test]
fn test_pattern_cache_is_bounded() {
    let capacity = super::pattern::pattern_cache_capacity();
    for index in 0..capacity * 2 {
        let candidate = format!(r"^cached-{index}$");
        assert!(pattern("no-match", &candidate, "Invalid").is_err());
    }

    assert!(super::pattern::cached_pattern_count() <= capacity);
}
