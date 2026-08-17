# Reference-Grade Underlay Architecture

Status: active
Owner: repo maintainers

## Purpose

Underlay should become the reference foundation for the current consumer family,
not just a collection of reusable crates and TypeScript helpers.

The current codebase is good enough to evolve in place, but the ideal shape is
stricter:

- fewer public seams
- clearer capability boundaries
- typed construction at every unsafe edge
- adapter code isolated from platform contracts
- devtools and migration tooling kept out of runtime application surface
- consumer compatibility proved deliberately, not inferred

The six known consumers are not in production, so breaking changes are allowed
when they buy durable clarity. They still need rollout proof under contract
`023`.

## Target Shape

### Platform Crates

Platform crates define stable concepts and app-facing traits. They should have
small public roots and no concrete backend policy.

Target families:

- `underlay-core`: IDs, envelopes, errors, shared primitive types.
- `underlay-http`: HTTP boundary, request context, response helpers, CORS,
  cookies, pagination, query contracts.
- `underlay-auth`: auth/session/user/credential traits and canonical domain
  rows.
- `underlay-data`: database identifiers, pool/migration contracts, pagination,
  existence checks, soft-delete conventions.
- `underlay-ops`: audit, security alerts, jobs, metrics, observability, and
  rate limiting contracts.
- `underlay-storage`: blob and media contracts, storage keys, and repository
  traits.
- `underlay-migration`: deterministic migration pipeline contracts and bundle
  reference types.

The exact crate names may be introduced through package renames or module
consolidation. The important rule is ownership: contracts first, adapters
second.

### Adapter Crates

Concrete backends should live behind explicit adapter packages or modules.

Target examples:

- Postgres adapters for data, auth, jobs, audit, security alerts, and media.
- AWS/S3/SES adapters for storage and email.
- SMTP and development email capture adapters.
- Local filesystem blob adapter.
- Testcontainers and HTTP test harnesses.

Adapters may depend on platform crates. Platform crates should not depend on
adapter crates.

### Tooling Crates

Migration bundles, seed bundles, sync tools, reports, and developer reset
helpers are tooling. They should not be imported by runtime application crates.

Target rule:

- `underlay-devtools` can depend on runtime contracts.
- runtime platform crates must not depend on `underlay-devtools`.
- consumer apps should use tooling through commands or explicit dev-only
  dependencies.

### Public Roots

Each public crate root should expose:

- canonical app-facing traits and typed values
- short constructors and validated builders
- feature-gated adapter entrypoints only when the adapter remains intentionally
  packaged inside the crate

Each public crate root should avoid:

- operation modules used only by a concrete backend
- migration SQL constants unless they are the documented adapter contract
- test-only helpers
- broad glob-style re-export barrels

### Typed Boundaries

Typed construction is mandatory where data crosses a safety boundary:

- SQL identifiers and qualified table names
- cookie names, paths, domains, and values
- blob object keys and storage prefixes
- migration bundle references
- email addresses and message headers
- job names, queue names, and idempotency keys
- auth token fingerprints and session identifiers
- OpenAPI/client route identifiers where generated contracts depend on them

Raw string constructors may remain temporarily, but only as compatibility
surface with a documented replacement and retirement path.

## Breaking-Change Posture

Because the six known consumers are not in production:

- breaking changes are allowed when they remove ambiguous public surface
- compatibility shims are optional, not default
- consumer app updates can happen in the same lane as Underlay changes
- every breaking batch must still name affected consumers and validation
  commands

Rules:

- break narrowly, in batches with one ownership theme
- update `underlay-reference` first unless another consumer is the clearer proof
- update the remaining affected consumers before declaring the batch complete
- leave release/upgrade notes even when there is no external production user
- do not create compatibility aliases unless the consumer rollout genuinely
  needs staged adoption

## Transition Strategy

### Phase 1: Authority And Inventory

Define the target package families, current-to-target mapping, and consumer
dependency graph.

Outputs:

- target architecture note
- crate family map
- consumer dependency matrix
- breaking-change policy for the reference-grade reset

### Phase 2: Public Surface Diet

Narrow crate roots and public barrels before large internal rewrites.

Preferred work:

- move implementation modules behind explicit submodules
- stop re-exporting adapter internals from platform roots
- mark raw-string constructors as compatibility targets
- remove dead public exports with consumer proof

### Phase 3: Adapter Isolation

Separate platform contracts from backend adapters.

Preferred work:

- split Postgres operation modules by repository responsibility
- keep adapter constructors concrete and typed
- move devtools-only helpers out of runtime crate dependency paths
- isolate AWS, SMTP, local filesystem, and test harness adapters

### Phase 4: Consumer Cutover

Update the six consumer apps as part of the generation.

Preferred order:

1. `underlay-reference`
2. the most affected app for the batch
3. remaining affected consumers

Every consumer update should remove old imports or patterns instead of adding
compatibility glue unless the batch explicitly needs a temporary bridge.

### Phase 5: Reference Baseline

Once the new package families are stable:

- refresh usage docs around the target shape
- retire compatibility exports
- run full Underlay validation and targeted consumer validation
- publish the final upgrade note for the reset generation

## Stop Conditions

Stop and re-enter planning if:

- a proposed package family hides two unrelated responsibilities
- a breaking change touches more than one ownership theme
- a consumer update requires app-local behavior to move into Underlay
- adapter code starts depending on devtools behavior
- a compatibility shim would outlive the current generation

## Next Task

Open a bounded `g10` roadmap card before retiring deferred compatibility
exports or starting another architecture reset lane. See
[`docs/roadmaps/g10/README.md`](../roadmaps/g10/README.md).
