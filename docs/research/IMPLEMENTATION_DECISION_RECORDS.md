# Implementation Decision Records Summary

This document catalogs all IDRs ready for execution by implementation threads.

## IDR-001: Passkey Client Hooks

**Status**: `proposed` (ready for execution)  
**Priority**: High  
**Estimated Effort**: 3-5 days  
**File**: `implementation-decision-records/idr-passkey-client-hooks.md`

### Problem
Every Underlay-based app with passkeys reinvents ~40 lines of WebAuthn boilerplate:
- Direct `navigator.credentials.create/get()` calls
- Error sanitization (removing w3.org URLs)
- Loading/error/success state management

### Research Basis
- **Acme Reference**: 422-line passkey page, 117-line login page with repeated patterns
- **Hanko**: Proves higher-level abstraction is viable via `<hanko-auth>`
- **Value Track**: Cross-specimen comparison validates approach

### Deliverables
1. `usePasskeyRegistration()` hook in `ts/src/patterns/passkey.ts`
2. `usePasskeyAuthentication()` hook with conditional UI support
3. `PasskeyManager.svelte` component for settings pages
4. Error mapping utilities in `ts/src/utils/webauthn.ts`

### Success Criteria
- [ ] Acme can reduce passkey page by ~40 lines
- [ ] Login page simplified
- [ ] Browser compatibility verified (Chrome, Safari, Firefox)
- [ ] Documentation includes migration guide

### Dependencies
None (additive feature)

---

## IDR-002: AI Runtime Resilience Middleware

**Status**: `proposed` (ready for execution)  
**Priority**: High  
**Estimated Effort**: 8-12 days  
**File**: `implementation-decision-records/idr-ai-runtime-resilience.md`

### Problem
`underlay-ai-runtime` provides only basic primitives. Production apps need:
- Circuit breakers for cascade failure prevention
- Retry with exponential backoff for transient errors
- Route chains with automatic fallback

Acowtancy has built 140+ lines of custom orchestration on top of Underlay.

### Research Basis
- **Acowtancy Production Code**: Custom circuit breaker (10 fails/15 min), retry, dead letter
- **Portkey**: Managed gateway with these features as core
- **LiteLLM**: OSS proxy with similar patterns
- **Value Track**: 5 repeated patterns across specimens

### Deliverables
1. `CircuitBreakerMiddleware<C>` - Per-provider failure tracking
2. `RetryMiddleware<C>` - Exponential backoff for retriable errors
3. `RouteChainExecutor` - Automatic fallback through provider chain
4. Integration tests with mock providers

### Success Criteria
- [ ] Acowtancy can remove custom circuit breaker implementation
- [ ] Retry handles rate limits gracefully
- [ ] Route chain falls back on provider failure
- [ ] Composable middleware pattern documented

### Dependencies
None (extends existing crate)

---

## IDR-003: Validation Zod Integration

**Status**: `proposed` (ready for execution)  
**Priority**: Medium  
**Estimated Effort**: 5-8 days  
**File**: `implementation-decision-records/idr-validation-zod-integration.md`

### Problem
Underlay defines validation rules in Rust, but these are not available in TypeScript. Consuming apps must either:
1. Accept delayed feedback (server-only validation)
2. Duplicate rules in Zod/Valibot, risk drift
3. Use limited HTML5 validation

Acme reference uses server-only with HTML5, providing poor UX for complex validations.

### Research Basis
- **Acme Analysis**: Rust validation with `#[derive(Validate)]`, TypeScript types only, HTML5 basics
- **Zod Ecosystem**: Industry standard, excellent DX, ~30kb bundle
- **Valibot**: Tree-shakable but pre-1.0
- **ts-rs**: Generates types only, not validation rules

### Deliverables
1. New export `@decodelabs/underlay/validation` with Zod schemas:
   - `emailSchema`, `passwordSchema`, `slugSchema`
   - Composed schemas: `registerRequestSchema`, etc.
2. `useValidatedForm()` hook for Svelte integration
3. Validation mapping documentation
4. Bundle size measurement

### Success Criteria
- [ ] Acme can use `registerRequestSchema` for client-side validation
- [ ] Bundle size impact documented
- [ ] Historical: form validation worked with the then-existing `FormValidationProvider`
- [ ] Validation mapping table in docs

### Dependencies
Zod as optional peer dependency

---

## IDR-004: Background Job Improvements

**Status**: `proposed` (ready for execution)  
**Priority**: Medium  
**Estimated Effort**: 6 days  
**File**: `implementation-decision-records/idr-background-job-improvements.md`

### Problem
`underlay-jobs` lacks production-grade reliability features found in mature job systems:
- No jitter in exponential backoff (thundering herd risk)
- Limited dead letter handling (failed jobs sit in DB)
- No built-in observability (apps must build their own)
- No job orchestration (complex workflows need manual state management)

Acowtancy implements custom circuit breaker and manual orchestration.

### Research Basis
- **Sidekiq**: Exponential backoff with jitter, dead letter morgue, excellent Web UI
- **BullMQ**: Job events, flows (DAGs), Bull Board observability
- **Temporal**: Durable execution, saga patterns (different use case)
- **Value Track**: 5 repeated patterns across systems

### Deliverables
1. Jitter in `BackoffStrategy::Exponential`
2. `job_dead_letters` table and `DeadLetterRepository` trait
3. Job lifecycle events (`JobEvent`, `JobEventHandler`)
4. Documentation and migration guide

### Success Criteria
- [ ] Thundering herd test passes (1000 simultaneous failing jobs)
- [ ] Dead letter retry works end-to-end
- [ ] Events fire correctly for all job states
- [ ] Backward compatibility maintained

### Dependencies
None (extends existing crate)

---

## IDR-005: Legacy Migration Framework Comparison

**Status**: `proposed` (ready for execution)  
**Priority**: Medium  
**Estimated Effort**: 8-10 days  
**File**: `implementation-decision-records/idr-migration-framework-gaps.md`

### Problem
`underlay-migration-core` provides solid foundations but lacks production-grade features for complex legacy migrations:
- No Change Data Capture (CDC) for zero-downtime sync
- Limited schema drift detection
- No observability hooks (only Prometheus)
- Missing workflow orchestration patterns
- No built-in conflict resolution strategies

### Research Basis
- **Airbyte**: CDC via Debezium, schema registry
- **dbt**: Idempotent transformations, snapshotting
- **Estuary**: Materializations, streaming sync
- **Value Track**: Migration system patterns across tools

### Deliverables
1. `ChangeDataCapture` trait + Debezium implementation
2. `SchemaDriftDetector` for automatic migration detection
3. Observability hooks for custom metrics/logging
4. Conflict resolution strategies (`replace`, `merge`, `reject`)
5. Comprehensive documentation

### Success Criteria
- [ ] CDC integration works with Postgres
- [ ] Schema drift detection catches table changes
- [ ] Observability hooks allow custom monitoring
- [ ] Migration tests pass with <5% performance overhead

### Dependencies
None (extends existing crate)

---

## IDR-006: Structured Content Editor Patterns

**Status**: `proposed` (ready for execution)  
**Priority**: Medium  
**Estimated Effort**: 6-8 days  
**File**: `implementation-decision-records/idr-content-editor-abstractions.md`

### Problem
Underlay's Nightfire editor provides solid block protocol foundations but lacks high-level UX patterns found in modern editors:
- No editor shell abstraction
- No collaborative editing support
- No export format registry
- Custom editor boilerplate in every app

### Research Basis
- **Notion**: Proves rich UX patterns are viable
- **Sanity**: Studio abstractions for content editing
- **Tiptap**: Modular architecture pattern
- **Value Track**: Structured content patterns across tools

### Deliverables
1. `EditorShell.svelte` - Composition wrapper with slots
2. `useCollaborativeEditing()` hook - Yjs integration
3. `SlashCommandPalette.svelte` - Quick block insertion
4. `ExportFormatRegistry` - Pluggable export system
5. Documentation and examples

### Success Criteria
- [ ] Acme can replace custom editor with `<EditorShell>`
- [ ] Collaborative editing works with y-websocket
- [ ] Slash commands extensible via registry
- [ ] Export formats pluggable without core changes

### Dependencies
None (extends existing components)

---

## IDR-007: Nightfire Slash Commands

**Status**: `proposed` (ready for execution)  
**Priority**: Medium  
**Estimated Effort**: 4 days  
**File**: `implementation-decision-records/idr-nightfire-slash-commands.md`

### Problem
Nightfire lacks quick block insertion UX found in Notion and other modern editors. Users must click UI buttons or know keyboard shortcuts.

### Research Basis
- **Notion**: `/` triggers command palette with 50+ block types
- **Editor.js**: Clean block structure, extensible
- **Lexical**: Command pattern architecture
- **Value Track**: High-impact UX pattern

### Deliverables
1. `SlashCommandPalette.svelte` component
2. `/` detection in editable areas
3. Default command set (paragraph, headings, lists, code, etc.)
4. Customizable command registry
5. Accessibility (ARIA, keyboard navigation)

### Success Criteria
- [ ] Slash palette appears when typing `/`
- [ ] Commands filter as user types
- [ ] Block inserts correctly on selection
- [ ] Keyboard navigation works
- [ ] Backward compatible (can be disabled)

### Dependencies
None (extends existing Nightfire component)

---

## Research Pipeline Status

### Immediate Priority (Complete)
| Topic | Status | IDR |
|-------|--------|-----|
| Passkey Client Abstractions | ✅ IDR ready | IDR-001 |
| AI Runtime Resilience | ✅ IDR ready | IDR-002 |
| Validation Patterns | ✅ IDR ready | IDR-003 |

### Medium Priority (Complete)
| Topic | Status | IDR |
|-------|--------|-----|
| Background Job Patterns | ✅ IDR ready | IDR-004 |
| Legacy Migration Frameworks | ✅ IDR ready | IDR-005 |
| Structured Content Editors | ✅ IDR ready | IDR-006 |
| Nightfire Slash Commands | ✅ IDR ready | IDR-007 |

### Longer Term (Backlog)
| Topic | Status |
|-------|--------|
| Media Processing | 🔴 Not started |
| Database Migration Patterns | 🔴 Not started |
| Admin UI / Low-Code | 🔴 Not started |
| Error Handling Standards | 🔴 Not started |
| Testing Patterns | 🔴 Not started |
| Design System Governance | 🔴 Not started |

---

## How to Execute an IDR

1. **Read the full IDR** - Check `implementation-decision-records/idr-*.md`
2. **Review research basis** - Follow links to specimens and value tracks
3. **Create implementation thread** - New branch/PR for the work
4. **Reference research in code** - Include research citations in comments
5. **Update validation checklist** - Mark criteria as complete
6. **Close the loop** - Update IDR status to `implemented` when merged

## Questions?

- Review `research-to-implementation-playbook.md` for workflow guidance
- Check `master-index.md` for navigation to related research
- See `RESEARCH_TOPICS.md` for full research roadmap
