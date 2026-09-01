# AGENTS (Underlay)

## Scope

`underlay` is the shared foundation other Inflatable Cookie apps build on:
reusable Rust crates, a typed TypeScript client, retained Svelte workflow and
template shells, and the cross-project guidance that keeps them coherent.

Nothing here is a private implementation detail. Every Rust crate and every
`@inflatable-cookie/underlay/*` subpath export is a published contract that
named consumer apps already import. Assume a change you make here reaches those
apps, and design for the general case rather than the caller in front of you.

## Hard Rules

- Keep shared code generic and project-agnostic.
- Do not move app-specific behavior from consumer repos into Underlay without a
  clear reusable boundary.
- Preserve the separation between `rust/`, `ts/`, `contracts/`, and `docs/`.
- Prefer extracting stable patterns over adding one-off compatibility shims.
- Treat the public Rust crate surface and the explicit TypeScript subpath
  exports as consumer contracts. `docs/contracts/122-rust-public-api-inventory.md`
  classifies which Rust APIs are stable, adapter-owned, or internal; check it
  before changing a signature, and follow
  `docs/contracts/023-release-and-compatibility-rollout.md` when a change is
  consumer-visible.
- The workspace is pre-1.0 (`0.9.x`) with MSRV 1.95. Breaking changes take the
  minor version, not the major. Do not raise MSRV, change edition, or drop a
  supported toolchain without an explicit decision.
- Treat the current consumer-app sweep family as:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy`
  - `songsprout`
  - `loophole/composer`
  - Treat each root as the rollout boundary. When config, secrets, shared admin
    surfaces, or retained template behavior change, inspect the root and all
    affected child packages inside that consumer workspace.

## Planning Authority

These are routing rules, not prohibitions. Follow them so execution cannot
outrun the plan.

- Treat `docs/roadmaps/README.md` and `docs/roadmaps/generation-index.md` as
  the live queue authority when active shared-surface or consumer-normalization
  work is in flight.
- Treat numbered files directly under the active generation as the roadmap
  queue. Strict batch cards may refine a roadmap but never replace that queue.
- Treat `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and `docs/specs/` as the strict
  control pack for the active lane.

## Effigy-First Execution

Effigy is the command surface; the contract block at the end of this file
covers how to route by job. What is specific to Underlay:

- Prefer `effigy health` as the day-to-day baseline. `effigy doctor` is useful
  for broader repo scans, but Underlay carries known structural scan backlog
  there, so its warnings are not a fresh regression signal.
- Northstar AGENTS reviews use `effigy qa:docs:agent-defaults` here. This repo
  does not ship `check:agent-instructions`.
- Fall back to raw `cargo`, `bun`, or `vitest` only when the needed operation is
  not represented in `effigy.toml`.
- First-time local bring-up from outside this repo:
  `effigy bootstrap git@github.com:inflatable-cookie/underlay.git`

## Validation

Run what your change touches, then the aggregate before you call it done.

```bash
effigy health            # cheap baseline
effigy qa                # validate + qa:docs + qa:northstar (full gate)
effigy rust:check        # cargo check --workspace --all-features
effigy rust:clippy       # denies warnings
effigy rust:test
effigy test:unit         # vitest
effigy test:components   # vitest component config
effigy test --plan       # when the test shape is what you need to know
# Use targeted raw tool commands only when Effigy does not cover the path
```

`underlay-db` Postgres integration tests need a running Docker runtime and are
`#[ignore]`d by default; see `README.md` for the bring-up.

## Documentation Rules

- Put active planning in `docs/roadmaps/`.
- Put execution evidence in `docs/logs/YYYY-MM/`.
- Keep one log per meaningful update cycle or batch.
- Do not leave compatibility shim docs behind when paths or sections change.

## Source of Truth

- `./README.md`
- `./docs/README.md`
- `./docs/vision/001-underlay-foundation-vision.md`
- `./docs/architecture/000-overview.md`
- `./docs/architecture/product-guardrails.md`
- `./docs/contracts/001-working-rules.md`
- `./docs/contracts/122-rust-public-api-inventory.md`
- `./docs/guides/README.md`
- `./docs/guides/000-overview.md`
- `./docs/guides/172-agents-files.md`
- `./docs/patterns/`
- `./docs/sweeps/`
- `./docs/roadmaps/README.md`
- `./docs/roadmaps/generation-index.md`
- `./docs/roadmaps/g10/README.md`
- `./docs/logs/README.md`

## Internal Writing Style

Use the repo-local style reference for internal work and normal replies:

- `docs/policy/internal-writing-style.md`

<!-- BEGIN EFFIGY AGENT CONTRACT -->
## Effigy Agent Contract

Use Effigy as the default command surface for supported project work.

Route by job, not by startup ritual:
- use `effigy graph` for code understanding
- use `effigy tasks` for selector inventory
- use `effigy doctor` for routing ambiguity or repo health
- use `effigy test --plan` when test execution shape matters

Use `effigy graph` when the job is code understanding: ownership, flow,
implementation, or changed-file impact. Do not insert graph into unrelated
deployment, state, docs, release, or direct task-execution work.

Prefer `effigy <task>`, `effigy test`, and the matching built-in surface over
raw package-manager or shell commands when Effigy covers the path. Use
`effigy --json <command>` whenever another agent or tool will consume output.

This repo's local `.agents/skills/effigy` copy is authoritative for this
project. When an agent supports both project-local and global skills, prefer
the project-local copy over any globally installed Effigy skill.

Do not add an explicit repo selector while already inside the target repo. Do not edit
`.github/workflows/` or run release mutations unless the user explicitly asks.

Reference docs:
- Effigy agent adoption: `docs/guides/047-agent-and-cross-repo-adoption.md`
- Graph workflows: `docs/guides/076-code-graph-and-agent-workflows.md`
- JSON contracts: `docs/guides/017-json-output-contracts.md`
<!-- END EFFIGY AGENT CONTRACT -->
