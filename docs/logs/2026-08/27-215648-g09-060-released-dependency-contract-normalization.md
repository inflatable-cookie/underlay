# 2026-08-27 21:56:48 - g09.060 Released Dependency Contract Normalization

## Outcome

Rewrote Contract `023` so release and consumer-pin teaching matches Contract
`024`, the live guides, and the six consumer roots. Underlay PR13 later merged
the reviewed exact head; see the closeout log linked below.

## Worktree

- root: `/Users/tom/.t3/worktrees/underlay/t3code-310e238b`
- branch: `t3code/normalize-released-dependency-contracts`
- base: `origin/main` at `98d0b130c767b02cd180cd16d6dd1217df7d26df`

## What Changed

- Contract `023` now treats `package.json` `private: true` as npm-private, not
  unreleased. Both language surfaces are released through immutable Git tags.
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

The root JavaScript package is npm-private (`private: true`) at synchronized
version `0.9.5`, with immutable tag `v0.9.5`. Holding `v0.9.4` is the live
hold-back shape. Registry-publishing mechanics beyond that npm guard stay
outside this correction.

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

## Review Follow-Up

PR13 requested a wording correction at `ed73e322`: `private: true` is an npm
guard only. The contract, `g09.060` Decision/Execution Evidence, and this log
now say that. No Cargo `publish = false` metadata and no crates.io policy were
added.

## Next Task

See
`docs/logs/2026-08/27-222835-g09-060-released-dependency-contract-closeout.md`
for the exact-head review and merge evidence. Re-enter planning inside `g09`;
do not open a later generation without explicit operator direction.
