# Migration And Testing Assessment Batch Compiled

Date: 2026-08-26
Roadmap: `g09.035`–`g09.036`

## Trigger

`g09.034` closed the foundation and transport repair wave. The contract index
orders database migration and testing posture next.

## Findings

- contracts `021` and `022` contain enough boundary, source, invariant, and
  caller detail to support assessment
- consumer evidence paths in both contracts predated the completed `apps/*` /
  `packages/*` rollout
- contract `022` also used machine-local absolute links
- several planning front doors still routed to completed `g09.032`
- migration proof ownership must settle before testing posture can be assessed
  without conflating DB workflow and general test quality

## Decisions

- compile two serial assessment roadmaps rather than one fleet-wide mixed pass
- mark only the migration assessment `g09.035` ready
- keep testing assessment `g09.036` planned behind the migration-policy gate
- assess all six workspace roots and affected child packages
- keep both roadmaps read-only across consumer repositories
- compile repair roadmaps only from confirmed assessment evidence

## Runway

1. execute `g09.035`
2. promote and execute `g09.036` if the migration boundary remains coherent
3. compile a bounded findings-driven repair wave
4. checkpoint before the `024`–`026` bootstrap/runtime assessment group

## Consumer Upgrade Notes

Assessment only. No consumer action is required until a confirmed finding is
promoted into a repair roadmap.

## Next Task

Execute `g09.035`. Do not apply state or reset consumer databases during the
assessment.
