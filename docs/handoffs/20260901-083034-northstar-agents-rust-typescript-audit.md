---
title: Underlay Northstar instruction and language audit
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom / Northstar orchestrator
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260901-083034-northstar-agents-rust-typescript-audit.md
base_required: pushed-main
tags: [coordination, handoff, worker, audit, agents, rust, typescript]
---

## What This Thread Was Doing

The operator opened `g10` for a current repository-scope Northstar audit of
Underlay's instruction journey, 37-crate Rust workspace, and TypeScript/Svelte
foundation. This worker owns `g10.001` and card 001 only.

## Current State

- **Repository:** `/Users/tom/Dev/projects/underlay`
- **Planning base:** `09f2641b` on pushed `main`
- **Worker branch:** `worker/northstar-agents-rust-typescript-audit`
- **Worker workspace:** Paseo-managed worktree; record its actual path
- **Required sibling links:** none
- **Authority:** `g10.001`, g10 card 001, strict audit spec, root and Rust
  `AGENTS.md`, working rules, Rust profile/deviations, installed Northstar
- **Northstar source:**
  `/Users/tom/Dev/projects/northstar/skills/northstar` at
  `dbce3856be6ec6093d2e5c071568a6dbe953df49` or later
- **PR base/head:** `main` <- `worker/northstar-agents-rust-typescript-audit`
- **Merge path:** orchestrator after exact-head review and passing checks

## Assignment

Run the complete card, not a sample:

1. Apply the Northstar AGENTS review to root `AGENTS.md`, `rust/AGENTS.md`, and
   the Claude bridge.
2. Run the explicit Northstar Rust audit across every workspace crate, target,
   and feature.
3. Run the explicit Northstar TypeScript/Svelte audit across every hand-written
   owned source unit with correct overlays. Inventory fixtures, but do not use
   fixture code as production-quality evidence.
4. Record findings before mutation and apply only recorder-authorized repairs.
5. Reconcile recorders, card, roadmap, spec, front doors, log, limitations, and
   PR body at closeout.

Use the repository-local Effigy skill for routing. Record the exact Northstar
source hash and do not mix audit tool versions.

## Boundaries

- Do not edit Underlay Reference, Poodle, or another consumer.
- Do not change dependencies, MSRV, published APIs, compatibility posture,
  migrations, workflows, releases, or consumer behavior under audit authority.
- Stop for a public contract, security, persistence, unsafe/FFI, realtime,
  release, or consumer-migration decision not settled by `g10.001`.
- Never edit the planning checkout. Never merge the PR.

## Preflight

1. Read this tracked handoff, root and Rust `AGENTS.md`, `g10.001`, card 001,
   and the strict spec.
2. Confirm `HEAD == origin/main`, planning base `09f2641b` is an ancestor, the
   worktree is clean, and branch/worktree match this lane.
3. Load the Northstar router and explicit AGENTS, Rust, and TypeScript/Svelte
   audit modes from the source above. Inventory repository tasks with Effigy
   before selecting validation.

## Proof And PR

Meet card 001's inventories, finalized recorder reports, changed-file
attribution, focused tests, repository QA, and adversarial review oracle. Open
one PR to `main`; report its URL and exact head. Do not merge.

If review requests changes, remain on this branch. The orchestrator will wake
this same worker; repair only posted in-bounds findings and report a new head.
