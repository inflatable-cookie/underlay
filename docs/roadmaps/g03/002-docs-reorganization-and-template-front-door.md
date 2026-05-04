# 002 - Docs Reorganization And Template Front Door

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

Underlay's `docs/guides/` contains 58 files mixing backend, frontend, deprecated
UI guidance, and integration docs. Guides `090`, `097`, and `098` teach
hand-rolled composition that the template system will replace. The docs need
reorganization so developers can find template docs without wading through
backend guides.

## Goals

- create `docs/usage/` as the single user-facing docs tree
- migrate retained guides from `docs/guides/` into `docs/usage/backend/`,
  `docs/usage/frontend/`, `docs/usage/runtime/`
- create `docs/usage/templates/` as the template system front door
- delete deprecated guides: `090-ui-kit.md`, `097-autonomous-list-components.md`,
  `098-shared-admin-patterns.md`
- move `180-admin-workflow-playbook.md` into `docs/usage/templates/`

## Non-Goals

- rewriting backend guides content (only moving them)
- touching Northstar structural docs (`architecture/`, `contracts/`, `roadmaps/`,
  `specs/`, etc.)
- rewriting Poodle guides (those stay in Poodle)

## Execution Plan

### Batch 2.1 - Usage Tree Creation

- [x] create `docs/usage/000-overview.md` — new entry point for user-facing docs
- [x] create `docs/usage/backend/` — migrate guides 040–079
- [x] create `docs/usage/frontend/` — migrate guides 100–120
- [x] create `docs/usage/runtime/` — migrate guides 080, 095, and runtime helpers
- [x] create `docs/usage/templates/` — new template system docs

### Batch 2.2 - Guide Migration

- [ ] move retained backend guides into `docs/usage/backend/` (pending)
- [ ] move retained frontend guides into `docs/usage/frontend/` (pending)
- [ ] move runtime/client guides into `docs/usage/runtime/` (pending)
- [ ] delete deprecated UI guides (090, 097, 098) (pending)
- [ ] move `180-admin-workflow-playbook.md` to
      `docs/usage/templates/admin-workflow-playbook.md` (pending)

### Batch 2.3 - Template Docs Skeleton

- [x] create `docs/usage/templates/000-template-system-overview.md`
- [x] create `docs/usage/templates/entity-list-page.md`
- [x] create `docs/usage/templates/entity-detail-page.md`
- [x] create `docs/usage/templates/entity-form-page.md`
- [x] create `docs/usage/templates/entity-list-section.md`
- [x] create `docs/usage/templates/entity-detail-section.md`
- [x] create `docs/usage/templates/entity-form-section.md`
- [x] create `docs/usage/templates/template-api-reference.md`

### Batch 2.4 - Index Updates

- [x] update `docs/usage/000-overview.md` with reading order and doc map
- [x] update `docs/README.md` to point at `docs/usage/`
- [x] update `docs/guides/README.md` with deprecation notice pointing to usage/
- [x] leave `docs/guides/` in place temporarily with a redirect notice

## Exit Criteria

- `docs/usage/` exists with all migrated guides in correct subdirectories
- deprecated UI guides are deleted or clearly marked deprecated
- template docs skeleton exists with placeholder content
- `docs/usage/000-overview.md` is the new user-facing front door

## Next Task

Execute `g03.003`: refactor `ts/src/` structure to create `ts/src/templates/` as
a first-class package surface and shrink `ts/src/patterns/` to auth workflows
and RelationSelector.
