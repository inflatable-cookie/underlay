# 062 - CopyActionsMenu Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 061

## Overview

`g01.061` finished the `ErrorBoundary` retirement wave.

The next meaningful remaining public pattern candidate is `CopyActionsMenu`:

- it is still a small public helper over `Menu`, clipboard copy, and toast
  feedback
- `MediaActionsMenu` is now gone, so the broader menu-action workflow family is
  no longer hiding this helper's true boundary
- the remaining live caller family should now be checked directly rather than
  treating it as automatically retained

This wave compared the retained Underlay `CopyActionsMenu` helper against the
real remaining callers and closed the question: it no longer earned a public
shared export and collapsed into app-local composition over Poodle `Menu`,
local clipboard handling, and local toast wiring.

## Research Basis

- Underlay:
  - `ts/src/patterns/CopyActionsMenu.svelte`
  - `ts/src/patterns/index.ts`
- caller sweep:
  - retained Underlay shells
  - `underlay-reference/acme-admin`
  - `contact-patch/cp-admin`
  - `acowtancy/dairy`

## Decision Summary

- `CopyActionsMenu` did not retain a meaningful shared contract.
- The live caller family was a small set of admin pages in `acme-admin` and
  `cp-admin` with no `dairy` or retained Underlay shell callers.
- The public Underlay export is retired, and the remaining convenience is now
  owned by small app-local helpers.

## Consumer Upgrade Impact

- Do not add new direct consumers of Underlay `CopyActionsMenu`; the public
  export is gone.

## Planned Batches

## Batch 62.1 - Strict Caller Review

- [x] Audit the live `CopyActionsMenu` caller family across retained Underlay
      shells and active app repos.
- [x] Separate generic clipboard/toast ergonomics from app-local action
      workflow and trigger composition.
- [x] Decide whether the next broad batch is retained hold, narrowing, or
      direct retirement.

## Outcome

- There were no `dairy` callers and no retained Underlay shell callers.
- The live caller family was limited to a few `acme-admin` and `cp-admin`
  routes using the same small menu shape: local actions, local copy labels, and
  shared `copyToClipboard` toast behavior.
- No missing Poodle capability was proven. The Underlay helper was only a tiny
  convenience layer over Poodle `Menu` and Underlay clipboard/toast utilities.
- The public Underlay pattern is retired, and the admin apps now own local
  `CopyActionsMenu` helpers.

## Next Task

Open the next focused reassessment wave on `DetailPageShell`, then compare the
retained structural shell against the active app caller family and the now much
smaller public Underlay surface to decide whether it still earns a public
shared export or should start collapsing into thinner app-local composition
over Poodle `PageHeader`, `Tabs`, and detail primitives.
