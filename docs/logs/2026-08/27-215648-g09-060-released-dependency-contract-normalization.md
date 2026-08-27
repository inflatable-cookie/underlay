# 2026-08-27 21:56:48 - g09.060 Released Dependency Contract Normalization

## Outcome

Rewrote Contract `023` so release and consumer-pin teaching matches Contract
`024`, the live guides, and the six consumer roots. The roadmap is in review.

## Worktree

- root: `/Users/tom/.t3/worktrees/underlay/t3code-310e238b`
- branch: `t3code/normalize-released-dependency-contracts`
- base: `origin/main` at `98d0b130c767b02cd180cd16d6dd1217df7d26df`

## What Changed

- Contract `023` now treats `private: true` as registry-private, not
  unreleased. Release happens through immutable Git tags.
- the only committed JavaScript example is
  `git+ssh://git@github.com/inflatable-cookie/underlay.git#vX.Y.Z`
- the only committed Cargo example is
  `{ git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "vX.Y.Z" }`
- both language surfaces pin the same released tag
- a consumer cannot pin an unreleased shared commit
- hold-back retains the previous proven tag; upgrade changes every declared
  tag, regenerates root locks, and validates from that root
- rollback returns to a known-good released tag
- committed `path` and `file:` edges remain unsupported
- sibling checkouts stay QA/tooling inputs or untracked Cargo patches
- versions follow the release process and semantic versioning, not generation
  numbers
- impact classification, compatibility windows, upgrade notes, caller proof,
  and narrow retirement stay in place
- PR12 repo-relative links and current monorepo evidence paths are unchanged

## Guide Inspection

Inspected guides `030`, `040`, `190`, and `200`. All four already teach tagged
Git dependencies and untracked local patches. No live contradiction required
an edit.

## Read-Only Fleet Evidence

| Root | JavaScript pin | Cargo pin | path/file |
| --- | --- | --- | --- |
| `underlay-reference` | `v0.9.5` | `v0.9.5` | none |
| `contact-patch` | `v0.9.4` | `v0.9.4` | none |
| `compli-me` | `v0.9.4` | `v0.9.4` | none |
| `acowtancy` | `v0.9.4` | `v0.9.4` | none |
| `songsprout` | `v0.9.4` | `v0.9.4` | none |
| `loophole/composer` | `v0.9.4` | `v0.9.4` | none |

Underlay itself is `private: true` at synchronized version `0.9.5`, with
immutable tag `v0.9.5`. Holding `v0.9.4` is the live hold-back shape.

## Currentness

Updated the contract index, contracts front door, `001` posture, `g09`
milestone, roadmap front doors, generation index, logs front door, and the
discovery triage note. Historical roadmaps, logs, and handoffs were left as
evidence.

## Consumer Upgrade Notes

- Impact class: documentation correction
- Affected consumers: none; all six known roots already conform
- Required actions: none
- Compatibility window: none; committed path/file edges remain unsupported

## Next Task

Review the `g09.060` PR at exact head and merge only with explicit operator
authorisation. Do not open a later generation from this worker.
