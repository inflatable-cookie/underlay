# 061 - ErrorBoundary Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 060

## Overview

`g01.060` finished the `MediaActionsMenu` retirement wave.

The next meaningful remaining public component candidate is `ErrorBoundary`:

- it still has live app-root and shell usage
- it is infrastructure-facing rather than a design-system primitive
- but the remaining shared value may now be narrow enough that it should be
  challenged rather than treated as automatically retained

This wave compared the retained Underlay error boundary shell against the real
caller family and closed the question: it no longer earned a public shared
surface and collapsed into app-local composition over base Svelte recovery UI
and Poodle `EmptyState`/`Button`.

## Research Basis

- Underlay:
  - `ts/src/components/ErrorBoundary.svelte`
  - `ts/src/components/index.ts`
- caller sweep:
  - retained Underlay shells
  - `underlay-reference/acme-admin/src/routes/(app)/+layout.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/+layout.svelte`
- adjacent UI:
  - `@inflatable-cookie/poodle-svelte-primitives/Callout`

## Decision Summary

- `ErrorBoundary` did not retain a meaningful shared contract.
- The only live callers were two admin app roots using the default recovery UI.
- The public Underlay export is retired, and the admin roots now own local
  `svelte:boundary` composition with the same fallback behavior.

## Consumer Upgrade Impact

- Do not add new direct consumers of Underlay `ErrorBoundary`; the public
  export is gone.

## Planned Batches

## Batch 61.1 - Strict Caller Review

- [x] Audit the live `ErrorBoundary` caller family across retained Underlay
      shells and active app roots.
- [x] Separate generic recovery UI from app-shell logging, navigation, and
      retry behavior.
- [x] Decide whether the next broad batch is retained hold, narrowing, or
      direct retirement.

## Outcome

- There were no retained Underlay shell callers.
- The only live app callers were the `acme-admin` and `cp-admin` app-root
  layouts, both using the default fallback with no custom reporting or
  navigation behavior.
- The shared Underlay wrapper did not prove a durable contract beyond a tiny
  local convenience component, so the surface was retired.
- The admin apps now use app-local `AdminErrorBoundary` components over
  `svelte:boundary` plus Poodle `EmptyState`/`Button`.

## Next Task

Open the next focused reassessment wave on `CopyActionsMenu`, then compare the
retained helper against the now-retired `MediaActionsMenu` successor posture
and the remaining live caller family to decide whether it still earns a public
shared export or should collapse into direct app-local composition over Poodle
`Menu`, local clipboard handling, and local toast wiring.
