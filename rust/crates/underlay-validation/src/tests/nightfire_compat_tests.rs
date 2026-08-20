use super::*;
use underlay_nightfire::StrategyCardinality;

#[test]
fn converts_cardinality_mismatch() {
    let err = NightfireValidationError::CardinalityMismatch {
        schema: "test:schema".to_string(),
        expected: StrategyCardinality::Single,
        actual_blocks: 3,
    };

    let app_err = nightfire_validation_to_app_error(
        err,
        "content.invalid",
        "body",
        "Content validation failed.",
    );

    assert_eq!(app_err.code, "content.invalid");
    assert_eq!(app_err.message, "Content validation failed.");
    assert!(app_err.field_errors.is_some());
    let field_errors = app_err.field_errors.unwrap();
    assert!(field_errors.contains_key("body"));
}

#[test]
fn converts_disallowed_block_type() {
    let err = NightfireValidationError::DisallowedBlockType {
        schema: "test:schema".to_string(),
        block_type: "forbidden.block".to_string(),
    };

    let app_err = nightfire_validation_to_app_error(
        err,
        "content.invalid",
        "body",
        "Content validation failed.",
    );

    let field_errors = app_err.field_errors.unwrap();
    assert!(field_errors
        .get("body")
        .unwrap()
        .contains("forbidden.block"));
}

#[test]
fn converts_unknown_block_type() {
    let err = NightfireValidationError::UnknownBlockType {
        schema: "test:schema".to_string(),
        block_type: "mystery.block".to_string(),
    };

    let app_err = nightfire_validation_to_app_error(
        err,
        "content.invalid",
        "body",
        "Content validation failed.",
    );

    let field_errors = app_err.field_errors.unwrap();
    assert!(field_errors.get("body").unwrap().contains("mystery.block"));
}

#[test]
fn converts_unknown_block_version() {
    let err = NightfireValidationError::UnknownBlockVersion {
        schema: "test:schema".to_string(),
        block_type: "callout".to_string(),
        version: "9".to_string(),
    };

    let app_err = nightfire_validation_to_app_error(
        err,
        "content.invalid",
        "body",
        "Content validation failed.",
    );

    let field_errors = app_err.field_errors.unwrap();
    let detail = field_errors.get("body").unwrap();
    assert!(detail.contains("callout"));
    assert!(detail.contains("9"));
}

#[test]
fn converts_unknown_strategy() {
    let err = NightfireValidationError::UnknownStrategy {
        schema: "unknown:schema".to_string(),
    };

    let app_err = nightfire_validation_to_app_error(
        err,
        "content.invalid",
        "body",
        "Content validation failed.",
    );

    let field_errors = app_err.field_errors.unwrap();
    assert!(field_errors.get("body").unwrap().contains("unknown:schema"));
}
