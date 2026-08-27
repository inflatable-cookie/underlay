# Contract 023 Released-Dependency Drift

Status: open
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

Keep open. Route to a bounded contract-023 normalization after the current
g09.047 release/promotion gate is settled; do not mix it into the release
mutation or Underlay Reference security decisions.
