# 090 - Retained Patterns Stop Point

`g01.089` retired the old `DetailMeta*` helper family after moving the compact
metadata-ribbon contract into Poodle `MetaBar` / `MetaItem`.

That leaves a very small public `@decodelabs/underlay/patterns` surface:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- `SpaFormShell`

This wave exists to close the ambiguity around that last shell/workflow family.
The question is no longer whether Underlay still hides generic Poodle-ready UI
debt. The question is whether these four public pattern exports are now the
honest retained stop point.

## Scope

- `ts/src/patterns/index.ts`
- `ts/src/patterns/auth-workflows.ts`
- `ts/src/patterns/auth-workflows/`
- `ts/src/patterns/SpaFormShell.svelte`
- active consumer-app callers in:
  - `underlay-reference/acme-admin`
  - `contact-patch/cp-admin`
  - `acowtancy/dairy`

## Goals

- Confirm the strict live caller boundary for the remaining public
  `@decodelabs/underlay/patterns` workflow surface.
- Decide whether any of the remaining four exports are still generic migration
  debt or whether they now form the true retained Underlay stop point.
- Align the roadmap front doors and durable inventory to that explicit stop
  point so future work starts as a fresh boundary challenge instead of implied
  cleanup debt.

## Non-Goals

- Do not reopen settled Poodle successor work for `MetaBar`, `MetaItem`,
  `TotpInput`, or the broader Svelte contraction line.
- Do not force a speculative `client` or `nightfire` extraction into this wave.
- Do not add new compatibility wrappers.

## Caller Matrix

### `LoginPage`

Live workflow callers are still the three auth route pages:

- `underlay-reference/acme-admin/src/routes/(auth)/login/+page.svelte`
- `contact-patch/cp-admin/src/routes/(auth)/login/+page.svelte`
- `acowtancy/dairy/src/routes/(auth)/login/+page.svelte`

The remaining mentions outside those routes are store comments and tests, not a
second app-owned replacement family.

### `ForgotPasswordFlow`

Live workflow callers are still the three auth reset routes:

- `underlay-reference/acme-admin/src/routes/(auth)/forgot-password/+page.svelte`
- `contact-patch/cp-admin/src/routes/(auth)/forgot-password/+page.svelte`
- `acowtancy/dairy/src/routes/(auth)/forgot-password/+page.svelte`

### `PasswordRequirements`

The live UI callers remain grouped and workflow-adjacent:

- retained shared reset internals under `ts/src/patterns/auth-workflows/`
- account-password pages in:
  - `underlay-reference/acme-admin`
  - `contact-patch/cp-admin`
  - `acowtancy/dairy`

The generic checklist UI is already in Poodle. The remaining Underlay contract
is the auth-policy adapter over that checklist.

### `SpaFormShell`

`SpaFormShell` still has the broadest remaining Svelte caller family by far.
The current live scan shows `79` references across the active sibling repos,
concentrated in real create/edit/copy/move route families in `acme-admin`,
`cp-admin`, and especially `dairy`.

It still owns workflow behavior rather than just stale framing:

- submit interception
- result and field-error handling
- save/save-close/delete orchestration
- redirect and navigation handoff
- optional `prepare(formData)` transformation

## Judgment

The remaining public `patterns` surface is now the honest retained stop point.

- `LoginPage` remains a shared auth workflow shell, not a generic page frame.
- `ForgotPasswordFlow` remains a shared reset workflow shell, not a generic
  card-plus-form composition problem.
- `PasswordRequirements` remains a retained auth-policy adapter over the Poodle
  checklist rather than a duplicate generic UI primitive.
- `SpaFormShell` remains a real workflow shell with broad live usage and no
  smaller generic successor hiding in Poodle today.

There is no smaller honest follow-on migration wave inside `patterns` right
now. Any further change from here should begin as a fresh challenge against one
of these retained workflow contracts, not as assumed residual cleanup.

## Consumer Upgrade Impact

No consumer migration is required in this wave.

This is a stop-point clarification wave only:

- no public API changes
- no import-path changes
- no runtime behavior changes

## Status

- [x] Sweep the live caller family for the remaining public
      `@decodelabs/underlay/patterns` workflow surface.
- [x] Confirm whether any of the remaining four exports are still generic
      migration debt.
- [x] Align the roadmap front doors and durable inventory to the explicit stop
      point.

## Complete

`g01.090` is complete. The remaining public `@decodelabs/underlay/patterns`
surface is now explicit as the true retained workflow stop point:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- `SpaFormShell`

There is no active contraction or successor wave left inside `patterns`.

## Next Task

If work continues immediately, the next honest follow-on is a fresh boundary
challenge on a different retained package surface such as `client` reshaping or
future `nightfire` extraction planning, not more incremental surgery inside
`patterns`.
