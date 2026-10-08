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
  exports as consumer contracts. `docs/knowledge/contracts/122-rust-public-api-inventory.md`
  classifies which Rust APIs are stable, adapter-owned, or internal; check it
  before changing a signature, and follow
  `docs/knowledge/contracts/023-release-and-compatibility-rollout.md` when a change is
  consumer-visible.
- The workspace is pre-1.0 (version in `Cargo.toml` `[workspace.package]` and
  `package.json`) with MSRV 1.95. Breaking changes take the minor version, not
  the major. Do not raise MSRV, change edition, or drop a supported toolchain
  without an explicit decision.
- Treat the current consumer-app sweep family as:
  - `underlay-reference`
  - `contact-patch`
  - `compli-me`
  - `acowtancy`
  - `songsprout`
  - Treat each root as the rollout boundary. When config, secrets, shared admin
    surfaces, or retained template behavior change, inspect the root and all
    affected child packages inside that consumer workspace.

## Knowledge and Planning

Underlay uses lean Northstar (`northstar` skill). The repository holds
knowledge and code; Queue holds tasks, briefs, status and outcomes. Never write
task status, handoffs or delivery logs into the repository.

- `docs/README.md` — current state and the doc map.
- `docs/knowledge/` — current truth, one owner per fact. Contracts are
  normative; guides only explain them.
- `docs/knowledge/retired.toml` — concepts that must not come back.
- `docs/knowledge/questions.md` — open questions; check here before asking.
- `docs/knowledge/contracts/release.md` — how a release is cut.

When a change alters what is true, update the owning knowledge file in the same
PR. An operator ruling given in conversation goes into its owning file before
the thread ends.

## Papercuts

File small, recurring friction in Queue with `papercut.add` (see the
`northstar` skill). The repository holds no papercut file or triage folder.

## Effigy-First Execution

Effigy is the command surface; the contract block at the end of this file
covers how to route by job. What is specific to Underlay:

- Prefer `effigy health` as the day-to-day baseline. `effigy doctor` is useful
  for broader repo scans, but Underlay carries known structural scan findings
  there, so its warnings are not a fresh regression signal.
- Northstar AGENTS reviews use `effigy qa:docs:agent-defaults` here. This repo
  does not ship `check:agent-instructions`.
- Fall back to raw `cargo`, `bun`, or `vitest` only when the needed operation is
  not represented in `effigy.toml`.
- First-time local bring-up from outside this repo:
  `effigy bootstrap git@github.com:inflatable-cookie/underlay.git`

## Validation

Run what your change touches, then stop. Choose the narrowest Effigy selector
for the change and run each required check once; do not repeat a passing run.

```bash
effigy health            # cheap baseline
effigy rust:check        # cargo check --workspace --all-features
effigy rust:clippy       # denies warnings
effigy rust:test         # filter {args} to the touched crate/package
effigy fmt:rust:check    # cargo fmt --all --check; read-only
effigy fmt:rust          # cargo fmt --all; mutates the workspace
effigy test:unit         # vitest, filter {args} to the touched code
effigy test:components   # vitest component config
effigy test --plan       # when the test shape is what you need to know
effigy qa:docs           # docs checks when docs changed
# Use targeted raw tool commands only when Effigy does not cover the path
```

Run full `effigy qa` (`validate` + `qa:docs`) on `main` at Queue milestones,
not as a per-task default. Format selectors stay out of health, validate, and
qa; run `fmt:rust:check` when formatted Rust source changed.

`underlay-db` Postgres integration tests need a running Docker runtime and are
`#[ignore]`d by default; see `README.md` for the bring-up.

## Documentation Rules

- Consumer-facing product docs live in `docs/guides/`, `docs/usage/`,
  `docs/patterns/` and `docs/sweeps/`; internal truth lives in
  `docs/knowledge/`.
- Do not leave compatibility shim docs behind when paths or sections change.
- Writing style: `docs/knowledge/contracts/writing-style.md`.

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

Effigy guidance is maintained in the installed shared Agent Skill. Read the
installed `effigy/SKILL.md` from one of these user skill roots when using
Effigy-specific agent guidance: `~/.agents/skills`, `~/.codex/skills`,
`~/.claude/skills`, or `~/.cursor/skills`. Resolve symlinks first; aliases to
the same canonical skill directory are one installation. If distinct roots
contain the skill, report the ambiguity and choose one source explicitly.

This repo's `.agents/skills/effigy` copy is optional project-local content,
not the maintained guidance source. Preserve it if it exists; plain init does
not create or refresh it. The named `skill.codex_project` init action is an
explicit snapshot opt-in and may replace files at maintained paths. Named
`effigy skill run` task lookup still gives an invocation project's local skill
source precedence, as defined by contract 042.

If no installed Effigy Agent Skill is present, say so and suggest
`npx skills add inflatable-cookie/effigy -g`; init does not download or install
skills. A filesystem check cannot prove what an already-running agent loaded;
use a fresh agent context to verify discovery.

Agent Skill guidance and the `effigy` executable are separate channels. A
current skill does not prove the binary on `PATH` is current or admission-capable;
check the binary independently with `command -v effigy` and
`effigy admission status --json`.

Do not add a current-directory repo override while already inside the target
repo. Do not edit
`.github/workflows/` or run release mutations unless the user explicitly asks.

Reference docs:
- Effigy agent adoption: `docs/guides/047-agent-and-cross-repo-adoption.md`
- Installed skill task sources: `docs/knowledge/contracts/042-external-skill-task-runner-contract.md`
- Heavy validation admission: `docs/guides/080-host-wide-validation-admission.md`
- Graph workflows: `docs/guides/076-code-graph-and-agent-workflows.md`
- JSON contracts: `docs/guides/017-json-output-contracts.md`
<!-- END EFFIGY AGENT CONTRACT -->
