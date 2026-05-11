# 021 - Dairy Blog Article Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.019` and `g03.020` proved `EntityFormPage` against two compact Dairy
content media pairs. The next honest consumer proof is the blog article
create/edit pair, which adds richer Nightfire body handling while keeping the
same page-shell posture.

## Targets

1. `/content/blog-articles/new` — Create blog article form (215 lines)
2. `/content/blog-articles/[blogArticleId]/edit` — Edit blog article form (355 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading, error, and not-found behavior on edit
- preserve Nightfire body preparation and validation handling
- preserve edit-page metadata, etag handling, and contextual back links

## Non-Goals

- changing blog workflow behavior
- widening templates for blog-specific Nightfire behavior
- tackling other richer content forms in the same batch

## Exit Criteria

- [x] Dairy blog article create/edit pages use `EntityFormPage`
- [x] loading, error, and not-found states still render correctly
- [x] Nightfire body handling still works correctly
- [x] edit-page metadata, etag handling, and back links remain correct

## Results

- migrated Dairy blog article create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved Nightfire body preparation and route-owned `prepare` wiring
- preserved edit-page loading, error, and not-found handling
- preserved edit-page metadata and etag conflict recovery
- proved the shell against the first richer content family with Nightfire and
  concurrency behavior

## Next Task

Execute `g03.022`.

## Next Task

Execute this proof batch.
