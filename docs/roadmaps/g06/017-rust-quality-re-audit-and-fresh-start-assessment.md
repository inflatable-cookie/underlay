# g06.017 - Rust Quality Re-Audit And Fresh-Start Assessment

## Why

`g06.016` closed the concrete hardening backlog left by the Rust platform
transition. The next useful step is to re-audit the Rust code from the new
baseline and answer the architectural question again with the current state in
view, not the pre-hardening state.

## Goal

Run a fresh Rust quality audit and produce the updated "starting over now"
assessment.

## Scope

In scope:

- re-audit Rust modularity, API clarity, security posture, and consumer
  extensibility
- distinguish resolved issues from remaining structural debt
- identify any remaining breaking-change risk for consumer apps
- update the fresh-start recommendation against the current codebase
- open follow-up cards only for actionable, bounded work

Out of scope:

- broad implementation work
- release execution or publishing
- consumer repo mutations unless the audit finds a concrete required update

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- current Rust state is audited after `g06.016`
- remaining issues are ranked by severity and consumer impact
- fresh-start recommendation is updated from the current baseline
- validation state and known gaps are recorded
- next implementation card is opened only if the audit finds bounded follow-up

## Current State

`g06.017` is complete.

Current audit result:

- Acute supply-chain posture is improved. `cargo-deny` passes advisories,
  licenses, sources, and bans with one explicit no-fix advisory acceptance for
  `RUSTSEC-2023-0071`.
- Underlay JWT signs and verifies with EdDSA. The accepted RSA advisory remains
  transitive through `jsonwebtoken`, not an active Underlay RSA signing path.
- The remaining Rust quality risk is structural, not a new security blocker:
  Effigy doctor still reports oversized files, comment-ratio drift, and
  attention-marker residue.
- The highest-value Rust cleanup targets are `underlay-migration-core`,
  `underlay-jobs`, `underlay-media`, and the remaining large auth test files.
- The current public API direction is materially better than the original
  broad shared-crate shape: typed construction boundaries, narrower adapter
  seams, and explicit compatibility notes are now in place.

Fresh-start assessment:

- Start with fewer crates, organized around platform capabilities rather than
  implementation families.
- Keep only stable public contracts at crate roots; put adapters behind
  capability modules and feature gates from the beginning.
- Design typed IDs, schema/table identifiers, cookie/cors config, storage keys,
  and auth tokens as first-class construction boundaries on day one.
- Put release compatibility, supply-chain policy, and consumer impact
  classification into the normal development loop, not a late audit pass.
- Keep migration/devtools code in a separate tool-facing package boundary so it
  cannot grow into runtime platform surface by accident.

Remaining follow-up:

- Split the remaining high-severity Rust god-files in bounded batches.
- Revisit the `jsonwebtoken` / `rsa` advisory when upstream publishes a safe
  patched path or when a lean EdDSA-only JWT dependency becomes worth adopting.
- Decide whether internal path dependency versioning should become a real
  `cargo-deny` bans policy or stay explicitly allowed for the workspace.

## Next Task

Open the next bounded Rust structural cleanup card if we continue the lane.
