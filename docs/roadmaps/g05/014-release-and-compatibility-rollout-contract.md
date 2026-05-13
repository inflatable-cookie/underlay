# g05.014 — Release And Compatibility Rollout Contract

## Why

The contract set now covers path cutovers, runtime posture, helper routes, and
template adoption. What is still weak is the rollout discipline across the six
consumer apps:

- when a shared change needs compatibility aliases
- how long those aliases live
- what proof is required before retirement
- how release notes and consumer upgrade impact should be written

This is where shared changes can still create avoidable churn.

## Goal

Write the shared release and compatibility rollout contract for Underlay and
its consumer fleet.

## Scope

Primary targets:

- deprecation windows
- compatibility alias policy
- cross-repo rollout order
- release-note expectations
- consumer upgrade proof before retirement
- when a change is safe to land without a broad rollout plan

Likely outputs:

- one new contract
- possible tightening of roadmap/release expectations

## Consumer Upgrade Impact

Expected:

- clearer upgrade communication
- safer retirement of compatibility surfaces
- more consistent rollout sequencing

Landed:

- [`docs/contracts/023-release-and-compatibility-rollout.md`](/Users/tom/Dev/projects/underlay/docs/contracts/023-release-and-compatibility-rollout.md)

## Outcome

The fleet rollout rule is no longer split across old roadmap lineage, API
cutover notes, and the upgrade guide.

It is now explicit that:

- consumer-affecting shared changes must classify impact as `additive`,
  `deprecation`, or `breaking`
- compatibility windows need concrete justification and explicit sunset posture
- shared rollout should move through a visible repo order instead of surprise
  breakage
- release notes and upgrade notes are required output, not optional polish
- compatibility retirement needs caller proof before removal

## Current State

`g05.014` is complete.

The next useful delivery-layer contract is:

- `g05.015` config and secrets contract

## Next Task

Execute `g05.015`: freeze the config and secrets contract now that bootstrap,
migration, testing, template adoption, and rollout posture are explicit.
