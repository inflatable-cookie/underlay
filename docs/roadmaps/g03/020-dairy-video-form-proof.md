# 020 - Dairy Video Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

After the Dairy audio proof, the adjacent video create/edit pair was the next
same-shape content media family still using `SpaFormShell`.

## Targets

1. `/content/videos/new` — Create video form (176 lines)
2. `/content/videos/[videoId]/edit` — Edit video form (255 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading, error, and not-found behavior on edit
- preserve form-level errors, redirects, and contextual back links
- preserve edit-page metadata and delete action behavior

## Non-Goals

- changing video workflow behavior
- widening templates for content-specific media behavior
- tackling richer article or Nightfire-heavy content forms in the same batch

## Exit Criteria

- [x] Dairy video create/edit pages use `EntityFormPage`
- [x] loading, error, and not-found states still render correctly
- [x] edit-page metadata and delete action remain intact
- [x] redirects and contextual back links remain correct

## Results

- migrated Dairy video create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved edit-page loading, error, and not-found handling
- preserved edit-page metadata and delete action behavior
- confirmed the same content media proof pattern works for a second adjacent
  media family

## Next Task

Execute `g03.021`.
