use super::*;

#[test]
fn valid_result_serializes_correctly() {
    let result = FieldValidationResult::valid();
    let json = serde_json::to_string(&result).unwrap();
    assert_eq!(json, r#"{"valid":true}"#);
}

#[test]
fn invalid_result_serializes_correctly() {
    let result = FieldValidationResult::invalid("Slug already exists");
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains(r#""valid":false"#));
    assert!(json.contains(r#""message":"Slug already exists""#));
}

#[test]
fn invalid_with_suggestion_serializes_correctly() {
    let result =
        FieldValidationResult::invalid_with_suggestion("Slug already exists", "try-this-slug");
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains(r#""suggestion":"try-this-slug""#));
}

#[test]
fn parse_uuid_valid() {
    let uuid_str = "01933f9a-7b1e-7c9f-8f3d-1a2b3c4d5e6f";
    let result = parse_uuid_for_validation(uuid_str, "moduleId");
    assert!(result.is_ok());
}

#[test]
fn parse_uuid_invalid() {
    let result = parse_uuid_for_validation("not-a-uuid", "moduleId");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.valid);
    assert!(err.message.unwrap().contains("moduleId"));
}

#[test]
fn parse_optional_uuid_none() {
    let result = parse_optional_uuid_for_validation(None, "excludeId");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn parse_optional_uuid_valid() {
    let uuid_str = "01933f9a-7b1e-7c9f-8f3d-1a2b3c4d5e6f";
    let result = parse_optional_uuid_for_validation(Some(uuid_str), "excludeId");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn parse_optional_uuid_invalid() {
    let result = parse_optional_uuid_for_validation(Some("bad"), "excludeId");
    assert!(result.is_err());
}
