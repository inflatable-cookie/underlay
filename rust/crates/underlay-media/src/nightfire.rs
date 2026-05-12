//! Shared Nightfire media-usage extraction helpers.

use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value;
use underlay_nightfire::{BlockData, BlockRegistration, NightfireMediaLocator, NightfireValue};
use uuid::Uuid;

use crate::domain::{
    MediaContentKind, MediaId, MediaLocatorKind, MediaUsageEdgeInput, MediaUsageProvenanceKind,
    MediaUsageRole,
};
use crate::error::{MediaError, MediaResult};
use crate::sync::{sync_media_usages_for_record, MediaUsageSyncReport, MediaUsageSyncRepository};
use crate::sync::{StructuredContentMediaExtractor, StructuredContentWalker};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireBlockMediaReference {
    pub media_id: MediaId,
    pub usage_role: MediaUsageRole,
    /// JSON Pointer relative to `block.data`.
    pub data_pointer: String,
}

impl NightfireBlockMediaReference {
    pub fn new(
        media_id: MediaId,
        usage_role: MediaUsageRole,
        data_pointer: impl Into<String>,
    ) -> Self {
        Self {
            media_id,
            usage_role,
            data_pointer: data_pointer.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NightfireNestedValuePointer {
    /// JSON Pointer relative to `block.data`.
    pub data_pointer: String,
}

impl NightfireNestedValuePointer {
    pub fn new(data_pointer: impl Into<String>) -> Self {
        Self {
            data_pointer: data_pointer.into(),
        }
    }
}

pub trait NightfireBlockMediaHandler: Send + Sync {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireBlockMediaReference>>;

    fn nested_nightfire_values(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireNestedValuePointer>> {
        Ok(Vec::new())
    }
}

pub trait NightfireBlockMediaHandlerRegistry: Send + Sync {
    fn handler_for(&self, block_type: &str) -> Option<&dyn NightfireBlockMediaHandler>;
}

#[derive(Clone)]
pub struct NightfireBlockMediaRegistration {
    pub block_type: String,
    pub handler: Arc<dyn NightfireBlockMediaHandler>,
}

impl NightfireBlockMediaRegistration {
    pub fn new(
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) -> Self {
        Self {
            block_type: block_type.into(),
            handler: Arc::new(handler),
        }
    }
}

#[derive(Default)]
pub struct NightfireBlockMediaHandlerMap {
    handlers: BTreeMap<String, Arc<dyn NightfireBlockMediaHandler>>,
}

impl NightfireBlockMediaHandlerMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_registrations(
        registrations: impl IntoIterator<Item = NightfireBlockMediaRegistration>,
    ) -> Self {
        let mut map = Self::new();
        map.extend_registrations(registrations);
        map
    }

    pub fn from_block_registrations<C>(
        registrations: impl IntoIterator<Item = BlockRegistration<C, NightfireBlockMediaRegistration>>,
    ) -> Self {
        let mut map = Self::new();
        map.extend_block_registrations(registrations);
        map
    }

    pub fn with_handler(
        mut self,
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) -> Self {
        self.register(block_type, handler);
        self
    }

    pub fn register(
        &mut self,
        block_type: impl Into<String>,
        handler: impl NightfireBlockMediaHandler + 'static,
    ) {
        self.handlers.insert(block_type.into(), Arc::new(handler));
    }

    pub fn with_registration(mut self, registration: NightfireBlockMediaRegistration) -> Self {
        self.register_registration(registration);
        self
    }

    pub fn register_registration(&mut self, registration: NightfireBlockMediaRegistration) {
        self.handlers
            .insert(registration.block_type, registration.handler);
    }

    pub fn extend_registrations(
        &mut self,
        registrations: impl IntoIterator<Item = NightfireBlockMediaRegistration>,
    ) {
        for registration in registrations {
            self.register_registration(registration);
        }
    }

    pub fn register_block_registration<C>(
        &mut self,
        registration: BlockRegistration<C, NightfireBlockMediaRegistration>,
    ) {
        let (_descriptor, metadata) = registration.into_parts();
        self.register_registration(metadata);
    }

    pub fn extend_block_registrations<C>(
        &mut self,
        registrations: impl IntoIterator<Item = BlockRegistration<C, NightfireBlockMediaRegistration>>,
    ) {
        for registration in registrations {
            self.register_block_registration(registration);
        }
    }
}

impl NightfireBlockMediaHandlerRegistry for NightfireBlockMediaHandlerMap {
    fn handler_for(&self, block_type: &str) -> Option<&dyn NightfireBlockMediaHandler> {
        self.handlers
            .get(block_type)
            .map(|handler| handler.as_ref() as &dyn NightfireBlockMediaHandler)
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

#[derive(Clone, Debug)]
struct BlockAnchor {
    block_id: Option<String>,
    rooted_data_pointer: String,
}

impl BlockAnchor {
    fn from_block(block: &BlockData, rooted_data_pointer: String) -> Self {
        Self {
            block_id: block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
            rooted_data_pointer,
        }
    }

    fn locator_for(&self, rooted_pointer: &str) -> MediaResult<(MediaLocatorKind, String)> {
        if let Some(block_id) = self.block_id.as_deref() {
            let relative_pointer = rooted_pointer
                .strip_prefix(&self.rooted_data_pointer)
                .unwrap_or(rooted_pointer);
            let locator =
                NightfireMediaLocator::new(block_id, relative_pointer).map_err(|err| {
                    MediaError::validation(format!("invalid Nightfire media locator: {err:?}"))
                })?;
            return Ok((MediaLocatorKind::BlockId, locator.to_locator_key()));
        }

        Ok((MediaLocatorKind::Path, rooted_pointer.to_string()))
    }
}

impl<M> NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    fn walk_block(
        &self,
        block: &BlockData,
        anchor: BlockAnchor,
        data_pointer: &str,
        value: &Value,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        let relative_pointer = data_pointer
            .strip_prefix(&anchor.rooted_data_pointer)
            .unwrap_or(data_pointer);

        let context = NightfireMediaVisitContext {
            block,
            data_pointer: relative_pointer,
            rooted_pointer: data_pointer,
        };

        if let Some(media_ref) = self.matcher.match_media_reference(&context, value)? {
            let (locator_kind, locator_key) = anchor.locator_for(data_pointer)?;
            edges.push(MediaUsageEdgeInput {
                media_id: media_ref.media_id,
                used_by_type: self.used_by_type.clone(),
                used_by_id: self.used_by_id,
                owner_field: Some(self.owner_field.clone()),
                content_kind: MediaContentKind::StructuredContent,
                locator_kind,
                locator_key,
                usage_role: media_ref.usage_role,
                provenance_kind: self.provenance_kind.clone(),
            });
        }

        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    let pointer = push_pointer_segment(data_pointer, &index.to_string());
                    self.walk_block(block, anchor.clone(), &pointer, item, edges)?;
                }
            }
            Value::Object(map) => {
                if let Some(nested_block) = as_nested_block(value) {
                    let nested_data_pointer = push_pointer_segment(data_pointer, "data");
                    let nested_anchor = nested_block
                        .id
                        .as_deref()
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(|_| {
                            BlockAnchor::from_block(&nested_block, nested_data_pointer.clone())
                        })
                        .unwrap_or_else(|| anchor.clone());

                    self.walk_block(
                        &nested_block,
                        nested_anchor,
                        &nested_data_pointer,
                        &nested_block.data,
                        edges,
                    )?;
                    return Ok(());
                }

                for (key, child) in map {
                    let pointer = push_pointer_segment(data_pointer, key);
                    self.walk_block(block, anchor.clone(), &pointer, child, edges)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl<R> NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    fn walk_root_value_at(
        &self,
        value: &NightfireValue,
        root_pointer: &str,
        fallback_anchor: Option<&BlockAnchor>,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        if let Some(block) = value.block.as_ref() {
            let rooted_pointer = join_rooted_pointer(root_pointer, "/block/data");
            let anchor = block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|_| BlockAnchor::from_block(block, rooted_pointer.clone()))
                .or_else(|| fallback_anchor.cloned())
                .unwrap_or_else(|| BlockAnchor::from_block(block, rooted_pointer.clone()));
            self.walk_block(block, anchor, &rooted_pointer, edges)?;
        }

        if let Some(blocks) = value.blocks.as_ref() {
            for (index, block) in blocks.iter().enumerate() {
                let rooted_pointer =
                    join_rooted_pointer(root_pointer, &format!("/blocks/{index}/data"));
                let anchor = block
                    .id
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(|_| BlockAnchor::from_block(block, rooted_pointer.clone()))
                    .or_else(|| fallback_anchor.cloned())
                    .unwrap_or_else(|| BlockAnchor::from_block(block, rooted_pointer.clone()));
                self.walk_block(block, anchor, &rooted_pointer, edges)?;
            }
        }

        Ok(())
    }

    fn walk_block(
        &self,
        block: &BlockData,
        anchor: BlockAnchor,
        rooted_data_pointer: &str,
        edges: &mut Vec<MediaUsageEdgeInput>,
    ) -> MediaResult<()> {
        let context = NightfireMediaVisitContext {
            block,
            data_pointer: "",
            rooted_pointer: rooted_data_pointer,
        };

        if let Some(handler) = self.registry.handler_for(&block.r#type) {
            for media_ref in handler.extract_media_references(&context)? {
                let rooted_pointer =
                    join_rooted_pointer(rooted_data_pointer, &media_ref.data_pointer);
                let (locator_kind, locator_key) = anchor.locator_for(&rooted_pointer)?;
                edges.push(MediaUsageEdgeInput {
                    media_id: media_ref.media_id,
                    used_by_type: self.used_by_type.clone(),
                    used_by_id: self.used_by_id,
                    owner_field: Some(self.owner_field.clone()),
                    content_kind: MediaContentKind::StructuredContent,
                    locator_kind,
                    locator_key,
                    usage_role: media_ref.usage_role,
                    provenance_kind: self.provenance_kind.clone(),
                });
            }

            for nested in handler.nested_nightfire_values(&context)? {
                let nested_root_pointer =
                    join_rooted_pointer(rooted_data_pointer, &nested.data_pointer);
                let nested_value = context
                    .resolve_relative_pointer(&nested.data_pointer)
                    .ok_or_else(|| {
                        MediaError::validation(format!(
                            "nested Nightfire pointer not found in block {}: {}",
                            block.r#type, nested.data_pointer
                        ))
                    })?
                    .clone();
                let nested_value =
                    serde_json::from_value::<NightfireValue>(nested_value).map_err(|err| {
                        MediaError::validation(format!(
                            "invalid nested Nightfire value in block {} at {}: {err}",
                            block.r#type, nested.data_pointer
                        ))
                    })?;
                self.walk_root_value_at(&nested_value, &nested_root_pointer, Some(&anchor), edges)?;
            }
        }

        for (nested_pointer, nested_block) in collect_nested_blocks(&block.data, "")? {
            let rooted_data_pointer =
                join_rooted_pointer(&anchor.rooted_data_pointer, &nested_pointer);
            let nested_anchor = nested_block
                .id
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|_| BlockAnchor::from_block(&nested_block, rooted_data_pointer.clone()))
                .unwrap_or_else(|| anchor.clone());

            self.walk_block(&nested_block, nested_anchor, &rooted_data_pointer, edges)?;
        }

        Ok(())
    }
}

fn as_nested_block(value: &Value) -> Option<BlockData> {
    serde_json::from_value::<BlockData>(value.clone()).ok()
}

fn as_nested_nightfire_value(value: &Value) -> Option<NightfireValue> {
    serde_json::from_value::<NightfireValue>(value.clone()).ok()
}

fn push_pointer_segment(pointer: &str, segment: &str) -> String {
    let escaped = segment.replace('~', "~0").replace('/', "~1");
    format!("{pointer}/{escaped}")
}

fn normalize_relative_pointer(pointer: &str) -> String {
    let trimmed = pointer.trim();
    if trimmed.is_empty() || trimmed == "/" {
        String::new()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn join_rooted_pointer(rooted_pointer: &str, relative_pointer: &str) -> String {
    let normalized = normalize_relative_pointer(relative_pointer);
    if normalized.is_empty() {
        rooted_pointer.to_string()
    } else {
        format!("{rooted_pointer}{normalized}")
    }
}

fn collect_nested_blocks(value: &Value, pointer: &str) -> MediaResult<Vec<(String, BlockData)>> {
    let mut nested = Vec::new();
    collect_nested_blocks_into(value, pointer, &mut nested)?;
    Ok(nested)
}

fn collect_nested_blocks_into(
    value: &Value,
    pointer: &str,
    nested: &mut Vec<(String, BlockData)>,
) -> MediaResult<()> {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                let child_pointer = push_pointer_segment(pointer, &index.to_string());
                collect_nested_blocks_into(item, &child_pointer, nested)?;
            }
        }
        Value::Object(map) => {
            if let Some(block) = as_nested_block(value) {
                nested.push((pointer.to_string(), block));
                return Ok(());
            }

            if as_nested_nightfire_value(value).is_some() {
                return Ok(());
            }

            for (key, child) in map {
                let child_pointer = push_pointer_segment(pointer, key);
                collect_nested_blocks_into(child, &child_pointer, nested)?;
            }
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/nightfire_tests.rs"]
mod tests;
