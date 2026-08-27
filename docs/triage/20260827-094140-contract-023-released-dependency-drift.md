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

Promotion candidate. The release/promotion and route-retirement gates are now
settled. At the next planning checkpoint, decide whether to compile a bounded
Contract `023` normalization inside `g09` or close the generation. This note is
not execution authority until a roadmap is ready.
