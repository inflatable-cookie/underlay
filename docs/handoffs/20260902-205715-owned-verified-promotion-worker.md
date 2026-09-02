---
title: Underlay owned verified promotion recovery worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260902-205715-owned-verified-promotion-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, blob, recovery, security, release]
---

## What This Thread Was Doing

Underlay v0.9.6 made verified destination creation immutable, but consumer
rollout proved it cannot distinguish its own completed create from a foreign
incumbent after process loss. Implement `g11.001` Card 003 only: additive,
token-bound ownership proof for restart recovery. Return one PR.

## Why It Matters

Underlay Reference is paused on a real recovery gap. Publication intent, key
knowledge, identical bytes, MIME, size, and ETag are not positive ownership.
The shared primitive must bind ownership to the same exclusive backend commit
as the published bytes before that consumer can finish safely.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning branch:** `main`
- **Planning base commit:** `b33b1df12da0ad2ad712bd209bd4658f63c480a5`
- **Pushed main verification:** the orchestrator will push this handoff and
  planning batch before dispatch; require local `HEAD == origin/main`
- **Planning checkout:** clean before this planning batch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 040 owned-recovery
  rule, strict promotion spec, roadmap `g11.001`, Cards 003/004, and the
  2026-09-02 owned-recovery planning log
- **Worker branch:** `worker/owned-verified-promotion-v097`
- **Worker worktree:** launcher-provided Paseo worktree
- **Worktree creation command:** Paseo `branch-off` from pushed `origin/main`
- **Worker worktree policy:** follow `Completion Protocol`; launcher worktree
  first, named/manual fallback only when required.
- **Required sibling worktree links:** none
- **Active spec lane:** `docs/specs/immutable-verified-blob-promotion.md`
- **Roadmap milestone:** `docs/roadmaps/g11/001-immutable-verified-media-publication-and-fleet-rollout.md`
- **Ready cards, in order:** `docs/roadmaps/g11/batch-cards/003-owned-verified-promotion-recovery.md`
- **Allowed runway:** Card 003 only
- **Remaining card budget:** one card
- **Dispatch topology:** one Underlay implementation lane; unrelated consumer
  and portfolio lanes may continue independently
- **Parallel safety check:** Card 004 is serial behind this reviewed merge;
  no other Underlay source or release lane may overlap
- **Surfaces this lane owns:** `rust/crates/underlay-blob/**`, focused tests,
  Contract 040, Contract 122 public API inventory, the relevant upgrade/media
  guide, changelog entry, Card 003, and one execution log
- **Integration ownership:** the orchestrator owns roadmap/front-door closeout,
  Card 004 release compilation, consumer resumption, review, and merge
- **Merge ordering:** same-repository PRs merge one at a time; the orchestrator
  refreshes this head against current `main` and re-reviews it if a sibling lane
  merges first
- **Canonical refs:** `docs/contracts/040-storage-blob-and-media-systems.md`;
  `docs/contracts/122-rust-public-api-inventory.md`;
  `docs/contracts/023-release-and-compatibility-rollout.md`
- **Review oracle:** Card 003 and the active immutable-promotion spec
- **Model capability profile:** ordinary capable implementation worker; subtle
  security/storage work is fully specified and exact-head reviewed
- **Frontier-worker justification:** none
- **Tool/runtime restrictions:** no consumer edit, release mutation, tag,
  workflow change, raw-token disclosure, or live S3 mutation; use deterministic
  S3 request fixtures and local filesystem composition
- **Required validation:** focused `underlay-blob` owned-promotion/S3/local
  tests; workspace Rust check, Clippy with denied warnings, and tests;
  `effigy qa`; `effigy doctor` with inherited findings identified;
  `git diff --check`
- **PR base/head:** `main` <- `worker/owned-verified-promotion-v097`
- **PR URL:** pending
- **Review state:** awaiting worker implementation
- **Merge path:** orchestrator after accepted review of the current head and
  passing required checks

## Boundaries

- **In scope:** Card 003 from counterexample through the smallest complete
  contract-valid repair, proof, documentation, and PR.
- **Out of scope:** Card 004 release execution, consumer schemas or code,
  existing API behavior changes, workflows, and unrelated storage features.
- **Outcome shape:** issue fix. Do not stop at diagnosis unless a named stop
  condition is proven.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- Test the local atomic-metadata stop condition before implementation. The
  accepted design is: the consumer persists a high-entropy opaque token and
  immutable destination authority before create; Underlay stores only a
  one-way verifier plus server-derived SHA-256, size, and validated MIME as
  reserved metadata atomically with exclusive create. S3 uses the same
  conditional PutObject. Local attaches equivalent metadata to the unpublished
  temp inode before its atomic link and returns it from `head`.
- Recovery may use only durable token, destination, provider/bucket authority,
  and head metadata. Never reread staging or mutable consumer/request state.
- Raw tokens must not enter object metadata, Debug/Display, URLs, errors, logs,
  or DTOs. Compare verifiers without timing-dependent early exit.
- Every absent, malformed, incomplete, mismatched, unsupported, or otherwise
  unproven case refuses. Ordinary collisions, including identical bytes, stay
  `BlobError::DestinationExists` and preserve the incumbent.
- Keep the public surface additive and existing adapters source-compatible via
  a fail-closed default. Stop if either backend cannot atomically publish bytes
  and reserved metadata or if this requires a breaking trait change.
- Work only in the clean worker worktree selected by `Completion Protocol`.
  Never edit the planning checkout or an unrelated dirty checkout.
- Do not merge the PR.

## Important Context

- **Planning lineage:** v0.9.6 / Card 001 supplied bounded capture and immutable
  create. Underlay Reference PR 14 exposed the remaining ownership gap during
  fleet rollout. The operator chose a v0.9.7 shared primitive rather than a
  consumer-specific schema workaround.
- **Why this card is ready:** ownership semantics, compatible API posture,
  backend commit boundary, recovery evidence, failure behavior, and release
  ordering are all explicit in the contract, spec, roadmap, and card.
- **Decisions and preferences:** token secrecy is not ownership; only the
  verifier atomically written with the object is. Byte equality never converts
  an ordinary collision into success.
- **Open tensions:** the local implementation may use platform metadata such as
  extended attributes, but must prove the metadata is attached before final
  visibility and that `head` reads the same committed facts. Stop if the
  supported local filesystem cannot provide that invariant.
- **Report after:** stop-condition probes, one coherent implementation/test
  tranche, or a real blocker
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

Run the `Completion Protocol` preflight before broad reads. Then read
`AGENTS.md`, `rust/AGENTS.md`, `PAPERCUTS.md`, the roadmap, Card 003, the active
spec, and Contracts 023/040/122. Resolve the exact AWS SDK and local filesystem
metadata primitives first. If both clear the stop condition, implement the
smallest additive surface and drive every oracle row.

## Completion Protocol

### Before you start

1. This handoff's `worker_mode: implementation` and
   `dispatch_authority: orchestrator` activate worker mode. Before broad reads,
   run `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. Accept a clean launcher-provided non-`main` worktree. Do not create another
   because its generated path differs from the planned branch name.
3. Fetch origin with non-interactive SSH. Require `HEAD == origin/main`, require
   `b33b1df12da0ad2ad712bd209bd4658f63c480a5` to be an ancestor, and verify this
   tracked handoff matches the absolute dispatch file. There are no sibling
   links to create.
4. Read the named authority and run the repo's cheap orientation checks.

### While you work

- Reproduce or construct every hostile case before trusting the repair.
- Keep commits aligned with meaningful implementation/test and documentation
  chunks. Append execution friction to `PAPERCUTS.md` without expanding scope.
- Stop and report if the atomic-metadata, compatibility, or security conditions
  cannot be met. Do not weaken the accepted invariant.

### When Card 003 is complete

1. Run the required validation in one final batch.
2. Falsify every universal/negative claim and map every review-oracle row to a
   deterministic test.
3. Update Card 003 and one lane log. Leave shared roadmap/front-door and Card
   004 release state to the orchestrator.
4. Push the worker branch and open one PR against current pushed `main`. If main
   moved, integrate it and re-run validation before reporting the head.
5. Report the PR URL, exact head, stop-condition evidence, public API shape,
   tests, inherited failures, and remaining release/consumer work. Do not merge.

### Review and merge path

The orchestrator reviews the exact provider head and posts every blocker on the
PR. Requested revisions return to this same worker and branch. Accepted current
head plus required passing checks is merged without another operator prompt.

- **Requested changes:** none yet
- **Closeout refs:** Card 003 and its lane log are worker-owned; roadmap/front
  doors, Card 004, release, and consumer resumption are orchestrator-owned.
