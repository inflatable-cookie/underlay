# g10.001 - Northstar Instruction And Language Quality Audit

Status: active
Owner: repo maintainers
Created: 2026-09-01
Depends on: `g09` closed
Governing refs: `AGENTS.md`, `rust/AGENTS.md`,
`docs/contracts/001-working-rules.md`,
`docs/contracts/rust-quality-profile.json`,
`docs/contracts/rust-quality-deviations.json`, installed Northstar
Planning state: card 001 ready

## Problem

Underlay's reusable Rust and TypeScript/Svelte foundation has changed
substantially since the last language-quality programme. Its current root and
Rust instruction journey and all owned source now need one current,
repository-scope explicit Northstar audit. The work is maintenance, not a
pretext for a consumer rollout or a new compatibility window.

## Goals

- review root `AGENTS.md`, `rust/AGENTS.md`, and the Claude bridge as one scoped
  reader journey;
- audit every owned Rust package/target/feature and every hand-written
  TypeScript/Svelte source surface with package-aware overlays;
- record findings before mutation and apply only recorder-authorized repairs;
- preserve generated, fixture, vendored, historical, consumer-owned, release,
  and workflow boundaries;
- remove the deleted Loophole Composer repository from live Underlay fleet
  authority while preserving frozen evidence and unrelated homonyms;
- finish with one reviewable PR and honest retained findings.

## Non-Goals

- no consumer application mutation, dependency upgrade, release, workflow,
  migration, public API redesign, or compatibility programme;
- no threshold-led file splitting, blanket lint fixing, or audit-driven
  generation expansion;
- no mutation of Underlay Reference, Poodle, or any other consumer.
- no design, naming, API, migration, or rollout work for a replacement online
  service.

## Acceptance Criteria

- every applicable instruction section has a human disposition and the
  reusable-vs-consumer ownership boundary survives;
- Rust scope covers every workspace crate, target, feature, public API,
  unsafe/FFI, async/concurrency, panic/error path, and exact forwarder;
- TypeScript/Svelte scope covers every hand-written owned source unit and
  explicitly excludes fixtures, generated output, and vendor-owned material;
- MSRV 1.95 evidence is separate from current-toolchain validation;
- every changed source file maps to a prior finding and authorized plan;
- finalized recorders, changed-file attribution, limitations, roadmap, card,
  log, and front doors agree;
- live AGENTS, contracts, manifests, guides, matrices, usage docs, and active
  sweeps no longer count or instruct the deleted repository as a consumer;
- surviving `loophole-composer` or equivalent matches are classified as frozen
  history or unrelated symbols rather than removed for a zero-grep result;
- repository validation records actual warnings, unavailable surfaces, and
  failures rather than promoting them to clean evidence.

## Review Oracle

| Invariant | Adversarial counterexample | Expected response | Required proof |
| --- | --- | --- | --- |
| Repository scope is complete. | One crate target, TS/Svelte source unit, export boundary, or instruction section lacks a disposition. | Review blocks. | Inventories and finalized recorders. |
| Repairs are finding-first. | A source edit has no prior authorized finding and plan. | Reject or revert it. | Changed-file attribution. |
| Shared contracts survive. | A public Rust/TS export, wire name, auth/error contract, or realtime path changes under cleanup authority. | Stop for planning. | API/export diff and focused tests. |
| Protected data stays protected. | `Debug`, diagnostics, or UI output renders credentials, tokens, or consumer content. | Finding remains blocking. | Adversarial tests and call-path review. |
| Consumer ownership survives. | The worker edits Underlay Reference, Poodle, or another consumer. | Stop. | Changed-repository inventory. |
| Live fleet authority is current. | A live contract, manifest, guide, matrix, or instruction still counts or directs the deleted Loophole Composer repository. | Review blocks; remove the stale row or instruction and reconcile dependent counts. | Classified match inventory plus live fleet/matrix consistency check. |
| Evidence is honest. | A warning, unavailable external service, fixture exclusion, or untested target is called clean. | Result remains degraded or limited. | Raw evidence and limitations. |

## Stop Conditions

- a repair needs a consumer-visible API, dependency, MSRV, persistence,
  migration, security, unsafe/FFI, release, or compatibility decision;
- unit ownership overlaps or generated/fixture boundaries cannot be resolved;
- the Northstar source changes during the audit;
- validation changes the plan or requires consumer mutation.

## Consumer Upgrade Impact

None is authorized. Any finding that needs consumer migration or changes a
published contract stops for a later planned lane.

## Next Task

Execute ready card 001 in one isolated Underlay worker and stop at its PR for
orchestrator exact-head review.
