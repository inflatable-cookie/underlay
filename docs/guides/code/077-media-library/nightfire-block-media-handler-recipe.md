# Nightfire block media handler recipe

Use this when a consumer app defines custom Nightfire blocks and wants media
extraction to live beside those block definitions instead of in API-route JSON
heuristics.

## Rule

Split responsibilities cleanly:

- the shared Underlay walker traverses the Nightfire tree
- each block handler extracts media refs for one block type
- a block handler may also expose nested Nightfire child values when its
  payload embeds inner Nightfire documents
- save-time sync still happens through the shared `media_usage` reconciliation
  path

## Core shared surfaces

- `NightfireBlockMediaUsageExtractor`
- `NightfireBlockMediaHandler`
- `NightfireBlockMediaRegistration`
- `NightfireBlockMediaHandlerMap`
- `NightfireBlockMediaReference`
- `NightfireNestedValuePointer`
- `NightfireMediaVisitContext`

## Leaf block example

```rust
use underlay_media::{
    MediaId, MediaUsageRole, NightfireBlockMediaHandler,
    NightfireBlockMediaReference, NightfireMediaVisitContext,
};
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

pub fn hero_media_registration() -> underlay_media::NightfireBlockMediaRegistration {
    underlay_media::NightfireBlockMediaRegistration::new("hero", HeroBlockHandler)
}
```

## Block with nested Nightfire child value

```rust
use underlay_media::{
    NightfireBlockMediaHandler, NightfireBlockMediaReference,
    NightfireMediaVisitContext, NightfireNestedValuePointer,
};

struct PopupBlockHandler;

impl NightfireBlockMediaHandler for PopupBlockHandler {
    fn extract_media_references(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> underlay_media::MediaResult<Vec<NightfireBlockMediaReference>> {
        Ok(Vec::new())
    }

    fn nested_nightfire_values(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> underlay_media::MediaResult<Vec<NightfireNestedValuePointer>> {
        Ok(vec![NightfireNestedValuePointer::new("/content")])
    }
}
```

Use that when one block embeds an inner `{ schema, block }` or
`{ schema, blocks }` value inside its own payload.

## Registry example

```rust
use underlay_media::{
    MediaUsageProvenanceKind, NightfireBlockMediaHandlerMap,
    NightfireBlockMediaUsageExtractor,
};

let registry = NightfireBlockMediaHandlerMap::from_registrations([
    hero_media_registration(),
    popup_media_registration(),
    media_block_media_registration(),
]);

let extractor = NightfireBlockMediaUsageExtractor::new(
    "lesson",
    Some(lesson_id),
    "body_blocks",
    MediaUsageProvenanceKind::ContentSync,
    registry,
);
```

## Handler placement rule

Implement handlers beside block definitions and block tests.

Good:

- `blocks/hero.rs` defines the block payload
- `blocks/hero_media.rs` or the same module defines the media handler
- tests for that block prove both block behavior and media extraction

Bad:

- route-level helper that pattern-matches `"image_id"` across unrelated block
  types
- one giant content extractor file that understands every block payload in the
  app

Assembly rule:

- define the block payload, validator, renderer/editor registration, and media
  handler from the same block module set
- export one `NightfireBlockMediaRegistration` per block module when the block
  owns media references
- build app registries from those same module exports in one place
- do not let the media-handler registry drift into a separate list that can
  forget a block the editor and validator already know about

## Locator rule

Handlers should emit pointers relative to `block.data`, not rooted whole-value
paths.

Examples:

- `/image_id`
- `/pages/1/mediaId`
- `/attachments/0/fileId`

The shared walker converts those into canonical `media_usage` locators:

- `block_id`: `gallery_02#/pages/1/mediaId`
- fallback `path`: `/blocks/4/data/pages/1/mediaId`

If a block embeds a child Nightfire document and that child content does not
yet have its own block ids, the walker falls back to the nearest stable outer
anchor instead of inventing a fake root:

- outer anchor example: `popup_01#/content/blocks/0/data/media_id`
- rooted path fallback when no stable outer id exists:
  `/blocks/0/data/content/blocks/0/data/media_id`
