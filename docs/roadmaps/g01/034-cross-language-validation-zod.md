# 034 - Cross-Language Validation with Zod

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add an opt-in `@decodelabs/underlay/validation` surface for reusable Zod schemas and Svelte form helpers so apps can share common client-side validation patterns without waiting for server responses.

## Research Basis

- `docs/research/implementation-decision-records/idr-validation-zod-integration.md`
- `docs/research/translation-memos/cross-language-validation.md`
- `docs/research/value-tracks/cross-language-validation.md`
- `docs/research/specimen-dossiers/zod.md`
- `docs/research/specimen-dossiers/valibot.md`
- `docs/research/specimen-dossiers/ts-rs.md`

## Likely Implementation Surface

- `package.json`
- new `ts/src/validation/` export surface
- new shared validated-form helper in TS patterns
- `docs/guides/075-validation.md`
- `docs/guides/096-form-helpers.md`

## Phase 34.1 - Schema Surface and Packaging

- [x] Add `zod` as an optional peer dependency with a documented opt-in contract.
- [x] Create a shared validation export for common primitives and composed request schemas.
- [x] Keep the initial schema set generic and project-agnostic.

## Phase 34.2 - Form Integration

- [x] Add a validated-form helper that works with existing form primitives instead of replacing them wholesale.
- [x] Document how the new helper fits with `FormValidationProvider`.
- [x] Add focused TS and component tests for schema usage and form-state behavior.

## Phase 34.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update validation and form-helper guides with a clear mapping between Rust rules and Zod equivalents.
- [x] Document the consumer install step for apps that import the new validation surface.
- [x] Record bundle-size expectations and the boundary between shared schemas and app-specific refinements.

## Deferred

- Automated Rust-to-TypeScript validation code generation.
- Validation surfaces tied to app-specific DTOs or contracts.
- A second validation provider library unless Zod stops fitting the repo's needs.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Apps do not need `zod` unless they opt into the new validation export.
- Documentation must state that server validation remains authoritative and that shared client schemas are a UX layer, not a protocol guarantee.
- Any later schema rename or removal must ship with a deprecation notice and replacement mapping.

## Validation

```bash
effigy validate
effigy test:components -- ts/tests/components/text-input-form-validation.test.ts ts/tests/patterns/form-validation-provider.component.test.ts
```

## Next Task

Roadmap complete on 2026-03-11. Expand the shared schema set only when cross-app duplication shows rules are stable and project-agnostic enough to belong in Underlay.
