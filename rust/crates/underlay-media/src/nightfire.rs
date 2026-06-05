//! Shared Nightfire media-usage extraction helpers.

mod registry;
mod walk;

pub use registry::{
    NightfireBlockMediaHandler, NightfireBlockMediaHandlerMap, NightfireBlockMediaHandlerRegistry,
    NightfireBlockMediaReference, NightfireBlockMediaRegistration, NightfireNestedValuePointer,
};

use serde_json::Value;
use underlay_nightfire::{BlockData, NightfireMediaLocator, NightfireValue};
use uuid::Uuid;

use crate::domain::{
    MediaId, MediaLocatorKind, MediaUsageEdgeInput, MediaUsageProvenanceKind, MediaUsageRole,
};
use crate::error::{MediaError, MediaResult};
use crate::sync::{sync_media_usages_for_record, MediaUsageSyncReport, MediaUsageSyncRepository};
use crate::sync::{StructuredContentMediaExtractor, StructuredContentWalker};

use self::walk::{normalize_relative_pointer, BlockAnchor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireMediaReferenceMatch {
    pub media_id: MediaId,
    pub usage_role: MediaUsageRole,
}

pub struct NightfireMediaVisitContext<'a> {
    pub block: &'a BlockData,
    pub data_pointer: &'a str,
    pub rooted_pointer: &'a str,
}

impl<'a> NightfireMediaVisitContext<'a> {
    pub fn block_type(&self) -> &str {
        &self.block.r#type
    }

    pub fn block_id(&self) -> Option<&str> {
        self.block.id.as_deref()
    }

    pub fn block_data(&self) -> &Value {
        &self.block.data
    }

    pub fn resolve_relative_pointer(&self, pointer: &str) -> Option<&Value> {
        let normalized = normalize_relative_pointer(pointer);
        if normalized.is_empty() {
            return Some(self.block_data());
        }

        self.block_data().pointer(&normalized)
    }
}

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

pub trait NightfireMediaReferenceMatcher: Send + Sync {
    fn match_media_reference(
        &self,
        context: &NightfireMediaVisitContext<'_>,
        value: &Value,
    ) -> MediaResult<Option<NightfireMediaReferenceMatch>>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireMediaFieldRule {
    pub field_name: String,
    pub usage_role: MediaUsageRole,
}

impl NightfireMediaFieldRule {
    pub fn new(field_name: impl Into<String>, usage_role: MediaUsageRole) -> Self {
        Self {
            field_name: field_name.into(),
            usage_role,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NightfireFieldNameMatcher {
    rules: Vec<NightfireMediaFieldRule>,
}

impl NightfireFieldNameMatcher {
    pub fn new(rules: Vec<NightfireMediaFieldRule>) -> Self {
        Self { rules }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_rule(mut self, rule: NightfireMediaFieldRule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn with_field(self, field_name: impl Into<String>, usage_role: MediaUsageRole) -> Self {
        self.with_rule(NightfireMediaFieldRule::new(field_name, usage_role))
    }

    pub fn with_common_media_fields() -> Self {
        Self::empty()
            .with_field("imageId", MediaUsageRole::Embedded)
            .with_field("mediaId", MediaUsageRole::Embedded)
            .with_field("iconMediaId", MediaUsageRole::Primary)
            .with_field("fileId", MediaUsageRole::Attachment)
            .with_field("attachmentId", MediaUsageRole::Attachment)
    }

    pub fn rules(&self) -> &[NightfireMediaFieldRule] {
        &self.rules
    }
}

impl NightfireMediaReferenceMatcher for NightfireFieldNameMatcher {
    fn match_media_reference(
        &self,
        context: &NightfireMediaVisitContext<'_>,
        value: &Value,
    ) -> MediaResult<Option<NightfireMediaReferenceMatch>> {
        let media_id = match value {
            Value::String(raw) => Uuid::parse_str(raw).ok().map(MediaId),
            _ => None,
        };

        let Some(media_id) = media_id else {
            return Ok(None);
        };

        let field_name = context
            .data_pointer
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty());

        let Some(field_name) = field_name else {
            return Ok(None);
        };

        let Some(rule) = self.rules.iter().find(|rule| rule.field_name == field_name) else {
            return Ok(None);
        };

        Ok(Some(NightfireMediaReferenceMatch {
            media_id,
            usage_role: rule.usage_role.clone(),
        }))
    }
}

pub struct NightfireMediaUsageExtractor<M> {
    used_by_type: String,
    used_by_id: Option<Uuid>,
    owner_field: String,
    provenance_kind: MediaUsageProvenanceKind,
    matcher: M,
}

pub struct NightfireBlockMediaUsageExtractor<R> {
    used_by_type: String,
    used_by_id: Option<Uuid>,
    owner_field: String,
    provenance_kind: MediaUsageProvenanceKind,
    registry: R,
}

impl<R> NightfireBlockMediaUsageExtractor<R> {
    pub fn new(
        used_by_type: impl Into<String>,
        used_by_id: Option<Uuid>,
        owner_field: impl Into<String>,
        provenance_kind: MediaUsageProvenanceKind,
        registry: R,
    ) -> Self {
        Self {
            used_by_type: used_by_type.into(),
            used_by_id,
            owner_field: owner_field.into(),
            provenance_kind,
            registry,
        }
    }

    pub async fn extract_and_sync<S>(
        &self,
        repo: &S,
        value: &NightfireValue,
    ) -> MediaResult<MediaUsageSyncReport>
    where
        S: MediaUsageSyncRepository,
        R: NightfireBlockMediaHandlerRegistry,
    {
        let Some(used_by_id) = self.used_by_id else {
            return Err(MediaError::validation(
                "Nightfire record sync requires a persisted used_by_id".to_string(),
            ));
        };

        let desired = self.extract_media_usages(&self.owner_field, value)?;

        sync_media_usages_for_record(
            repo,
            &self.used_by_type,
            used_by_id,
            &desired,
            &self.provenance_kind,
        )
        .await
    }
}

impl<M> NightfireMediaUsageExtractor<M> {
    pub fn new(
        used_by_type: impl Into<String>,
        used_by_id: Option<Uuid>,
        owner_field: impl Into<String>,
        provenance_kind: MediaUsageProvenanceKind,
        matcher: M,
    ) -> Self {
        Self {
            used_by_type: used_by_type.into(),
            used_by_id,
            owner_field: owner_field.into(),
            provenance_kind,
            matcher,
        }
    }

    /// Extract desired usage edges for one persisted Nightfire-bearing record
    /// and reconcile them through the shared usage-sync path.
    ///
    /// Recommended consumer flow:
    ///
    /// 1. persist a `NightfireValue` with stable block ids
    /// 2. call `extract_and_sync(...)`
    /// 3. later resolve stored locator rows with
    ///    `resolve_nightfire_media_usage(...)`
    pub async fn extract_and_sync<R>(
        &self,
        repo: &R,
        value: &NightfireValue,
    ) -> MediaResult<MediaUsageSyncReport>
    where
        R: MediaUsageSyncRepository,
        M: NightfireMediaReferenceMatcher,
    {
        let Some(used_by_id) = self.used_by_id else {
            return Err(MediaError::validation(
                "Nightfire record sync requires a persisted used_by_id".to_string(),
            ));
        };

        let desired = self.extract_media_usages(&self.owner_field, value)?;

        sync_media_usages_for_record(
            repo,
            &self.used_by_type,
            used_by_id,
            &desired,
            &self.provenance_kind,
        )
        .await
    }
}

impl<M> StructuredContentMediaExtractor<NightfireValue> for NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    fn extract_media_usages(
        &self,
        owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        if owner_field != self.owner_field {
            return Err(MediaError::validation(format!(
                "Nightfire extractor owner_field mismatch: expected {}, got {}",
                self.owner_field, owner_field
            )));
        }

        self.walk_media_usages(owner_field, value)
    }
}

impl<R> StructuredContentMediaExtractor<NightfireValue> for NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    fn extract_media_usages(
        &self,
        owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        if owner_field != self.owner_field {
            return Err(MediaError::validation(format!(
                "Nightfire extractor owner_field mismatch: expected {}, got {}",
                self.owner_field, owner_field
            )));
        }

        self.walk_media_usages(owner_field, value)
    }
}

impl<M> StructuredContentWalker<NightfireValue> for NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    fn walk_media_usages(
        &self,
        _owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        let mut edges = Vec::new();

        if let Some(block) = value.block.as_ref() {
            let anchor = BlockAnchor::from_block(block, "/block/data".to_string());
            self.walk_block(block, anchor, "/block/data", &block.data, &mut edges)?;
        }

        if let Some(blocks) = value.blocks.as_ref() {
            for (index, block) in blocks.iter().enumerate() {
                let rooted_pointer = format!("/blocks/{index}/data");
                let anchor = BlockAnchor::from_block(block, rooted_pointer.clone());
                self.walk_block(block, anchor, &rooted_pointer, &block.data, &mut edges)?;
            }
        }

        Ok(edges)
    }
}

impl<R> StructuredContentWalker<NightfireValue> for NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    fn walk_media_usages(
        &self,
        _owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        let mut edges = Vec::new();
        self.walk_root_value_at(value, "", None, &mut edges)?;
        Ok(edges)
    }
}

#[cfg(test)]
#[path = "tests/nightfire_tests.rs"]
mod tests;
