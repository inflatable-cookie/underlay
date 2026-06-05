# g06.054 - Media Renditions Internal Split

## Why

`g06.052` deferred `underlay-media/src/renditions.rs` until the media domain
model was easier to scan. `g06.053` completed that split, so the rendition
service file is now the next media-adjacent structural target.

The file is security-adjacent because it handles source blob reads, derived
image generation, object-key writing, and repository finalization.

## Goal

Split `underlay-media/src/renditions.rs` into focused internal modules while
preserving the feature-gated public rendition service API.

## Scope

In scope:

- split rendition configuration and result types into a focused module
- split image-processing helpers from service orchestration
- split repository/blob adapter write flow helpers where it reduces reasoning
  load
- preserve `underlay_media::renditions::*` exports
- preserve typed object-key validation and generated result-key behavior
- update tests only where module paths require it

Out of scope:

- changing the repository trait
- changing generated object-key semantics
- changing image processing behavior
- changing feature flags
- consumer rollout unless public imports move

## Acceptance Criteria

- `renditions.rs` becomes a small module front door
- public rendition service exports remain source-compatible
- `underlay-media` tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports or generated key semantics
must move, stop and re-enter planning.

## Current State

`g06.054` is next after `g06.053`.

## Next Task

Execute `g06.054`: media renditions internal split.
