# g11.002 - Five-Consumer Verified-Media Adoption

Status: ready
Owner: repo maintainers
Created: 2026-09-09
Depends on: `g11.001` complete (`v0.9.7` at `8a7ce84b`)
Governing refs: `docs/contracts/023-release-and-compatibility-rollout.md`,
`docs/contracts/040-storage-blob-and-media-systems.md`,
`docs/contracts/050-media-library-and-usage.md`,
`docs/specs/immutable-verified-blob-promotion.md`

## Outcome

Contact Patch, Underlay Reference, Compli Me, Acowtancy, and Songsprout
adopt the released promotion helper wherever their live
upload-finalisation paths can publish mutable or client-described
bytes: derive the digest server-side and make ready/current one
database transition per app. No consumer pins an unreleased commit or
local path.

## Ready-state rubric

- [x] Objective is bounded: five named lanes off one released tag.
- [x] Governing refs point at current canonical surfaces.
- [x] Scope, acceptance criteria, validation, evidence, and stop
  conditions are explicit below.
- [x] No unresolved planning gaps; consumer DTO/schema choices belong
  to each target repo's own authority.

## Dispatch manifest

- **State:** ready. Underlay Reference first, then the remaining lanes
  as independent repository lanes subject to their own shared-authority
  and same-repo merge ordering. Contact Patch Card 015 resumes on its
  retained agent/workspace rather than a duplicate lane.
- **Completion:** all five lanes merged on the released tag with the
  acceptance oracle proved per lane.
- **Owned mutable paths:** consumer repositories only; no Underlay
  source change is authorized here.
- **Worker:** per-lane workers under each target repo's authority.
- **Excluded:** new Underlay releases, DTO/schema redesign, retention
  or cleanup-policy choices not already settled by the target's
  authority.
- **Escalation:** target repo owners for app-local decisions.

## Work

1. Resume Contact Patch Card 015 on its retained worker: adopt the
   released helper, derive the digest server-side, one ready/current
   transition.
2. Apply the same boundary in Underlay Reference (first), Compli Me,
   Acowtancy, and Songsprout wherever live paths publish mutable or
   client-described bytes.
3. Prove each lane against the acceptance oracle with the target's
   handler/DB tests.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Derived metadata is authoritative. | Client submits valid-shape but wrong digest, MIME, or length. | Lane persists only server-derived digest/size and validated MIME, or refuses. Consumer handler/DB oracle. |
| Publication and selection are atomic in the app DB. | Ready update wins but current selection fails. | Neither state commits. Consumer DB transaction test. |
| Retry is explicit. | Crash after destination creation but before the app DB commit. | Documented retry path proves token-bound ownership and converges or returns a typed conflict; never overwrites. |
| Adopted tag is released. | A lane pins a commit, branch, or local path. | Lane lockfile resolves `v0.9.7` or later released tag. |

Known per-lane pressure (from the completed shared task): Underlay
Reference signs the final key directly with client digest and split
ready/current; Contact Patch stopped on mutable verification; Compli Me
must preserve Card 011 retry guarantees; Acowtancy reads repeatedly
without durable staging identity; Songsprout signs the final key
directly with client digest persisted.

## Stop conditions

- a consumer needs a public DTO, migration, retention, or
  cleanup-policy choice not already settled by its own authority:
  stop for that repo's planning;
- release validation or target DB/storage oracles cannot execute.

## Evidence

Per lane record: outcome, validation run, PR link, reviewed exact head,
merge commit, and material limits.

## Next task

`g11.003` fleet closeout once all applicable consumer lanes merge.
