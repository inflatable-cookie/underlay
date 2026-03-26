# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Use generation folders such as `docs/roadmaps/g01/`.
- Roadmap filenames use `NNN-<slug>.md`.
- Roadmap references use generation-qualified IDs such as `g01.021`.
- Keep roadmap status in the file itself and keep this index aligned when statuses change.
- Generation rollover is manual only.
- Keep unscheduled work in [backlog/](backlog/).
- Keep durable inventories, CSVs, and machine-readable reference artifacts in [../contracts/](../contracts/).
- If active work changes consumer-visible behavior, APIs, configuration, migrations, or integration patterns, include a `Consumer Upgrade Impact` section in the roadmap and point to the compatibility note plan.

## Active generation

- [g01/README.md](g01/README.md)
- [generation-index.md](generation-index.md)

## Current Queue

- `g01.001` through `g01.041` record the current Underlay roadmap corpus
- `g01.042` is complete for Poodle adoption and Underlay UI contraction
- `g01.043` is complete for Poodle public prop normalization and downstream API migration
- `g01.044` is now active for the shared UI documentation and demo refresh
- the next net-new active roadmap should open as `g01.045`
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
| 044 | [Shared UI Documentation and Demo Refresh](./g01/044-shared-ui-documentation-and-demo-refresh.md) | In Progress | Rebuild the shared UI documentation and demo layer around the stabilized post-migration Underlay surface and its boundary with Poodle |

**Complete:** 41 | **Extracted:** 2 | **In progress:** 1 | **Not started:** 0

## Next Task

Execute `g01.044` Batch 44.3 by broadening Storybook coverage across the
highest-value retained Underlay workflow shells and helpers now that the
bootstrap and first catalog shell are in place.
