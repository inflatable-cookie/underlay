---
title: Underlay immutable verified blob promotion
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / Northstar orchestrator
created: 2026-09-02
updated: 2026-09-02
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260902-121455-g11-001-immutable-verified-blob-promotion.md
base_required: pushed-main
tags: [coordination, handoff, worker, blob, media, security, rollout]
---

## What This Thread Owns

Implement `g11.001` card 001 only. Add bounded capture and immutable
verified-promotion capability to `underlay-blob`, update its authoritative docs,
and return one PR. Do not edit or locally link a consumer.

## Current State

- Repository: `/Users/tom/Dev/projects/underlay`
- Planning base before this batch: `453c44d359a8f499bba4116b070d4505e4803bf8`
- Worker branch: `worker/g11-001-immutable-verified-blob-promotion`
- Required sibling links: none
- Authority: `g11.001`, card 001, strict immutable-promotion spec, Contracts
  023/040/050, root and Rust `AGENTS.md`
- PR base/head: `main` <- worker branch
- Merge path: orchestrator after exact-head review and passing checks

## Assignment

Read the handoff, roadmap, card, spec, contracts, `AGENTS.md`, `rust/AGENTS.md`,
and `PAPERCUTS.md`. Test both storage stop conditions before implementation:
the exact AWS SDK must support conditional destination create, and the local
runtime must support containment-safe exclusive no-follow creation. Then:

1. Add source-compatible, default-fail-closed bounded-capture and create-only
   byte-write methods to the blob adapter boundary.
2. Implement them for S3 and local storage. S3 reads at most max plus one
   sentinel and sends one conditional PUT. Local reads under the same bound,
   refuses symlinks/non-regular files without blocking, and never truncates or
   follows an occupied destination.
3. Add a verified-promotion result and extension method. Capture once, validate
   size/MIME/magic bytes, derive lowercase SHA-256, and create a distinct
   destination from that exact captured vector. Preserve staging.
4. Keep old mutable APIs unchanged. A collision is typed, redacted, and never
   retried as an unconditional write. If convergent retry is added, require
   exact destination byte equality; metadata or ETag alone is insufficient.
5. Update Contract 040, Contract 050 only where its higher-level guidance needs
   the new seam, the Rust public API inventory, media upload pattern/guide,
   changelog, card/roadmap/spec, and one delivery log.

## Boundaries

- No consumer edits, tag, release execution, dependency pin, workflow, app DTO,
  app migration, or app database transaction changes.
- Do not remove or silently strengthen old trait methods.
- Do not use unbounded `get_bytes` inside verified promotion.
- Do not make unsupported custom adapters compile into unsafe overwrite
  behavior; their new defaults must refuse.
- Stop on the roadmap/card conditions. Never merge the PR.

## Proof And PR

Drive every card oracle row, including oversized capture, mutable staging swap,
two-writer destination race, occupied/symlink/non-regular local destination,
S3 conditional request plus 409/412-style collision mapping as applicable to
the SDK, custom-adapter compatibility/refusal, crash/retry posture, and hostile
provider diagnostics. Run focused blob tests, repository Rust checks, docs and
Northstar QA, doctor, test plan, and `git diff --check` in one final batch.

Open one PR to `main`. Return URL, exact head, stop-condition probes, public API
shape, proof results, inherited failures, and consumer upgrade note. Revisions
stay on this agent and branch.
