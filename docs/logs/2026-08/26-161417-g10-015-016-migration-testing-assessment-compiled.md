# Migration And Testing Assessment Batch Compiled

Date: 2026-08-26
Roadmap: `g10.015`–`g10.016`

## Trigger

`g10.014` closed the foundation and transport repair wave. The contract index
orders database migration and testing posture next.

## Findings

- contracts `021` and `022` contain enough boundary, source, invariant, and
  caller detail to support assessment
- consumer evidence paths in both contracts predated the completed `apps/*` /
  `packages/*` rollout
- contract `022` also used machine-local absolute links
- several planning front doors still routed to completed `g10.012`
- migration proof ownership must settle before testing posture can be assessed
  without conflating DB workflow and general test quality

## Decisions

- compile two serial assessment cards rather than one fleet-wide mixed pass
- mark only the migration assessment `g10.015` ready
- keep testing assessment `g10.016` planned behind the migration-policy gate
- assess all six workspace roots and affected child packages
- keep both cards read-only across consumer repositories
- compile repair cards only from confirmed assessment evidence

## Runway

1. execute `g10.015`
2. promote and execute `g10.016` if the migration boundary remains coherent
3. compile a bounded findings-driven repair wave
4. checkpoint before the `024`–`026` bootstrap/runtime assessment group

## Consumer Upgrade Notes

Assessment only. No consumer action is required until a confirmed finding is
promoted into a repair card.

## Next Task

Execute `g10.015`. Do not apply state or reset consumer databases during the
assessment.
