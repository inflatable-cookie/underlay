---
title: Runtime I18n Helper Split
owner: Codex
status: complete
updated: 2026-03-30
---

# Runtime I18n Helper Split

## Goal

Challenge the retained `runtime/i18n` family and decide whether its exports are
really runtime orchestration or just pure helpers that belong on a cleaner
standalone utility surface.

## Outcome

The pure helper families now have clearer homes on `@inflatable-cookie/underlay/utils`
while `runtime/i18n` stays in place as a compatibility barrel.

The important split is:

- `utils/i18n`
  - date, time, number, currency, file-size, and date-range formatting helpers
- `utils/slug`
  - `slugify`, slug-format validation, and reserved-slug helpers
- `runtime/i18n`
  - remains stable only as a convenience/compatibility layer over those pure
    helpers

## Judgment

This was a real boundary cleanup, not another naming pass.

The exports in `runtime/i18n` were pure synchronous helper logic:

- no stores
- no browser state
- no framework integration
- no runtime orchestration

That means they do not belong in the same category as retained runtime helpers
such as navigation context, toast orchestration, or authenticated data hooks.

At the same time, there is no value in breaking existing `runtime` consumers
immediately, so the right move is:

- promote the pure helper families to explicit `utils/*` homes
- repoint the active docs and the small live caller set
- keep `runtime/i18n` as compatibility instead of forcing churn

## Changes

- added:
  - `ts/src/utils/i18n.ts`
  - `ts/src/utils/slug.ts`
- added public package subpaths:
  - `@inflatable-cookie/underlay/utils/i18n`
  - `@inflatable-cookie/underlay/utils/slug`
- updated `runtime/i18n` to re-export from the new utility homes
- migrated the small live caller tail in `acme-admin` and `dairy`
- updated guides to teach the utility subpaths instead of the runtime/patterns
  buckets for these pure helpers

## Next Task

This retained runtime cleanup line is complete. If work continues immediately,
the next honest boundary challenge is the retained `client` surface or a future
standalone-extraction planning pass for `nightfire`, not more helper reshuffling
inside `runtime`.
