# g10.010 - Fleet Proof And Closeout

Status: complete
Completed: 2026-08-26
Owner: repo maintainers
Spec: `docs/specs/archive/monorepo-consumer-workspace-rollout.md`

## Purpose

Prove the six-consumer family against the new contract and close the rollout
without leaving transient guidance behind.

## Scope

- Run the workspace-shape check against all six consumer roots.
- Record final manifest, lock, internal dependency, and Effigy evidence.
- Update roadmap, spec, log, generation front doors, and consumer upgrade notes.
- Redistribute Underlay Build bootstrap guidance from the canonical docs.
- Classify or remove any rollout-only triage notes.

## Acceptance

- All six roots pass the same conformance check.
- No active guide, prompt, skill, or fixture advertises polyrepo support.
- No consumer retains child Bun locks or internal `file:` dependencies.
- The next `g10` card is explicit and unrelated rollout scope is not inferred.

## Validation

- six-consumer workspace-shape evidence
- Underlay `effigy qa:docs`, `effigy qa:northstar`, and `effigy validate`
- targeted consumer health/check evidence from merged PRs
- `git diff --check`

## Stop Conditions

Stop if any consumer PR is unmerged, any exception lacks contract authority, or
distribution cannot be verified.

## Completion Evidence

| Consumer | Verified `main` | Result |
| --- | --- | --- |
| Acowtancy | `5d1a55d6dad58211ab7503332c9ae8b0bb564211` | workspace shape passed |
| Underlay Reference | `40924bc93fc9bf29a0a5d686cd1870f728ca48ce` | workspace shape passed |
| Contact Patch | `3c85a5e57ce29af448c338f7fd29ad9e45d72ac8` | workspace shape passed |
| Compli Me | `240dce062ef5f0817b34caffaf7743542337d45a` | workspace shape passed |
| Songsprout | `618a5323571fcb2db8f4fac82a42a0b469274d4e` | workspace shape passed |
| Composer | `153b47afa68b61aaaf7e64daa6d79ac0be566343` | workspace shape passed |

Active contracts, guides, prompts, and the `underlay-build` skill now teach one
Git root with `apps/*`, `packages/*`, root `docs/`, one Bun workspace manifest,
and one root lock. The completed strict spec is archived. Full closeout evidence
lives in `docs/logs/2026-08/26-151525-g10-006-010-fleet-closeout.md`.

## Next Task

Execute `g10.011`, the foundation and transport contract assessment.
