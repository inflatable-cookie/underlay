# 015 - Acme-Admin Task Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-10

## Context

`g03.014` proved `EntityFormPage` against the acme-admin project create/edit
pages. The next honest proof is the task form family, which is larger and
includes richer Nightfire content and project-scoped routing.

## Targets

1. `/projects/[projectId]/tasks/new` — Create task form (333 lines)
2. `/projects/[projectId]/tasks/[taskId]/edit` — Edit task form (414 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve all behavior: validation, error handling, navigation context, etag handling
- preserve Nightfire task notes handling
- preserve project-scoped back-link and redirect behavior

## Non-Goals

- Changing the visual design
- Adding new features — parity only
- Moving TaskForm component logic into templates

## Exit Criteria

- [x] `/projects/[projectId]/tasks/new` renders and creates tasks correctly
- [x] `/projects/[projectId]/tasks/[taskId]/edit` renders and updates tasks correctly
- [x] Validation errors display inline
- [x] API errors display as form-level errors
- [x] Navigation context (back links) preserved
- [x] Etag handling for edit page preserved
- [x] Nightfire notes handling preserved

## Results

- added a shared `TaskForm` in acme-admin
- migrated both task create/edit pages from `SpaFormShell` to `EntityFormPage`
- preserved Nightfire notes handling, project-scoped redirects, inline field
  errors, and edit-page etag recovery
- kept the richer page-owned SPA submit/result flow local to the task routes,
  which is the right proof boundary for this batch

## Next Task

Execute `g03.016`.
