# g10.006 - Contact Patch Normalization

Status: ready
Owner: Contact Patch maintainers
Spec: `docs/specs/monorepo-consumer-workspace-rollout.md`

## Scope

- Move `cp-api`, `cp-admin`, and `cp-front` into `apps/`.
- Move `cp-client` and `cp-ui` into `packages/`.
- Move `cp-docs` to root `docs/`.
- Consolidate four child locks into one root lock.
- Replace internal `file:` edges with `workspace:*`.
- Update Effigy catalogs, bundle dirs, aliases, tests, config paths, docs, and
  instruction surfaces.

## Acceptance And Validation

- One frozen root install; workspace-shape check green.
- `effigy health`, planned targeted tests/checks, and `git diff --check` green.
- No old-path fallback or compatibility symlink.

## Stop Conditions

Stop on package-role ambiguity, release dependency failure, or scope-expanding
application behavior.

Stop if Effigy cannot bootstrap and mount the external Book checkout as a
sibling source without making it a catalog or workspace member.

## External Book Contract

- Keep Book external to the product workspace at sibling host path `../book`.
- Bootstrap it as a child from `git@github.com:contact-patch/book.git`.
- Mount that ordinary source at `/workspace-root/book`, read-only if the
  effective Effigy schema supports it; do not make Book a catalog member.
- After `cp-api` moves, resolve Book as `../../../book` from the API on host and
  in the container.
- Remove the ignored nested-root checkout posture and its `/book/` ignore.

## Readiness Evidence

Contact Patch `origin/main` at
`bd596e55b3a8ec8e68352c045f6ecd12b8effb4f` has no open PR. The local checkout
is two commits behind and has a pre-existing `cp-api/Cargo.lock` modification;
workers must branch from clean `origin/main`.

The target map and root workspace list are unambiguous:

- `cp-api` -> `apps/cp-api`
- `cp-admin` -> `apps/cp-admin`
- `cp-front` -> `apps/cp-front`
- `cp-client` -> `packages/cp-client`
- `cp-ui` -> `packages/cp-ui`
- `cp-docs` -> root `docs`
- JavaScript members: Admin, Front, Client, and UI

The root manifest lacks `packageManager` and `workspaces`; four child locks
remain; Admin and Front each use one internal `file:` client edge; and no root
lock exists. A clean checkout has the expected seven committed-shape
violations. The current machine reports an eighth because ignored `book/`
contains a nested Git repository. Active docs do not define that checkout's
location; API tasks and three PHP helpers merely assume it. The contract above
resolves that operational ambiguity without changing Book ownership or product
behavior. No tracked submodule, Git link, operational symlink, or local
dependency override exists.

Released dependencies are correct: all JavaScript and 24 Rust Underlay edges
request `v0.9.4`, while Poodle requests `0.2.2`. Admin and Front locks still
embed a stale Underlay `v0.9.2` snapshot through the file-linked Client. Root
lock generation must remove that stale resolution.

Path-sensitive work spans all six bundle dirs, root/API Effigy tasks, Client
aliases in both Svelte configs and Front Vite, an Admin contract test, root
public-config generation, `scripts/check-admin-freshness.sh`, nested AGENTS
references, and three API import helpers. Root bootstrap and system mounts must
add sibling Book; the API helpers move from `../book` to `../../../book`.

Read-only Effigy discovery and test planning pass. Full health was not run
because it can rewrite generated public config. Doctor already reports manifest
schema debt, god files, a stale graph, heavy sibling health, and one generated
source file. Six existing papercuts, including the Admin subject-test and stale
nested-lock behavior, remain outside this topology card.

## Next Task

Return a reviewable PR to the orchestrator; do not merge.
