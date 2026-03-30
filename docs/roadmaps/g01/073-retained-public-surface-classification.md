# 073 - Retained Public Surface Classification

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 072

## Overview

The Poodle-adoption and public-surface contraction line is complete, but the
repo still needs one honest final classification pass over what remains
exported. This wave records the end state explicitly instead of leaving the
remaining public API to be inferred from what happened not to be retired.

This is not another migration wave. It is a boundary-classification closeout
for:

- public `components`
- public `patterns`
- public `nightfire`

## Research Basis

- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`
- `ts/src/nightfire/index.ts`
- `package.json` public exports
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`

## Decision Focus

- Confirm which remaining public surfaces are deliberate retained Underlay API
- Distinguish retained UI shells from retained helper/controller exports
- Record `nightfire` as a separate package surface rather than lingering UI
  contraction residue

## Findings

### Public `patterns`

The public `patterns` barrel now splits into two deliberate categories.

Retained public UI/workflow surface:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- `SpaFormShell`
- `DetailMeta`
- `DetailMetaItem`
- `DetailMetaId`
- `DetailMetaStatus`
- `DetailMetaSeparator`

Retained public helper surface:

- auth shared types
- date-range formatter helpers

Retained helper/controller surface:

- relation-selector helper layer
- authenticated-data helper
- toast helper
- selection-history helper
- pagination, batch, list, reorder, media-upload, AI-routing, and related
  app-facing controllers/utilities

The repo no longer needs the old `patterns/...` subpath imports in its own app
and docs surface. The explicit helper subpath exports may remain for now, but
they are no longer carrying hidden migration debt.

### Public `nightfire`

`nightfire` is not part of the generic Poodle-contraction residue. It is a
separate retained editor/runtime package surface with its own public contract:

- editor and renderer components
- slash-command surface
- registries
- validation and serialization helpers
- strategies and media context

It should be treated as a deliberate Underlay package boundary unless a future
editor-specific roadmap challenges it directly.

## Final Outcome

`g01.073` is complete.

The remaining public Underlay surface is now explicitly classified:

- retained auth/workflow UI
- retained structural shell and metadata helpers
- retained helper/controller exports
- retained `nightfire` package surface

The contraction line no longer has hidden package-boundary debt in the active
repo surface.

## Next Task

Complete. The public Underlay surface is now explicitly classified, and any
future contraction work should start as a fresh boundary challenge rather than
continue this migration line by inertia.
