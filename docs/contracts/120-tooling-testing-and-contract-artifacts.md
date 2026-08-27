# Contract: Tooling Testing and Contract Artifacts

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `030-auth-and-session-systems.md`, `040-storage-blob-and-media-systems.md`, `060-jobs-events-and-operator-systems.md`, `070-nightfire-and-migration-systems.md`, `090-ts-runtime-and-client-orchestration.md`, `100-shared-patterns-and-workflow-shells.md`, `110-admin-template-system.md`

## Purpose

Define the retained support-layer contract Underlay owns across:

- reusable Rust test infrastructure for DB-backed and HTTP-bound systems
- reusable migration and schema devtools
- TS architectural guardrails and rule-pack templates
- TS test doubles for the shared client surface
- machine-readable contract artifacts that pin durable compatibility rules

This contract does not define repo-local release automation, CI wiring,
consumer-app test suites, or one-off maintenance scripts. It fixes the shared
support surfaces other systems rely on when they are built, tested, migrated,
or audited.

## Sources of Truth

Primary Rust support:

- [`../../rust/crates/underlay-testing/src/lib.rs`](../../rust/crates/underlay-testing/src/lib.rs)
- [`../../rust/crates/underlay-testing/src/fixtures.rs`](../../rust/crates/underlay-testing/src/fixtures.rs)
- [`../../rust/crates/underlay-testing/src/test_db.rs`](../../rust/crates/underlay-testing/src/test_db.rs)
- [`../../rust/crates/underlay-testing/src/test_server.rs`](../../rust/crates/underlay-testing/src/test_server.rs)
- [`../../rust/crates/underlay-devtools/src/lib.rs`](../../rust/crates/underlay-devtools/src/lib.rs)
- [`../../rust/crates/underlay-devtools/src/sync_migrations.rs`](../../rust/crates/underlay-devtools/src/sync_migrations.rs)
- [`../../rust/crates/underlay-devtools/src/migration_bundle.rs`](../../rust/crates/underlay-devtools/src/migration_bundle.rs)
- [`../../rust/crates/underlay-devtools/src/seed_bundle.rs`](../../rust/crates/underlay-devtools/src/seed_bundle.rs)
- [`../../rust/crates/underlay-devtools/src/migration_report.rs`](../../rust/crates/underlay-devtools/src/migration_report.rs)

Primary TS support:

- [`../../ts/src/tools/guardrails.ts`](../../ts/src/tools/guardrails.ts)
- [`../../ts/src/tools/guardrails-config.ts`](../../ts/src/tools/guardrails-config.ts)
- [`../../ts/src/tools/guardrails/scanner.ts`](../../ts/src/tools/guardrails/scanner.ts)
- [`../../ts/src/tools/guardrails/line-utils.ts`](../../ts/src/tools/guardrails/line-utils.ts)
- [`../../ts/src/tools/guardrails/suppressions.ts`](../../ts/src/tools/guardrails/suppressions.ts)
- [`../../ts/src/tools/templates/sveltekit-ssr.ts`](../../ts/src/tools/templates/sveltekit-ssr.ts)
- [`../../ts/src/tools/templates/banned-apis.ts`](../../ts/src/tools/templates/banned-apis.ts)
- [`../../ts/src/tools/workspace-shape.ts`](../../ts/src/tools/workspace-shape.ts)
- [`../../ts/src/tools/env-authority.ts`](../../ts/src/tools/env-authority.ts)
- [`../../ts/src/testing/index.ts`](../../ts/src/testing/index.ts)
- [`../../ts/src/testing/http-client-mock.ts`](../../ts/src/testing/http-client-mock.ts)

Primary machine-readable artifacts:

- [`../../contracts/openapi/underlay.openapi.yaml`](../../contracts/openapi/underlay.openapi.yaml)
  — the shared response-envelope schema reference. Kept in sync with
  `ts/src/client/envelopes.ts` by the `envelope-contract-drift` test (fails when
  the two surfaces declare different envelopes or required fields).
- [`api-surface/endpoint-family-matrix.csv`](./api-surface/endpoint-family-matrix.csv)

Historical audit snapshots (from the `g01` poodle-adoption wave; retained for the
archival record, read by no live check — do not treat as authoritative):

- [`../../contracts/ui/poodle-underlay-coexistence-contract.json`](../../contracts/ui/poodle-underlay-coexistence-contract.json)
- [`../../contracts/ui/poodle-adoption-underlay-surface-groups.json`](../../contracts/ui/poodle-adoption-underlay-surface-groups.json)
- [`../../contracts/ui/poodle-prop-normalization-manifest.json`](../../contracts/ui/poodle-prop-normalization-manifest.json)

Supporting:

- [`contract-index.md`](./contract-index.md)
- [`../architecture/system-inventory.md`](../architecture/system-inventory.md)

If these diverge, the code and machine-readable artifacts win over planning
docs.

## Contract Goal

Underlay should provide one honest shared support layer with clear seams:

- shared systems can be tested without re-inventing DB and HTTP harnesses in
  every repo
- migration and schema operations have repeatable shared tooling instead of ad
  hoc scripts
- TS consumers can enforce SSR and banned-API rules with shared scanners and
  rule packs
- machine-readable artifacts pin the narrow parts of the contract surface that
  need durable, automatable checks

The goal is reusable platform support and compatibility evidence, not a grab
bag of repo-local ops glue.

## Shared Boundary

### Rust test infrastructure

`underlay-testing` owns the retained shared Rust test harnesses.

Core pieces:

- `Fixtures`
- `AuthFixtures`
- `TimestampFixtures`
- `TestDb`
- `TestServer`
- `TestResponse`

Rules:

- fixture helpers provide stable seed values and timestamp/user defaults for
  shared-system tests
- `TestDb` is the shared DB-backed integration-test seam
- `TestDb` owns container startup, unique schema isolation, migration/seed
  helpers, fixture-file loading, and cleanup
- `TestDb` must route generated schema names through the shared typed SQL
  identifier boundary before creating schemas, setting search paths, or cleanup
- DB-backed tests are feature-gated and must fail clearly when Docker is not
  available rather than silently degrading
- `TestServer` is the shared in-memory HTTP/router test seam over Axum
- `TestServer` owns request building, auth-header helpers, JSON body helpers,
  and response decoding helpers
- shared crates should prefer these harnesses over re-creating one-off DB and
  router test setups

### TS test helpers

`ts/src/testing` owns the retained TS test double surface.

Core piece:

- `createMockHttpClient()`

Rules:

- the mock must match the shared `HttpClient` contract shape closely enough for
  runtime, pattern, and template tests to use it without app-local adapters
- callers may queue whole responses or register per-method/path responders
- the mock records calls as inspectable test evidence
- this surface stays narrow and transport-focused; it is not a general frontend
  test framework

### Rust devtools and migration operations

`underlay-devtools` owns the reusable migration/schema operations layer.

Core pieces:

- DB env and connection helpers:
  - `require_env()`
  - `connect()`
  - `migrate()`
  - `migrate_with()`
  - `reset_schemas()`
  - `migrate_from_env()`
  - `reset_from_env()`
- migration sync:
  - `sync_migrations()`
- migration bundle operations:
  - build
  - publish
  - pull
  - run
- seed bundle operations:
  - build
  - publish
  - pull
- migration report operations:
  - load
  - summarize
  - audit
  - drift/integrity/policy/recovery/verification formatting

Rules:

- DB env helpers are the shared seam for repeatable migration/reset operations
  against configured databases
- `sync_migrations()` owns canonical migration mirroring from source crates into
  target dirs, including filename validation, mismatch detection, and dry-run
  support
- migration bundle and seed bundle formats are shared operational artifacts, not
  consumer-app private conventions
- `MigrationBundleRef` is the public digest-pinned bundle-ref construction seam
  for migration run operations
- local bundle-store and remote registry modules are implementation internals;
  callers use build, publish, pull, and run options instead of store-specific
  helpers
- migration reports are durable post-run evidence over
  `underlay-migration-core` artifacts, not ephemeral console-only output
- devtools may expose CLI-friendly wrappers, but the shared contract is the
  library surface and artifact behavior, not one specific binary invocation

### TS guardrails and scanner rule packs

`ts/src/tools` owns the retained architectural guardrail scanner surface.

Core pieces:

- `guardrails.ts`
- guardrail config loading
- the custom scanner for module-scope browser API detection
- suppression parsing
- shared rule-pack templates:
  - `sveltekit-ssr`
  - `banned-apis`

Rules:

- the scanner exists to enforce shared architectural constraints across TS and
  Svelte source, not only to lint Underlay itself
- rule packs must stay declarative and reusable by consumer repos through the
  exported `@inflatable-cookie/underlay/tools/templates/*` package surface rather
  than only by source-checkout-relative paths
- SSR guardrails focus on module-scope browser API misuse and similar runtime
  boundary mistakes
- banned-API rules focus on prohibited public browser APIs and similar shared
  safety/UX bans
- suppressions must be explicit and parseable rather than hidden in arbitrary
  comments

### Consumer workspace-shape checker

`ts/src/tools/workspace-shape.ts` owns the retained consumer Bun workspace
topology check.

Core pieces:

- `checkWorkspaceShape()`
- `formatWorkspaceShapeReport()`
- stable rule ids for Git root posture, root manifest fields, explicit workspace
  paths, supported `apps/*` / `packages/*` prefixes, root/child lockfiles,
  internal JavaScript dependency edges, and committed `file:` Underlay/Poodle
  dependencies

Rules:

- the checker validates contract `024` workspace topology mechanically and stays
  separate from security conformance in
  `scripts/check-consumer-conformance.sh`
- consumers invoke the distributed export
  `@inflatable-cookie/underlay/tools/workspace-shape` through the published
  `underlay-workspace-shape` bin entry from an Effigy-owned task such as
  `qa:workspace-shape`, then compose that task into `health` or `validate`
- Underlay wires fixture self-tests through `check:workspace-shape` and does not
  run the consumer topology check against its own foundation root
- diagnostics must identify a stable rule id, repo-relative path, and offending
  value with deterministic sort order and a non-zero exit on drift

### Consumer env-authority checker

`ts/src/tools/env-authority.ts` owns the retained static env/secret-inventory
check.

Core pieces:

- `checkEnvAuthority()`
- `formatEnvAuthorityReport()`
- stable rule ids for missing authority files, invalid key-file syntax, and
  required keys that are not declared in the env manifest

Rules:

- the checker validates contract `024` env/secret authority files mechanically
  and stays separate from workspace-shape and from
  `scripts/check-env-manifest.sh`
- it must not read `.env` files or secret values, and it must not invent which
  product keys are mandatory
- consumers invoke the distributed export
  `@inflatable-cookie/underlay/tools/env-authority` through the published
  `underlay-env-authority` bin entry from an Effigy-owned task such as
  `qa:env-authority`
- Underlay wires fixture self-tests through `check:env-authority` and does not
  run the consumer env check against its own foundation root
- live value presence remains `scripts/check-env-manifest.sh` and must not
  become a CI requirement for material secrets
- diagnostics must identify a stable rule id, repo-relative path, and a redacted
  reason (line number or key token, never a secret value) with deterministic
  sort order and a non-zero exit on drift

### Machine-readable contract artifacts

`contracts/**` owns durable machine-readable reference artifacts where prose
alone is not enough.

Core families:

- `openapi/underlay.openapi.yaml` (envelope schema; drift-checked against
  `ts/src/client/envelopes.ts`)
- `docs/contracts/api-surface/endpoint-family-matrix.csv`

Historical (g01 poodle-adoption snapshots, read by no live check):

- `ui/poodle-underlay-coexistence-contract.json`
- `ui/poodle-adoption-underlay-surface-groups.json`
- `ui/poodle-prop-normalization-manifest.json`

Rules:

- machine-readable artifacts pin high-value shared compatibility boundaries that
  benefit from automation, bulk audit, or cross-repo coordination
- the OpenAPI artifact is currently a narrow schema reference for shared
  envelope shapes, not a full API catalog
- the endpoint-family matrix is a durable planning and normalization artifact
  for the current app-server route-family posture; it is not a generated route
  dump and should stay human-reviewable
- UI JSON artifacts are retained migration and coexistence policy records. They
  are currently preserved compatibility evidence, not part of the live automated
  repo check surface
- when a machine-readable artifact exists for a contract area, consumer or repo
  checks may rely on it directly only if that enforcement path is still live
- these artifacts must stay versioned and human-reviewable even when generated
  or bulk-maintained

## Ownership Split

Underlay owns:

- reusable shared test harnesses
- reusable migration/schema/bundle/report tooling
- reusable TS architectural scanners and rule packs
- narrow shared TS test doubles
- durable machine-readable contract artifacts and preserved compatibility
  evidence for the retained shared surface

Apps own:

- app-local fixtures and seed semantics beyond the shared helpers
- app-local release scripts, CI workflows, and deployment automation
- app-local lint rules and bans that are not genuinely shared portfolio policy
- test suites and migration orchestration around the shared support libraries

Other system families own:

- the actual lower protocol and runtime contracts these tools exercise
- feature-specific docs, APIs, and workflow semantics outside the support layer

## Invariants

- support crates and artifacts must stay reusable across multiple repos and may
  not hard-code one consumer app's domain language
- testing helpers must preserve isolation and determinism rather than optimizing
  for convenience through hidden shared state
- devtools outputs that affect migrations or replay must be durable enough for
  audit and recovery workflows
- guardrail checks must remain source-readable and suppression-aware rather than
  relying on opaque build-step magic
- machine-readable contract artifacts must stay narrower than full roadmap prose
  and only encode durable compatibility or migration rules

## Known Drift To Assess Later

- `TestDb` isolates one generated schema. The `g09.044` operator decision keeps
  fixed named multi-schema whole-app suites app-owned unless later evidence
  justifies a separately designed shared lifecycle.
- `TestServer` has one bounded Underlay Reference health-route proof; broader
  adoption remains demand-led rather than a fleet rewrite.
- `underlay-devtools` mixes genuinely generic DB/env helpers with machinery that
  is tightly coupled to Underlay's migration/media stack, so its internal
  ownership split may still be too broad
- the machine-readable contract surface is uneven: OpenAPI remains a live narrow
  schema artifact, while the UI JSON files are now closer to preserved migration
  evidence than active checked authority
- `contracts/openapi/underlay.openapi.yaml` is only a thin envelope/schema
  fragment and may over-signal completeness compared with the real shared API
  surface
- the TS implementation test surface is broad, but some retained public import
  paths still rely on indirect implementation tests rather than focused package
  compatibility coverage
- the guardrail scanner runs as part of repo health, but scanner/config/template
  behavior still needs direct self-tests
- some guardrail rule packs may still reflect historical migration pressure
  rather than a clearly bounded long-term shared policy set

## Assessment Questions

- should `underlay-devtools` stay one crate, or does the migration-bundle and
  report stack now deserve a sharper package split from the simpler DB/env
  helpers
- which additional machine-readable artifacts would materially improve contract
  enforcement without recreating the whole repo in JSON or OpenAPI
- which retained TS public paths need direct package-compatibility tests beyond
  implementation-level pattern/client coverage
- are the UI coexistence and prop-normalization manifests still accurate enough
  to remain preserved compatibility evidence, or should they be archived or
  replaced

## Next Task

Keep the completed workspace-shape modularization stable. A later change must
preserve the checker facade, diagnostics, and separation from env-authority and
security conformance through a new numbered roadmap.
