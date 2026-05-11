# 028 - Dairy User Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-11

## Context

`g03.027` closed the smaller mock-config exams proof. The next honest adjacent
pair is the Dairy user create/edit family: compact routes, real edit metadata,
and shared CRUD-style form behavior in a non-reference consumer.

## Targets

1. `/users/new` — Create user form (121 lines)
2. `/users/[userId]/edit` — Edit user form (178 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve loading and error states where present
- preserve route-owned validation and user loading
- preserve edit-page metadata, banner, and contextual back links

## Non-Goals

- changing user-management workflow behavior
- widening templates for user-specific policy or auth logic
- tackling broader learning form families in the same batch

## Exit Criteria

- [x] Dairy user create/edit pages use `EntityFormPage`
- [x] loading and error states still render correctly
- [x] route-owned validation and user loading still work correctly
- [x] edit-page metadata, banner, and back links remain correct

## Results

- migrated Dairy user create/edit pages from `SpaFormShell` to
  `EntityFormPage`
- preserved route-owned validation and user loading
- preserved edit-page metadata, status banner, and contextual back links
- proved the shell on a compact non-reference consumer CRUD family after the
  heavier content and exams waves

## Next Task

Execute `g03.029`.
