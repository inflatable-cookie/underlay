# Contract 023 Released-Dependency Drift

Status: in review
Captured: 2026-08-27

## Observation

Contract `023-release-and-compatibility-rollout.md` still describes sibling
Cargo `path` and npm `file:` dependencies as the default fleet workflow and
calls Underlay unpublished. Contract `024`, the completed g09 monorepo rollout,
and current consumer pins require released Git/tag dependencies instead.

## Impact

The release contract can send maintainers back to a retired polyrepo dependency
shape while the active bootstrap contract and workspace checker reject that
shape.

## Disposition

Promoted to
[`g09.060`](../roadmaps/g09/060-released-dependency-rollout-contract-normalization.md)
and now in review after the Contract `023` rewrite. This note remains only as
the discovery record.
