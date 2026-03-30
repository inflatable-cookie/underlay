# 065 - Poodle TotpInput Capability Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 064

## Overview

`g01.064` closed the remaining auth-surface reassessment.

That review split the auth family cleanly:

- `AuthLayout` was thin wrapper residue and is now retired
- `LoginPage` and `ForgotPasswordFlow` remain explicit retained Underlay auth
  composites for now
- `PasswordRequirements` remains an explicit retained Underlay auth helper for
  now
- `TotpInput` is the strongest next focused auth-helper capability candidate

This wave exists to move the reusable one-time-code input behavior out of
Underlay and into a Poodle-owned input contract, then use the retained shared
auth flows plus the grouped account-security pages in `acme-admin`,
`cp-admin`, and `dairy` as the proof family before retiring public Underlay
`TotpInput`.

## Research Basis

- current Underlay helper:
  - `ts/src/components/auth/TotpInput.svelte`
- retained shared auth internals:
  - `ts/src/components/auth/TwoFactorStep.svelte`
- grouped account-security callers:
  - `underlay-reference/acme-admin/src/routes/(app)/account/2fa/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/account/password/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/account/2fa/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/account/password/+page.svelte`
  - `acowtancy/dairy/src/routes/(app)/account/2fa/+page.svelte`
  - `acowtancy/dairy/src/routes/(app)/account/password/+page.svelte`

## Decision Summary

- `TotpInput` is a genuine reusable one-time-code input surface, not retained
  auth-workflow glue.
- The reusable contract is input behavior:
  - one hidden real input
  - visual digit slots
  - one-time-code autofill
  - paste/password-manager compatibility
- The first honest move is a focused Poodle capability landing, not another
  Underlay-local wrapper iteration.

## Consumer Upgrade Impact

- Do not add new public Underlay `TotpInput` consumers while this wave is in
  progress.

## Planned Batches

## Batch 65.1 - Poodle Contract Definition

- [x] Compare current Underlay `TotpInput` against existing Poodle input
      primitives.
- [x] Define the smallest honest Poodle one-time-code input contract.
- [x] Decide which current Underlay props/callbacks should carry forward
      directly and which should be normalized.

## Batch 65.1 Findings

- Poodle already owns the adjacent primitives:
  - [Field.svelte](../../../../poodle/packages/svelte/primitives/src/Field.svelte)
  - [TextInput.svelte](../../../../poodle/packages/svelte/primitives/src/TextInput.svelte)
  - [PinInput.svelte](../../../../poodle/packages/svelte/primitives/src/PinInput.svelte)
- `PinInput` is not the right successor surface:
  - it models multiple real cell inputs
  - current Underlay `TotpInput` intentionally uses one hidden real input with
    visual digit slots layered over it
  - that hidden-real-input contract is what enables better one-time-code
    autofill, password-manager compatibility, and simpler paste behavior
- So the next honest move is not to stretch `PinInput` into auth behavior. It
  is a dedicated Poodle one-time-code input that composes with `Field` rather
  than reusing the multi-cell input model.

## Proposed Poodle Contract

- Working shape: a new Poodle one-time-code input primitive
- The contract should keep reusable input behavior, not auth workflow nouns
- Core props to carry forward directly:
  - `value`
  - `name`
  - `label`
  - `hint`
  - `error`
  - `length`
  - `disabled`
- Behavior callbacks should be normalized toward Poodle event language:
  - current Underlay shape: `oninput`, `onchange`, `oncomplete`
  - target Poodle shape should prefer `valueChange`, `complete`, and possibly
    `commit` instead of raw DOM-style callback prop names
- Structural behavior should remain:
  - hidden real input for form submission and autofill
  - visual digit slots
  - click-to-focus slot interaction
  - digit filtering and paste support
  - `autocomplete="one-time-code"`

## Boundary Judgment

- The smallest honest implementation is a new Poodle primitive, not a
  compatibility wrapper around `PinInput`.
- Retained shared auth internals and grouped account-security pages remain the
  right proof family for the next batch.

## Batch 65.2 - Poodle Capability Landing

- [x] Implement the new Poodle one-time-code input.
- [x] Add specimen/docs coverage for the shared input contract.
- [x] Migrate retained shared auth internals onto the Poodle surface.

## Batch 65.2 Outcome

- Poodle now owns a dedicated `TotpInput` primitive instead of stretching the
  multi-input `PinInput` model.
- The new primitive keeps the reusable Underlay behavior that actually mattered:
  - one hidden real input for submission and autofill
  - visual digit slots
  - digit filtering
  - paste and password-manager compatibility
  - `autocomplete="one-time-code"`
- The retained shared auth internal proof is complete:
  - `ts/src/components/auth/TwoFactorStep.svelte` now uses Poodle `TotpInput`
- The grouped account-security proof family is also migrated already:
  - `underlay-reference/acme-admin/src/routes/(app)/account/2fa/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/account/password/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/account/2fa/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/account/password/+page.svelte`
  - `acowtancy/dairy/src/routes/(app)/account/2fa/+page.svelte`
  - `acowtancy/dairy/src/routes/(app)/account/password/+page.svelte`

## Validation

- `effigy svelte:build` in `poodle`: passed
- `effigy check` in `underlay`: passed, with only the known `PageHeader`
  `<slot>` deprecation warnings and the same pre-existing Underlay warnings
- `bun x svelte-check --tsconfig ./tsconfig.json` in `acme-admin`: `0 errors`
  and only the known `PageHeader` warnings
- `bun x svelte-check --tsconfig ./tsconfig.json` in `cp-admin`: `0 errors`
  and only the known `PageHeader` warnings
- `bun x svelte-check --tsconfig ./tsconfig.json` in `dairy`: `0 errors` and
  only the known `PageHeader` warnings

## Batch 65.3 - Proof Family And Retirement

- [x] Migrate the grouped account-security pages in `acme-admin`, `cp-admin`,
      and `dairy`.
- [x] Retire public Underlay `TotpInput` once live residue is clean.
- [x] Update auth guidance and the durable inventory to reflect the finished
      boundary.

## Batch 65.3 Outcome

- Public Underlay `TotpInput` is retired.
- The public export is removed from:
  - `ts/src/components/index.ts`
  - `ts/src/components/index.d.ts`
- The old Underlay helper implementation and Storybook story are deleted:
  - `ts/src/components/auth/TotpInput.svelte`
  - `ts/stories/Auth/TotpInput.stories.ts`
- The retained auth guide now teaches the real boundary:
  - Poodle `TotpInput`
  - retained Underlay `PasswordRequirements`
  - retained Underlay `LoginPage`
  - retained Underlay `ForgotPasswordFlow`

## Completion

`g01.065` is complete. The reusable one-time-code input contract now lives in
Poodle, the retained shared auth internals and grouped account-security proof
family are migrated, and public Underlay `TotpInput` is retired.

## Next Task

Open the next focused auth-helper follow-on on `PasswordRequirements`, then
write the strict caller and contract matrix across the retained shared auth
flows plus the grouped account-password pages in `acme-admin`, `cp-admin`, and
`dairy` before deciding whether it still earns public Underlay ownership.
