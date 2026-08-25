# g10.004 - Acowtancy Evidence Repair

Status: planned
Blocked by: `g10.003`
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

## Next Task

After merge, prepare `g10.005` in Underlay Reference.
