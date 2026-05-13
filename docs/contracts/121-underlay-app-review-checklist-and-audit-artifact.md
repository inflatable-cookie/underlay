# Contract: Underlay App Review Checklist and Audit Artifact

Status: active
Owner: repo maintainers
Depends on: `021-database-migration-and-schema-workflow.md`, `022-testing-posture-and-shared-harnesses.md`, `024-new-app-bootstrap-and-bring-up.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `111-consumer-template-adoption-and-exception-policy.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Define the retained checklist and machine-readable artifact used to review an
normal Underlay consumer app against the live contract set.

This contract covers:

- the review domains every normal app audit should check
- the classification model for findings
- the split between prose checklist and machine-readable artifact
- the minimum evidence expected from a consumer audit

It does not replace the underlying contracts. It turns them into a repeatable
review tool.

## Sources of Truth

Primary supporting contracts:

- [`docs/contracts/021-database-migration-and-schema-workflow.md`](/Users/tom/Dev/projects/underlay/docs/contracts/021-database-migration-and-schema-workflow.md)
- [`docs/contracts/022-testing-posture-and-shared-harnesses.md`](/Users/tom/Dev/projects/underlay/docs/contracts/022-testing-posture-and-shared-harnesses.md)
- [`docs/contracts/023-release-and-compatibility-rollout.md`](/Users/tom/Dev/projects/underlay/docs/contracts/023-release-and-compatibility-rollout.md)
- [`docs/contracts/024-new-app-bootstrap-and-bring-up.md`](/Users/tom/Dev/projects/underlay/docs/contracts/024-new-app-bootstrap-and-bring-up.md)
- [`docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`](/Users/tom/Dev/projects/underlay/docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md)
- [`docs/contracts/026-route-families-and-access-model.md`](/Users/tom/Dev/projects/underlay/docs/contracts/026-route-families-and-access-model.md)
- [`docs/contracts/028-runtime-surface-and-openapi-maturity-levels.md`](/Users/tom/Dev/projects/underlay/docs/contracts/028-runtime-surface-and-openapi-maturity-levels.md)
- [`docs/contracts/031-config-and-secrets.md`](/Users/tom/Dev/projects/underlay/docs/contracts/031-config-and-secrets.md)
- [`docs/contracts/032-openapi-quality-and-declaration.md`](/Users/tom/Dev/projects/underlay/docs/contracts/032-openapi-quality-and-declaration.md)
- [`docs/contracts/033-error-codes-and-operator-audit.md`](/Users/tom/Dev/projects/underlay/docs/contracts/033-error-codes-and-operator-audit.md)
- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- [`docs/contracts/111-consumer-template-adoption-and-exception-policy.md`](/Users/tom/Dev/projects/underlay/docs/contracts/111-consumer-template-adoption-and-exception-policy.md)
- [`docs/contracts/120-tooling-testing-and-contract-artifacts.md`](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)

Machine-readable artifact:

- [`docs/contracts/app-review/underlay-app-review-checklist.json`](/Users/tom/Dev/projects/underlay/docs/contracts/app-review/underlay-app-review-checklist.json)

Supporting workflow guidance:

- [`docs/guides/200-project-sync.md`](/Users/tom/Dev/projects/underlay/docs/guides/200-project-sync.md)
- [`docs/guides/172-agents-files.md`](/Users/tom/Dev/projects/underlay/docs/guides/172-agents-files.md)

If these diverge, the contract plus the machine-readable checklist win.

## Contract Goal

Underlay should make consumer audits repeatable.

A maintainer should not have to rediscover:

- which domains must be checked in every normal app
- what counts as compliant versus drift versus explicit exception
- how to summarize review outcomes without rewriting the contract stack from
  scratch

The goal is one retained audit frame that stays aligned with the live contract
set.

## Scope Boundary

In scope:

- normal Underlay consumer app audits
- admin, API, and workspace-level contract review
- checklist domain set
- finding classification and scoring posture
- machine-readable checklist artifact

Out of scope:

- auto-remediation
- CI enforcement implementation
- one-off product/domain review criteria outside the shared contract surface

## Shared Boundary

### Review-domain rule

Every normal Underlay consumer audit should review these domains:

- workspace bootstrap and docs authority
- migration and schema workflow
- testing posture
- Rust runtime assembly
- route families and access model
- runtime/OpenAPI maturity
- config and secrets
- error-code and operator-audit posture
- admin template adoption
- media/system/dashboard retained shell adoption where relevant
- release and compatibility rollout posture for any ongoing migrations

Rules:

- the checklist should stay contract-shaped, not repo-shaped
- do not add app-specific product review lanes to the shared artifact
- domains may be skipped only when the app genuinely does not own that surface

### Finding-classification rule

Every review item should end in one of:

- `compliant`
- `drift`
- `exception`
- `not_applicable`

Rules:

- use `drift` when the app is off the shared contract and should converge
- use `exception` only when a retained contract already allows that posture
  explicitly
- use `not_applicable` when the app truly does not own the surface
- do not hide drift inside vague “special case” notes

### Severity rule

Findings may also carry a severity:

- `high`
- `medium`
- `low`
- `none`

Rules:

- use `high` for security, runtime-breakage, migration, or compatibility-risk
  drift
- use `medium` for clear contract divergence that is not immediately dangerous
- use `low` for polish or weaker maturity-level gaps where the app is still
  broadly compatible
- `exception` and `not_applicable` normally carry `none`

### Evidence rule

Every audit should preserve lightweight evidence.

Minimum evidence:

- app/workspace id
- review date
- reviewer
- checklist version or artifact version
- one note per domain with classification

Rules:

- the machine-readable artifact should stay human-reviewable
- evidence should point back to the governing contract id for each domain
- a prose audit may expand the notes, but the domain coverage must still map
  back to the shared artifact

### Machine-readable artifact rule

The JSON artifact is the durable audit skeleton.

It should define:

- stable domain ids
- labels
- governing contract refs
- whether the domain is normally required
- the allowed classification vocabulary

Rules:

- keep the artifact small and durable
- do not turn it into a generated dump of app-specific results
- app-specific audit outputs may consume this artifact, but should not mutate
  the source checklist in place

### Prose-checklist rule

The contract is the human audit guide.

Rules:

- the prose contract explains how to use the artifact
- the artifact defines the stable machine-readable domain set
- if the two drift, update both in the same batch

## Minimum Audit Output

A normal consumer audit should produce:

- app name
- review date
- reviewer
- per-domain classification
- any `high` or `medium` drift callouts
- the next recommended normalization move

Optional stronger output:

- absolute file references
- grouped fix batches
- comparison against a previous audit snapshot

## What Good Looks Like

Good outcomes:

- audits across `underlay-reference`, `acowtancy`, `compli-me`,
  `contact-patch`, `songsprout`, and `loophole/composer` use the same domain
  vocabulary
- drift is separated cleanly from explicit exceptions
- follow-on work can be opened as bounded cards instead of broad rediscovery

Bad outcomes:

- each audit invents its own checklist
- maturity gaps, exceptions, and drift are all mixed together
- audit notes do not point back to governing contracts
- the machine-readable artifact grows into an app-specific report dump

## Next Task

Use this contract and the companion checklist artifact whenever a consumer app
needs a contract-backed audit or review refresh.
