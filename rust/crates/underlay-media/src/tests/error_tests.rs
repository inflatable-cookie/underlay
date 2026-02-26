use super::*;

#[test]
fn test_error_display() {
    let err = MediaError::not_found("abc-123");
    assert_eq!(err.to_string(), "Media not found: abc-123");

    let err = MediaError::InUse(5);
    assert_eq!(err.to_string(), "Media is in use by 5 entities");
}
