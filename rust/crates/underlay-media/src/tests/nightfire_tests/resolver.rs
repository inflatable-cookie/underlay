use serde_json::json;
use underlay_nightfire::NightfireValue;

use super::support::block;
use crate::nightfire::resolve_nightfire_media_usage;
use crate::MediaLocatorKind;

#[test]
fn resolve_nightfire_media_usage_reads_block_id_locator_values() {
    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000210",
                "children": [
                    {
                        "id": "gallery_02",
                        "type": "gallery",
                        "version": "initial",

                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000211" }]
                        }
                    }
                ]
            }),
        ),
    );

    let top_level =
        resolve_nightfire_media_usage(&value, &MediaLocatorKind::BlockId, "hero_01#/imageId");
    let nested = resolve_nightfire_media_usage(
        &value,
        &MediaLocatorKind::BlockId,
        "gallery_02#/pages/0/imageId",
    );

    assert_eq!(
        top_level,
        Some(json!("019473c4-4a6d-7000-8000-000000000210"))
    );
    assert_eq!(nested, Some(json!("019473c4-4a6d-7000-8000-000000000211")));
}

#[test]
fn resolve_nightfire_media_usage_reads_path_fallback_values() {
    let value = NightfireValue::multi(
        "test:schema@1",
        vec![block(
            None,
            json!({
                "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000212" }]
            }),
        )],
    );

    let resolved = resolve_nightfire_media_usage(
        &value,
        &MediaLocatorKind::Path,
        "/blocks/0/data/pages/0/imageId",
    );

    assert_eq!(
        resolved,
        Some(json!("019473c4-4a6d-7000-8000-000000000212"))
    );
}
