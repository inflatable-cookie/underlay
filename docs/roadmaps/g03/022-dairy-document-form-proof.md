# 022 - Dairy Document Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

After the Dairy blog article proof, the adjacent document create/edit pair was
the next richer content family with route-owned Nightfire body preparation and
document-specific schema logic still using `SpaFormShell`.

## Targets

1. `/content/documents/new` — Create document form (218 lines)
2. `/content/documents/[documentId]/edit` — Edit document form (331 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading, error, and not-found behavior on edit
- preserve document-specific Nightfire body preparation and schema handling
- preserve edit-page metadata and contextual back links

## Non-Goals

- changing document workflow behavior
- widening templates for document-specific schema behavior
- tackling the next content authoring families in the same batch

## Exit Criteria

- [x] Dairy document create/edit pages use `EntityFormPage`
- [x] loading, error, and not-found states still render correctly
- [x] Nightfire body preparation and schema handling still work correctly
- [x] edit-page metadata and back links remain correct

## Results

- migrated Dairy document create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved document-specific Nightfire body preparation and schema selection
- preserved edit-page loading, error, and not-found handling
- extended the richer-content proof to a second Nightfire-backed content family

## Next Task

Execute `g03.023`.
