# Nightfire save/sync/resolve recipe

Use this when a consumer app stores Nightfire JSON and wants one exact shared
flow for media usage edges.

TS-side pairing:

- use `writePreparedNightfireToFormData(...)` for normal form save paths
- preserve inner Nightfire block `data` keys verbatim on the wire
- only map outer DTO field names at the API boundary
- pair the Rust side with the API ingest recipe in
  `docs/guides/code/070-api-handlers/nightfire-persist-and-media-sync.rs`

## Goal

Go from:

- one saved `NightfireValue`

To:

- exact `MediaUsageEdgeInput` rows at save time
- stable `block_id` or `path` locators in `media_usage`
- later reverse lookup from a stored usage row back into the current Nightfire
  JSON

## Save-time recipe

```rust
use underlay_media::{
    MediaId, MediaLocatorKind, MediaUsageProvenanceKind, MediaUsageRole,
    NightfireBlockMediaHandler, NightfireBlockMediaHandlerMap,
    NightfireBlockMediaReference, NightfireBlockMediaUsageExtractor,
    NightfireMediaVisitContext, resolve_nightfire_media_usage,
};
use underlay_nightfire::{ensure_block_ids, NightfireValue};
use uuid::Uuid;

struct HeroBlockHandler;

impl NightfireBlockMediaHandler for HeroBlockHandler {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> underlay_media::MediaResult<Vec<NightfireBlockMediaReference>> {
        let Some(media_id) = context
            .resolve_relative_pointer("/image_id")
            .and_then(|value| value.as_str())
            .and_then(|value| Uuid::parse_str(value).ok())
            .map(MediaId::from_uuid)
        else {
            return Ok(Vec::new());
        };

        Ok(vec![NightfireBlockMediaReference::new(
            media_id,
            MediaUsageRole::Embedded,
            "/image_id",
        )])
    }
}

async fn save_body_blocks<R>(
    repo: &R,
    lesson_id: Uuid,
    mut body_blocks: NightfireValue,
) -> anyhow::Result<()>
where
    R: underlay_media::MediaUsageSyncRepository,
{
    // 1. Make block ids stable before persistence or extraction.
    ensure_block_ids(&mut body_blocks);

    // 2. Persist the Nightfire JSON in your app-owned record/table.
    // sqlx::query("UPDATE lessons SET body_blocks = $1::jsonb WHERE id = $2")
    //     .bind(serde_json::to_value(&body_blocks)?)
    //     .bind(lesson_id)
    //     .execute(pool)
    //     .await?;

    // 3. Extract exact media edges and reconcile them through shared sync.
    let registry = NightfireBlockMediaHandlerMap::new()
        .with_handler("hero", HeroBlockHandler);

    let extractor = NightfireBlockMediaUsageExtractor::new(
        "lesson",
        Some(lesson_id),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        registry,
    );

    let report = extractor.extract_and_sync(repo, &body_blocks).await?;

    assert!(report.inserted + report.retained + report.removed >= 0);

    // 4. Later, reverse a stored media_usage row back into the current JSON.
    let current_value = resolve_nightfire_media_usage(
        &body_blocks,
        &MediaLocatorKind::BlockId,
        "gallery_02#/pages/1/image_id",
    );

    assert!(current_value.is_some());

    Ok(())
}
```

## Locator decision rule

Use:

- `field`
  - plain record column reference
  - example: `cover_media_id`
- `block_id`
  - structured content has a stable block id
  - example: `gallery_02#/pages/1/image_id`
- `path`
  - structured content does not yet have a stable block id
  - example: `/blocks/4/data/pages/1/image_id`

Do not:

- invent fake block ids just to avoid `path`
- keep emitting coarse one-row-per-field usage edges once exact structured
  references are available
- reimplement resolver logic in every consumer app

## Re-anchor rule

If nested content later gets a stable child block id:

- old honest fallback:
  - `hero_01#/children/0/data/pages/0/image_id`
- better later anchor:
  - `gallery_02#/pages/0/image_id`

That is an extractor-output improvement, not a schema change.

## Compatibility note

`NightfireMediaUsageExtractor` plus `NightfireFieldNameMatcher` still exists as
the compatibility path for older consumers. New block work should prefer the
registry-backed handler model so extraction semantics stay beside block
definitions instead of drifting into route-local JSON heuristics.
