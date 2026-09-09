# g11.003 - Verified-Media Fleet Closeout

Status: blocked
Owner: repo maintainers
Created: 2026-09-09
Depends on: `g11.002` (all applicable consumer lanes merged)
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/040-storage-blob-and-media-systems.md`,
`docs/contracts/050-media-library-and-usage.md`

## Outcome

All five consumers are proved on a released tag under the `g11.001`
invariant, and `g11` closes with a fleet matrix. No consumer pins an
unreleased commit or local path.

## Dispatch manifest

- **State:** blocked until every applicable `g11.002` lane merges.
- **Completion:** fleet matrix showing per-consumer tag, digest
  derivation, atomic ready/current transition, and oracle proof, then
  generation closeout.
- **Owned mutable paths:** `docs/roadmaps/g11/README.md` closeout
  record only.
- **Excluded:** new adoption scope; anything unmerged belongs to
  `g11.002`.
- **Escalation:** repo maintainers for closeout scope disputes.

## Work

1. Collect per-lane evidence from `g11.002` (tag, server-derived
   digest, atomic transition, oracle proof).
2. Record the fleet matrix in the generation README.
3. Close the generation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The fleet is on the invariant. | A consumer still publishes mutable or client-described bytes. | Fleet matrix with per-lane oracle evidence. |
| Pins are released. | A lane resolves a commit, branch, or local path. | Every lane lockfile resolves a released tag. |

## Stop conditions

- a consumer lane cannot merge: the blockage stays in `g11.002`,
  this task stays blocked.

## Evidence

Fleet matrix plus per-lane PR links, exact heads, and merge commits.

## Next task

None in `g11`. Further lanes need explicit planning and a new task.
