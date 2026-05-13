# Contract: New App Bootstrap and Bring-Up

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `110-admin-template-system.md`, `120-tooling-testing-and-contract-artifacts.md`

## Purpose

Define the default repo shape and bring-up posture for a normal Underlay app
workspace.

This contract covers:

- root workspace layout
- expected package and crate families
- docs authority placement
- Effigy-first bootstrap and bring-up posture
- default local dependency and mount expectations
- minimum day-one config and env surfaces

It does not define detailed route grammar, template APIs, or migration policy.
Those build on top of this layer.

## Sources of Truth

Reference workspace evidence:

- [`underlay-reference/README.md`](/Users/tom/Dev/projects/underlay-reference/README.md)
- [`underlay-reference/effigy.toml`](/Users/tom/Dev/projects/underlay-reference/effigy.toml)
- [`underlay-reference/acme-api/README.md`](/Users/tom/Dev/projects/underlay-reference/acme-api/README.md)
- [`underlay-reference/acme-admin/README.md`](/Users/tom/Dev/projects/underlay-reference/acme-admin/README.md)
- [`acowtancy/README.md`](/Users/tom/Dev/projects/acowtancy/README.md)
- [`compli-me/README.md`](/Users/tom/Dev/projects/compli-me/README.md)
- [`compli-me/effigy.toml`](/Users/tom/Dev/projects/compli-me/effigy.toml)
- [`compli-me/api/README.md`](/Users/tom/Dev/projects/compli-me/api/README.md)
- [`compli-me/admin/README.md`](/Users/tom/Dev/projects/compli-me/admin/README.md)
- [`contact-patch/README.md`](/Users/tom/Dev/projects/contact-patch/README.md)
- [`contact-patch/effigy.toml`](/Users/tom/Dev/projects/contact-patch/effigy.toml)
- [`songsprout/README.md`](/Users/tom/Dev/projects/songsprout/README.md)
- [`songsprout/effigy.toml`](/Users/tom/Dev/projects/songsprout/effigy.toml)
- [`songsprout/nursery/README.md`](/Users/tom/Dev/projects/songsprout/nursery/README.md)
- [`songsprout/greenhouse/README.md`](/Users/tom/Dev/projects/songsprout/greenhouse/README.md)
- [`loophole/composer/README.md`](/Users/tom/Dev/projects/loophole/composer/README.md)
- [`loophole/composer/effigy.toml`](/Users/tom/Dev/projects/loophole/composer/effigy.toml)
- [`loophole/composer/composer-api/README.md`](/Users/tom/Dev/projects/loophole/composer/composer-api/README.md)
- [`loophole/composer/composer-admin/README.md`](/Users/tom/Dev/projects/loophole/composer/composer-admin/README.md)

Supporting shared contracts:

- [`docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md`](/Users/tom/Dev/projects/underlay/docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md)
- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- [`docs/contracts/120-tooling-testing-and-contract-artifacts.md`](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)

If these diverge, the contract plus the clearest modern workspace posture
(`underlay-reference`, `compli-me`, `contact-patch`) win.

## Contract Goal

Underlay should make a new app workspace boring to start.

A normal team should not have to rediscover:

- which packages belong in the workspace
- where docs authority lives
- how root and package Effigy tasks are split
- which local sibling mounts are expected
- which env and config files must exist on day one

The goal is one declared bootstrap posture that new workspaces can copy without
local folklore.

## Scope Boundary

In scope:

- monorepo or workspace root shape
- default package families
- root and package README/Effigy posture
- local bootstrap and bring-up flow
- app-local env-manifest and secret-bootstrap expectations
- root-level dev orchestration

Out of scope:

- app domain modeling
- detailed API route shape
- release policy
- migration policy beyond bring-up hooks

## Shared Boundary

### Workspace root shape

A normal Underlay app workspace should expose a clear root with:

- one root `README.md`
- one root `effigy.toml`
- one root package-manager manifest when JS orchestration is used
- one docs authority directory
- explicit child packages for API and apps

Normal package family:

- `api/` or equivalent Rust backend
- `admin/` admin SvelteKit app
- optional `front/` product-facing SvelteKit app
- optional `client/` shared API client package
- optional `ui/` shared UI package
- `docs/` or equivalent docs authority

Allowed naming variation:

- names may be product-specific:
  - `cp-api`
  - `nursery`
  - `greenhouse`
  - `composer-admin`
- but the ownership split should stay recognizable

Rules:

- the root should describe package roles plainly
- docs authority must be explicit
- app-local naming is allowed, but package roles should still map back to the
  normal Underlay family

### Docs authority rule

The workspace must declare where planning and architecture authority lives.

Allowed patterns:

- local `docs/` inside the workspace root
- a named docs package inside the workspace
- a clearly declared sibling docs authority repo when the product already uses
  that model

Rules:

- do not leave docs authority implicit
- package READMEs may point to the docs authority instead of duplicating it
- new workspaces should prefer one obvious local docs authority unless there is
  an established multi-repo reason not to

### Effigy-first root posture

The root workspace should use Effigy as the default command surface.

Expected root loop:

- `effigy tasks`
- `effigy health`
- `effigy test --plan`

Expected common root commands:

- `effigy dev`
- `effigy validate`
- `effigy qa`
- `effigy db:migrate`
- `effigy db:reset`

Rules:

- root tasks own cross-package orchestration
- root tasks should not duplicate child-only responsibilities
- DB tasks may resolve through child-catalog routing instead of being
  reimplemented at the root

### Package-local Effigy posture

Each owned app package should also expose a small repo-local Effigy loop.

Expected package loop:

- `effigy tasks --repo .`
- `effigy health --repo .`
- `effigy test --plan --repo .`

Expected common package commands:

- API package:
  - `effigy dev --repo .`
  - `effigy db:migrate --repo .`
  - `effigy db:reset --repo .`
- admin/front package:
  - `effigy dev --repo .`
  - `effigy check --repo .`
  - `effigy validate --repo .`

Rules:

- package READMEs should point at direct `effigy ... --repo .` usage as the
  canonical local loop
- package scripts may remain convenience wrappers, but should not be treated as
  the main shared posture

### Bootstrap rule

A normal Underlay workspace should support first-time setup through:

- `effigy bootstrap <repo-url>`
- optional `--start`

Rules:

- bootstrap should set up owned packages and declared children
- bootstrap should run the dependency setup task
- bootstrap should be able to leave the workspace ready for immediate health or
  dev commands
- `--start` should be the explicit opt-in for launching the live dev stack

### Bundle and child-workspace rule

When a workspace uses the shared Effigy bundle model, the root `effigy.toml`
should declare:

- `bundle.base`
- host/project metadata
- `bundle.dirs`
- local system mounts

Rules:

- `bundle.dirs` should map the real docs/api/client/ui/front/admin ownership
  split
- child repos are allowed when a workspace is intentionally multi-repo
- child bootstraps should stay explicit in the manifest rather than hidden in
  prose

### Local dependency rule

Current Underlay app workspaces repeatedly depend on sibling checkouts for:

- `../underlay` or equivalent relative path
- `../poodle` or equivalent relative path

Rules:

- required sibling mounts must be declared in the root workspace README and
  Effigy config
- bootstrap notes should say whether those siblings are expected or managed
- relative mount differences are acceptable when repo nesting differs, but the
  dependency model should stay explicit

### Environment and config rule

A normal workspace should expose at least:

- `config/env-manifest.txt` for runtime app packages that still read env
- `config/required-secrets.txt` for startup-critical secret/runtime keys
- API config layering notes where the API supports typed config files

Expected API config posture when applicable:

1. committed defaults
2. local override file
3. env injection highest precedence

Rules:

- runtime packages should not rely on undocumented env keys
- runtime packages should not depend on committed `.env` files
- admin/front packages should document the minimum public env surface
- API packages should document config precedence when they support layered
  config

### Bring-up rule

A new workspace should have one boring bring-up path:

1. bootstrap the workspace
2. run `effigy health`
3. run `effigy test --plan`
4. run `effigy dev` or `effigy dev <surface>`
5. use `db:migrate` or `db:reset` from the root when DB ownership is routed

Rules:

- the root README should make this path obvious
- package READMEs should refine local loops without fighting the root story
- local URLs, gateway hosts, and container/dev notes belong in the root
  workspace docs when they matter

## What Good Looks Like

Good outcomes:

- every new workspace has the same basic package family and root story
- docs authority is explicit
- Effigy is the obvious root and package loop
- bootstrap and bring-up do not depend on tribal memory
- sibling `underlay` and `poodle` dependencies are declared, not discovered by
  failure

Bad outcomes:

- root and child tasks duplicate each other arbitrarily
- package roles are unclear from names and docs
- bootstrap requires undocumented manual setup
- `config/env-manifest.txt` is missing for runtime apps that still read env
- docs authority is spread or implied

## Questions This Contract Should Settle

- What must a normal Underlay app repo contain on day one?
- Which roles belong at the root versus inside child packages?
- What is the canonical first-run bring-up flow?
- What should come from reference posture versus app-local naming?

## Next Task

Use this contract as the base for new-app scaffolding, checklist artifacts, and
consumer repo normalization where workspace bring-up still drifts.
