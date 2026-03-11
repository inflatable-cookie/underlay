# Underlay Research Topics

This document catalogs strategic research areas for Underlay's evolution. Each topic identifies the problem space, why it matters for Underlay, and specific specimens/source families to study.

---

## Topic 1: Modern Authentication Patterns & Standards

**Problem space:** Authentication is evolving rapidly (Passkeys, OAuth 2.1, FedCM). Underlay supports JWT/Ed25519, Argon2, TOTP, WebAuthn, and OAuth2, but standards are shifting.

**Why it matters:** Auth is security-critical and user-facing. Getting it wrong has severe consequences. Underlay needs to track emerging standards while maintaining backward compatibility.

**Research targets:**
- **WebAuthn/Passkeys evolution** - Platform authenticator trends, hybrid transport, passkey sync across devices
- **OAuth 2.1 / OIDC** - Security BCP changes, PKCE enforcement, Pushed Authorization Requests (PAR)
- **FedCM (Federated Credential Management)** - Google's alternative to third-party cookies for identity
- **Zero Trust Architecture** - BeyondCorp patterns, device trust, continuous authentication
- **Passkey adoption friction** - Real-world UX studies, fallback patterns, enterprise deployment challenges

**Specimens to study:**
- Auth0 / Okta - Enterprise identity patterns, tenant isolation
- Keycloak - Open source alternative, protocol support breadth
- Hanko - Passwordless-first modern stack
- Clerk - Developer experience focused auth
- AWS Cognito - Managed auth at scale, limitations

---

## Topic 2: Background Job Queue Patterns & Reliability

**Problem space:** Underlay has `underlay-jobs` with PostgreSQL persistence and cron scheduling. Background job systems have well-documented failure modes and reliability patterns.

**Why it matters:** Jobs handle critical async work (emails, media processing, migrations). Silent failures, retry storms, and queue poisoning are common production issues.

**Research targets:**
- **Exactly-once semantics** - Delivery guarantees, idempotency patterns, transaction outbox
- **Retry and dead letter policies** - Exponential backoff with jitter, circuit breakers, poison pill handling
- **Queue observability** - Lag metrics, throughput, failure rate alerting
- **Priority and rate limiting** - Fair queuing, rate limiting per tenant/job type
- **Scheduler accuracy** - Cron drift, daylight saving time handling, missed execution recovery

**Specimens to study:**
- Sidekiq (Ruby) - Industry standard, reliability patterns, pro features
- BullMQ / Bee-Queue (Node.js) - Redis-based, job progress tracking, sandboxed processors
- Celery (Python) - Task routing, chords/groups, result backends
- Faktory (Go) - Language-agnostic, transaction safety
- Temporal / Cadence - Durable execution, saga patterns
- PostgreSQL queue patterns - SKIP LOCKED, advisory locks, NOTIFY/LISTEN

---

## Topic 3: AI Runtime Routing & Provider Abstractions

**Problem space:** Underlay has `underlay-ai-runtime` with OpenAI-compatible client and basic routing. The LLM provider landscape is fragmenting rapidly.

**Why it matters:** Apps need to route between providers for cost, capability, and resilience. Underlay's abstraction should be future-proof against provider API divergence.

**Research targets:**
- **Multi-provider routing strategies** - Cost-based, capability-based, latency-based, A/B testing
- **Structured output / JSON mode** - Provider differences, schema validation, streaming parsers
- **Tool calling patterns** - Function calling standardization, parallel tool execution, recursive calls
- **Embeddings routing** - Different embedding models per use case, normalization strategies
- **Provider failover** - Health checking, circuit breakers, graceful degradation
- **Cost tracking & attribution** - Per-request cost estimation, budget enforcement, showback

**Specimens to study:**
- LiteLLM - Unified interface for 100+ providers, routing, cost tracking
- Portkey.ai - Gateway pattern, observability, guardrails
- OpenRouter - Model routing as a service, unified API
- LangSmith / Langfuse - Tracing, evaluation, observability
- Vercel AI SDK - Streaming patterns, React integration
- Anthropic's MCP (Model Context Protocol) - Emerging standard for tool/context

---

## Topic 4: Validation Patterns & Cross-Language Contracts

**Problem space:** Underlay has `underlay-validation` with a declarative `Validate` trait and derive macro. Validation logic often needs to be shared between Rust backend and TypeScript frontend.

**Why it matters:** Duplicate validation logic creates drift. Single-source-of-truth validation that compiles to both languages reduces bugs and improves UX (instant feedback).

**Research targets:**
- **Schema-first validation** - JSON Schema, OpenAPI, Protocol Buffers as source of truth
- **Compile-to-TypeScript patterns** - Rust proc macros generating TS validators
- **Partial validation** - Form field-level validation vs full record validation
- **Async validation** - Server-side uniqueness checks, availability validation
- **Cross-field validation** - Dependencies between fields, conditional rules
- **Error message internationalization** - Structured errors, i18n keys, parameter interpolation

**Specimens to study:**
- Zod (TypeScript) - Type inference, composition, refinements
- Valibot (TypeScript) - Tree-shakable alternative to Zod
- Joi / Yup - Mature validation libraries, error customization
- class-validator (TypeScript) - Decorator-based, similar to Underlay's derive approach
- CUE Language - Configuration validation, unified type system
- JSON Type Definition (JTD) - Standardized schema format

---

## Topic 5: Media Processing & Modern Image Formats

**Problem space:** Underlay has `underlay-image` and `underlay-media` with basic thumbnail/rendition support. Image processing is complex and format standards evolve.

**Why it matters:** Media handling impacts performance (Core Web Vitals), storage costs, and visual quality. Modern formats (AVIF, HEIC, JXL) offer significant improvements.

**Research targets:**
- **Modern format adoption** - AVIF browser support, HEIC/HEIF handling, JPEG XL status
- **Responsive image strategies** - srcset, sizes, art direction, CDN optimization
- **On-demand vs pre-generated renditions** - Tradeoffs, cache invalidation, cost
- **Video processing pipeline** - Transcoding, adaptive bitrate, thumbnail extraction
- **Storage cost optimization** - Tiered storage, lifecycle policies, CDN integration
- **Content-aware cropping** - Smart crop, focal point detection, face detection

**Specimens to study:**
- Sharp (Node.js) - High-performance resizing, format conversion, streaming
- ImageMagick / GraphicsMagick - Swiss army knife, security track record
- Cloudinary / Imgix - Managed transformation URLs, automatic format selection
- Sanity image pipeline - GROQ-based asset handling, Hotspot/crop
- AWS Elemental MediaConvert - Video processing at scale
- Squoosh (Google) - Web-based optimization, WASM codecs

---

## Topic 6: Database Migration & Schema Evolution Patterns

**Problem space:** Underlay has `underlay-db` with SQLx migrations and `underlay-migration-core` for legacy data migration. Schema evolution is a chronic pain point.

**Why it matters:** Zero-downtime deployments require careful migration sequencing. Column additions, renames, and type changes have different safety profiles.

**Research targets:**
- **Zero-downtime migration patterns** - Expand-contract, shadow tables, trigger-based replication
- **Migration system comparisons** - Versioned vs timestamp ordering, transaction handling
- **Schema validation** - Drift detection, schema as code, compatibility checking
- **Column renaming strategies** - Views, dual-write, backfill patterns
- **Data masking / PII handling** - Migration-time anonymization, column-level encryption rotation
- **Rollback safety** - Reversible migrations, downgrade testing

**Specimens to study:**
- Rails Active Record Migrations - Mature ecosystem, strong conventions
- Django Migrations - Dependency resolution, squash support
- Flyway / Liquibase - Enterprise Java, repeatability guarantees
- Atlas (Ariga) - Modern SQL schema management, drift detection
- gh-ost / pt-online-schema-change - Online DDL for MySQL
- Reshape (Fabian Lindfors) - PostgreSQL-specific, reversible migrations

---

## Topic 7: Legacy Data Migration Framework Patterns

**Problem space:** Underlay has a sophisticated `underlay-migration-core` for deterministic legacy migration with decision journaling. This is a complex domain with many prior implementations.

**Why it matters:** Legacy migration is high-risk, often one-way, and business-critical. Learning from prior art prevents costly mistakes.

**Research targets:**
- **ETL vs ELT patterns** - Transform timing, data warehouse loading patterns
- **CDC (Change Data Capture)** - Debezium, WAL parsing, real-time sync
- **Decision preservation** - Human-in-the-loop, AI-assisted decisions, replay semantics
- **Verification strategies** - Reconciliation, checksums, sampling, acceptance testing
- **Rollback and idempotency** - Partial failure handling, checkpointing
- **Bundle distribution** - OCI artifacts, versioning, signature verification

**Specimens to study:**
- Airbyte / Fivetran - Managed ETL, connector ecosystem
- Dbt (data build tool) - Transformation layer, testing, documentation
- AWS DMS / Azure Data Factory - Cloud migration services
- Striim - Real-time CDC, streaming migration
- Trunk.io / Sqitch - Database change management, Git-native workflows
- Data Mesh patterns - Domain-oriented ownership, federated governance

---

## Topic 8: Admin UI Patterns & Low-Code Builders

**Problem space:** Underlay has extensive admin patterns (AutonomousList, RelationSelector, BatchActions, FormShell). Admin UIs are often repetitive and could benefit from more generic scaffolding.

**Why it matters:** Admin interfaces consume significant development time. Better patterns could reduce boilerplate while maintaining flexibility for domain-specific needs.

**Research targets:**
- **CRUD scaffolding generators** - Rails Admin, Django Admin, ActiveAdmin patterns
- **Low-code admin builders** - Retool, Appsmith, Budibase internal tool patterns
- **Data table advanced features** - Virtual scrolling, column pinning, aggregation, pivoting
- **Form builders** - JSON Schema forms, conditional visibility, dynamic validation
- **Dashboard frameworks** - Grid layouts, widget systems, real-time updates
- **Permission-aware UIs** - Field-level permissions, conditional actions, read-only modes

**Specimens to study:**
- TanStack Table (React) - Headless, extensible, virtualization
- React Admin - Resource-driven CRUD, data providers
- Refine (React) - Open source Retool alternative, data provider pattern
- AG Grid - Enterprise features, Excel-like experience
- FormKit / Vue Formulate - Form composition, validation, accessibility
- Payload CMS / Directus - Schema-driven admin, content management patterns

---

## Topic 9: Structured Content (Block-Based) Editor Patterns

**Problem space:** Underlay has Nightfire, a block-based structured content system. Block-based editing is complex with UX and performance challenges.

**Why it matters:** Rich content is central to many apps. Getting the editor UX, validation, and rendering pipeline right affects content quality and author experience.

**Research targets:**
- **Block definition schemas** - Versioning, validation, migration between versions
- **Collaborative editing** - OT vs CRDT, conflict resolution, presence awareness
- **Real-time validation** - Schema validation during editing, linting rules
- **Paste handling** - HTML/markdown to block conversion, cleanup rules
- **Accessibility** - Screen reader support, keyboard navigation, focus management
- **Performance at scale** - Large document handling, virtualization, lazy loading

**Specimens to study:**
- Editor.js - Block-based, clean JSON output, plugin architecture
- Notion - Gold standard UX, nested blocks, databases
- Sanity.io - Portable Text, schema-driven, patches
- Strapi / Contentful - Headless CMS content modeling
- TipTap / ProseMirror - Schema-based, collaborative-ready, extensible
- Slate.js - Nested document model, normalization
- Lexical (Meta) - Modern architecture, React integration

---

## Topic 10: Error Handling & API Response Contracts

**Problem space:** Underlay has standardized error envelopes (`ErrorEnvelope`) and DTO patterns. Error handling conventions vary widely across ecosystems.

**Why it matters:** Consistent, actionable error responses improve developer experience and client resilience. Poor error handling is a major API friction point.

**Research targets:**
- **RFC 7807 (Problem Details)** - Standard error format, extension mechanisms
- **GraphQL error patterns** - Path-based errors, partial success, extensions
- **gRPC status codes** - Rich error details, retry semantics
- **Field-level error mapping** - Form validation, nested object errors
- **Error code taxonomy** - Machine-readable codes, documentation linkage
- **Retry guidance** - Retry-After headers, exponential backoff signals

**Specimens to study:**
- Stripe API errors - Clear messages, request IDs, documentation links
- GitHub REST API - Error object consistency, rate limit headers
- Twilio - Detailed error codes, debugging assistance
- Kubernetes API - Status conditions, reason codes
- Twirp (gRPC for HTTP) - Error code mapping, metadata

---

## Topic 11: Testing Patterns & Developer Experience

**Problem space:** Underlay has `underlay-testing` with `TestDb` and `TestServer`. Testing async, database-backed systems has known patterns and anti-patterns.

**Why it matters:** Testing is essential for confidence but often creates friction. Fast, reliable tests improve velocity. Brittle, slow tests are ignored.

**Research targets:**
- **Test database strategies** - Transaction rollback, template databases, Testcontainers
- **Property-based testing** - Hypothesis/Proptest for finding edge cases
- **Snapshot testing** - Approval tests, contract testing, regression prevention
- **Mocking vs fakes** - Tradeoffs, HTTP mocking, time manipulation
- **Contract testing** - Pact, consumer-driven contracts, API compatibility
- **Mutation testing** - Test quality measurement, surviving mutants
- **Parallel test execution** - Isolation guarantees, resource contention

**Specimens to study:**
- Ruby on Rails testing - Fixtures, transactional tests, integration patterns
- pytest (Python) - Fixtures, parametrize, plugins
- Jest / Vitest - Snapshot testing, mocking, watch mode
- cargo-nextest - Parallel execution, test grouping, faster Rust tests
- Playwright / Cypress - E2E testing, traceability, component testing
- Pact - Consumer contract testing, provider verification

---

## Topic 12: Design System Governance & Component Architecture

**Problem space:** Underlay exports many Svelte components. Component library maintenance, versioning, and governance are well-studied problems.

**Why it matters:** UI components are highly visible. Breaking changes affect all consuming apps. Consistency, accessibility, and documentation are ongoing concerns.

**Research targets:**
- **Component composition patterns** - Compound components, slots vs props, render props
- **Styling architecture** - CSS-in-JS, Tailwind, CSS variables, design tokens
- **Accessibility governance** - WCAG compliance, automated testing (axe), screen reader testing
- **Versioning strategy** - Breaking change policies, codemods, deprecation timelines
- **Documentation patterns** - Storybook, prop tables, usage examples
- **Token architecture** - Semantic tokens, theme switching, dark mode

**Specimens to study:**
- Radix UI / Headless UI - Unstyled, accessible primitives
- shadcn/ui - Copy-paste components, no dependency
- Carbon Design System (IBM) - Enterprise governance, accessibility
- Material Design - Token system, component specifications
- Chakra UI / Mantine - Modern React component APIs
- Lightning Design System (Salesforce) - Enterprise scale, documentation

---

## Research Prioritization

### Immediate Priority (Next 3 months)
1. **AI Runtime Routing** - Active development area, rapidly evolving landscape
2. **Modern Authentication** - Security-critical, Passkey adoption accelerating
3. **Validation Patterns** - High leverage for DX improvement across all apps

### Medium Priority (3-6 months)
4. **Background Job Patterns** - Reliability improvements, operational maturity
5. **Legacy Migration Frameworks** - Validate Underlay's approach against prior art
6. **Structured Content Editors** - Nightfire evolution, collaborative editing

### Longer Term (6+ months)
7. **Media Processing** - Format evolution, performance optimization
8. **Database Migration Patterns** - Zero-downtime deployment maturity
9. **Admin UI / Low-Code** - Potential for generational improvement in DX
10. **Error Handling Standards** - API consistency, developer experience
11. **Testing Patterns** - Quality and velocity enablement
12. **Design System Governance** - Scale and maintenance maturity

---

## Next Steps

1. Create source hubs for the immediate priority topics
2. Start with specimen dossiers for the most relevant external systems
3. Build value track syntheses as patterns emerge across specimens
4. Write translation memos when recommendations are concrete enough to guide implementation
