# 019 - Dairy Audio Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.017` and `g03.018` proved `EntityFormPage` against a compact Dairy exam
form family. The next honest consumer proof is a content create/edit pair with
slightly richer field handling but the same page-shell posture.

## Targets

1. `/content/audios/new` — Create audio form (179 lines)
2. `/content/audios/[audioId]/edit` — Edit audio form (257 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and not-found behavior on edit
- preserve form-level errors, redirects, and contextual back links
- preserve edit-page metadata and delete action behavior

## Non-Goals

- changing audio workflow behavior
- widening templates for content-specific form behavior
- tackling other content media families in the same batch

## Exit Criteria

- [x] Dairy audio create/edit pages use `EntityFormPage`
- [x] loading, error, and not-found states still render correctly
- [x] edit-page metadata and delete action remain intact
- [x] redirects and contextual back links remain correct

## Results

- migrated Dairy audio create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved edit-page loading, error, and not-found handling
- preserved edit-page metadata and delete action behavior
- extended the Dairy proof from exam forms into the first content media pair

## Next Task

Execute `g03.020`.
