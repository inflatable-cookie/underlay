# 072 - Auth Boundary Refinement Wave

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 071

## Overview

`g01.071` closed the auth reassessment line as an explicit retained stop point.
That stop point is directionally correct, but two follow-on refinements are now
honest rather than speculative:

- `LoginPage` and `ForgotPasswordFlow` should be public `patterns`, not public
  `components`
- `PasswordRequirements` should split into:
  - a Poodle-owned agnostic UI surface
  - an Underlay-owned auth-policy adapter that keeps fetch, fallback, and
    shared auth-specific wording behavior

This wave exists to tighten that boundary without pretending the whole auth
workflow family should migrate out of Underlay.

## Research Basis

- current Underlay auth surface:
  - `ts/src/components/auth/LoginPage.svelte`
  - `ts/src/components/auth/ForgotPasswordFlow.svelte`
  - `ts/src/components/auth/PasswordRequirements.svelte`
  - `ts/src/components/auth/PasswordResetStep.svelte`
- current public barrels:
  - `ts/src/components/index.ts`
  - `ts/src/components/index.d.ts`
  - `ts/src/patterns/index.ts`
- current active callers:
  - `underlay-reference/acme-admin/src/routes/(auth)/`
  - `contact-patch/cp-admin/src/routes/(auth)/`
  - `acowtancy/dairy/src/routes/(auth)/`
  - grouped account-password pages in the same apps
- current Poodle primitive surface:
  - `../poodle/packages/svelte/primitives/src`

## Decision Focus

- Move the shared auth workflow pages to the correct public Underlay layer
  (`patterns`)
- Promote the generic password-requirements rendering and evaluation UI into
  Poodle without moving auth-policy transport or fallback behavior there
- Keep Underlay responsible only for the auth-policy adapter and the retained
  shared auth workflows

## Consumer Upgrade Impact

- Consumer apps should stop importing `LoginPage` and `ForgotPasswordFlow` from
  `@decodelabs/underlay/components` and use `@decodelabs/underlay/patterns`
  instead.
- `PasswordRequirements` remains public in Underlay for now, but its UI is no
  longer the canonical owner; the canonical agnostic checklist moves to
  Poodle.

## Planned Batches

## Batch 72.1 - Boundary Reset And Export Migration

- [x] Open the auth-boundary refinement wave and update the roadmap front
      doors plus durable inventory.
- [x] Move public `LoginPage` and `ForgotPasswordFlow` ownership from
      `components` to `patterns`.
- [x] Migrate the active app caller family and guide surface to the new public
      `patterns` boundary.

### Batch 72.1 Findings

The public auth workflow shells now live on the correct Underlay layer:

- `LoginPage` is exported from `@decodelabs/underlay/patterns`
- `ForgotPasswordFlow` is exported from `@decodelabs/underlay/patterns`
- the public `components` barrel no longer owns those workflow pages

The grouped live auth caller family in `acme-admin`, `cp-admin`, and `dairy`
now imports the workflow pages from `patterns`, and the active auth guide
teaches that same boundary.

## Batch 72.2 - Poodle Password Requirements UI Split

- [x] Add a Poodle-owned agnostic password-requirements UI surface.
- [x] Rebuild Underlay `PasswordRequirements` as an auth-policy adapter over
      the new Poodle UI surface.
- [x] Revalidate the grouped account-password and retained auth-reset callers.

### Batch 72.2 Findings

Poodle now owns the agnostic checklist UI through `PasswordRequirements`,
driven by caller-supplied policy data. Underlay `PasswordRequirements` is now
just the auth-policy adapter:

- fetch requirements on mount
- apply fallback defaults on fetch failure
- pass normalized requirements and current password into the Poodle primitive

That split keeps server-side auth-policy behavior out of Poodle while still
moving the reusable evaluation and rendering surface into the design system.

## Final Outcome

`g01.072` is complete.

The auth boundary is tighter now:

- workflow pages stay in Underlay, but under public `patterns`
- the password-policy checklist UI is owned by Poodle
- the retained Underlay `PasswordRequirements` surface is now explicitly an
  auth-policy adapter rather than the canonical checklist renderer

## Next Task

Complete. The remaining public Underlay auth surface is now split at the right
layer boundaries.
