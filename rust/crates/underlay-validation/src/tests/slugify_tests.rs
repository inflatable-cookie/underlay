use super::*;

#[cfg(feature = "slugify")]
#[test]
fn test_slugify_basic() {
    assert_eq!(slugify("Hello World"), "hello-world");
    assert_eq!(slugify("Hello, World!"), "hello-world");
    assert_eq!(slugify("  spaces  "), "spaces");
}

#[cfg(feature = "slugify")]
#[test]
fn test_slugify_numbers() {
    assert_eq!(slugify("FA1 2024"), "fa1-2024");
    assert_eq!(slugify("Module 123"), "module-123");
}

#[cfg(feature = "slugify")]
#[test]
fn test_slugify_unicode() {
    assert_eq!(slugify("Über Café"), "uber-cafe");
    assert_eq!(slugify("日本語"), "ri-ben-yu");
}

#[test]
fn test_valid_slug_format() {
    assert!(is_valid_slug_format("hello-world"));
    assert!(is_valid_slug_format("fa1-2024"));
    assert!(is_valid_slug_format("ab"));
    assert!(!is_valid_slug_format("a")); // too short
    assert!(!is_valid_slug_format("Hello-World")); // uppercase
    assert!(!is_valid_slug_format("hello--world")); // double hyphen
    assert!(!is_valid_slug_format("-hello")); // leading hyphen
    assert!(!is_valid_slug_format("hello-")); // trailing hyphen
}

#[test]
fn test_reserved_slugs() {
    assert!(is_reserved_slug("new"));
    assert!(is_reserved_slug("edit"));
    assert!(is_reserved_slug("admin"));
    assert!(!is_reserved_slug("hello-world"));
}

#[test]
fn test_validate_slug() {
    assert!(validate_slug("hello-world").is_ok());
    assert_eq!(validate_slug("a"), Err(SlugValidationError::TooShort));
    assert_eq!(validate_slug("new"), Err(SlugValidationError::Reserved));
    assert_eq!(
        validate_slug("Hello-World"),
        Err(SlugValidationError::InvalidFormat)
    );
}
