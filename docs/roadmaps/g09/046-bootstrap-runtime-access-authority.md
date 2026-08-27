# g09.046 - Bootstrap, Runtime, And Access Authority

Status: complete
Owner: repo maintainers
Contracts: `024-new-app-bootstrap-and-bring-up.md`,
`025-rust-app-runtime-assembly-and-router-topology.md`,
`026-route-families-and-access-model.md`,
`121-underlay-app-review-checklist-and-audit-artifact.md`
Found by: `g09.045`
Depends on: `g09.045`

## Purpose

Repair the shared authority and conformance seams exposed by the six-root
assessment before any consumer runtime or security rollout starts.

## Scope

- replace retired pre-monorepo and machine-local source links in contracts
  `025` and `026` with current portable evidence
- remove completed “write the next contract” directions and replace the stale
  fleet-drift summaries with current assessment state
- align guide `070` with the thin-binary, dedicated-router posture
- make runtime ownership distinct from shared business routes across `025` and
  `026`
- settle API-version wording: path versioning is baseline; header policy is
  optional until declared; declared headers apply consistently to business
  routes and exclude runtime routes
- distinguish workspace children from explicit external read-only inputs and
  sibling tooling mounts without reopening polyrepo support
- extend workspace-shape proof to reject declared JavaScript workspaces outside
  `apps/*` / `packages/*` and committed `file:` Underlay/Poodle dependencies
- expose env-manifest validation as a separate contract-121 mechanical review
  seam, covering both `env-manifest.txt` and `required-secrets.txt`
- update templates, checklist artifact, active guides, and focused tests

## Acceptance

- active contracts and guide `070` describe one runtime/router posture
- no active contract source link names a retired consumer path or local machine
  root
- header versioning has one non-contradictory rule
- external tooling/content inputs cannot be mistaken for workspace ownership,
  nested repos, or released-dependency substitutes
- workspace-shape rejects unsupported workspace prefixes and shared external
  `file:` dependencies with stable rule IDs and tests
- the app-review artifact has a distinct env/secret-inventory mechanical check
- the env check fails when a runtime env reader has no declared authority and
  does not infer which product secrets are mandatory
- published tool/bin compatibility and the existing six-root workspace pass are
  preserved

## Validation

- `effigy test --plan`
- focused workspace-shape and env-manifest tests through Effigy
- `effigy health`
- `effigy validate`
- `effigy qa:docs`
- `effigy qa:northstar`
- published-bin smoke against Underlay Reference
- `git diff --check`

## Stop Conditions

Stop if a generic check would need to invent an app's mandatory secret list,
deployment proxy topology, CSRF exception, or route compatibility window. Keep
those decisions in the owning consumer roadmap.

## Consumer Upgrade Impact

- Impact class: additive conformance and documentation hardening
- Affected consumers: all six roots
- Required action: adopt the env/secret authority files and satisfy any new
  workspace rule before `g09.053`
- Compatibility window: the existing workspace topology stays supported; new
  conformance failures identify contract drift, not a new topology

## Evidence

- contracts `024`-`026` use repo-local Underlay links and current
  repo-relative consumer paths; runtime is a distinct family; path versioning
  is baseline and header policy is optional until declared
- guide `070` shows a thin `main.rs`, `state.rs`, and
  `routes/{runtime,shared,admin,front?}` ownership
- workspace-shape adds `workspace-prefix-unsupported` and
  `shared-file-dependency`; existing six-root pass is preserved
- `underlay-env-authority` is a published static check; live values stay in
  `scripts/check-env-manifest.sh`
- contract `121` and the JSON artifact add the env-authority mechanical check
- live Underlay Reference: workspace-shape pass; env-authority fails on missing
  `config/env-manifest.txt` and `config/required-secrets.txt` without reading
  secrets
- merged in PR9 as merge commit `7d8c0bae`

See
[`docs/logs/2026-08/26-232742-g09-046-bootstrap-runtime-access-authority.md`](../../logs/2026-08/26-232742-g09-046-bootstrap-runtime-access-authority.md).

## Next Task

Keep `g09.047` planned until the merged tooling is released, Underlay Reference
is current, and its app-owner security decisions are explicit.
