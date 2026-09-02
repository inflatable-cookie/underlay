# 2026-09-02 - g11.004 Underlay v0.9.7 Release

Date: 2026-09-02
Roadmap: `g11.001`, Card 004 executed under explicit operator authorization
Branch: `main`

## Outcome

Underlay `v0.9.7` is published. The additive token-bound owned promotion
recovery surface accepted in Card 003 is now available to consumers as one
immutable tag. No consumer pin moved in this card.

- Handoff candidate: `081af80b9e096d586c75a7c73bc2c7fef2fb4e05`
- Pre-release candidate (after changelog link finalization):
  `f941cfc80b3ed23707314806f5146f8b2898f9db`
- Release commit: `8a7ce84b0501f6902da3ec1daf03f67ef0f42d4f`
- Annotated tag: `v0.9.7` (tag object `f274e49f589a86cab63ec37692fb85f4a905adf3`)
- GitHub Release: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.7

## Candidate Evidence

- local `HEAD` == `origin/main` == `081af80b` at preflight; worktree clean;
  Card 003 merge `c8378e6b` in ancestry; `v0.9.7` absent locally and remotely
- hosted CI green on the handoff candidate: Rust run `33688798162`,
  `headSha` `081af80b`, conclusion `success`
- hosted CI green on the pre-release candidate: Rust run `33689262627`,
  `headSha` `f941cfc8`, conclusion `success`
- hosted CI green on the release commit: Rust run `33689656264`,
  `headSha` `8a7ce84b`, conclusion `success`

This repository runs `rust.yml` on push without `workflow_dispatch`. The
exact-SHA push runs above are the CI evidence; the canonical dispatch route in
the vendored release protocol was not available and was not claimed.

## Honest Deviation: Candidate Moved Once

The handoff named `081af80b` as the candidate and also required changelog link
references to be finalized before the immutable tag. Effigy's changelog
promotion does not manage link references (`--json release simulate` diff
touches no `releases/tag` or `compare/` line), so the link-reference fix is a
separate commit, as it was for `v0.9.5` (`2eb3f678`). That commit,
`f941cfc8` "docs: finalize v0.9.7 changelog links", changed only two
`CHANGELOG.md` lines. Every gate, the exact-SHA CI check, and prepare/execute
ran against `f941cfc8`. No other drift entered the candidate.

## Release Sequence

- `effigy release simulate` — 4/4 gates pass, ready
- `effigy --json release status --check-gates` — `ready: true`,
  `blockers: []`, `next_version: 0.9.7`, `tag: v0.9.7`; re-run on `f941cfc8`
  with the same result
- `effigy release prepare --plan` — only the four expected mutations
- `effigy release prepare --yes --check-gates` — gates `version-sync`,
  `validate`, `clippy`, `rust` all pass; prepared against `f941cfc8`
- prepared-drift inspection — `git status` showed only `Cargo.toml`,
  `CHANGELOG.md`, `Cargo.lock`, `package.json` plus `.release-prepared.json`;
  `Cargo.lock` moved no line other than `0.9.6` → `0.9.7` (37 workspace
  members); `CHANGELOG.md` gained only the `## [0.9.7] - 2026-09-02` heading;
  `git diff --check` clean
- `effigy release execute --plan` — prepared HEAD == current HEAD ==
  `f941cfc8`, `stale: false`, fingerprint drift `[]`, no unexpected files
- `effigy release execute --yes` — run exactly once; committed `8a7ce84b`,
  tagged, pushed, state file removed
- `gh release create v0.9.7 --verify-tag` with the extracted changelog body;
  Effigy execute does not publish the GitHub Release (closed papercut)

## Tag Validation

`effigy release validate --tag` and `release verify-install` were not used;
the `v0.9.6` log records why they are Effigy self-hosting routes. Direct
validation instead:

- `git ls-remote --tags origin refs/tags/v0.9.7` — tag object `f274e49f`,
  peeled `8a7ce84b`
- `git cat-file -t v0.9.7` — `tag` (annotated); tagger Tom Wright
- GitHub API `git/ref/tags/v0.9.7` — `type: tag`, `sha: f274e49f`
- GitHub API `git/tags/f274e49f` — `object.sha: 8a7ce84b`, `type: commit`
- `origin/main` == local `HEAD` == `v0.9.7^{}` == `8a7ce84b`
- GitHub Release `v0.9.7`: not draft, not prerelease

## Tagged Consumer Smoke

A throwaway Cargo consumer under `/tmp` resolved the released surface from the
tag alone:

```toml
underlay-blob = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.7", features = ["local"] }
```

It calls `promote_verified_owned` and `recover_owned_publication` through a
generic `A: BlobAdapterPromotionExt` bound against a released `LocalAdapter`,
so the tagged signatures are type-checked rather than merely named. At runtime
it published a staging PNG with a fresh 32-byte `OwnershipToken`, recovered it
with the same token and `OwnedDestinationAuthority`, confirmed the recovered
SHA-256 matched, confirmed a wrong token refuses with
`BlobError::DestinationExists`, confirmed `Debug` renders
`OwnershipToken([redacted])`, and derived `OwnedPublicationFacts` from the tag.

- `cargo run` — compiled `underlay-blob v0.9.7` and completed all assertions
- consumer lockfile source:
  `git+ssh://git@github.com/inflatable-cookie/underlay.git?tag=v0.9.7#8a7ce84b0501f6902da3ec1daf03f67ef0f42d4f`
- no `branch=`, `rev=`, or `path+file` entry anywhere in that lockfile
- the throwaway consumer directory was removed afterwards

## Changelog

`effigy changelog extract CHANGELOG.md --version 0.9.7` returns the single
`### Added` entry for the `underlay-blob` owned promotion recovery surface,
which is also the GitHub Release body. Unlike `v0.9.6`, the tagged
`CHANGELOG.md` already carries the `[0.9.7]` link reference and its
`[Unreleased]` line compares from `v0.9.7`.

## Consumer Upgrade Notes

- Impact class: `additive`
- Affected consumers: apps depending on `underlay-blob`; any crate that
  implements `BlobAdapter` itself
- Required actions:
  - none are mandatory; `v0.9.6` pins keep working unchanged
  - to adopt, move the `underlay-blob` pin to `tag = "v0.9.7"`, persist a
    fresh `OwnershipToken` (at least 32 bytes) and
    `OwnedDestinationAuthority` before publication, publish with
    `BlobAdapterPromotionExt::promote_verified_owned`, and recover after
    process loss with `recover_owned_publication`
  - custom `BlobAdapter` implementations keep compiling; they refuse the
    owned path with `BlobError::Unsupported` until they implement
    `put_bytes_create_only_owned` with atomic reserved metadata
- Validation:
  - `cargo check --workspace --all-features`
  - `cargo clippy --workspace --all-targets -- -D warnings`
- Deprecation/removal date: `n/a` — no surface is deprecated or removed.
  `promote_verified` and every `v0.9.6` method are unchanged.
- Reference docs:
  - `docs/contracts/040-storage-blob-and-media-systems.md`
  - `docs/contracts/023-release-and-compatibility-rollout.md`
  - `docs/specs/immutable-verified-blob-promotion.md`
  - `docs/roadmaps/g11/batch-cards/004-underlay-v0-9-7-release.md`

## Closeout Validation

- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — clean

## Boundaries Held

- no gate bypass and no bypass retry
- one execute, no rewritten or re-cut tag
- no consumer repository edit and no consumer pin move
- no PR
- no `.github/workflows/` edit and no CI expansion
- `release validate --tag` and `release verify-install` not used

## Next Task

Return to the orchestrator. Resume Underlay Reference first on `v0.9.7`, then
route the other consumer ownership upgrades as independent repository lanes.
