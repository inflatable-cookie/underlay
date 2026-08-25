# g10.010 - Fleet Proof And Closeout

Status: planned
Blocked by: `g10.006`, `g10.007`, `g10.008`, `g10.009`
Owner: repo maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

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

## Next Task

Return to the `g10` candidate runway after closeout.
