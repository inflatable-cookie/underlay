# Research Master Index

Status: Active
Owner:
Last updated: 2026-03-11
Purpose: Navigate from architecture or implementation questions to the most relevant research artifacts.

## Quick Reference: Architecture Area -> Research

| Architecture area | Primary memos | Value tracks | Dossiers or source hubs | Prototype refs |
| --- | --- | --- | --- | --- |
| `underlay-ai-runtime` | `idr-ai-runtime-resilience.md` | `value-tracks/ai-runtime-patterns.md` | `source-hubs/ai-llm-providers.md`, `specimen-dossiers/portkey.md`, `specimen-dossiers/litellm.md` | `P-AI-001` |
| `underlay-auth-webauthn` | `idr-passkey-client-hooks.md` | `value-tracks/passkey-ux-patterns.md` | `source-hubs/modern-authentication.md`, `specimen-dossiers/hanko.md`, `specimen-dossiers/acme-reference-implementation.md` | `P-AUTH-001` |
| `underlay-validation` | `idr-validation-zod-integration.md` | `value-tracks/cross-language-validation.md` | `source-hubs/cross-language-validation.md`, `source-hubs/cross-language-codegen.md`, `specimen-dossiers/zod.md`, `specimen-dossiers/valibot.md`, `specimen-dossiers/ts-rs.md` | `P-VAL-001` |
| `underlay-jobs` | `idr-background-job-improvements.md` | `value-tracks/background-job-patterns.md` | `source-hubs/background-job-patterns.md`, `specimen-dossiers/sidekiq.md`, `specimen-dossiers/bullmq.md`, `specimen-dossiers/temporal.md` | `P-JOBS-001` |
| `underlay-migration-core` | `idr-migration-framework-gaps.md` | `value-tracks/legacy-migration-patterns.md` | `source-hubs/legacy-migration-patterns.md`, `specimen-dossiers/airbyte.md`, `specimen-dossiers/dbt.md`, `specimen-dossiers/debezium.md` | `P-MIGRATION-001` |
| `underlay-nightfire` | `idr-content-editor-abstractions.md`, `idr-nightfire-slash-commands.md` | `value-tracks/structured-content-editors.md` | `specimen-dossiers/notion.md`, `specimen-dossiers/sanity.md`, `specimen-dossiers/editor-js.md`, `specimen-dossiers/lexical.md` | `P-NIGHTFIRE-001` |

## By Implementation Concern

| Concern | Start here | Key decisions or questions |
| --- | --- | --- |
| AI provider routing | `source-hubs/ai-llm-providers.md` | Gateway vs direct? Cost tracking approach? |
| Passkey UX gaps | `specimen-dossiers/hanko.md` | What are apps reinventing that Underlay should provide? |
| Validation codegen | `source-hubs/cross-language-validation.md` | Share validation between Rust and TypeScript? |
| Job reliability | `value-tracks/background-job-patterns.md` | Retry strategies, observability, dead letters? |
| Migration tooling | `value-tracks/legacy-migration-patterns.md` | CDC, schema drift, observability? |
| Content editing UX | `value-tracks/structured-content-editors.md` | Slash commands, collaboration, export formats? |

## Implementation Decision Records (IDRs)

| IDR | Feature | Status | Est. Effort | Priority |
| --- | --- | --- | --- | --- |
| `idr-passkey-client-hooks.md` | Passkey client hooks & components | `proposed` | 3-5 days | High |
| `idr-ai-runtime-resilience.md` | Circuit breaker, retry, route chain | `proposed` | 8-12 days | High |
| `idr-validation-zod-integration.md` | Zod validation schemas | `proposed` | 5-8 days | Medium |
| `idr-background-job-improvements.md` | Job reliability & observability | `proposed` | 6 days | Medium |
| `idr-migration-framework-gaps.md` | Legacy migration framework | `proposed` | 8-10 days | Medium |
| `idr-content-editor-abstractions.md` | Structured content editor abstractions | `proposed` | 6-8 days | Medium |
| `idr-nightfire-slash-commands.md` | Slash command palette for Nightfire | `proposed` | 4 days | Medium |

## By Prototype or Validation Work

| Prototype | Validates | Related research |
| --- | --- | --- |
| `P-AUTH-001` | Higher-level WebAuthn TS hooks | `specimen-dossiers/hanko.md`, `specimen-dossiers/acme-reference-implementation.md` |
| `P-AI-001` | AI runtime resilience middleware | `value-tracks/ai-runtime-patterns.md` |
| `P-JOBS-001` | Job reliability patterns | `value-tracks/background-job-patterns.md` |
| `P-NIGHTFIRE-001` | Slash command UX | `translation-memos/nightfire-enhancements.md` |

## By Specimen or External System

| Specimen | Studied for | Key documents |
| --- | --- | --- |
| LiteLLM | AI runtime routing | `specimen-dossiers/litellm.md` |
| Portkey | AI gateway patterns | `specimen-dossiers/portkey.md` |
| Hanko | Passkey-first auth | `specimen-dossiers/hanko.md` |
| Zod | TypeScript validation | `specimen-dossiers/zod.md` |
| Valibot | Tree-shakeable validation | `specimen-dossiers/valibot.md` |
| Sidekiq | Background job patterns | `specimen-dossiers/sidekiq.md` |
| BullMQ | Job orchestration | `specimen-dossiers/bullmq.md` |
| Temporal | Durable execution | `specimen-dossiers/temporal.md` |
| Airbyte | Data migration | `specimen-dossiers/airbyte.md` |
| dbt | Data transformations | `specimen-dossiers/dbt.md` |
| Notion | Editor UX patterns | `specimen-dossiers/notion.md` |
| Sanity | Studio abstractions | `specimen-dossiers/sanity.md` |
| Editor.js | Block-based editing | `specimen-dossiers/editor-js.md` |
| Acme Reference | Underlay usage patterns | `specimen-dossiers/acme-reference-implementation.md` |
| Acowtancy Production | Production usage validation | Validation notes in various IDRs |

## By Value Track

| Value Track | Topics | IDRs |
| --- | --- | --- |
| `value-tracks/ai-runtime-patterns.md` | AI provider routing, resilience | IDR-002 |
| `value-tracks/passkey-ux-patterns.md` | WebAuthn, passkeys | IDR-001 |
| `value-tracks/cross-language-validation.md` | Validation, codegen | IDR-003 |
| `value-tracks/background-job-patterns.md` | Job reliability, observability | IDR-004 |
| `value-tracks/legacy-migration-patterns.md` | Data migration, CDC | IDR-005 |
| `value-tracks/structured-content-editors.md` | Content editing, collaboration | IDR-006, IDR-007 |

## Research Status Summary

### Immediate Priority (Complete)
| Topic | Status | IDR |
|-------|--------|-----|
| Passkey Client Abstractions | ✅ Complete | IDR-001 |
| AI Runtime Resilience | ✅ Complete | IDR-002 |
| Validation Patterns | ✅ Complete | IDR-003 |

### Medium Priority (Complete)
| Topic | Status | IDR |
|-------|--------|-----|
| Background Job Improvements | ✅ Complete | IDR-004 |
| Legacy Migration Framework | ✅ Complete | IDR-005 |
| Structured Content Editor Patterns | ✅ Complete | IDR-006 |
| Nightfire Slash Commands | ✅ Complete | IDR-007 |

### Longer Term (Backlog)
| Topic | Status |
|-------|--------|
| Media Processing | 🔴 Not started |
| Database Migration Patterns | 🔴 Not started |
| Admin UI / Low-Code | 🔴 Not started |
| Error Handling Standards | 🔴 Not started |
| Testing Patterns | 🔴 Not started |
| Design System Governance | 🔴 Not started |

## Maintenance Rule

- Update this index whenever a new memo, major synthesis, or architecture cross-reference becomes part of active implementation work.
- Prefer links to durable artifacts over prose summaries.
