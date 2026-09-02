# 2026-09-02 - g11.002 Underlay v0.9.6 Release

Date: 2026-09-02
Roadmap: `g11.001`, Card 002 executed under explicit operator authorization
Branch: `main`

## Outcome

Underlay `v0.9.6` is published. The additive immutable verified
staging-to-published blob promotion surface is now available to consumers as
one immutable tag. No consumer pin moved in this card.

- Pre-release candidate: `003d7fdecf6f61d7dc37b5544a61a98e885bbc78`
- Release commit: `4f6d75522c553fa9279b1ce36871ccc1cc1ce99d`
- Annotated tag: `v0.9.6` (tag object `975fd9e99323c8153c8cd6e0fa417179aed85a8a`)
- GitHub Release: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.6

## Candidate Evidence

- local `HEAD` == `origin/main` == `003d7fde` before prepare; worktree clean
- `v0.9.6` absent locally and on the remote before execute
- hosted CI green on the exact candidate SHA: Rust run `33667265135`,
  `headSha` `003d7fde`, conclusion `success`
- hosted CI green on the release commit: Rust run `33668443911`,
  `headSha` `4f6d7552`

The candidate carries the bounded fixture-hermeticity repair merged through
PR #24, which cleared the `validate` gate that stopped the first attempt. See
`02-191522-workspace-shape-retired-fixture-hermeticity.md`.

## Release Sequence

- `effigy release simulate` — 4/4 gates pass, ready to prepare and execute
- `effigy --json release status --check-gates` — `ready: true`,
  `blockers: []`, `next_version: 0.9.6`, `tag: v0.9.6`, changelog valid with
  0 diagnostics
- `effigy release prepare --plan --version 0.9.6` — only the four expected
  mutations
- `effigy release prepare --yes --check-gates --version 0.9.6` — gates
  `version-sync`, `validate`, `clippy`, `rust` all pass; prepared
- prepared-drift inspection — `git status` showed only `Cargo.toml`,
  `CHANGELOG.md`, `Cargo.lock`, `package.json` plus `.release-prepared.json`;
  `Cargo.lock` moved no line other than `0.9.5` to `0.9.6`;
  `git diff --check` clean; no unrelated source or documentation change
- `effigy release execute --plan` — prepared HEAD == current HEAD ==
  `003d7fde`, not stale, ready
- `effigy release execute --yes` — committed, tagged, pushed

## Tag Validation

`effigy release validate --tag v0.9.6` is not usable in this repository. It
fails trying to write `.github/workflows/release-binaries.yml`, which is
Effigy's self-hosting binary-distribution scaffolding. That is the same
routing error class as `release verify-install` for a library repo. It was not
retried, the workflow file was not created, and the working tree stayed clean.

The tag was validated directly instead:

- `git ls-remote --tags origin refs/tags/v0.9.6` — tag object
  `975fd9e9`, peeled `4f6d7552`
- `git cat-file -t v0.9.6` — `tag` (annotated, not lightweight)
- GitHub API `git/ref/tags/v0.9.6` — `type: tag`, `sha: 975fd9e9`
- GitHub API `git/tags/975fd9e9` — `object.sha: 4f6d7552`, `type: commit`
- `origin/main` == `4f6d7552`, so the tag points at the release commit

## Tagged Consumer Smoke

A throwaway Cargo consumer outside this repository resolved the released
surface from the tag alone:

```toml
underlay-blob = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.6" }
```

It imports `BlobAdapterPromotionExt`, `VerifiedPromotionResult`,
`BlobAdapter`, `BlobObjectKey`, and `BlobUploadConfig`, calls
`promote_verified` through a generic `BlobAdapterPromotionExt` bound so the
released signature is type-checked rather than merely named, and references
`get_bytes_bounded` and `put_bytes_create_only` on the released `BlobAdapter`
trait.

- `cargo run` — compiled `underlay-blob v0.9.6` and printed
  `underlay-blob v0.9.6 promotion surface resolved from tag v0.9.6`
- consumer lockfile source:
  `git+ssh://git@github.com/inflatable-cookie/underlay.git?tag=v0.9.6#4f6d75522c553fa9279b1ce36871ccc1cc1ce99d`
- no `branch=`, `rev=`, or `path+file` entry anywhere in that lockfile

Effigy's `release verify-install` was not used.

## Changelog

`effigy changelog extract CHANGELOG.md --version 0.9.6` returns the single
`### Added` entry for the `underlay-blob` promotion surface, which is also the
GitHub Release body.

Known gap in the tag: `CHANGELOG.md` at `v0.9.6` has no `[0.9.6]` link
reference and its `[Unreleased]` line still compares from `v0.9.5`. Effigy's
changelog promotion does not manage link references; for `v0.9.5` they were
finalized in a separate pre-release commit (`2eb3f678`) and that step was not
run this time. The tag is immutable and was not rewritten. This closeout
commit repairs the links on `main`.

## Consumer Upgrade Notes

- Impact class: `additive`
- Affected consumers: apps depending on `underlay-blob`; any crate that
  implements `BlobAdapter` itself
- Required actions:
  - none are mandatory; existing pins keep working unchanged
  - to adopt, move the `underlay-blob` pin to `tag = "v0.9.6"` and call
    `BlobAdapterPromotionExt::promote_verified` for immutable publication
  - custom `BlobAdapter` implementations keep compiling; they refuse the new
    path with `BlobError::Unsupported` until they implement
    `get_bytes_bounded` and `put_bytes_create_only`
- Validation:
  - `cargo check --workspace --all-features`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Deprecation/removal date: `n/a` — no surface is deprecated or removed.
  `finalise_upload_verified` and the existing mutable upload/read/finalise
  APIs are unchanged, and none of them establish immutable publication.
- Reference docs:
  - `docs/contracts/040-storage-blob-and-media-systems.md`
  - `docs/contracts/023-release-and-compatibility-rollout.md`
  - `docs/roadmaps/g11/batch-cards/002-underlay-v0-9-6-release.md`

## Closeout Validation

- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — clean

## Boundaries Held

- no gate bypass and no bypass retry
- no rewritten or re-cut tag
- no consumer repository edit and no consumer pin move
- no PR
- no `.github/workflows/` edit and no CI expansion
- `release verify-install` not used

## Next Task

Repoint `underlay-reference` to `v0.9.6` as the first consumer proof, per the
cross-repo rollout order in `023-release-and-compatibility-rollout.md`.
