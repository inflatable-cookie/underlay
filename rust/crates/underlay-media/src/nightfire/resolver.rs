use serde_json::Value;
use underlay_nightfire::{NightfireMediaLocator, NightfireValue};

use crate::domain::MediaLocatorKind;

/// Resolve one stored Nightfire media-usage locator back into the current
/// Nightfire value.
///
/// This is the inverse of the shared Nightfire extraction path:
///
/// - `NightfireMediaUsageExtractor` emits `MediaUsageEdgeInput`
/// - `sync_media_usages_for_record(...)` stores those edges
/// - later audit or UI code can resolve `locator_kind + locator_key` with
///   this helper
///
/// Supported Nightfire locator kinds:
///
/// - `MediaLocatorKind::BlockId`
/// - `MediaLocatorKind::Path`
///
/// Other locator kinds return `None` because they are not Nightfire-local
/// addresses.
pub fn resolve_nightfire_media_usage(
    value: &NightfireValue,
    locator_kind: &MediaLocatorKind,
    locator_key: &str,
) -> Option<Value> {
    match locator_kind {
        MediaLocatorKind::BlockId => NightfireMediaLocator::parse(locator_key)
            .ok()
            .and_then(|locator| locator.resolve_in_value(value).cloned()),
        MediaLocatorKind::Path => serde_json::to_value(value)
            .ok()
            .and_then(|json| json.pointer(locator_key).cloned()),
        _ => None,
    }
}
