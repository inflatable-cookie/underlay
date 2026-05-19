# AGENTS (Underlay)

## Scope

`underlay` is the shared foundation repo for reusable Rust crates, TypeScript utilities, Svelte components, and cross-project guidance.

## Hard Rules

- Keep shared code generic and project-agnostic.
- Do not move app-specific behavior from consumer repos into Underlay without a clear reusable boundary.
- Preserve the separation between `rust/`, `ts/`, `contracts/`, and `docs/`.
- Prefer extracting stable patterns over adding one-off compatibility shims.
- Treat `docs/roadmaps/README.md` and `docs/roadmaps/g02/README.md` as the
  live queue authority when active shared-surface or consumer-normalization
  work is in flight.
- Treat `docs/architecture/product-guardrails.md`,
  `docs/contracts/001-working-rules.md`, and `docs/specs/` as the strict
  control pack for the active lane.
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

## Effigy-First Execution

- Start with `effigy tasks` to inspect Underlay's local task surface.
- Prefer `effigy health` as the default repo-owned baseline.
- Use `effigy test --plan` before picking a concrete test runner.
- Prefer local Effigy tasks such as `effigy qa:docs`, `effigy qa:northstar`, `effigy validate`, `effigy rust:check`, `effigy rust:test`, and `effigy test:components`.
- Use `effigy doctor` when you want broader repo scans; it currently includes structural scan findings beyond the task surface itself.
- Fall back to raw `cargo`, `bun`, or `vitest` only when the needed operation is not represented in `effigy.toml`.

For first-time local bring-up from outside this repo:
- use `effigy bootstrap git@github.com:inflatable-cookie/underlay.git`

## Validation

```bash
effigy health
effigy qa:docs
effigy qa:northstar
effigy validate
effigy test --plan
# Use targeted raw tool commands only when Effigy does not cover the path
```

## Source of Truth

- `./README.md`
- `./docs/architecture/product-guardrails.md`
- `./docs/contracts/001-working-rules.md`
- `./docs/roadmaps/README.md`
- `./docs/roadmaps/g02/README.md`
- `./docs/guides/README.md`
- `./docs/guides/000-overview.md`
- `./docs/guides/172-agents-files.md`

## Internal Writing Style

Use the repo-local style reference for internal work and normal replies:

- `docs/policy/internal-writing-style.md`

<!-- BEGIN EFFIGY AGENT CONTRACT -->
## Effigy Agent Contract

Use Effigy as the default command surface for supported project work.

Default entry sequence:
1. Run `effigy doctor`.
2. Run `effigy tasks`.
3. Run `effigy test --plan`.

Use `effigy graph` when the job is code understanding: ownership, flow,
implementation, or changed-file impact. Do not insert graph into unrelated
deployment, state, docs, release, or direct task-execution work.

Prefer `effigy <task>`, `effigy test`, and the matching built-in surface over
raw package-manager or shell commands when Effigy covers the path. Use
`effigy --json <command>` whenever another agent or tool will consume output.

This repo's local `.agents/skills/effigy` copy is authoritative for this
project. When an agent supports both project-local and global skills, prefer
the project-local copy over any globally installed Effigy skill.

Do not add `--repo .` while already inside the target repo. Do not edit
`.github/workflows/` or run release mutations unless the user explicitly asks.

Reference docs:
- Effigy agent adoption: `docs/guides/047-agent-and-cross-repo-adoption.md`
- Graph workflows: `docs/guides/076-code-graph-and-agent-workflows.md`
- JSON contracts: `docs/guides/017-json-output-contracts.md`
<!-- END EFFIGY AGENT CONTRACT -->
