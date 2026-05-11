# 016 - Acme-Admin Simple Form Proof

Status: complete
Owner: repo maintainers
Updated: 2026-05-10

## Context

`g03.014` and `g03.015` proved `EntityFormPage` against the heavier project and
task form families. The next honest batch is the remaining smaller acme-admin
form pages that still use `SpaFormShell`.

## Targets

1. `/categories/new` — Create category form (150 lines)
2. `/categories/[categoryId]/edit` — Edit category form (236 lines)
3. `/users/new` — Create user form (129 lines)
4. `/users/[userId]/edit` — Edit user form (214 lines)

## Goals

- migrate all four pages to `EntityFormPage`
- preserve validation and form-level error handling
- preserve contextual back-link behavior
- preserve SPA submit/result/redirect behavior

## Non-Goals

- changing visual design
- adding new features
- forcing shared form extraction where the pages do not need it

## Exit Criteria

- [x] category create/edit pages use `EntityFormPage`
- [x] user create/edit pages use `EntityFormPage`
- [x] validation errors still render inline
- [x] API and auth errors still render as form-level errors
- [x] contextual back links and redirects remain correct

## Results

- migrated the remaining acme-admin category and user create/edit pages from
  `SpaFormShell` to `EntityFormPage`
- preserved shared `CategoryForm` and `UserForm` ownership instead of forcing
  extra template-layer extraction
- preserved edit-page loading, error, metadata, and contextual back-link
  behavior through the `EntityFormPage` surface
- completed the acme-admin form-proof family for `EntityFormPage`

## Next Task

Execute `g03.017`.
