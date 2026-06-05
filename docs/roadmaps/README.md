# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Keep one active queue per generation and use backlog for deferred scope.
- In sequential mode, maintain one active generation.
- In parallel mode, keep every active generation front door accurate for its
  thread.
- Keep durable inventories, CSVs, and machine-readable reference artifacts in
  [../contracts/](../contracts/).
- If active work changes consumer-visible behavior, APIs, configuration,
  migrations, or integration patterns, include a `Consumer Upgrade Impact`
  section in the roadmap.

## Generation model

- Use generation folders such as `docs/roadmaps/g01/`.
- Roadmap filenames use `NNN-<slug>.md`.
- Roadmap references use generation-qualified IDs such as `g01.021`.
- Generation rollover is manual only.
- Treat generations as substantial sequencing eras, not one-or-two-file
  buckets. A healthy default is roughly 20 to 40 roadmap files in one
  generation before rollover is even worth discussing.
- In sequential mode, close or rehome every roadmap in the current generation
  and purge stale specs from `docs/specs/` before opening the next generation.
- In parallel mode, multiple active generations may coexist when the work
  streams are genuinely independent. Each generation then operates as its own
  queue.

## Layout

- `gNN/` generation milestones
- `gNN/batch-cards/` generation-local execution cards when that generation uses
  strict posture
- `generation-index.md` active generation mode and history
- `backlog/` deferred items with promotion criteria

## Evidence Boundary

Roadmap bodies are execution records as well as plans. They may retain
sibling-repo file references, local path evidence, and concrete caller
inventories when that is necessary to preserve delivery history. Do not treat
that historical evidence style as the model for active library-facing guides or
README surfaces.

## Mode

- `parallel`

## Active generation

- `g06`

## Current Queue

- `g06.001` is complete as the Rust platform contract transition and public API
  inventory gate
- `g06.002` is complete as the typed safety primitive and construction-boundary
  migration lane
- `g06.003` is complete as the auth/session contract reset and refresh-rotation
  rollout proof
- `g06.004` is complete as the HTTP safe-builder consolidation and consumer
  cookie cleanup lane
- `g06.005` is complete as the DB identifier and schema boundary normalization
  lane
- `g06.006` is complete as the media repository contract and adapter split
  completion lane
- `g06.007` is complete as the devtools bundle/store boundary isolation lane
- `g06.008` is complete as the six-consumer compatibility proof and
  release-note closeout lane
- `g06.009` is complete as the Effigy doctor structural backlog triage lane
- `g06.010` is complete as the first Rust god-file split repair batch
- `g06.011` is complete as the second Rust structural split repair batch
- `g06.012` is complete as the high-severity Rust structural backlog triage
  lane
- `g06.013` is complete as the security-adjacent Rust adapter split batch
- `g06.014` is complete as the Rust platform transition validation and
  release-readiness closeout
- `g06.015` is complete as the Rust platform transition release-note handoff
- `g06.016` is complete as the Rust platform hardening backlog batch
- `g06.017` is complete as the Rust quality re-audit and fresh-start assessment
- `g06.018` is superseded by `g06.019`
- `g06.019` is complete as the reference-grade architecture reset inventory
- `g06.020` is complete as the public Rust surface diet and consumer import
  matrix
- `g06.021` is complete as the media Postgres adapter extraction proof
- `g06.022` is complete as the Postgres runtime adapter isolation batch
- `g06.023` is complete as the jobs Postgres adapter extraction plan
- `g06.024` is complete as the jobs Postgres adapter extraction execution batch
- `g06.025` is complete as the six-consumer rollout and compatibility
  retirement proof
- `g06.026` is complete as the reference-grade docs and upgrade-note closeout
- `g06.027` is complete as the post-reset Rust quality re-audit
- `g06.028` is complete as the typed operator table config batch
- `g06.029` is complete as the consumer typed operator table adoption and
  raw-wrapper deprecation decision batch
- `g06.030` is complete as the raw operator wrapper removal readiness and remaining
  dynamic-identifier audit batch
- `g06.031` is complete as the remaining typed DB helper migration plan
- `g06.032` is complete as the typed `ExistsCheck` execution and rollout batch
- `g06.033` is complete as the raw existence helper deprecation decision
- `g06.034` is complete as the test DB typed schema cleanup batch
- `g06.035` is complete as the remaining dynamic identifier closeout audit
- `g06.036` is complete as the Postgres media config typed identifier cleanup
- `g06.037` is complete as the typed DB identifier lane closeout audit
- `g06.038` is complete as the blob object key helper alignment plan
- `g06.039` is complete as the typed media storage key helper batch
- `g06.040` is complete as the blob adapter typed method decision
- `g06.041` is complete as the typed blob adapter extension method batch
- `g06.042` is complete as the stored object-key parse-boundary audit
- `g06.043` is complete as the typed media domain object-key field rollout
- `g06.044` is complete as the consumer app-local media object-key
  parse-boundary rollout
- `g06.045` is complete as the media object-key boundary closeout audit
- `g06.046` is complete as the non-media blob object-key boundary policy
- `g06.047` is complete as the consumer non-media blob object-key adoption
  proof
- `g06.048` is complete as the post-blob-key Rust quality checkpoint
- `g06.049` is complete as the devtools migration-bundle boundary split
- `g06.050` is complete as the migration-core public model modularity audit
- `g06.051` is complete as the migration-core pipeline internal split
- `g06.052` is complete as the Rust structural backlog checkpoint
- `g06.053` is complete as the media domain internal split
- `g06.054` is complete as the media renditions internal split
- `g06.055` is complete as the jobs public model modularity audit
- `g06.056` is complete as the jobs types internal split
- `g06.057` is complete as the DB pagination public model modularity audit
- `g06.058` is complete as the DB pagination internal split
- `g06.059` is complete as the HTTP query public model modularity audit
- `g06.060` is complete as the HTTP query internal split
- `g06.061` is complete as the HTTP cookies public model modularity audit
- `g06.062` is complete as the HTTP cookies internal split
- `g06.063` is complete as the HTTP error logging public model modularity audit
- `g06.064` is complete as the HTTP error logging internal split
- `g06.065` is complete as the migration-core orchestrator public model
  modularity audit
- `g06.066` is complete as the migration-core orchestrator internal split
- `g06.067` is complete as the devtools migration-bundle public model
  modularity audit
- `g06.068` is complete as the devtools migration-bundle internal split
- `g06.069` is complete as the migration-core verification-rules public model
  modularity audit
- `g06.070` is complete as the migration-core verification-rules internal split
- `g06.071` is complete as the jobs-postgres repository public model
  modularity audit
- `g06.072` is complete as the jobs-postgres repository internal split
- `g06.073` is complete as the auth JWT service tests modularity audit
- `g06.074` is complete as the auth JWT service tests internal split
- `g06.075` is complete as the media Nightfire tests modularity audit
- `g06.076` is complete as the media Nightfire tests internal split
- `g06.077` is complete as the auth email TOTP service tests modularity audit
- `g06.078` is complete as the auth email TOTP service tests internal split
- `g06.079` is complete as the devtools migration-bundle tests modularity audit
- `g06.080` is complete as the devtools migration-bundle tests internal split
- `g06.081` is complete as the auth OAuth tests modularity audit
- `g06.082` is complete as the auth OAuth tests internal split
- `g06.083` is complete as the auth password service tests modularity audit
- `g06.084` is complete as the auth password service tests internal split
- `g06.085` is complete as the migration-core decision-memory modularity audit
- `g06.086` is complete as the migration-core decision-memory internal split
- `g06.087` is complete as the AI runtime tests modularity audit
- `g06.088` is complete as the AI runtime tests internal split
- `g06.089` is complete as the auth WebAuthn service modularity audit
- `g06.090` is complete as the auth WebAuthn service internal split
- `g06.091` is complete as the config crate modularity audit
- `g06.092` is complete as the config crate internal split
- `g06.093` is complete as the auth TOTP crate modularity audit
- `g06.094` is complete as the auth TOTP crate internal split
- `g06.095` is complete as the devtools migration report modularity audit
- `g06.096` is next as the devtools migration report internal split
- `g05` is closed as the shared-page, workflow-template, media capability,
  dev-flow contract, and query-variant convergence generation
- `g05.024` is complete as the Marking Hub query variant proof and rollout
- `g05.023` is complete as the EntityList query variant integration
- `g05.022` is complete as the list query variant API contract
- `g05.021` is complete as the Poodle card toggle query variant control
- `g05.020` is complete as the compli-me and songsprout media family rollout
- `g05.019` is complete as the fleet media library capability mandate
- `g05.018` is complete as the Underlay app review checklist and audit artifact
- `g05.017` is complete as the error-code and operator-audit contract
- `g05.016` is complete as the OpenAPI quality and declaration contract
- `g05.015` is complete as the config and secrets contract
- `g05.014` is complete as the release and compatibility rollout contract
- `g05.013` is complete as the consumer template adoption contract
- `g05.012` is complete as the testing posture and shared harness contract
- `g05.011` is complete as the migration and schema workflow contract
- `g05.010` is complete as the new Underlay app bootstrap and bring-up contract
- `g05.009` is complete as the Rust runtime contract audit and next contract set
- `g05.006` is complete as the admin dashboard page template proof
- `g05.005` is complete as the system index page template proof
- `g05.004` is complete as the cross-app media-library template consolidation
  lane
- `g05.007` is complete as the media upload page proof, absorbed into
  `g05.004`
- `g05.008` is complete as the media detail workflow page proof, absorbed into
  `g05.004`
- `g05.003` is complete as the underlay-reference template completion and
  contract-hardening sweep
- `g05.002` is complete as the compli-me and contact-patch detail-page
  convergence thread
- `g05.001` is complete as the Dairy detail-page and tab-list convergence
  thread
- `g03` is complete as the template-system thread
- `g04` is closed. Its contract-coverage and assessment history remains
  available under [g04/README.md](g04/README.md)
- `g02` is closed. Its batch-card history now lives under `g02/batch-cards/`

## Historical Generations

- [g06/README.md](g06/README.md) — active Rust platform-contract transition
  generation, now continuing into the reference-grade reset
- [g05/README.md](g05/README.md) — shared-page, workflow-template, media
  capability, dev-flow contract, and query-variant convergence generation
  (complete)
- [g02/README.md](g02/README.md) — Poodle-era consumer normalization and
  overhaul (complete)
- [g01/README.md](g01/README.md) — Extraction, contraction, and retained-surface
  definition (complete) It carries forward the `g01.098` recovery findings, freezes
  the live shared-surface posture across Underlay and the current consumer
  family, and sequences the first honest bounded execution waves from that
  evidence.
- `g01.098` is complete as the generation-closing recovery lane. It stopped the
  old “no active roadmap” drift, reopened the real queue, and proved the next
  work belonged in a fresh generation rather than as one more oversized `g01`
  tail milestone.
- `g01.094` through `g01.097` remain complete as the Poodle-first UI-guide,
  long-tail prune, archival-doc evidence, and pattern-catalogue translation
  line. They are useful lineage inside the now-complete `g01` generation, but
  they are no longer the live queue.
- the active problem now is not generic visible UI migration itself. It is
  shared-surface normalization across the current consuming apps
  (`acowtancy`, `compli-me`, `contact-patch`, `underlay-reference`,
  `loophole/composer`, and `songsprout`) while Underlay, Poodle, and the app
  layer settle on honest ownership boundaries again.

- `g01.001` through `g01.041` record the current Underlay roadmap corpus
- `g01.042` is complete for Poodle adoption and Underlay UI contraction
- `g01.043` is complete for Poodle public prop normalization and downstream API migration
- `g01.044` is complete for the shared UI documentation and demo refresh
- `g01.045` is complete for the remaining Underlay surface contraction tail
- `g01.046` is complete for reassessing the supposedly retained generic Underlay surface, retiring the card and action residue that really belonged in Poodle, and freezing the remaining true workflow/runtime holds explicitly
- `g01.047` is complete for moving the richer generic table runtime from Underlay `DataTable` into Poodle and retiring the public Underlay table surface
- `g01.048` is complete for moving the richer generic multi-field sort-builder runtime from Underlay `OrderBy` into Poodle and retiring the public Underlay sort-builder surface
- `g01.049` is complete for moving the reusable reorder-session workflow shell from Underlay `ReorderableList` into Poodle and retiring the public Underlay reorder pattern
- `g01.050` is complete for splitting the final runtime-facing generic holds, moving the inline loading contract into Poodle `PageLoading`, and retiring public Underlay `PageLoading`
- `g01.051` is complete for moving the final retained runtime host from Underlay `ToastHost` into a Poodle-owned toast runtime shell over `ToastStack` and retiring the public Underlay host
- `g01.052` is complete for challenging the remaining obvious-equivalent public Underlay residue, opening the focused successor waves, and carrying that direct-successor queue through to completion
- `g01.053` is complete for expanding Poodle `LogList` from a console-style log viewer into the broader audit/activity list contract, migrating the active caller family, and retiring public Underlay `LogList`
- `g01.054` is complete for widening Poodle `BulkActionBar`, migrating the grouped caller families, and retiring public Underlay `BatchActionBar`
- `g01.055` is complete for moving the remaining shared `PageHeader` shell onto Poodle `PageHeader` or direct Poodle-based composition, then retiring the public Underlay header wrapper
- `g01.056` is complete for moving the reusable callback-driven media workflow out of Underlay and retiring public Underlay `MediaPicker`
- `g01.057` is complete for replacing Underlay `DropdownMenu` with direct Poodle `Menu` composition plus the smallest honest Poodle menu-ergonomics expansion, then retiring public Underlay `DropdownMenu`
- `g01.058` is complete for confirming `AutonomousList` has no live caller family and retiring the dead public shell, helper, and type exports
- `g01.059` is complete for removing the dead public `PageHeaderMeta` family after the broader `PageHeader` successor wave
- `g01.060` is complete for retiring public Underlay `MediaActionsMenu` after moving the app-local wrappers onto direct Poodle `Menu` / `AlertDialog` plus local workflow wiring
- `g01.061` is complete for retiring public Underlay `ErrorBoundary` after proving the remaining caller family had collapsed to two tiny app-root wrappers
- `g01.062` is complete for retiring public Underlay `CopyActionsMenu` after proving the remaining caller family was just a small admin-local convenience helper
- `g01.063` is complete for retiring public Underlay `DetailPageShell` after migrating the final Dairy route and internal caller tail onto direct Poodle `PageHeader` / `Tabs` composition
- `g01.064` is complete for reassessing the remaining public auth component family, retiring `AuthLayout`, confirming `LoginPage` / `ForgotPasswordFlow` as retained shared workflow surfaces for now, and splitting the helper layer cleanly
- `g01.065` is complete for moving the reusable one-time-code input contract from Underlay `TotpInput` into Poodle, migrating the retained shared auth and grouped account-security proof family, and retiring public Underlay `TotpInput`
- `g01.066` is complete for confirming `PasswordRequirements` still earns retained Underlay ownership for now because it bundles auth-policy fetch, fallback defaults, and shared password-rule rendering across retained auth internals and grouped account-password pages
- `g01.067` is complete for confirming `SpaFormShell` as an explicit retained Underlay structural shell after the strict create/edit caller review and resetting the queue around the next honest shell challenge
- `g01.068` is complete for confirming that `AiRoutingAdmin` no longer earns a public Underlay export, moving the guide/example surface onto direct Poodle composition over `createAiRoutingOpsController`, and retiring the public shell
- `g01.069` is complete for retiring the public `RelationSelector` UI wrapper family, keeping only the retained lower-level helper layer in Underlay, and moving the guide surface to app-local selector composition
- `g01.070` is complete for confirming that `DetailMeta*` still earns retained Underlay ownership as a broad compact metadata-row helper family and aligning the guide/inventory surface to that stop point
- `g01.071` is complete for confirming that the remaining retained auth workflow surface still earns shared public Underlay ownership and closing the auth family out as an explicit retained stop point
- `g01.072` is complete for refining the auth boundary by moving `LoginPage` / `ForgotPasswordFlow` to public `patterns` and splitting `PasswordRequirements` into a Poodle UI surface plus an Underlay auth-policy adapter
- `g01.075` is complete for auditing the remaining TypeScript surface after the Svelte contraction line, classifying what is truly retained, what may become future Poodle helper work, and what looks more like standalone-package material than Underlay UI debt
- `g01.076` is complete for retiring the dead `embed` package surface, introducing a dedicated `runtime` namespace, and narrowing `patterns` to retained workflow/page shells
- `g01.077` is complete for organizing the retained `runtime` helper surface into deliberate domain sub-barrels so the namespace is explicit rather than flat compatibility residue
- `g01.078` is complete for confirming that the retained toast/context helper family is still runtime orchestration, not Poodle UI, and belongs explicitly on `@decodelabs/underlay/runtime/feedback`
- `g01.079` is complete for removing duplicated client auth-store exports from `runtime/auth`, confirming the retained auth-runtime boundary, and aligning the active guides to the narrower `@decodelabs/underlay/runtime/auth` surface
- `g01.080` is complete for confirming the retained browser-runtime boundary, keeping storage/timezone/keyboard helpers on `@decodelabs/underlay/runtime/browser`, and moving clipboard-plus-toast workflow to `@decodelabs/underlay/runtime/feedback`
- `g01.081` is complete for confirming the retained `data`, `media`, and `relations` runtime families, narrowing `runtime/relations` to the actual helper/context/type layer, and aligning the active guides to the stable subpaths
- `g01.082` is complete for confirming the root `@decodelabs/underlay/runtime` barrel as an explicit retained convenience surface rather than trimming it into churn now that the stable subpaths are documented
- `g01.083` is complete for confirming the `client` / `runtime` seam as an explicit retained split: SvelteKit navigation and transport stay on `client`, while framework-agnostic navigation context/state stays on `runtime`
- `g01.084` is complete for confirming the retained `nightfire` package surface, keeping the public package boundary stable, and cleaning up the internal editor-only widget placement for future extraction readiness
- `g01.085` is complete for confirming the retained `utils` package surface, exposing focused `utils/*` subpaths, and removing raw Base64URL helpers from the root public barrel
- `g01.086` is complete for aligning the front-door and architecture docs to the real retained TypeScript/Svelte package surfaces after the contraction and runtime audit line
- `g01.087` is complete for splitting the pure formatting and slug helpers out of `runtime/i18n` into explicit `utils/i18n` and `utils/slug` homes while keeping `runtime/i18n` stable as a compatibility barrel
- `g01.088` is complete for closing the sibling-repo package-boundary recovery line, restoring Dairy validation after the manual import repair, and confirming that only historical references remain to the retired `components`, `embed`, and deep `patterns` entrypoints
- `g01.089` is complete for moving the compact metadata-ribbon contract into Poodle `MetaBar` / `MetaItem`, migrating the live admin caller family, and retiring the public Underlay `DetailMeta*` wrappers

## Next Task

Execute `g06.033`: raw existence helper deprecation decision.
- `g01.090` is complete for confirming that the remaining public `@decodelabs/underlay/patterns` surface is the true retained workflow stop point rather than unresolved migration debt
- `g01.091` is complete for confirming the retained `client` package boundary, exposing explicit `client/*` public subpaths for the live feature families, and keeping the root barrel stable as a convenience surface
- `g01.092` is complete for confirming the retained `nightfire` package boundary, exposing explicit `nightfire/*` public subpaths for the live extension families, and keeping the root barrel stable as a convenience surface
- `g01.093` is complete for confirming the root `@decodelabs/underlay` barrel as a compatibility-only surface and stopping active docs from teaching the old flat import path
- use `backlog/` for deferred work that is not yet active execution scope

## Historical language boundary

- New or actively maintained roadmap docs must use roadmap IDs and batch language.
- Historical logs or imported roadmap content may retain phase-era wording when they are recording past work.
- Normalize local historical wording only when the affected doc is reopened for active work or when an old label causes live path/reference drift.

## g01 Status Overview

| ID | Title | Status | Summary |
|---|---|---|---|
| 001 | [Extraction Roadmap](./g01/001-extraction-roadmap.md) | Complete | Extract shared Rust crates from Farmyard and Nursery |
| 002 | [Frontend Extraction](./g01/002-frontend-extraction-roadmap.md) | Complete | Extract shared TS and Svelte components |
| 003 | [Frontend Guardrails](./g01/003-frontend-guardrails-and-quirk-management.md) | Complete | Svelte and TypeScript guardrails and quirk management |
| 004 | [Auth System](./g01/004-underlay-auth-system-roadmap.md) | Complete | Authentication provider system |
| 005 | [Auth DB Migrations](./g01/005-auth-database-migrations.md) | Complete | Auth-related database schema migrations |
| 006 | [Rust Test Coverage](./g01/006-rust-test-coverage-improvement.md) | Complete | Improve test coverage across Rust crates |
| 007 | [Quickstart Guide](./g01/007-quickstart-guide-improvements.md) | Complete | Improve new-project quickstart docs |
| 008 | [Extract Patterns](./g01/008-extract-patterns.md) | Complete | Extract reusable patterns from Acowtancy |
| 009 | [Quick Wins: Testing and DX](./g01/009-quick-wins-testing-and-dx.md) | Complete | Testing utilities and developer experience |
| 010 | [Medium Enhancements](./g01/010-medium-value-enhancements.md) | Complete | Data tables, validation, and UX improvements |
| 011 | [Optimistic Updates](./g01/011-optimistic-updates.md) | Complete | Optimistic form state updates |
| 012 | [Nightfire Extraction](./g01/012-nightfire-extraction.md) | Complete | Extract Nightfire content protocol |
| 013 | [RelationSelector](./g01/013-relation-selector.md) | Complete | RelationSelector component |
| 014 | [Generic Validation](./g01/014-generic-field-validation.md) | Complete | Generic field validation system |
| 015 | [Error Reporting](./g01/015-unified-error-reporting-roadmap.md) | Complete | Unified error reporting across layers |
| 016 | [JSON Naming](./g01/016-json-naming-standardization-roadmap.md) | Complete | `snake_case` JSON standardization |
| 017 | [Module Splitting](./g01/017-rust-module-splitting-roadmap.md) | Complete | Split oversized Rust modules |
| 018 | [Documentation and AI](./g01/018-documentation-ai-agent-improvements.md) | Complete | Documentation and AI agent context improvements |
| 019 | [Codebase Improvements](./g01/019-codebase-improvements.md) | Complete | Simplification, deduplication, and reorganization |
| 020 | [Configuration Standardization](./g01/020-configuration-standardization-and-env-reduction.md) | Complete | Move app behavior config from env files into typed structures |
| 021 | [Shared Admin Components](./g01/021-shared-admin-components.md) | Complete | Shared admin components and patterns |
| 022 | [CLI Runner and Pulse](./g01/022-underlay-cli-runner-and-pulse.md) | Extracted | Runner and Pulse were extracted into Effigy |
| 023 | [Quality Hardening](./g01/023-underlay-quality-hardening-roadmap.md) | Complete | Type safety tightening, TS coverage gates, Rust panic-path hardening, and test deduplication ([log](../logs/2026-02/28-000000-underlay-effigy-cross-repo-validation.md)) |
| 024 | [Nested Task Catalogs and Config Consolidation](./g01/024-nested-task-catalogs-and-config-consolidation.md) | Extracted | Nested task-catalog runner behavior was extracted to Effigy while Underlay retains historical notes ([log](../logs/2026-02/28-000000-underlay-effigy-cross-repo-validation.md)) |
| 025 | [Universal Legacy Migration Foundation](./g01/025-universal-legacy-migration-foundation.md) | Complete | Core reusable migration framework boundaries, deterministic stage model, and plugin contracts |
| 026 | [Migration Bundles and OCI Distribution](./g01/026-migration-bundles-and-oci-distribution.md) | Complete | OCI bundle spec, digest-pinned replay contract, bundle lifecycle tooling, and promotion record contracts |
| 027 | [Incremental Decision Memory and AI Reuse](./g01/027-incremental-decision-memory-and-ai-reuse.md) | Complete | Decision fingerprinting, sidecar merge and reuse rules, invalidation policy, and AI-call suppression metrics |
| 028 | [Migration Operations and Hardening](./g01/028-migration-operations-and-hardening.md) | Complete | Resume and recovery, drift detection, integrity and audit enforcement, governance reporting, signature rollout gates, and promotion readiness |
| 029 | [Northstar Doctrine Alignment](./g01/029-underlay-northstar-doctrine-alignment.md) | Complete | Move Underlay docs onto the Northstar contract inside `docs/` |
| 030 | [Research Execution Intake and Wave Planning](./g01/030-research-execution-intake-and-wave-planning.md) | Complete | Assess the March 2026 research corpus, schedule only canonical IDR-backed work, and open the next execution wave |
| 031 | [Consumer Upgrade and Change Communication](./g01/031-consumer-upgrade-and-change-communication.md) | Complete | Make downstream app upgrade guidance a required deliverable for any Underlay behavior, API, config, or migration change |
| 032 | [Passkey Client Abstractions](./g01/032-passkey-client-abstractions.md) | Complete | Shared passkey hooks, error mapping, and management UI for WebAuthn consumer flows |
| 033 | [AI Runtime Resilience Middleware](./g01/033-ai-runtime-resilience-middleware.md) | Complete | Opt-in retry, circuit-breaker, and route-chain primitives for `underlay-ai-runtime` |
| 034 | [Cross-Language Validation with Zod](./g01/034-cross-language-validation-zod.md) | Complete | Optional Zod schema export and Svelte form helpers for client-side validation |
| 035 | [Background Job Reliability and Observability](./g01/035-background-job-reliability-and-observability.md) | Complete | Retry jitter, dead letters, and lifecycle events for `underlay-jobs` |
| 036 | [Declarative Migration Verification Rules](./g01/036-declarative-migration-verification-rules.md) | Complete | Add reusable declarative verification to `underlay-migration-core` without removing custom verification |
| 037 | [Nightfire Slash Command Palette](./g01/037-nightfire-slash-command-palette.md) | Complete | Add keyboard-driven slash-command insertion to `NightfireEditor` |
| 038 | [OpenTelemetry Span Integration](./g01/038-opentelemetry-span-integration.md) | Complete | Add feature-gated trace-context propagation and request-span correlation across `underlay-http` and `underlay-observability` |
| 039 | [Storage Expiration Support](./g01/039-storage-expiration-support.md) | Complete | Add optional TTL and expiration support to the SSR-safe storage wrappers without breaking existing persisted values |
| 040 | [Smart Skeletons](./g01/040-smart-skeletons.md) | Complete | Add a higher-level `DataSkeleton` surface and reusable preset registry over the existing `Skeleton` primitives |
| 041 | [Form Draft Persistence](./g01/041-form-draft-persistence.md) | Complete | Add opt-in draft persistence and restoration to `createFormState` using the shared storage wrappers |
| 042 | [Poodle Adoption and Underlay UI Contraction](./g01/042-poodle-adoption-and-underlay-ui-contraction.md) | Complete | Move primitives and generic composites to Poodle, keep Underlay focused on structural shells and specialized systems, and run the coexistence migration across consuming apps |
| 043 | [Poodle Public Prop Normalization](./g01/043-poodle-public-prop-normalization.md) | Complete | Normalize Poodle onto one plain-state boolean prop language, migrate Underlay and all consumer app call sites, and track the sweep through a durable manifest |
| 044 | [Shared UI Documentation and Demo Refresh](./g01/044-shared-ui-documentation-and-demo-refresh.md) | Complete | Rebuild the shared UI documentation and demo layer around the stabilized post-migration Underlay surface and its boundary with Poodle |
| 045 | [Remaining Underlay Surface Contraction](./g01/045-remaining-underlay-surface-contraction.md) | Complete | Remove the remaining low-value generic Underlay export tail so the public surface matches the true retained workflow-shell boundary |
| 046 | [Retained Underlay Surface Reassessment](./g01/046-retained-underlay-surface-reassessment.md) | Complete | Reassess the supposedly final retained generic surface, move the real design-system capability into Poodle, and leave only the explicit workflow/runtime holds in Underlay |
| 047 | [Poodle DataTable Capability Expansion](./g01/047-poodle-data-table-capability-expansion.md) | Complete | Expand Poodle `DataTable` in staged batches, migrate the active caller family, and retire the public Underlay `DataTable` surface |
| 048 | [Poodle OrderBy Capability Expansion](./g01/048-poodle-order-by-capability-expansion.md) | Complete | Expand Poodle `OrderBy` from a single-sort toolbar into the multi-field sort-builder contract the active admin caller family needed, then retire public Underlay `OrderBy` |
| 049 | [Poodle ReorderableList Workflow Expansion](./g01/049-poodle-reorderable-list-workflow-expansion.md) | Complete | Expand Poodle reorder support from a low-level list primitive into the reusable reorder-session workflow shell the active admin caller family still needed, then retire public Underlay `ReorderableList` |
| 050 | [Runtime Host And Inline Loading Reassessment](./g01/050-runtime-host-and-inline-loading-reassessment.md) | Complete | Split `ToastHost` and `PageLoading` into explicit runtime capability decisions, move inline loading into Poodle `PageLoading`, and retire public Underlay `PageLoading` |
| 051 | [Poodle Toast Host Runtime Expansion](./g01/051-poodle-toast-host-runtime-expansion.md) | Complete | Expand Poodle over `ToastStack` to absorb the final store-aware toast host/runtime shell, migrate the root-layout callers, and retire public Underlay `ToastHost` |
| 052 | [Obvious Equivalent Surface Reassessment](./g01/052-obvious-equivalent-surface-reassessment.md) | Complete | Reassess the remaining public Underlay surface that already has direct Poodle equivalents or thin composition boundaries, retire the clean residue, and route the harder surfaces into focused follow-on waves |
| 053 | [Poodle LogList Capability Expansion](./g01/053-poodle-log-list-capability-expansion.md) | Complete | Expand Poodle `LogList` into the generic audit/activity log-list contract, migrate the active proof family, and retire the public Underlay `LogList` surface |
| 054 | [BatchActionBar Successor Wave](./g01/054-batch-action-bar-successor-wave.md) | Complete | Widen Poodle `BulkActionBar`, migrate the grouped live caller family, and retire public Underlay `BatchActionBar` |
| 055 | [PageHeader Successor Wave](./g01/055-page-header-successor-wave.md) | Complete | Reassess the remaining shared header shell, expand Poodle `PageHeader` only where the live retained/app caller family proves it is reusable, then retire public Underlay `PageHeader` |
| 056 | [MediaPicker Successor Wave](./g01/056-media-picker-successor-wave.md) | Complete | Move the reusable callback-driven media workflow layer into a Poodle-owned helper/controller split over the existing media surfaces, then retire public Underlay `MediaPicker` |
| 057 | [DropdownMenu Successor Wave](./g01/057-dropdown-menu-successor-wave.md) | Complete | Reassess the thin Underlay menu wrapper against Poodle `Menu`, add only the smallest generic ergonomics gap if needed, then retire public Underlay `DropdownMenu` |
| 058 | [AutonomousList Successor Wave](./g01/058-autonomous-list-successor-wave.md) | Complete | Reassess the retained `AutonomousList` shell, confirm whether any live caller family remains, then retire the dead public shell if the active surface is already gone |
| 059 | [PageHeaderMeta Cleanup Wave](./g01/059-page-header-meta-cleanup-wave.md) | Complete | Sweep the dead public `PageHeaderMeta` helper family after the `PageHeader` migration and retire the export residue if live callers are gone |
| 060 | [MediaActionsMenu Reassessment Wave](./g01/060-media-actions-menu-reassessment-wave.md) | Complete | Reassess the shared media action workflow helper, move the app-local wrappers onto direct Poodle composition, and retire the public Underlay export |
| 061 | [ErrorBoundary Reassessment Wave](./g01/061-error-boundary-reassessment-wave.md) | Complete | Reassess the remaining public error-boundary shell, confirm the shared contract is gone, and retire the public export in favor of local app-root composition |
| 062 | [CopyActionsMenu Reassessment Wave](./g01/062-copy-actions-menu-reassessment-wave.md) | Complete | Reassess the remaining public clipboard action helper, confirm the shared contract is gone, and retire the public export in favor of app-local helpers |
| 063 | [DetailPageShell Reassessment Wave](./g01/063-detail-page-shell-reassessment-wave.md) | Complete | Reassess the retained structural detail-page shell, migrate the live caller family onto direct Poodle composition, and retire the public export |
| 064 | [Auth Surface Reassessment Wave](./g01/064-auth-surface-reassessment-wave.md) | Complete | Reassess the remaining public auth component family and decide which parts belong in Poodle, local app composition, or explicit retained Underlay ownership |
| 065 | [Poodle TotpInput Capability Wave](./g01/065-poodle-totp-input-capability-wave.md) | Complete | Move the reusable one-time-code input behavior from Underlay `TotpInput` into Poodle, migrate the grouped auth/account proof family, and retire public Underlay `TotpInput` |
| 066 | [PasswordRequirements Reassessment Wave](./g01/066-password-requirements-reassessment-wave.md) | Complete | Reassess whether `PasswordRequirements` still earns public Underlay ownership; confirm that it remains an explicit retained Underlay auth helper for now |
| 067 | [SpaFormShell Reassessment Wave](./g01/067-spa-form-shell-reassessment-wave.md) | Complete | Confirm `SpaFormShell` as an explicit retained Underlay structural shell after the strict create/edit caller review and reset the queue around the next honest shell challenge |
| 068 | [AiRoutingAdmin Reassessment Wave](./g01/068-ai-routing-admin-reassessment-wave.md) | Complete | Confirm that `AiRoutingAdmin` no longer earns a public Underlay export, move the guide/example surface onto direct Poodle composition over `createAiRoutingOpsController`, and retire the public shell |
| 069 | [RelationSelector Reassessment Wave](./g01/069-relation-selector-reassessment-wave.md) | Complete | Split `RelationSelector` into a retained helper layer plus app-local UI composition, then retire the public Underlay UI wrapper family |
| 070 | [DetailMeta Reassessment Wave](./g01/070-detail-meta-reassessment-wave.md) | Complete | Reassess the remaining public `DetailMeta` helper family and record the then-current retained stop point before the later successor wave |
| 071 | [Auth Workflow Reassessment Wave](./g01/071-auth-workflow-reassessment-wave.md) | Complete | Recheck the remaining retained auth workflow surface, confirm the remaining auth workflows/helpers still earn shared public Underlay ownership, and close the family out as an explicit retained stop point |
| 072 | [Auth Boundary Refinement Wave](./g01/072-auth-boundary-refinement-wave.md) | Complete | Move the retained auth workflow pages to public `patterns` and split `PasswordRequirements` into a Poodle UI surface plus an Underlay auth-policy adapter |
| 073 | [Retained Public Surface Classification](./g01/073-retained-public-surface-classification.md) | Complete | Classify the remaining public Underlay surface across `components`, `patterns`, and `nightfire` so the post-contraction boundary is explicit and durable |
| 074 | [Non-Public Svelte Surface Recovery](./g01/074-non-public-svelte-surface-recovery.md) | Complete | Recover from the too-optimistic post-contraction stop point by deleting dead non-public Svelte wrappers, removing the old `components` namespace, and reducing the remaining internal `ts/src` surface to the truly necessary retained implementation files |
| 075 | [TS Surface Boundary Audit](./g01/075-ts-surface-boundary-audit.md) | Complete | Audit the remaining TypeScript surface after the Svelte contraction line and classify which retained helpers belong in Underlay, which may become future Poodle helper work, and which look more like standalone package candidates |
| 076 | [Runtime Namespace and Embed Retirement](./g01/076-runtime-namespace-and-embed-retirement.md) | Complete | Retire the dead `embed` surface, introduce `@decodelabs/underlay/runtime`, and narrow `patterns` to retained workflow/page shells |
| 077 | [Runtime Surface Organization](./g01/077-runtime-surface-organization.md) | Complete | Organize the retained runtime helper surface into explicit domain sub-barrels so the namespace no longer behaves like a flat compatibility dump |
| 078 | [Feedback Runtime Reassessment](./g01/078-feedback-runtime-reassessment.md) | Complete | Confirm the retained toast/context helper family as runtime orchestration under `@decodelabs/underlay/runtime/feedback` rather than design-system UI |
| 079 | [Auth Browser Runtime Seam](./g01/079-auth-browser-runtime-seam.md) | Complete | Remove duplicated client auth-store exports from `runtime/auth` and confirm the narrower retained auth-runtime boundary |
| 080 | [Browser Runtime Seam](./g01/080-browser-runtime-seam.md) | Complete | Confirm the retained browser-runtime boundary, keeping storage/timezone/keyboard helpers on `runtime/browser` and clipboard-plus-toast workflow on `runtime/feedback` |
| 081 | [Data Media Relations Runtime Seam](./g01/081-data-media-relations-runtime-seam.md) | Complete | Confirm the retained `data`, `media`, and `relations` runtime families and narrow `runtime/relations` to the actual helper/context/type layer |
| 082 | [Runtime Root Barrel Reassessment](./g01/082-runtime-root-barrel-reassessment.md) | Complete | Confirm the root `@decodelabs/underlay/runtime` barrel as an explicit retained convenience surface while teaching the narrower subpaths for new focused contracts |
| 083 | [Client Runtime Navigation Seam](./g01/083-client-runtime-navigation-seam.md) | Complete | Confirm the retained split between `@decodelabs/underlay/client` and `@decodelabs/underlay/runtime` so further namespace churn is avoided |
| 084 | [Nightfire Surface Audit](./g01/084-nightfire-surface-audit.md) | Complete | Confirm the retained `nightfire` package surface and remove duplicated tiny editor wrappers where direct Poodle or local Nightfire markup already covers the needed behavior |
| 085 | [Utils Surface Audit](./g01/085-utils-surface-audit.md) | Complete | Confirm the retained `utils` package surface, tighten its public boundary, and expose focused helper subpaths for WebAuthn, HTML sanitization, and sequence utilities |
| 086 | [Retained Package Surface Docs Alignment](./g01/086-retained-package-surface-docs-alignment.md) | Complete | Align the architecture and front-door docs to the real retained `patterns`, `runtime`, `utils`, `client`, and `nightfire` package surfaces |
| 087 | [Runtime I18n Helper Split](./g01/087-runtime-i18n-helper-split.md) | Complete | Move the pure formatting and slug helpers out of `runtime/i18n` into explicit `utils` homes while keeping the runtime surface stable for compatibility |
| 088 | [Sibling Repo Package Boundary Recovery](./g01/088-sibling-repo-package-boundary-recovery.md) | Complete | Close the loop on the package-boundary recovery by manually repairing Dairy import fallout, revalidating the active sibling repos, and proving that only historical references remain to retired Underlay entrypoints |
| 089 | [MetaBar Successor Wave](./g01/089-meta-bar-successor-wave.md) | Complete | Move the compact metadata-ribbon contract into Poodle `MetaBar` / `MetaItem`, migrate the live admin caller family, and retire the public Underlay `DetailMeta*` wrappers |
| 090 | [Retained Patterns Stop Point](./g01/090-retained-patterns-stop-point.md) | Complete | Confirm the remaining public `@decodelabs/underlay/patterns` surface as the true retained workflow stop point and close the last implied contraction ambiguity |
| 091 | [Client Surface Organization](./g01/091-client-surface-organization.md) | Complete | Confirm the retained `client` package boundary, expose explicit `client/*` public subpaths for the live feature families, and keep the root barrel stable as a convenience surface |
| 092 | [Nightfire Extraction Readiness](./g01/092-nightfire-extraction-readiness.md) | Complete | Confirm the retained `nightfire` package boundary, expose explicit `nightfire/*` public subpaths for the live extension families, and make the future extraction seam explicit without forcing churn now |
| 093 | [Root Package Barrel Reassessment](./g01/093-root-package-barrel-reassessment.md) | Complete | Confirm the root `@decodelabs/underlay` barrel as compatibility-only and stop active docs from teaching the old flat import path |
| 098 | [Poodle-Era Consumer Normalization And Overhaul Recovery](./g01/098-poodle-era-consumer-normalization-and-overhaul-recovery.md) | Complete | Recover the real active shared-surface queue across Underlay and the current consumer family, then compile the next bounded normalization waves from current evidence |
| 096 | [Archival Doc Evidence Boundary Audit](./g01/096-archival-doc-evidence-boundary-audit.md) | Complete | Confirm that the active docs surface is normalized while the remaining raw local-path residue is acceptable frozen evidence in archival logs, roadmaps, research notes, and sweeps |

## Generation Snapshot

| ID | Roadmap | Status | Summary |
| --- | --- | --- | --- |
| 001 | [Poodle-Era Consumer Normalization And Overhaul Runway](./g02/001-poodle-era-consumer-normalization-and-overhaul-runway.md) | Active | Carry the recovered overhaul queue into a fresh generation and compile the first bounded execution waves across Underlay, Poodle, and the current consumer family |

**g01 Complete:** 91 | **Extracted:** 2 | **In progress:** 0 | **Not started:** 0
**g02 Complete:** 7 | **In progress:** 0 | **Not started:** 0
**g03 Complete:** 51 | **In progress:** 0 | **Not started:** 0
**g04 Active:** 8 | **In progress:** 1 | **Not started:** 0

## Current Boundary

There are no remaining meaningful generic Underlay holds or hidden package
boundary surprises in the active app/docs surface. The non-public `ts/src`
recovery sweep is complete too, so the supported Svelte boundary is now aligned
with the real retained surface.

The remaining public API is intentionally retained and more explicit:

- `@decodelabs/underlay/patterns`
  - `LoginPage`
  - `ForgotPasswordFlow`
  - `PasswordRequirements`
  - `SpaFormShell`
  - retained workflow/page-shell exports only
- `@decodelabs/underlay/runtime`
  - retained helper/controller exports
  - auth shared types
  - formatter helpers
- `@decodelabs/underlay/nightfire`
  - retained editor/runtime package surface

Dead public residue like `SlugField`, `EntityActionsMenu`, the old restore
views, and the unused `embed` package surface is no longer exported.

## Active Lane

`g06.067` is now the live planning/control lane.

Underlay’s Rust platform-contract transition is materially complete enough to
continue into the reference-grade reset inside `g06`, with controlled breaking
changes and six-consumer rollout proof.

## Complete

`g01.091` is complete. The Svelte contraction line, non-public residue
recovery, retained TS package-surface audit, front-door package-language
alignment, the final obvious pure-helper split out of `runtime`, the active
sibling-repo package-boundary recovery, the `DetailMeta*` successor cleanup,
the explicit retained `patterns` stop point, the retained `client` surface
organization, the Nightfire extraction-readiness seam, and the root-barrel
compatibility boundary are now durable. `g01.094` and `g01.095` completed the
Poodle-first UI-guide translation and long-tail Underlay prune line. `g01.096`
completed the archival-doc evidence audit, making the active-doc normalization
rule explicit while leaving historical records as frozen evidence by policy.
`g01.098` then closed the generation by recovering the real live overhaul
queue and proving the next honest work belonged in `g02`.

## Next Task

Execute the live queue:

- `g06.067`: devtools migration-bundle public model modularity audit
