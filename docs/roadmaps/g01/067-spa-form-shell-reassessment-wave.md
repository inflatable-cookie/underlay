# 067 - SpaFormShell Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 066

## Overview

`g01.066` is complete. `PasswordRequirements` remains an explicit retained
Underlay auth helper, and the auth family is at a sensible stop point.

The strongest remaining shared structural shell is `SpaFormShell`.

This surface still has a broad live caller family across create/edit routes,
especially in Dairy. It bundles status framing, submit/result handling,
redirect flow, and form-page structure in a way that may or may not still
justify a public Underlay shell now that so many generic pieces have already
moved to Poodle.

This wave exists to answer that directly.

## Research Basis

- current shared shell:
  - `ts/src/patterns/SpaFormShell.svelte`
- related internal shell:
  - `ts/src/patterns/FormShell.svelte`
- representative live caller family:
  - create/edit routes across `acowtancy/dairy`
  - any surviving `acme-admin` and `cp-admin` create/edit pages
- related guidance:
  - `docs/guides/190-upgrade-compatibility.md`

## Decision Focus

- Determine whether `SpaFormShell` still earns public Underlay ownership as a
  reusable structural shell
- or whether it has become thin enough to collapse into:
  - local route composition
  - Poodle status/framing primitives
  - retained Underlay form/navigation utilities only

## Consumer Upgrade Impact

- Do not add new public `SpaFormShell` consumers while this wave is in
  progress.

## Planned Batches

## Batch 67.1 - Contract And Caller Matrix

- [x] Sweep the live `SpaFormShell` caller family across the active apps.
- [x] Compare the shared shell contract against current Poodle framing/status
      surfaces and retained Underlay form/navigation helpers.
- [x] Decide the smallest honest next boundary before opening any migration
      batch.

### Batch 67.1 Findings

The strict caller sweep shows that `SpaFormShell` is not thin stale wrapper
residue.

The active caller family is still broad and concentrated in real create/edit
route work, especially across Dairy. Representative callers range from simple
admin create pages to higher-variance edit and transform flows:

- simple create routes in `acme-admin`
- grouped edit routes in `cp-admin`
- large create/edit/copy/move flows in `acowtancy/dairy`

The contract still owns shared workflow behavior rather than just framing:

- SPA submit interception over retained form helpers
- submit/loading/result state lifecycle
- success, error, and field-error framing
- redirect and navigate handoff
- optional `prepare(formData)` transformation
- page-shell composition through `FormShell`

Current Poodle already owns most of the generic visual pieces around this shell:

- `PageHeader`
- `Card`
- `FormActions`
- `PageLoading`
- callout/status framing

But the live review does not show a smaller honest Poodle gap. What remains is
controller-style SPA form workflow, not missing design-system capability.

## Current Judgment

`SpaFormShell` still earns retained public Underlay ownership for now.

The next honest move is not a migration batch. It is explicit retained-shell
documentation and a queue reset so the next active wave does not accidentally
treat `SpaFormShell` like another obvious-equivalent successor surface.

## Next Task

Execute `g01.068` Batch `68.1` by writing the strict caller and contract matrix
for `AiRoutingAdmin` across the live guide/example surface and any remaining
app usage, then decide whether it still earns a public Underlay export or
should start collapsing into internal helpers plus direct Poodle composition.
