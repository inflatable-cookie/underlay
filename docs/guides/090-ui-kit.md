# 090 - UI Kit

This page is now a boundary guide only.

Generic UI implementation guidance no longer lives in Underlay. Use Poodle as
the canonical source for:

- primitives
- generic composites
- page/header/list/detail/dialog composition
- admin shell composition
- media browse and picker composition
- shared presentational display-format helpers like file-size and simple
  display-date labels

Start with these Poodle guides:

- `Poodle Svelte Developer Guide`
- `Form Layout And Field Recipes`
- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Auth UI And Workflow Recipes`
- `Page Shell And Admin Recipes`
- `Media Library And Upload Recipes`
- `Admin Feature Delivery Recipes`
- `Admin App Shell Recipes`

## What Underlay Still Owns

Underlay now keeps only the retained package surfaces that still express
workflow shells, runtime orchestration, transport integration, or Nightfire
editor/runtime behavior.

Use:

- `@inflatable-cookie/underlay/patterns` for retained workflow/page shells only
- `@inflatable-cookie/underlay/runtime/*` for shared app/runtime helpers and controllers
- `@inflatable-cookie/underlay/utils/*` for small standalone helpers
- keep broader app-formatting helpers on `@inflatable-cookie/underlay/utils/i18n`
  until they earn a true shared UI contract
- treat only direct presentation helpers as Poodle candidates: things like
  file-size and simple display-date labels, not broader app formatting policy
- `@inflatable-cookie/underlay/client/*` for transport and framework-facing helpers
- `@inflatable-cookie/underlay/nightfire/*` for structured content editor/runtime

## Import Rules

- do not deep-import internal Underlay Svelte files
- use Poodle packages directly for primitives and generic composites
- use the narrow `runtime/*`, `client/*`, `utils/*`, and `nightfire/*`
  subpaths when they match the feature area

## Reference Surface

For real visible implementations, use the ACME reference apps:

- `underlay-reference/apps/acme-admin`
- `underlay-reference/apps/acme-front`

These live in the separate `underlay-reference` repository.

For retained shared-surface examples in this repo, use the local Storybook
catalog:

```bash
effigy storybook
effigy storybook:build
```

## Decision

- Poodle is the implementation home for generic UI
- Underlay is the retained home for workflow/runtime/client/nightfire concerns
- new generic UI recipes should be added in Poodle, not here

## Next Task

Keep this page small. If it starts growing implementation examples again, move
 them to Poodle or replace them with ACME reference links instead.
