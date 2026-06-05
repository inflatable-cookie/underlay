# g06.104 - Media Renditions Service Internal Split

## Why

`g06.103` found that `underlay-media/src/renditions/service.rs` mixes service
construction, direct generation, deletion, legacy version-generation behavior,
standardized key generation, repository persistence, and clone behavior in one
production file.

The next split should reduce reasoning load without changing public service
methods or generated rendition behavior.

## Goal

Split the media renditions service into focused internal modules while
preserving all public methods and behavior.

## Scope

In scope:

- keep `service.rs` as the small module front door
- move `RenditionService` type, construction, accessors, and `Clone` into a
  focused core module
- move direct thumbnail, preview, and raw-byte generation into a focused
  generation module
- move single-rendition and version-rendition deletion into a focused deletion
  module
- move legacy and standardized version rendition generation into a focused
  version module
- preserve existing media tests

Out of scope:

- changing media rendition public APIs
- changing storage or repository semantics
- changing generated rendition keys or metadata
- adding new image integration tests
- changing consumer apps

## Acceptance Criteria

- `service.rs` becomes a small module front door
- responsibility groups live in focused modules
- `underlay_media::renditions::RenditionService` remains stable
- media tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public service methods, generated keys, metadata,
or repository semantics must change, stop and re-enter planning.

## Current State

`g06.104` is ready.

## Next Task

Execute `g06.104`: media renditions service internal split.
