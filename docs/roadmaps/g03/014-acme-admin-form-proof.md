# 014 - Acme-Admin Form Page Proof

Status: not started
Owner: repo maintainers
Updated: 2026-05-04

## Context

`EntityFormPage` is now implemented. Need to prove it works in practice by
migrating acme-admin's project create and edit pages.

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

- [ ] `/projects/new` renders and creates projects correctly
- [ ] `/projects/[id]/edit` renders and updates projects correctly
- [ ] Validation errors display inline
- [ ] API errors display as form-level errors
- [ ] Navigation context (back links) preserved
- [ ] Etag handling for edit page preserved

## Next Task

Execute `g03.015`: migrate acme-admin task create/edit forms.
