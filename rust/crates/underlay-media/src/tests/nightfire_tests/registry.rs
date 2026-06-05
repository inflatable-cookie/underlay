use serde_json::json;
use underlay_nightfire::{BlockData, NightfireValue};
use uuid::Uuid;

use super::support::{HeroBlockHandler, MediaBlockHandler, PopupBlockHandler};
use crate::nightfire::{
    NightfireBlockMediaHandlerMap, NightfireBlockMediaHandlerRegistry,
    NightfireBlockMediaRegistration, NightfireBlockMediaUsageExtractor,
};
use crate::sync::StructuredContentMediaExtractor;
use crate::{MediaLocatorKind, MediaUsageProvenanceKind};

#[test]
fn registry_backed_extractor_walks_block_handlers_and_declared_nested_values() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        NightfireBlockMediaRegistration::new("popup", PopupBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    let extractor = NightfireBlockMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        registry,
    );

    let nested_content = NightfireValue::single(
        "test:nested@1",
        BlockData {
            id: Some("popup_media_01".to_string()),
            r#type: "media".to_string(),
            version: "initial".to_string(),
            hash: "media123".to_string(),
            data: json!({
                "mediaId": "019473c4-4a6d-7000-8000-000000000214"
            }),
        },
    );

    let value = NightfireValue::single(
        "test:schema@1",
        BlockData {
            id: Some("hero_01".to_string()),
            r#type: "hero".to_string(),
            version: "initial".to_string(),
            hash: "hero123".to_string(),
            data: json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000213",
                "children": [
                    {
                        "id": "popup_01",
                        "type": "popup",
                        "version": "initial",
                        "hash": "popup123",
                        "data": {
                            "content": serde_json::to_value(&nested_content).unwrap()
                        }
                    }
                ]
            }),
        },
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 2);

    let mut locator_keys = usages
        .iter()
        .map(|usage| usage.locator_key.as_str())
        .collect::<Vec<_>>();
    locator_keys.sort_unstable();

    assert_eq!(
        locator_keys,
        vec!["hero_01#/imageId", "popup_media_01#/mediaId"]
    );
}

#[test]
fn registry_backed_extractor_falls_back_to_outer_anchor_for_nested_child_without_ids() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("popup", PopupBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    let extractor = NightfireBlockMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        registry,
    );

    let nested_content = NightfireValue::single(
        "test:nested@1",
        BlockData {
            id: None,
            r#type: "media".to_string(),
            version: "initial".to_string(),
            hash: "media123".to_string(),
            data: json!({
                "mediaId": "019473c4-4a6d-7000-8000-000000000215"
            }),
        },
    );

    let value = NightfireValue::single(
        "test:schema@1",
        BlockData {
            id: Some("popup_01".to_string()),
            r#type: "popup".to_string(),
            version: "initial".to_string(),
            hash: "popup123".to_string(),
            data: json!({
                "content": serde_json::to_value(&nested_content).unwrap()
            }),
        },
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::BlockId);
    assert_eq!(
        usages[0].locator_key,
        "popup_01#/content/block/data/mediaId"
    );
}

#[test]
fn handler_map_accepts_block_module_registrations() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    assert!(registry.handler_for("hero").is_some());
    assert!(registry.handler_for("media").is_some());
    assert!(registry.handler_for("popup").is_none());
}

#[test]
fn handler_map_accepts_generic_block_registrations() {
    #[derive(Clone, Copy)]
    enum TestCategory {
        Content,
    }

    let registry = NightfireBlockMediaHandlerMap::from_block_registrations([
        underlay_nightfire::BlockRegistration::new(
            underlay_nightfire::BlockDescriptor {
                type_name: "hero",
                label: "Hero",
                category: TestCategory::Content,
            },
            NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        ),
    ]);

    assert!(registry.handler_for("hero").is_some());
}
