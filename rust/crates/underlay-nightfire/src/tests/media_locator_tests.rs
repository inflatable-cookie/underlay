use crate::{BlockData, NightfireMediaLocator, NightfireValue};
use serde_json::json;

fn block(id: &str, data: serde_json::Value) -> BlockData {
    BlockData {
        id: id.to_string(),
        r#type: "test".to_string(),
        version: "initial".to_string(),
        data,
    }
}

#[test]
fn parses_and_formats_locator_key() {
    let locator =
        NightfireMediaLocator::parse("hero_01#/pages/1/image_id").expect("locator should parse");

    assert_eq!(locator.block_id, "hero_01");
    assert_eq!(locator.data_pointer, "/pages/1/image_id");
    assert_eq!(locator.to_locator_key(), "hero_01#/pages/1/image_id");
}

#[test]
fn rejects_locator_without_separator() {
    let error =
        NightfireMediaLocator::parse("hero_01:/pages/1/image_id").expect_err("locator should fail");

    assert!(matches!(
        error,
        crate::media_locator::NightfireMediaLocatorError::MissingSeparator
    ));
}

#[test]
fn rejects_locator_with_invalid_pointer() {
    let error =
        NightfireMediaLocator::new("hero_01", "pages/1/image_id").expect_err("locator should fail");

    assert!(matches!(
        error,
        crate::media_locator::NightfireMediaLocatorError::InvalidDataPointer(_)
    ));
}

#[test]
fn resolves_pointer_inside_single_block_value() {
    let value = NightfireValue::single(
        "test:schema",
        block(
            "hero_01",
            json!({ "image_id": "media-1", "caption": "Hero" }),
        ),
    );

    let locator = NightfireMediaLocator::parse("hero_01#/image_id").unwrap();
    let resolved = locator
        .resolve_in_value(&value)
        .expect("reference should resolve");

    assert_eq!(resolved, &json!("media-1"));
}

#[test]
fn resolves_pointer_inside_multi_block_value() {
    let value = NightfireValue::multi(
        "test:schema",
        vec![
            block("intro_01", json!({ "text": "Hello" })),
            block(
                "gallery_02",
                json!({ "pages": [{ "image_id": "media-1" }, { "image_id": "media-2" }] }),
            ),
        ],
    );

    let locator = NightfireMediaLocator::parse("gallery_02#/pages/1/image_id").unwrap();
    let resolved = locator
        .resolve_in_value(&value)
        .expect("reference should resolve");

    assert_eq!(resolved, &json!("media-2"));
}

#[test]
fn resolves_pointer_inside_nested_block_id() {
    let value = NightfireValue::single(
        "test:schema",
        block(
            "hero_01",
            json!({
                "children": [
                    {
                        "id": "gallery_02",
                        "type": "gallery",
                        "version": "initial",
                        "data": {
                            "pages": [{ "image_id": "media-1" }, { "image_id": "media-2" }]
                        }
                    }
                ]
            }),
        ),
    );

    let locator = NightfireMediaLocator::parse("gallery_02#/pages/1/image_id").unwrap();
    let resolved = locator
        .resolve_in_value(&value)
        .expect("reference should resolve");

    assert_eq!(resolved, &json!("media-2"));
}

#[test]
fn returns_none_when_block_or_path_does_not_exist() {
    let value = NightfireValue::single(
        "test:schema",
        block("hero_01", json!({ "image_id": "media-1" })),
    );

    let missing_block = NightfireMediaLocator::parse("gallery_02#/image_id").unwrap();
    let missing_path = NightfireMediaLocator::parse("hero_01#/pages/1/image_id").unwrap();

    assert!(missing_block.resolve_in_value(&value).is_none());
    assert!(missing_path.resolve_in_value(&value).is_none());
}
