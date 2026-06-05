# g06.108 - Media Nightfire Internal Split

## Why

`g06.107` found that `underlay-media/src/nightfire.rs` mixes the module front
door, resolver, visit context, matcher types, field-rule helpers, extractor
construction, sync composition, owner-field validation, and walker entry points
in one production file.

The next split should reduce reasoning load without changing Nightfire media
usage APIs or extraction behavior.

## Goal

Split the media Nightfire front file into focused internal modules while
preserving all public exports and behavior.

## Scope

In scope:

- keep `nightfire.rs` as the small module front door
- move `NightfireMediaVisitContext` into a focused context module
- move `resolve_nightfire_media_usage()` into a focused resolver module
- move matcher types and field-name rules into a focused matcher module
- move both extractor types, sync methods, extractor trait impls, and walker
  trait impls into a focused extractor module
- keep existing `registry.rs` and `walk.rs` modules intact unless imports need
  updating
- preserve existing Nightfire tests

Out of scope:

- changing media public APIs
- changing Nightfire locator, extraction, registry, walking, or sync semantics
- changing media repository behavior
- changing consumer apps

## Acceptance Criteria

- `nightfire.rs` becomes a small module front door
- responsibility groups live in focused modules
- public `underlay_media::nightfire::*` exports remain stable
- media tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports or Nightfire extraction semantics
must change, stop and re-enter planning.

## Current State

`g06.108` is ready.

## Next Task

Execute `g06.108`: media Nightfire internal split.
