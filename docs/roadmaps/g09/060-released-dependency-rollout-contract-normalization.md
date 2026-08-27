# g09.060 - Released Dependency Rollout Contract Normalization

Status: in review
Owner: Underlay maintainers
Contract: `023`
Depends on: `g09.059` and papercuts wave 3 contract-link closeout (`complete`)

## Purpose

Align Contract `023` with the released-dependency and single-repository posture
already established by Contract `024`, the live guides, the workspace checker,
and all six consumer roots.

## Decision

The operator chose to continue `g09` for this bounded normalization. Underlay
PR12 completed the overlapping papercuts wave 3 contract-link lane at reviewed
head `d2cb5cd9`, merge commit `9e26ba9a`. The dispatch gate is clear.

- the root JavaScript package is npm-private (`private: true`). Both language
  surfaces are released to consumers through immutable Git tags.
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
- Underlay PR12 exact-head review, green CI, and merge commit `9e26ba9a`
- one execution log under `docs/logs/2026-08/`

## Dispatch Evidence

- planning base: `ec67dfbfb2add489e4309f801fdac6fbc953aeb0`
- handoff:
  `docs/handoffs/20260827-214255-g09-060-released-dependency-contract-normalization.md`
- dispatch topology: one serial Underlay docs-only worker
- planning-base validation: `effigy health`, `effigy qa:docs`,
  `effigy qa:northstar`, and `git diff --check` passed

## Stop Conditions

Stop if the correction requires a new registry-publishing policy, a release
mutation, an Effigy behavior change, a consumer manifest change, or a choice
between multiple supported dependency shapes. Do not preserve path/file
compatibility: Contract `024` has already made that unsupported.

Base the implementation handoff on pushed `main` after merge commit `9e26ba9a`.
Preserve the repo-relative links and current monorepo evidence paths delivered
there.

## Consumer Upgrade Impact

- Impact class: documentation correction
- Affected consumers: none; all six known roots already conform
- Required action: none
- Compatibility window: none; committed path/file edges are already unsupported

## Execution Evidence

- Contract `023` now distinguishes the npm-private root package from Git-tag
  release of both language surfaces. Committed consumer pins are the tagged
  SSH Git forms on JavaScript and Cargo.
- hold-back and rollback retain or return to a known released tag. Upgrade
  moves every declared tag in the consumer root, regenerates root locks, and
  validates from that root.
- committed Cargo `path` and JavaScript `file:` edges remain unsupported.
  Sibling checkouts stay untracked QA/tooling inputs.
- versions follow the release process and semantic versioning, not roadmap
  generation numbers.
- guides `030`, `040`, `190`, and `200` already taught this shape; no live
  contradiction required an edit.
- read-only fleet check: all six roots use tagged Git dependencies. Underlay
  Reference pins `v0.9.5`; Contact Patch, Compli Me, Acowtancy, Songsprout, and
  Composer retain `v0.9.4`. No committed path/file edges.
- execution log:
  `docs/logs/2026-08/27-215648-g09-060-released-dependency-contract-normalization.md`

## Next Task

Review the worker PR at exact head and merge only with explicit operator
authorisation. Do not open a later generation from this closeout.
