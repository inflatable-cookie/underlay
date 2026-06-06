# g08.003 - Migration-Core Policy Model Audit

Status: complete
Owner: repo maintainers
Roadmap: `g08`
Depends on: `001`, `023`, `122`

## Scope

Audit residual migration-core public model fields for:

- governance policy JSON models
- OCI bundle config/layout models
- bundle manifest decision-policy models

## Decision

`OciBundleConfig` now uses constructors and accessors. Its serialized field
shape stays unchanged.

`GovernancePolicy`, `PolicyOwner`, `RetentionPolicy`, `AccessControlPolicy`,
`RedactionPolicy`, and `DecisionPolicyConfig` remain public serialized document
records. They are loaded from JSON policy/manifest documents, then validated by
Underlay evaluators. Forcing builders onto those records would make config-file
and fixture authoring worse without adding a stronger invariant than the
existing validators.

## Consumer Upgrade Impact

Impact: breaking for direct `OciBundleConfig` field construction or mutation.

Current six-consumer scan found no named consumer direct construction of
`OciBundleConfig`, `GovernancePolicy`, or `DecisionPolicyConfig`.

## Goals

- [x] Classify governance and manifest policy structs as retained serialized
  document shapes.
- [x] Add constructors/accessors for `OciBundleConfig`.
- [x] Move Underlay and devtools code off `OciBundleConfig` field construction.
- [x] Preserve OCI package JSON shape.

## Acceptance Criteria

- [x] `cargo test -p underlay-migration-core`
- [x] `cargo test -p underlay-devtools`
- [x] Consumer scan for scoped literals remains clean.
- [x] `effigy rust:check`
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`

## Stop Conditions

- A named consumer owns direct OCI config construction.
- Serialized package or governance policy JSON shape needs to change.
- Retained governance/manifest public fields create a bypass around existing
  validators.

## Next Task

After this batch, continue with `g08.004`.
