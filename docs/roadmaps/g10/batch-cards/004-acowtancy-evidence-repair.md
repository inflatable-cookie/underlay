# g10.004 - Acowtancy Evidence Repair

Status: complete
Completed: 2026-08-25
Owner: Acowtancy maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Purpose

Make the proven Acowtancy workspace describe its real dependency model.

## Scope

- Remove README claims that Underlay and Poodle compile from sibling `file:`
  dependencies.
- Explain that sibling mounts serve QA/tooling and application manifests use
  releases.
- Verify root workspace, lockfile, frozen install, and internal `workspace:*`
  language against the live manifests.

## Acceptance

- README and AGENTS no longer contradict package manifests.
- No application dependency regresses to a sibling path.
- Acowtancy remains the passing proof for the workspace-shape check.

## Validation

- root docs QA
- `effigy health`
- workspace-shape conformance check
- `git diff --check`

## Stop Conditions

Stop if a sibling path is still required for runtime compilation rather than
QA/tooling.

## Review And Merge Evidence

The worker corrected only root `README.md`, root `AGENTS.md`, and
`packages/cattle-grid/README.md`. The final prose distinguishes Effigy
QA/tooling and optional local co-development mounts from released application
dependencies, preserves the frozen root install and `workspace:*` contract,
and removes the stale Cattle Grid polyrepo link.

PR [#57](https://github.com/acowtancy/market/pull/57) merged to Acowtancy
`main` as `b995fad517783ee09b00e384f903988dccbb2b79` on 2026-08-25. The
implementation commit was `dec01128d87ae6e6d7384bf1f95d59a15e933054`.
The canonical review record is the
[Acowtancy-side orchestrator comment](https://github.com/acowtancy/market/pull/57#issuecomment-5417592324).

The PR changed no manifests, locks, task wiring, runtime code, workflows, or
historical evidence. Acowtancy docs QA, full `effigy health`, the live
workspace-shape check, and `git diff --check` passed. Health retained the known
29 Dairy Svelte warnings and no errors.

## Next Task

Closed after review, operator-authorized merge, and independent merged-state
verification. Execute `g10.005` through a fresh Underlay Reference worker
handoff. No other card is ready.
