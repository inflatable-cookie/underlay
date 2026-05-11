# 014 - Acme-Admin Form Page Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-10

## Results

`EntityFormPage` now carries the real proof behavior the project forms needed:

- subtitle and contextual back-link support
- header metadata slot
- field-error summary
- internal form wrapper
- SPA submit/result/redirect handling

The acme-admin project create and edit pages now use `EntityFormPage` instead
of `SpaFormShell`.

## Targets

1. `/projects/new` — Create project form (154 lines)
2. `/projects/[projectId]/edit` — Edit project form (239 lines)

## Goals

- migrate both pages to `EntityFormPage`
- preserve all behavior: validation, error handling, navigation context, etag handling
- target: ~50 lines each
- validate that custom fields (RelationSelector) work via `type: "custom"`

## Non-Goals

- Changing the visual design
- Adding new features — parity only
- Moving ProjectForm component logic into templates (RelationSelector stays custom)

## Exit Criteria

- [x] `/projects/new` renders and creates projects correctly
- [x] `/projects/[id]/edit` renders and updates projects correctly
- [x] Validation errors display inline
- [x] API errors display as form-level errors
- [x] Navigation context (back links) preserved
- [x] Etag handling for edit page preserved

## Next Task

Execute `g03.015`: migrate acme-admin task create/edit forms.
