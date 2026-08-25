# g10.001 — Monorepo Contract Authority

Date: 2026-08-25
Card: `docs/roadmaps/g10/batch-cards/001-monorepo-contract-authority.md`
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`
Milestone: `docs/roadmaps/g10/README.md`
Status: implemented, awaiting orchestrator review

## What Changed

Underlay taught two incompatible workspace models. It now teaches one.

Contract `024` was rewritten around the shape Acowtancy proved: one Git root,
`apps/*`, `packages/*`, root `docs/`, one root Bun manifest and lockfile,
`workspace:*` internal edges, released Underlay/Poodle dependencies, and an
Effigy-owned frozen install. Polyrepo layouts are now explicitly unsupported.

The six scoped front doors were aligned against it:

| Surface | Change |
| --- | --- |
| `docs/contracts/024-new-app-bootstrap-and-bring-up.md` | Rewritten. Topology, root manifest, Rust ownership, dependency, docs authority, Effigy, bundle, bootstrap, config, and bring-up rules. |
| `docs/guides/README.md` | "Modes (Multi-repo vs Monorepo)" replaced by one workspace shape; `libs/*` path mapping removed; raw-fallback commands rebuilt around one frozen root install. |
| `docs/guides/000-overview.md` | Architecture diagram and layer tables moved to `apps/*` / `packages/*`; Underlay/Poodle shown as released dependencies rather than `libs/underlay/`. |
| `docs/guides/020-project-structure.md` | Multi-repo setup procedure replaced by a single-repository procedure with the normative root manifest, one `git init`, one lockfile, one root `AGENTS.md`; directory templates rehomed under `apps/*` and `packages/*`. |
| `docs/guides/030-underlay-integration.md` | Symlink/submodule/`file:` options replaced by pinned release tags on both the Bun and Cargo surfaces, plus an upgrade procedure. |
| `docs/architecture/060-new-project-quickstart.md` | Supersession pointer restated around the single-repository contract and the strict spec. |
| `docs/patterns/new-project-bootstrap-prompt.md` | Monorepo-vs-multi-repo question removed; contract `024` added as the first source; pre-contract-layout stop conditions added. |

Index currentness for `024` was updated in `docs/contracts/README.md` and
`docs/contracts/contract-index.md`.

## Evidence

Acowtancy was inspected read-only:

- root `package.json`: `@acowtancy/market`, `private: true`, `packageManager`
  `bun@1.3.14`, explicit `workspaces` array
- `apps/cream`, `apps/dairy`, `apps/farmyard`, `packages/cattle-grid`,
  `packages/froyo`, root `docs/`
- one root `bun.lock`; no child lockfiles
- internal edges use `workspace:*`; Underlay resolves through
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4`; Poodle
  through released `0.2.2`
- `apps/farmyard/Cargo.toml` pins every Underlay crate to `tag = "v0.9.4"` and
  keeps the Cargo workspace app-local
- `infra/tasks.toml` declares `workspace:js:prepare` =
  `bun install --frozen-lockfile`
- `effigy.toml` uses sibling `../underlay/scripts/*` only for QA tasks
- `infra/workspace.toml` maps `bundle.dirs` onto `apps/*`, `packages/*`, and
  `docs/` with no child repos

Two corrections came out of that evidence. The first draft of the integration
guide described Rust consumption as crates.io versions; Acowtancy proves both
language surfaces use pinned Git tags, and the guide and contract were fixed to
match. The contract's dependency rule was also extended to cover the
machine-local `effigy deps link cargo` `[patch]` affordance, which must stay
untracked.

## Validation

- `effigy qa:docs` — pass (link, vision index, forbidden, next-action)
- `effigy qa:northstar` — pass (roadmaps, vision, `g01` heading checks)
- `effigy health` — pass (exports, component-test hygiene, Poodle prop names,
  release-version sync at `0.9.4`, guardrails)
- targeted `rg` over the seven scoped files for `--repo .`, `libs/`,
  `multi-repo`, `submodule`, `symlink`, `ln -s`, `file:..`, and absolute
  `/Users/tom` paths — every remaining hit is a prohibition or the quickstart's
  historical supersession note
- `git diff --check` — clean

`effigy doctor` still reports the pre-existing unsupported `isolation` key in
`effigy.toml` plus attention-marker and god-file scan debt. Both predate the
planning base and were deliberately left alone.

## Boundaries Held

- No consumer repository was edited. Acowtancy was read-only evidence.
- Second-tier guide cleanup (`g10.002`) and conformance code (`g10.003`) were
  not started.
- No closed roadmap, log, handoff, or frozen migration record was rewritten.
- Acowtancy's obsolete README `file:` prose was left in place; it is `g10.004`.
- No release mutation and no `.github/workflows/` edit.

## Next Task

Orchestrator review of the `g10.001` PR. `g10.002` stays blocked until that
review lands and the operator authorises the merge.
