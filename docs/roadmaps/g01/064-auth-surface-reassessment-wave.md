# 064 - Auth Surface Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 063

## Overview

`g01.063` finished the `DetailPageShell` retirement wave.

The remaining public component surface in Underlay is now concentrated in auth:

- `AuthLayout`
- `LoginPage`
- `ForgotPasswordFlow`
- `TotpInput`
- `PasswordRequirements`

Those exports are no longer just incidental leftovers. They are the largest
still-public grouped component family after the long generic-surface
contraction, and they should be challenged directly instead of being treated as
implicitly permanent.

This wave exists to decide which part of that auth family still earns a shared
Underlay boundary, which part should move to Poodle, and which part should
collapse into thinner app-local composition.

## Research Basis

- Underlay auth components:
  - `ts/src/components/auth/AuthLayout.svelte`
  - `ts/src/components/auth/LoginPage.svelte`
  - `ts/src/components/auth/ForgotPasswordFlow.svelte`
  - `ts/src/components/auth/TotpInput.svelte`
  - `ts/src/components/auth/PasswordRequirements.svelte`
- live caller family:
  - `underlay-reference/acme-admin/src/routes/(auth)/+layout.svelte`
  - `underlay-reference/acme-admin/src/routes/(auth)/login/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(auth)/forgot-password/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(auth)/+layout.svelte`
  - `contact-patch/cp-admin/src/routes/(auth)/login/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(auth)/forgot-password/+page.svelte`
  - `acowtancy/dairy/src/routes/(auth)/+layout.svelte`
  - `acowtancy/dairy/src/routes/(auth)/login/+page.svelte`
  - `acowtancy/dairy/src/routes/(auth)/forgot-password/+page.svelte`
  - account 2FA and password pages that still use `TotpInput` /
    `PasswordRequirements`

## Decision Summary

- The auth family splits cleanly into three layers rather than one retained
  bundle.
- `AuthLayout` is thin wrapper residue:
  - only three live callers remain
  - each caller just centers auth children inside a Poodle `Card`
  - no reusable workflow behavior is hidden inside the wrapper
- `LoginPage` and `ForgotPasswordFlow` still form the real shared auth workflow
  surface:
  - grouped live callers in `acme-admin`, `cp-admin`, and `dairy`
  - shared multi-step auth and recovery behavior
  - app-owned API wiring only
  - they do not yet collapse cleanly into thin local composition without
    re-creating the same auth-state workflow in every app
- `TotpInput` and `PasswordRequirements` are lower-level helper surfaces and
  should be assessed after the auth workflow decision settles.

## Consumer Upgrade Impact

- Do not add new direct consumers of the public Underlay auth component family
  until this reassessment is complete.

## Planned Batches

## Batch 64.1 - Strict Auth Caller Review

- [x] Audit the live auth caller family across `acme-admin`, `cp-admin`, and
      `dairy`.
- [x] Separate auth page-shell behavior from lower-level input/helper behavior.
- [x] Decide whether the strongest next move is direct composition,
      Poodle-owned auth composites, or explicit retained Underlay ownership.

## Batch 64.2 - Retire `AuthLayout`

- [x] Migrate the three auth layouts in `acme-admin`, `cp-admin`, and `dairy`
      onto direct local Poodle `Card` composition.
- [x] Remove the public Underlay `AuthLayout` export plus its dedicated
      implementation, story, and standalone test harness.
- [x] Update the auth guide/examples and durable inventory so `AuthLayout` is
      no longer described as a stable shared auth shell.

## Batch 64.3 - Workflow Boundary Review

- [x] Compare `LoginPage` and `ForgotPasswordFlow` against the grouped
      `acme-admin`, `cp-admin`, and `dairy` auth callers.
- [x] Decide whether the workflow layer still earns shared Underlay ownership
      or should start collapsing toward thinner Poodle-plus-local composition.
- [x] Separate that workflow judgment from the later helper follow-on for
      `TotpInput` and `PasswordRequirements`.

## Workflow Findings

### `LoginPage`

- The grouped caller family is still broad and materially shared across all
  three active apps.
- The live shared behavior is not just styling:
  - password, passkey, and optional Google method tabs
  - password-login outcome handling
  - 2FA step transitions
  - optional email fallback and resend flow
  - post-verification setup-prompt routing
- The app differences stay outside the component:
  - auth client imports
  - passkey command wiring
  - redirect behavior
  - Dairy's setup-prompt snooze behavior

### `ForgotPasswordFlow`

- The grouped caller family is effectively identical across all three apps.
- The shared behavior remains real:
  - request code
  - verify code
  - reset password
  - success handoff
  - password-requirements fetch handoff
- The app differences are only API wiring.

### Boundary Judgment

- `LoginPage` and `ForgotPasswordFlow` still earn shared Underlay ownership for
  now.
- The current commonality is workflow-first, not just generic design-system
  chrome, so collapsing them now would mostly duplicate auth-state machinery in
  each app.
- The next auth follow-on should therefore stay narrower:
  - assess `TotpInput` and `PasswordRequirements` as lower-level helpers
  - only open a later auth-composite wave if we decide there is a durable
    Poodle auth-composite boundary beyond primitive composition

## Batch 64.4 - Helper Boundary Review

- [x] Compare `TotpInput` and `PasswordRequirements` against the retained auth
      flows and the account-security pages in `acme-admin`, `cp-admin`, and
      `dairy`.
- [x] Decide whether the helper layer still earns shared Underlay ownership or
      should move toward Poodle or direct local composition.
- [x] Separate the two helper judgments instead of treating them as one
      bundled auth-helper decision.

## Helper Findings

### `TotpInput`

- The live caller family is broad inside the auth domain:
  - retained shared auth internals in [TwoFactorStep.svelte](../../../ts/src/components/auth/TwoFactorStep.svelte)
  - account 2FA and password pages across `acme-admin`, `cp-admin`, and
    `dairy`
- The contract itself is genuinely reusable:
  - single hidden real input
  - visual digit slots
  - one-time-code autofill
  - paste and password-manager compatibility
- There is no current Poodle equivalent.
- Boundary judgment:
  - this is the strongest next focused auth-helper capability candidate for
    Poodle
  - it should not stay in Underlay indefinitely as a pseudo-primitive

### `PasswordRequirements`

- The live caller family is much narrower:
  - retained shared auth internals in [PasswordResetStep.svelte](../../../ts/src/components/auth/PasswordResetStep.svelte)
  - account password pages across `acme-admin`, `cp-admin`, and `dairy`
- The shared behavior is not just static display:
  - requirement fetch on mount
  - auth-policy fallback defaults
  - password-policy-specific validation rendering
- That makes it less honest as a Poodle primitive today. It is still closer to
  auth-policy workflow UI than to a broadly reusable design-system component.
- Boundary judgment:
  - keep it in Underlay for now
  - reassess later only if we split the server-policy fetch/model from the
    visual requirement checklist in a cleaner way

## Auth Helper Judgment

- `TotpInput` and `PasswordRequirements` do not resolve the same way.
- `TotpInput` should be the lead target of the next focused auth-helper wave.
- `PasswordRequirements` remains an explicit retained Underlay auth helper for
  now.

## Next Task

Execution continues in `g01.065`, which turns the `TotpInput` helper judgment
into a focused Poodle capability wave.
