use super::*;
use underlay_nightfire::StrategyCardinality;

#[test]
fn converts_cardinality_mismatch() {
    let err = NightfireValidationError::CardinalityMismatch {
        schema: "test:schema@1".to_string(),
        expected: StrategyCardinality::Single,
        actual_blocks: 3,
        is_single: false,
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
        schema: "test:schema@1".to_string(),
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
        schema: "test:schema@1".to_string(),
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
fn converts_unknown_strategy() {
    let err = NightfireValidationError::UnknownStrategy {
        schema: "unknown:schema@1".to_string(),
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
        .contains("unknown:schema@1"));
}
