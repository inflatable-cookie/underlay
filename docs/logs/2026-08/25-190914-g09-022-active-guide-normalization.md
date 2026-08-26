# g09.022 — Migration Contract And Active Guide Normalization

Date: 2026-08-25
Roadmap: `docs/roadmaps/g09/022-active-guide-normalization.md`
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`
Milestone: `docs/roadmaps/g09/README.md`
Status: complete

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

The review correction pass also:

- moved the remaining active currentness front doors to `g09.022` in review and
  kept `g09.023` blocked pending operator-authorized merge;
- made guide `175` start from a complete scaffold with package manifests,
  Effigy/state/secrets surfaces, and an initial lock before any frozen task;
- provisioned the pinned Effigy action in the guide CI examples and routed the
  guide `160` migration troubleshooting through the API package;
- removed machine-local links from contract `021`.

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
doctor findings were changed in this roadmap.

## Review And Merge

Review round 1 requested four corrections: align all currentness front doors,
make guide `175` executable from a complete scaffold, provision Effigy in the CI
examples and route migration troubleshooting correctly, and remove
workstation-absolute links from active contract `021`. The worker applied all
four in `51ba9118db99e31fb204a049b3fa8ba86d69bca3`.

Review round 2 approved that head for operator-authorized merge. PR
[#7](https://github.com/inflatable-cookie/underlay/pull/7) merged to `main` as
`db61051197efab8cd814df6302ff577091559f20` on 2026-08-25. The canonical
approval record is the
[orchestrator review comment](https://github.com/inflatable-cookie/underlay/pull/7#issuecomment-5415241912).

## Boundaries Held

- No consumer repository was edited.
- No conformance code was added.
- No release mutation or workflow edit was made.
- No historical log, closed roadmap, or frozen migration record was rewritten.
- `g09.023` remained blocked until orchestrator review and operator-authorized
  merge completed.

## Next Task

Execute `g09.023` — workspace-shape conformance — through a fresh
orchestrator-dispatched worker handoff. No other roadmap is ready.
