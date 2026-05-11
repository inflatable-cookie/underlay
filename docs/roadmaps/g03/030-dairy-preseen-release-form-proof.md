# 030 - Dairy PreSeen Release Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.029` proved `EntityFormPage` against the first real learning form pair.
The next smallest adjacent learning family is pre-seen releases: still compact,
but with real edit loading and shared learning hierarchy behavior.

## Targets

1. `/learning/preseen-releases/new` — Create pre-seen release form (181 lines)
2. `/learning/preseen-releases/[preseenReleaseId]/edit` — Edit pre-seen release form (263 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and release loading
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing pre-seen release workflow behavior
- widening templates for learning-specific hierarchy logic
- tackling broader learning form families in the same batch

## Exit Criteria

- [x] Dairy pre-seen release create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and release loading still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy pre-seen release create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned module lookup, validation, and release loading
- preserved edit-page etag conflict handling and contextual back links
- proved the shell on the next adjacent learning hierarchy pair after levels

## Next Task

Execute `g03.031`.
