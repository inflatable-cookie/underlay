use serde_json::json;
use underlay_nightfire::NightfireValue;
use uuid::Uuid;

use super::support::{block, matcher};
use crate::nightfire::{NightfireFieldNameMatcher, NightfireMediaUsageExtractor};
use crate::sync::StructuredContentMediaExtractor;
use crate::{MediaLocatorKind, MediaUsageProvenanceKind};

#[test]
fn extracts_block_id_locators_for_top_level_and_nested_block_ids() {
    let used_by_id = Uuid::now_v7();
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(used_by_id),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000010",
                "children": [
                    {
                        "id": "gallery_02",
                        "type": "gallery",
                        "version": "initial",
                        "hash": "def456",
                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000011" }]
                        }
                    }
                ]
            }),
        ),
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 2);
    assert!(usages
        .iter()
        .all(|usage| usage.locator_kind == MediaLocatorKind::BlockId));

    let mut locator_keys = usages
        .iter()
        .map(|usage| usage.locator_key.as_str())
        .collect::<Vec<_>>();
    locator_keys.sort_unstable();

    assert_eq!(
        locator_keys,
        vec!["gallery_02#/pages/0/imageId", "hero_01#/imageId"]
    );
}

#[test]
fn falls_back_to_ancestor_block_pointer_when_nested_block_has_no_id() {
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "children": [
                    {
                        "type": "gallery",
                        "version": "initial",
                        "hash": "def456",
                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000012" }]
                        }
                    }
                ]
            }),
        ),
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::BlockId);
    assert_eq!(
        usages[0].locator_key,
        "hero_01#/children/0/data/pages/0/imageId"
    );
}

#[test]
fn falls_back_to_rooted_path_when_top_level_block_has_no_id() {
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::multi(
        "test:schema@1",
        vec![block(
            None,
            json!({
                "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000013" }]
            }),
        )],
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::Path);
    assert_eq!(usages[0].locator_key, "/blocks/0/data/pages/0/imageId");
}

#[test]
fn common_field_matcher_covers_default_media_reference_names() {
    let matcher = NightfireFieldNameMatcher::with_common_media_fields();
    let rules = matcher
        .rules()
        .iter()
        .map(|rule| (rule.field_name.as_str(), rule.usage_role.as_str()))
        .collect::<Vec<_>>();

    assert!(rules.contains(&("imageId", "embedded")));
    assert!(rules.contains(&("mediaId", "embedded")));
    assert!(rules.contains(&("iconMediaId", "primary")));
    assert!(rules.contains(&("fileId", "attachment")));
    assert!(rules.contains(&("attachmentId", "attachment")));
}
