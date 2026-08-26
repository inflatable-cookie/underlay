# g10.007 - Compli Me Normalization

Status: ready
Owner: Compli Me maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `api`, `admin`, and `front` into `apps/`.
- Move `api-client` and `ui` into `packages/`; retain root `docs/`.
- Delete the empty API `package.json`; do not make the Rust API a Bun workspace.
- Consolidate four child locks and replace internal `file:` edges.
- Replace child hydration with one repo-owned frozen root install selector.
- Update Effigy catalogs, bundle dirs, aliases, tests, config/QA paths, docs,
  and instruction surfaces.
- Preserve the released Underlay tag already proven with Bun `1.3.14`; do not
  introduce an exact-SHA exception.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.
- Any retained SHA exception is explicit and reproducible.

## Stop Conditions

Stop if tag resolution reproduces the prior Bun cache defect or requires an
unplanned release change.

Also stop if root lock generation retains Underlay `v0.9.2`/SHA drift, moved
catalogs require an Effigy schema change, or validation would require repairing
the unrelated reorder-conflict behavior.

## Readiness Evidence

Compli Me `origin/main` at
`db5741f63dfb1cc82d9b49436370edfe66366bb2` is two commits ahead of the clean
local checkout and has no open PR. A second dirty worktree belongs to the
earlier Underlay adoption lane and must not be reused or cleaned. Workers must
branch from clean `origin/main`.

The target map and root workspace list are unambiguous:

- `api` -> `apps/api`; delete its empty JavaScript manifest
- `admin` -> `apps/admin`
- `front` -> `apps/front`
- `api-client` -> `packages/api-client`
- `ui` -> `packages/ui`
- retain root `docs`
- JavaScript members: Admin, Front, API Client, and UI

The root manifest lacks `packageManager` and `workspaces`; four child locks
remain; three internal `file:` edges connect Admin/Front to API Client/UI; and
no root lock exists. The workspace checker reports the expected seven
violations. No tracked submodule, nested Git repository, symlink, or runtime
source override exists.

Released dependency resolution is proven by merged PR #3: Bun `1.3.14`, all
JavaScript and 26 Rust Underlay edges at `v0.9.4`, and Poodle `0.2.2` passed
four frozen installs plus package validation. No SHA exception is needed.
Admin and Front locks still embed Underlay `v0.9.2` metadata through current
file-linked packages; the consolidated lock must contain no `ddba2640` result.

Path-sensitive work spans five bundle dirs, 27 child Effigy `cd` guards, root
template QA, UI child hydration, Svelte/Vite client aliases, public-config
root calculation, docs rollout scripts, API tooling, `.gitignore`, AGENTS,
active docs, and five child READMEs. The config generators currently import
sibling Underlay source; replace that with the released
`@inflatable-cookie/underlay/server/config-stack` export. README claims a root
`bootstrap:deps` selector that task discovery cannot find; adopt the proven
`g10.005` frozen-root-install pattern.

Read-only task discovery and test planning pass. Doctor already reports shared
Underlay manifest debt, attention markers, god files, generated source, and
heavy sibling health. Recent remote evidence has green health, package
validations, frozen installs, and docs QA. Root validation still has unrelated
reorder-conflict debt; update its physical path only. Closed roadmaps and logs
retain historical wording.

## Dispatch

Worker handoff
`docs/handoffs/20260826-112215-g10-007-compli-me-normalization.md` is published
on Compli Me `main` at `a5ccf0b92df64d8687e8255051219ccffbabe126`.
The operator owns worker launch; the worker must return a PR and must not merge
it.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
