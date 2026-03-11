# 041 - Form Draft Persistence

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031, 039

## Overview

Add opt-in draft persistence to `createFormState` so consuming apps can restore in-progress form input with shared storage, debouncing, and expiration controls instead of hand-rolling the same `storage.session` wiring in every long-form flow.

## Research Basis

- `docs/roadmaps/backlog/advanced-forms.md`
- `docs/guides/100-frontend-web.md`
- `ts/src/patterns/forms.ts`
- `ts/src/patterns/storage.ts`

## Likely Implementation Surface

- `ts/src/patterns/forms.ts`
- `ts/tests/patterns/forms.test.ts`
- `docs/guides/100-frontend-web.md`
- `docs/guides/190-upgrade-compatibility.md`

## Phase 41.1 - Draft Persistence API

- [x] Add an opt-in `autoSave` configuration to `createFormState`.
- [x] Support debounced draft writes to shared session or local storage.
- [x] Reuse storage expiration options so drafts can expire without a separate form-specific timer model.

## Phase 41.2 - Form Restoration and Lifecycle

- [x] Restore saved draft values into a bound form element when `form.enhance` attaches.
- [x] Clear drafts automatically after successful submission while keeping the feature configurable.
- [x] Keep the first batch generic and DOM-form-based instead of introducing app-specific wizard or schema abstractions.

## Phase 41.3 - Documentation and Consumer Guidance

- [x] Replace the manual draft example in `100-frontend-web.md` with the shared pattern.
- [x] Add an upgrade note entry in `190-upgrade-compatibility.md`.
- [x] Document current boundaries such as skipped file inputs and the requirement to use `form.enhance` for automatic restore/save wiring.

## Deferred

- Multi-step wizard primitives.
- Conditional field orchestration helpers.
- Async field-level validation.
- Undo and redo history stacks.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing `createFormState` usage should remain valid without change.
- Consumers adopting the new draft surface should decide whether drafts are session-scoped or long-lived and set `ttl` / `expiresAt` intentionally.
- Upgrade guidance must document that successful submit clears drafts by default and that file inputs are not persisted in this batch.

## Validation

```bash
bun x vitest run ts/tests/patterns/forms.test.ts
effigy validate --repo .
```

## Completion

This batch is complete. The current active roadmap set is complete again.
