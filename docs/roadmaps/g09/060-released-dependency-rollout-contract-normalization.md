# g09.060 - Released Dependency Rollout Contract Normalization

Status: planned - waiting on papercuts wave 3 contract-link closeout
Owner: Underlay maintainers
Contract: `023`
Depends on: `g09.059` and papercuts wave 3 contract-link closeout

## Purpose

Align Contract `023` with the released-dependency and single-repository posture
already established by Contract `024`, the live guides, the workspace checker,
and all six consumer roots.

## Decision

The operator chose to continue `g09` for this bounded normalization. Dispatch
waits for the already-published papercuts wave 3 contract-link worker because
both lanes otherwise edit Contract `023`.

- Underlay remains private from a package-registry perspective but is released
  through immutable Git tags.
- committed consumer JavaScript dependencies use the tagged Git URL; committed
  Cargo dependencies use the same release tag.
- holding a consumer back means retaining its previous proven tag.
- upgrading means changing every declared Underlay tag in the consumer root,
  regenerating the root locks, and validating from that root.
- sibling Underlay checkouts remain QA/tooling inputs or untracked local Cargo
  patches only. Committed `path` and `file:` edges are unsupported.
- shared versions follow the release process and semantic versioning. Roadmap
  generation numbers do not determine package versions.

## Scope

### Contract authority

- replace Contract `023`'s unpublished/path-first default with the released-tag
  dependency rule
- rewrite the hold-back and upgrade examples around retaining or moving tags
- add the release boundary to rollout order: a consumer cannot pin an unreleased
  shared commit
- preserve impact classification, compatibility-window, upgrade-note, caller-
  proof, and narrow-retirement rules
- preserve the repo-relative Contract `023` links delivered by papercuts wave 3;
  do not reopen its broader link sweep

### Currentness

- align the contract index, contracts front door, roadmap/front-door state,
  triage disposition, and one execution log
- inspect guides `190`, `200`, `030`, and `040` for contradictions; edit only a
  live contradiction introduced or exposed by this contract correction

## Out Of Scope

- changing consumer manifests or lockfiles
- changing Underlay or Poodle versions
- cutting, rewriting, or backfilling a release or Git tag
- changing Effigy release behavior or `.github/workflows/`
- publishing Underlay to npm or crates.io
- adding a local-path compatibility workflow
- broad cleanup of absolute links outside the touched Contract `023` surface

## Acceptance

- Contract `023` no longer calls path/file dependencies the default
- Contract `023` no longer calls Underlay unreleased merely because
  `package.json` is private
- JavaScript and Cargo examples use one immutable `vX.Y.Z` tag
- hold-back guidance retains an older tag; it never reverts to `path` or `file:`
- rollout order requires a validated Underlay release before consumer tag bumps
- local sibling checkouts are explicitly non-authoritative and untracked
- version policy is release/SemVer-driven rather than generation-driven
- active front doors and the triage note point at this roadmap honestly
- no consumer source, manifest, lockfile, release artifact, or tooling code
  changes

## Validation

- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- targeted scan of active docs for the retired Contract `023` claims
- `git diff --check`

## Evidence

- Contract `024` released-dependency rule
- active guides `030`, `040`, `190`, and `200`
- all six current consumer roots using tagged Underlay Git dependencies
- Underlay `v0.9.5` release/tag and synchronized Rust/JavaScript version surface
- one execution log under `docs/logs/2026-08/`

## Stop Conditions

Stop if the correction requires a new registry-publishing policy, a release
mutation, an Effigy behavior change, a consumer manifest change, or a choice
between multiple supported dependency shapes. Do not preserve path/file
compatibility: Contract `024` has already made that unsupported.

Do not dispatch before the papercuts wave 3 contract-link PR is merged and its
exact `main` result is verified. Rebase the implementation handoff on that
result.

## Consumer Upgrade Impact

- Impact class: documentation correction
- Affected consumers: none; all six known roots already conform
- Required action: none
- Compatibility window: none; committed path/file edges are already unsupported

## Next Task

Launch the existing papercuts wave 3 contract-link worker from
`docs/handoffs/20260827-210040-papercuts-wave3-contract-links.md`. After its PR
is reviewed, authorised, merged, and verified on `main`, promote and dispatch
this roadmap from the new exact base.
