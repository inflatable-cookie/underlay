# 074 - Non-Public Svelte Surface Recovery

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 073

## Overview

`g01.073` correctly classified the remaining public Underlay surface, but it
stopped too early. The public boundary is now much smaller, yet `ts/src`
still contains a broader tail of non-public Svelte wrappers that should have
been retired during the earlier contraction work.

This wave exists to recover from that mismatch. It is not another broad Poodle
reassessment line. It is a structured sweep over the remaining non-public
Svelte surface to remove dead wrappers, collapse stale internal composition,
and leave only the truly necessary retained pattern/auth/editor components.

## Research Basis

- `ts/src/components/**/*.svelte`
- `ts/src/patterns/**/*.svelte`
- live internal references in `ts/src`
- live consumer references across sibling repos

## Decision Focus

- delete dead non-public wrappers with no remaining internal callers
- keep only the internal implementation files still required by retained
  auth/pattern/nightfire boundaries
- surface any remaining cross-repo deep-import drift before deleting files that
  other repos still rely on

## Planned Batches

## Batch 74.1 - Dead Wrapper Tier Removal

- [x] Reopen the roadmap/front-door state as a recovery wave.
- [x] Delete the clearly dead non-public wrapper tier with no remaining
      internal callers:
  - `ActionArea`
  - `AlertDialog`
  - `CompactGroupedBar`
  - `CompactGroupedBarGroup`
  - `ContentCard`
  - `DetailsCard`
  - `DetailsItem`
  - `DetailsSection`
  - `Dialog`
  - `FileUpload`
  - `InlineActionGroup`
  - `MediaThumbnail`
  - `Skeleton`
- [x] Delete the dead component-test residue tied to that tier.
- [ ] Revalidate the Underlay repo and refresh the durable inventory/front
      doors around the recovery status.

### Batch 74.1 Findings

The first dead-wrapper tier was real residue, not a retained internal layer.
These files had no remaining live internal callers in `underlay/ts/src`, and
the active app/docs surface no longer depended on them either.

The remaining non-public Svelte surface is now concentrated in three places:

- retained auth workflow implementation
- retained form/shell internals (`Form`, `FormShell`)
- retained editor/runtime internals (`nightfire`, plus the smaller legacy form
  widgets it still depends on today)

## Batch 74.2 - Tabs Compat Retirement

- [x] Confirm the `Tabs*` compatibility family has no remaining live callers.
- [x] Delete:
  - `TabsRoot`
  - `TabsList`
  - `TabsTrigger`
  - `TabsContent`
  - `tabs-compat.ts`
- [ ] Revalidate the repo and reset the next internal-tail target around the
      remaining widget/editor layer.

### Batch 74.2 Findings

The tabs compatibility layer was fully dead. There were no remaining live
callers in `underlay`, the active app repos, or the wider workspace beyond
historical docs/log references.

That means the remaining non-public Svelte tail is now narrower and more
honest:

- retained auth workflow implementation
- retained form/shell internals (`Form`, `FormShell`)
- retained editor/runtime internals and the smaller widget layer they still
  depend on today:
  - `MarkdownEditor`
  - `TextInput`
  - `NumberInput`
  - `Select`

## Next Task

Execute Batch 74.3 by sweeping the remaining internal widget/editor tier:
`MarkdownEditor`, `TextInput`, `NumberInput`, and `Select`. Start with the
nightfire and retained-auth caller matrix, then decide which should collapse
into direct Poodle usage, which should move under `nightfire`, and which, if
any, still earn a shared internal Underlay home.

## Batch 74.3 - Widget Ownership Cut

- [x] Confirm `TextInput` and `NumberInput` have no remaining live internal or
      consumer callers.
- [x] Delete the dead `TextInput` / `NumberInput` wrapper tier and the old
      local text-input affordance files.
- [x] Move `Select` ownership under `nightfire` and update retained Nightfire
      callers/tests to use the new local path.
- [x] Move the EasyMDE-backed markdown-editor runtime and context contract
      under `nightfire/markup` ownership, updating retained callers/tests to
      the new path.
- [ ] Revalidate the repo and reset the remaining non-public tail around the
      auth/form implementation layer.

### Batch 74.3 Findings

The widget/editor tier split cleanly:

- `TextInput` and `NumberInput` were dead residue. There were no remaining
  live Underlay or consumer-app callers after the broader Poodle migration
  sweep, so keeping those wrappers in `ts/src/components` was just stale
  surface area.
- `Select` and the EasyMDE-backed `MarkdownEditor` were not really shared
  Underlay widgets anymore. Their only remaining callers were retained
  Nightfire internals, so the honest move was ownership transfer under
  `ts/src/nightfire` rather than pretending they still belonged in shared
  `components/`.

That leaves the non-public `ts/src/components` tail much smaller and more
honest:

- retained auth workflow implementation
- retained form/shell internals (`Form`, `FormShell`)
- tiny leftover helper files like field/date formatting support

## Next Task

Execute Batch 74.4 by taking the remaining non-public auth/form implementation
layer together:
- `Form.svelte`
- retained auth internals under `ts/src/components/auth/`

Start with a strict internal caller matrix from `SpaFormShell`, retained auth
workflow pages, and any Nightfire/runtime overlap, then collapse or internalize
anything that no longer needs to sit in shared `components/` while preserving
the minimal truly necessary pattern/auth implementation boundary.

## Batch 74.4 - Auth and Form Internalization

- [x] Confirm `Form.svelte` is only a `FormShell` helper and not a real
      retained shared component surface.
- [x] Fold the `Form` behavior into `FormShell` and delete the dead helper/test
      residue.
- [x] Move the retained auth workflow implementation from
      `ts/src/components/auth/` into `ts/src/patterns/auth-workflows/`.
- [x] Update tests/stories and the public pattern re-export surface to the new
      auth-workflow ownership.
- [ ] Revalidate the repo and reset the final non-public cleanup target around
      the tiny remaining helper tail.

### Batch 74.4 Findings

This layer also split cleanly:

- `Form.svelte` was pure single-caller residue behind `FormShell`, so keeping
  it in `components/` added no real shared boundary. The submit-prepare /
  enhancement behavior now lives directly in `FormShell`.
- `LoginPage`, `ForgotPasswordFlow`, and their supporting leaf files were never
  honest `components/`. They are retained implementation for public auth
  workflow patterns, so they now live under `patterns/auth-workflows/`.

After this batch, the non-public `components/` Svelte surface is down to the
single retained auth-policy adapter:

- `auth/PasswordRequirements.svelte`

Everything else left in `ts/src/components` is non-Svelte helper/type support:

- `auth/types.ts`
- `date-range.ts`
- `field/a11y-context.ts`

## Next Task

Execute Batch 74.5 by taking the final helper-placement tail together:
- `ts/src/components/auth/PasswordRequirements.svelte`
- `ts/src/components/date-range.ts`
- `ts/src/components/field/a11y-context.ts`

Confirm whether `PasswordRequirements` is the only justified remaining
`components/` Svelte file, move any non-component helpers to a better neutral
home if needed, and then decide whether `g01.074` can close with `components/`
reduced to its true minimal retained boundary.

## Batch 74.5 - Final Helper Placement

- [x] Reassess the last helper-placement tail in `ts/src/components/`:
  - `auth/PasswordRequirements.svelte`
  - `date-range.ts`
  - `field/a11y-context.ts`
- [x] Move the date-range formatters to the more honest i18n home under
      `ts/src/patterns/i18n/`.
- [x] Delete the dead `field/a11y-context.ts` helper and its test residue.
- [x] Confirm that `PasswordRequirements.svelte` is now the only justified
      remaining Svelte file under `ts/src/components/`.
- [x] Move that final retained auth-policy adapter into `patterns/` and remove
      the now-empty public `components` barrel entirely.
- [x] Revalidate the repo and close the recovery wave with `ts/src/components/`
      removed.

### Batch 74.5 Findings

The final helper-placement tail collapsed completely:

- the date-range formatter now lives under `patterns/i18n`
- the dead field a11y helper is gone
- `PasswordRequirements` now lives under `patterns/auth-workflows/`
- the public `components` barrel and its package export are gone

That means the recovery line no longer ends at a "minimal components surface".
It ends with no `components/` package surface at all. The remaining Underlay
Svelte UI is entirely `patterns/` plus the retained `nightfire/` package.

The final helper tail also resolved cleanly:

- `PasswordRequirements.svelte` is the only remaining justified Svelte file in
  `ts/src/components/`. It still represents the retained auth-policy adapter
  boundary over the Poodle checklist UI.
- `date-range.ts` was not really component-owned. It now lives under
  `patterns/i18n/date-range.ts`, which matches what it actually is: shared
  formatting logic.
- `field/a11y-context.ts` had no remaining callers after the old local input
  wrappers were removed, so it was dead residue and is now gone.

At this point the `components/` tree is reduced to its true minimal retained
boundary:

- `auth/PasswordRequirements.svelte`
- `auth/types.ts`
- barrel/type files

## Complete

`g01.074` is complete. The non-public Svelte recovery line is finished, and the
old hidden wrapper residue is gone.
