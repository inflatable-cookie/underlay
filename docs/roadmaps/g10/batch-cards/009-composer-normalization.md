# g10.009 - Composer Normalization

Status: ready
Owner: Composer maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `composer-api`, `composer-admin`, and `composer-front` into `apps/`.
- Move `composer-api-client` into `packages/`.
- Move `composer-docs` to root `docs/`.
- Consolidate three child locks and replace internal `file:` edges.
- Replace repeated child `bun install --no-cache` commands with one locked root
  setup; remove suppressed install failures.
- Remove old-path fallbacks, sibling Underlay validation, and unused `/underlay`
  symlink hacks.
- Update Effigy catalogs, bundle dirs, aliases, tests, docs paths, and
  instruction surfaces.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- Child dev/build/check tasks are pure package tasks.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.

## Stop Conditions

Stop if removing an Underlay mount changes runtime compilation or if Effigy
cannot route package setup from the root.

## Readiness Evidence

Composer `origin/main` at
`a088e7b260e4d4aceb3eff9e81178c8d76a2f001` is clean locally and has no open
PR. The target map is unambiguous:

- `composer-api` -> `apps/composer-api`
- `composer-admin` -> `apps/composer-admin`
- `composer-front` -> `apps/composer-front`
- `composer-api-client` -> `packages/composer-api-client`
- `composer-docs` -> root `docs`

The root manifest is private but lacks `packageManager` and `workspaces`.
Three child locks remain, Admin and Front each use one internal `file:` edge,
and no root lock exists. The workspace checker reports six violations: three
child locks plus missing root package-manager, root-lock, and workspaces
fields. The final root workspace list has three JavaScript members: Admin,
Front, and API Client. The Rust API remains application-local and outside the
Bun workspace.

Released dependencies are already correct: JavaScript and Rust request
Underlay `v0.9.4`; Poodle packages request `0.2.2`. The child locks still embed
stale Underlay `v0.9.2` metadata through the file-linked client. Root lock
generation must remove that stale resolution and retain the released tag.

Path-sensitive work spans the root and five child Effigy catalogs, root bundle
dirs, child `cd` guards, Svelte and Vite client aliases, three rollout scripts,
README/AGENTS references, and docs QA paths. API Effigy tasks still create an
`/underlay` symlink despite tag-only Cargo dependencies. Root `validate` also
validates sibling Underlay. Both are retired by this card; sibling mounts and
scripts remain valid only for explicit QA/tooling.

Baseline debt is not migration scope: `composer-docs/health` already fails its
semantic-role and parameter freshness checks. `effigy doctor` also reports
shared Underlay manifest debt plus generated-asset and god-file findings.
The worker must preserve or separately evidence these failures, not repair
application behavior. No nested Git repository, submodule, or committed
compatibility symlink was found.

## Dispatch

Worker handoff
`docs/handoffs/20260826-112215-g10-009-composer-normalization.md` is published
on Composer `main` at `230297ee6638e100365eba37e6499613c0dda954`.
The operator owns worker launch; the worker must return a PR and must not merge
it.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
