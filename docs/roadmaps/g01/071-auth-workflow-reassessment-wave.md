# 071 - Auth Workflow Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 070

## Overview

`g01.070` is complete. `DetailMeta*` remains an explicit retained Underlay
helper family, so the queue should stop pretending that compact detail rows are
the next easy retirement.

The strongest remaining public component surface to challenge is the retained
auth workflow family:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`

These were already reviewed once, but the public Underlay UI surface is now
much smaller. That changes the boundary question enough that a fresh strict
caller and contract pass is now justified.

## Research Basis

- current public auth surface:
  - `ts/src/components/auth/LoginPage.svelte`
  - `ts/src/components/auth/ForgotPasswordFlow.svelte`
  - `ts/src/components/auth/PasswordRequirements.svelte`
- representative live callers:
  - `underlay-reference/acme-admin/src/routes/(auth)/`
  - `contact-patch/cp-admin/src/routes/(auth)/`
  - `acowtancy/dairy/src/routes/(auth)/`
  - account password pages in the same three apps

## Decision Focus

- Recheck whether the remaining retained auth workflow surface still earns
  shared public Underlay ownership
- or whether the combination of Poodle primitives plus thinner app-local
  composition is now good enough to challenge that boundary again

## Consumer Upgrade Impact

- Do not add new public auth workflow consumers while this wave is in progress.

## Planned Batches

## Batch 71.1 - Caller And Contract Matrix

- [x] Sweep the live caller family for `LoginPage`, `ForgotPasswordFlow`, and
      `PasswordRequirements` across `acme-admin`, `cp-admin`, and `dairy`.
- [x] Compare the retained auth workflow contract against current Poodle
      primitives and the now-reduced public Underlay surface.
- [x] Decide whether the family still earns shared public ownership or whether
      one smaller follow-on migration/capability wave is now honest.

### Batch 71.1 Findings

The live caller family is still small but real, and the auth boundary has not
thinned enough to make another migration wave honest.

Current active-app usage is:

- `LoginPage`: 3 live route callers
  - `acme-admin/src/routes/(auth)/login/+page.svelte`
  - `cp-admin/src/routes/(auth)/login/+page.svelte`
  - `dairy/src/routes/(auth)/login/+page.svelte`
- `ForgotPasswordFlow`: 3 live route callers
  - `acme-admin/src/routes/(auth)/forgot-password/+page.svelte`
  - `cp-admin/src/routes/(auth)/forgot-password/+page.svelte`
  - `dairy/src/routes/(auth)/forgot-password/+page.svelte`
- `PasswordRequirements`: 3 live account-password callers plus retained shared
  auth internals
  - `acme-admin/src/routes/(app)/account/password/+page.svelte`
  - `cp-admin/src/routes/(app)/account/password/+page.svelte`
  - `dairy/src/routes/(app)/account/password/+page.svelte`
  - retained shared `PasswordResetStep.svelte`

The contract review still shows three different retained reasons:

- `LoginPage` is still a shared auth workflow shell, not just framing:
  - multi-method login state
  - passkey and optional Google handoff
  - 2FA transitions
  - optional email fallback/resend behavior
  - setup-prompt handoff after verification
- `ForgotPasswordFlow` is still a shared reset workflow:
  - request-code step
  - verify-code step
  - password-reset step
  - success completion step
- `PasswordRequirements` is still a shared auth helper rather than a generic
  design-system primitive:
  - requirement fetch on mount
  - sensible fallback policy defaults
  - shared password-rule phrasing and checklist rendering across reset/change
    flows

Current Poodle primitives cover the low-level form controls and layout pieces,
but they do not replace the auth-state orchestration or policy-fetch behavior
that these retained surfaces still own.

The app differences are still mostly API wiring, redirect targets, and small
copy or prompt variations. Collapsing these surfaces now would mostly
duplicate auth-state machinery in three apps rather than honestly simplifying
the boundary.

## Current Judgment

`LoginPage`, `ForgotPasswordFlow`, and `PasswordRequirements` still earn shared
public Underlay ownership for now.

The next honest move is not another migration or Poodle capability wave. It is
a guide and inventory closeout that records the remaining auth surface as an
explicit retained stop point, then resets the queue around the next non-auth
public shell/helper question.

## Batch 71.2 - Guide And Inventory Closeout

- [x] Update the active auth guide surface so the retained workflow/helper
      boundary is explicit.
- [x] Update the roadmap front doors and durable inventory so the queue no
      longer treats the remaining auth family like an immediate migration
      target.
- [x] Reset the queue around the next honest non-auth public-surface
      challenge.

### Batch 71.2 Findings

The active guide surface and durable inventory now record the real stop point:

- `LoginPage` remains a retained shared auth workflow shell
- `ForgotPasswordFlow` remains a retained shared reset workflow shell
- `PasswordRequirements` remains a retained auth-policy helper
- `TotpInput` remains a completed Poodle migration

That closes the auth reassessment family cleanly. There is no smaller honest
follow-on migration wave hiding here.

With `g01.071` complete, the public-surface contraction line is now at an
explicit stop point: the remaining public Underlay surface is retained on
purpose rather than by drift.

## Next Task

Complete. The remaining public Underlay auth, shell, and helper surfaces are
explicit retained boundaries rather than migration debt.
