# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Use generation folders such as `docs/roadmaps/g01/`.
- Roadmap filenames use `NNN-<slug>.md`.
- Roadmap references use generation-qualified IDs such as `g01.021`.
- Keep roadmap status in the file itself and keep this index aligned when statuses change.
- Generation rollover is manual only.
- Keep unscheduled work in [backlog/](backlog/).
- Keep supporting artifacts such as inventories and CSVs in [supporting/](supporting/).

## Active generation

- [g01/README.md](g01/README.md)
- [generation-index.md](generation-index.md)

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

**Complete:** 27 | **Extracted:** 2 | **In progress:** 0 | **Planned:** 0
