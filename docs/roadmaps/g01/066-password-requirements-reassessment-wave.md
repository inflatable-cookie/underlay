# 066 - PasswordRequirements Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 065

## Overview

`g01.065` is complete. `TotpInput` moved into Poodle, the retained shared auth
internals and grouped account-security proof family migrated, and public
Underlay `TotpInput` is retired.

That leaves one lower-level auth helper still on the public Underlay surface:
`PasswordRequirements`.

Unlike `TotpInput`, this helper may still earn retained Underlay ownership
because it sits closer to auth-policy fetching, default-rule fallback, and the
password reset/change workflow than to a pure design-system primitive.

This wave exists to test that boundary directly instead of leaving
`PasswordRequirements` implicitly retained.

## Research Basis

- current Underlay helper:
  - `ts/src/components/auth/PasswordRequirements.svelte`
- retained shared auth internals:
  - `ts/src/components/auth/PasswordResetStep.svelte`
- grouped account-password callers:
  - `underlay-reference/acme-admin/src/routes/(app)/account/password/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/account/password/+page.svelte`
  - `acowtancy/dairy/src/routes/(app)/account/password/+page.svelte`

## Decision Focus

- Determine whether `PasswordRequirements` is:
  - still an honest retained Underlay auth helper
  - a Poodle capability candidate
  - or thin enough to collapse into local app composition plus shared policy
    utilities

## Consumer Upgrade Impact

- Do not add new public Underlay `PasswordRequirements` consumers while this
  wave is in progress.

## Planned Batches

## Batch 66.1 - Contract And Caller Matrix

- [x] Compare current Underlay `PasswordRequirements` against the existing
      Poodle field and helper surface.
- [x] Sweep retained shared auth internals plus the grouped account-password
      proof family.
- [x] Decide the smallest honest next boundary before any implementation work.

## Batch 66.1 Findings

- The live caller family is narrow but real:
  - retained shared auth internal:
    - `ts/src/components/auth/PasswordResetStep.svelte`
  - grouped app pages:
    - `underlay-reference/acme-admin/src/routes/(app)/account/password/+page.svelte`
    - `contact-patch/cp-admin/src/routes/(app)/account/password/+page.svelte`
    - `acowtancy/dairy/src/routes/(app)/account/password/+page.svelte`
- Every live caller uses the same higher-order contract:
  - `password`
  - `fetchRequirements`
  - requirement fetch on mount
  - fallback policy defaults if the fetch fails
  - shared password-policy checklist rendering
- The current Poodle surface does not have a matching helper. The nearest
  primitives are lower-level composition pieces like `Field`, `Callout`, and
  `TextInput`, but no shared password-policy or checklist helper.
- Unlike `TotpInput`, this helper is not just reusable input behavior. It
  bundles auth-policy fetch, fallback policy defaults, and shared password-rule
  phrasing in a way that is still closer to auth workflow than to a generic
  design-system primitive.

## Boundary Judgment

- `PasswordRequirements` remains an honest retained Underlay auth helper for
  now.
- The next honest move is not a Poodle capability wave and not immediate
  retirement.
- If this gets challenged later, the likely split is:
  - shared auth-policy utility or data contract
  - caller-owned Poodle composition for the rendered checklist
- That split is not justified yet by the current live surface.

## Batch 66.2 - Durable Boundary Closeout

- [x] Update the auth-guide surface to record `PasswordRequirements` as an
      explicit retained Underlay auth helper.
- [x] Update the durable inventory and roadmap state to reflect the retained
      decision.
- [x] Reassess whether the auth family needs another immediate wave.

## Batch 66.2 Outcome

- The auth guide now records the real helper split explicitly:
  - Poodle `TotpInput`
  - retained Underlay `PasswordRequirements`
- The retained reason is now durable instead of implied:
  - auth-policy fetch on mount
  - fallback defaults on fetch failure
  - shared password-rule checklist rendering across retained reset/change
    flows and grouped account-password pages
- There is no honest immediate follow-on auth wave after this closeout.
  `LoginPage`, `ForgotPasswordFlow`, and `PasswordRequirements` are now the
  stable retained auth surface until a larger auth-workflow redesign or a
  cleaner auth-policy utility split is worth opening.

## Completion

`g01.066` is complete. `PasswordRequirements` remains an explicit retained
Underlay auth helper, and the auth family is at a sensible stop point for now.

## Next Task

The auth family is stable for now. The next honest work should be a different
shared-platform or domain-system wave rather than another auth-helper
reassessment.
