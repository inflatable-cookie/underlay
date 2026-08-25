# g10.002 — Migration Contract And Active Guide Normalization

Date: 2026-08-25
Card: `docs/roadmaps/g10/batch-cards/002-active-guide-normalization.md`
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`
Milestone: `docs/roadmaps/g10/README.md`
Status: in review

## Worker

- Worktree: `/Users/tom/.t3/worktrees/underlay/t3code-00f4dadc`
- Branch: `t3code/normalize-active-guides`
- Base: `d367c192a0f7634348e558a4817cf3ee9601e4fe`
- Base check: `HEAD == origin/main`; required planning base is an ancestor;
  handoff exists in `HEAD`

## What Changed

Contract `021` now matches the live migration operator posture:

- root state uses `effigy state plan` and
  `effigy state apply local --yes`
- schema work stays API-owned behind routed `migration:*` tasks
- root and package `db:migrate`, `db:reset`, and `db:drop` aliases are
  explicitly retired
- durable migrations, dev overlays, reset/replay, forward-only authoring, and
  verification semantics remain intact

Active guides `040`, `050`, `080`, `130`, `140`, `150`, `160`,
`170`, `172`, `175`, `190`, and `200` now teach:

- one Git workspace with `apps/*`, `packages/*`, and root `docs/`
- one root Bun manifest and lockfile with explicit workspaces
- one frozen root install through Effigy
- `workspace:*` internal edges
- released Underlay/Poodle dependencies
- app-local Rust ownership and root Effigy routing

The LLM bootstrap pattern, guide indexes, and currentness pointers were aligned
with the same shape. Historical consumer proof, closed roadmaps, and frozen
records were left unchanged.

## Live Proof Used

Acowtancy was read-only evidence. Its root README and task/config surfaces
confirm the state-stack commands and package-owned `migration:*` routing. Its
migration ladder classifies `db:drop`, `db:migrate`, and `db:reset` as
removed selectors.

## Validation

- `effigy qa:docs` — pass
- `effigy qa:northstar` — pass
- `effigy health` — pass
- targeted active-doc `rg` proof — pass; remaining retired-shape hits are
  explicit prohibitions or retained historical evidence
- `git diff --check` — pass

`effigy doctor` still reports the pre-existing unsupported `isolation` key,
attention-marker findings, comment-ratio warning, and god-file warnings. No
doctor findings were changed in this card.

## Boundaries Held

- No consumer repository was edited.
- No conformance code was added.
- No release mutation or workflow edit was made.
- No historical log, closed roadmap, or frozen migration record was rewritten.
- `g10.003` remains blocked pending orchestrator review and
  operator-authorized merge.

## Next Task

Open the implementation PR for orchestrator review. After review and
operator-authorized merge, promote `g10.003` to ready.
