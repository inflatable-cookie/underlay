# Research

Purpose: give the project a durable place to study external systems, standards, competitors, and research programs without mixing raw research into architecture docs or execution roadmaps.

## When to use this section

- architecture or roadmap decisions depend on external comparison or source-backed learning
- the team needs durable source maps and syntheses instead of ad hoc notes
- repeated discovery work would otherwise be lost in logs or scattered notes

## Structure

- `master-index.md`: research navigation by architecture area, concern, or prototype
- `research-to-implementation-playbook.md`: workflow for carrying research into delivery
- `quick-start-checklist.md`: short daily checklist for contributors using the corpus
- `research-to-architecture-crossref.md`: promotion and gap-tracking map between research and architecture
- `gaps-found-during-implementation.md`: implementation-discovered research gaps
- `specimen-dossiers/`: per-system or per-specimen studies
- `value-tracks/`: cross-specimen syntheses by problem area
- `source-hubs/`: curated source maps and source-quality hierarchy
- `translation-memos/`: project-facing recommendations derived from research
- `templates/`: reusable templates for the research workflow
- `discovery-intake.md` (optional): rules for triaging inbound signals
- `discovery-triage-log.md` (optional): staging area for research-now vs watchlist items

## Operating model

1. Start with a problem, not a product wishlist.
2. Gather primary sources before secondary commentary.
3. Record strengths, chronic failures, and between-version corrections together.
4. Convert findings into project implications only after cross-specimen comparison.
5. Promote stable conclusions into architecture or roadmaps only when the recommendation is specific enough to constrain design or execution.

## Source hierarchy

Prefer sources in this order:
1. official docs, source trees, release notes, talks, white papers, standards, and postmortems
2. first-party or partner technical programs with concrete claims
3. engineering blogs, staff interviews, and conference material with specific evidence
4. community synthesis only when it points back to stronger sources or documents observable behavior

## Promotion rule

Keep tentative findings here until they can answer all of:
- what problem the project is solving
- which evidence supports the recommendation
- which tradeoffs the project accepts
- what must be measured or prototyped before adoption

## Using This Research During Delivery

As the corpus grows:
- use `master-index.md` to find the right memo, value track, dossier, or prototype quickly
- use `research-to-implementation-playbook.md` to keep implementation work research-aware
- use `research-to-architecture-crossref.md` to track where memo findings are aligned, missing, or conflicting in architecture
- use `gaps-found-during-implementation.md` to capture missing research instead of losing it in PR chatter
- use `templates/implementation-decision-record-template.md` when a build decision needs explicit research traceability

## Templates

- `templates/specimen-dossier-template.md`
- `templates/value-track-synthesis-template.md`
- `templates/translation-memo-template.md`
- `templates/source-hub-template.md`
- `templates/implementation-decision-record-template.md`
- `templates/discovery-intake-template.md` (optional)
- `templates/discovery-triage-log-template.md` (optional)

## Research Topics

See [`RESEARCH_TOPICS.md`](./RESEARCH_TOPICS.md) for a catalog of strategic research areas prioritized for Underlay's evolution. Topics include:

- **Immediate Priority:** AI Runtime Routing, Modern Authentication, Validation Patterns
- **Medium Priority:** Background Jobs, Legacy Migration Frameworks, Structured Content Editors
- **Longer Term:** Media Processing, Database Migration, Admin UI Patterns, Error Handling, Testing, Design Systems

## Active Research

### Immediate Priority

1. **AI Runtime Routing** (`source-hubs/ai-llm-providers.md`)
   - Source hub created
   - Specimens: LiteLLM, Portkey dossiers created
   - Specimen: **Acowtancy internal** usage analyzed
   - Value track: AI runtime patterns synthesized
   - **IDR**: `idr-ai-runtime-resilience.md` - **READY FOR EXECUTION**
   - Finding: Apps building circuit breakers, retry, route chains on top of basic primitives

2. **Modern Authentication** (`source-hubs/modern-authentication.md`)
   - Source hub created
   - Specimen: Hanko dossier created
   - Specimen: **Acme Reference** dossier created (internal usage validation)
   - Value track: Passkey UX patterns synthesized
   - **Translation memo**: `passkey-client-abstractions.md` - **VALIDATED**
   - **Finding confirmed**: Acme reinvents `navigator.credentials.*` calls and error handling

3. **Validation Patterns** (`source-hubs/cross-language-validation.md`)
   - Source hubs created: Validation patterns, Codegen strategies
   - Specimens: Zod, Valibot, ts-rs dossiers created
   - Value track: Cross-language validation patterns synthesized
   - **IDR**: `idr-validation-zod-integration.md` - **READY FOR EXECUTION**
   - Finding: Server-only validation provides poor UX; Zod schemas recommended

## Completed Research Artifacts

| Artifact | Topic | Status |
|----------|-------|--------|
| `source-hubs/ai-llm-providers.md` | AI routing | Complete |
| `source-hubs/modern-authentication.md` | Auth standards | Complete |
| `source-hubs/cross-language-validation.md` | Validation | Complete |
| `specimen-dossiers/litellm.md` | AI specimen | Complete |
| `specimen-dossiers/hanko.md` | Auth specimen | Complete |
| `specimen-dossiers/zod.md` | Validation specimen | Complete |
| `specimen-dossiers/acme-reference-implementation.md` | Internal usage | Complete |
| `specimen-dossiers/portkey.md` | AI gateway specimen | Complete |
| `specimen-dossiers/valibot.md` | Validation alternative | Complete |
| `specimen-dossiers/ts-rs.md` | Codegen exploration | Complete |
| `specimen-dossiers/sidekiq.md` | Job queue patterns | Complete |
| `specimen-dossiers/bullmq.md` | Modern job observability | Complete |
| `specimen-dossiers/temporal.md` | Durable execution | Complete |
| `specimen-dossiers/airbyte.md` | ELT patterns | Complete |
| `specimen-dossiers/dbt.md` | Data testing patterns | Complete |
| `specimen-dossiers/debezium.md` | CDC patterns | Complete |
| `value-tracks/passkey-ux-patterns.md` | Passkey synthesis | Complete |
| `value-tracks/ai-runtime-patterns.md` | AI runtime synthesis | Complete |
| `value-tracks/cross-language-validation.md` | Validation synthesis | Complete |
| `value-tracks/background-job-patterns.md` | Job patterns synthesis | Complete |
| `value-tracks/legacy-migration-patterns.md` | Migration patterns synthesis | Complete |
| `translation-memos/passkey-client-abstractions.md` | Passkey recommendation | **Validated** |
| `translation-memos/cross-language-validation.md` | Validation recommendation | **Ready for execution** |
| `translation-memos/background-job-improvements.md` | Jobs recommendation | **Ready for execution** |
| `translation-memos/migration-framework-enhancements.md` | Migration recommendation | **Ready for execution** |
| `idr-passkey-client-hooks.md` | IDR for passkey hooks | **Ready for execution** |
| `idr-ai-runtime-resilience.md` | IDR for AI resilience | **Ready for execution** |
| `idr-validation-zod-integration.md` | IDR for Zod validation | **Ready for execution** |
| `idr-background-job-improvements.md` | IDR for job improvements | **Ready for execution** |
| `idr-migration-verification-rules.md` | IDR for migration rules | **Ready for execution** |

## Research Findings Summary

### Finding 1: Passkey Client Abstraction Gap (High Confidence)

**Evidence**: Analysis of Acme reference implementation shows:
- `acme-admin/src/routes/account/passkeys/+page.svelte`: 422 lines, ~40 lines direct WebAuthn API
- `acme-admin/src/routes/login/+page.svelte`: Similar `navigator.credentials.get()` implementation
- Error sanitization manually implemented (`sanitizePasskeyError`)
- Loading/error/success state management repeated

**Impact**: Every Underlay-based app with passkeys reinvents this boilerplate.

**Recommendation**: Provide `usePasskeyRegistration()`, `usePasskeyAuthentication()` hooks and `PasskeyManager.svelte` component.

**IDR**: `idr-passkey-client-hooks.md` ready for execution (3-5 days)

### Finding 2: AI Runtime Resilience Gap (High Confidence)

**Evidence**: Analysis of Acowtancy production implementation shows:
- Custom circuit breaker: 50+ lines (10 failures / 15 min threshold)
- Custom retry logic: 30+ lines with exponential backoff
- Custom route chain execution: 40+ lines for fallback
- Custom dead letter queue: 20+ lines

**Comparison**: Portkey and LiteLLM provide these as built-in features.

**Impact**: Every production AI app needs circuit breakers, retry, fallbacks.

**Recommendation**: Add `CircuitBreakerMiddleware`, `RetryMiddleware`, and `RouteChainExecutor` to `underlay-ai-runtime`.

**IDR**: `idr-ai-runtime-resilience.md` ready for execution (8-12 days)

### Finding 3: Validation Cross-Language Gap (Medium Confidence)

**Evidence**: Acme reference uses server-only validation with HTML5:
- Rust: `#[validate(email)]`, `#[validate(length(min = 12, max = 128))]`
- TypeScript: `interface RegisterRequest { email: string; password: string; }`
- Svelte: HTML5 `type="email"`, `maxlength` only

**Gap**: No client-side email format validation, no password length validation until server response.

**Comparison**: Zod is industry standard, Valibot offers tree-shaking, ts-rs doesn't handle validation.

**Recommendation**: Add `@decodelabs/underlay/validation` export with Zod schemas.

**IDR**: `idr-validation-zod-integration.md` ready for execution (5-8 days)

### Finding 4: Background Job Reliability Gap (Medium Confidence)

**Evidence**: Underlay `underlay-jobs` compared to mature systems:
- **No jitter**: Sidekiq/BullMQ use exponential backoff + jitter; Underlay lacks jitter
- **Limited dead letters**: Failed jobs sit in DB; no dedicated dead letter management
- **No observability**: No built-in events or metrics; apps must build their own
- **No orchestration**: BullMQ has Flows, Temporal has workflows; Underlay has nothing

**Acowtancy implements**: Custom circuit breaker, manual state management between job steps

**Comparison**:
| Feature | Sidekiq | BullMQ | Underlay |
|---------|---------|--------|----------|
| Retry jitter | ✅ | ✅ | ❌ |
| Dead letter UI | ✅ | ✅ | ❌ |
| Job events | ✅ | ✅ | ❌ |
| Flows/DAGs | Pro | ✅ | ❌ |

**Recommendation**: Add jitter to backoff, dead letter table, job lifecycle events.

**IDR**: `idr-background-job-improvements.md` ready for execution (6 days)

### Finding 5: Migration Verification Gap (Medium Confidence)

**Evidence**: Underlay migration framework compared to industry:
- **dbt**: Declarative SQL tests, auto-generated docs
- **Airbyte**: Row counts, basic stats
- **Debezium**: Transaction boundaries
- **Underlay**: Custom code in verify stage

**Gap**: No declarative verification rules; custom code for each migration.

**Comparison**:
| Feature | dbt | Underlay |
|---------|-----|----------|
| Declarative tests | ✅ SQL-based | ❌ Code only |
| Standard rules | ✅ built-in | ❌ custom each time |
| Auto-docs | ✅ | ❌ |

**Recommendation**: Add `VerificationRule` system with standard rules.

**IDR**: `idr-migration-verification-rules.md` ready for execution (4-5 days)

## Ready for Implementation Threads

### IDR-001: Passkey Client Hooks (High Priority, 3-5 days)
- **Artifact**: `implementation-decision-records/idr-passkey-client-hooks.md`
- **Scope**: `usePasskeyRegistration()`, `usePasskeyAuthentication()`, `PasskeyManager.svelte`
- **Validation**: Test in Acme reference implementation

### IDR-002: AI Runtime Resilience (High Priority, 8-12 days)
- **Artifact**: `implementation-decision-records/idr-ai-runtime-resilience.md`
- **Scope**: Circuit breaker, retry middleware, route chain executor
- **Validation**: Port Acowtancy's implementation to Underlay, verify in production

### IDR-003: Validation Zod Integration (Medium Priority, 5-8 days)
- **Artifact**: `implementation-decision-records/idr-validation-zod-integration.md`
- **Scope**: `@decodelabs/underlay/validation` export, `useValidatedForm` hook
- **Validation**: Test in Acme reference forms, measure bundle impact

### IDR-004: Background Job Improvements (Medium Priority, 6 days)
- **Artifact**: `implementation-decision-records/idr-background-job-improvements.md`
- **Scope**: Jitter in backoff, dead letter queue, job lifecycle events
- **Validation**: Test thundering herd prevention, dead letter retry

### IDR-005: Migration Verification Rules (Medium Priority, 4-5 days)
- **Artifact**: `implementation-decision-records/idr-migration-verification-rules.md`
- **Scope**: Declarative `VerificationRule`, standard rule library
- **Validation**: Test with Acme migration, convert existing verification

## Next Task for Research Thread

**Immediate priority research complete.** Three IDRs ready for execution.

Options for continuing research:
1. **Medium Priority**: Background Job Patterns (Sidekiq, BullMQ, Temporal comparison)
2. **Medium Priority**: Migration Frameworks (ETL/ELT patterns, CDC, Airbyte)
3. **Longer Term**: Structured Content Editors (Nightfire evolution, collaborative editing)

Or: Support implementation threads with validation and research questions as needed.
