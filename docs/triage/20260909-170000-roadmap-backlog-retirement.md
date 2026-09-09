# Roadmap Backlog Retirement (2026-09-09)

Status: open
Captured: 2026-09-09

## Observation

Northstar retired the roadmap backlog surface because it duplicated triage
and blurred the boundary between a candidate and approved execution.
`docs/roadmaps/backlog/` held fourteen deferred items. None is approved
executable work: no owning `gNN.NNN` task authorizes any of them, and the
active `g11`/`g12` frontiers do not include them. Their current meaning is
preserved below as non-authoritative candidates. This note is the single
active home for all fourteen items after
`docs/roadmaps/backlog/ts-7-adoption.md` (referenced from the `g09`
roll-up and the flattened-task switchover manifest).

## Impact

Until promotion, these candidates must not be executed, dispatched, or
treated as approved scope. Promotion requires explicit planning authority
(a bounded numbered `gNN.NNN` task with scope, generation, dependencies,
ordering, and frontier placement) — migration is not approval.

## Dispositions

### 1. Advanced form features (was `backlog/advanced-forms.md`)

- Prior status/priority/effort: Backlog, Low, 4-8 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: possible future additions on top of `createFormState` —
  multi-step wizard, auto-save drafts, conditional fields, async field
  validation, undo/redo. Basic form state (roadmap 009) plus
  `storage.session` drafts already cover single-page forms.
- Constraints/open questions: each feature must work with the existing
  `createFormState` API, stay accessible (focus management, announcements),
  and integrate with SvelteKit form actions. No concrete requesting
  workflow is known.
- Promotion condition: promote when basic forms prove insufficient for a
  concrete workflow (users report complexity pain, or a specific lane
  requires a multi-step process).
- Owner/next check: unknown; re-check on the next form-complexity request.

### 2. API reference documentation (was `backlog/api-reference-docs.md`)

- Prior status/priority/effort: Backlog, Medium, 6-8 hours.
- Source: deferred from roadmap 009 (Quick Wins); created 2026-01-12.
- Current meaning: generate and publish rustdoc plus TypeDoc output behind
  a unified static site (Docusaurus/VitePress/mdBook undecided) with CI
  deployment, replacing source-code spelunking for API discovery.
- Constraints/open questions: hosting solution undecided; versioning
  strategy undecided; docs must be auto-generated to avoid going stale.
  No concrete discovery-pain driver is known.
- Promotion condition: promote when API discovery pain is reported or a
  concrete docs-publishing driver appears.
- Owner/next check: unknown; re-check on the next docs-infrastructure ask.

### 3. Background job dashboard (was `backlog/background-job-dashboard.md`)

- Prior status/priority/effort: Backlog, Low, 8-10 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: web UI over `underlay-jobs` for listing, inspecting,
  retrying, and cancelling jobs plus success-rate/duration statistics.
- Constraints/open questions: needs admin authentication; UI framework
  choice (embed in existing admin?) undecided; real-time updates optional.
- Promotion condition: promote when debugging failed jobs becomes painful
  or job volume demands shared visibility.
- Owner/next check: unknown; re-check when job-debugging pain is reported.

### 4. Caching layer (was `backlog/caching-layer.md`)

- Prior status/priority/effort: Backlog, Low, 6-8 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: declarative `#[cached]`-style function caching plus a
  manual cache API with TTL and pattern invalidation, in-memory and Redis
  backends, hit/miss metrics.
- Constraints/open questions: storage backend and serialization strategy
  undecided. Standing decision rule: profile first, cache second — no
  caching complexity without proven performance need.
- Promotion condition: promote when profiling shows repeated queries above
  ~100ms where caching would help significantly.
- Owner/next check: unknown; re-check on the next performance-driven lane.

### 5. CLI scaffolding (was `backlog/cli-scaffolding.md`)

- Prior status/priority/effort: Backlog, Low, 10-15 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: `underlay generate` commands (crud, migration,
  component, command, test) from customizable templates.
- Constraints/open questions: deliberately deferred until generation
  patterns stay stable for 3+ months; premature scaffolding risks
  cargo-culting and template maintenance burden.
- Promotion condition: promote when teams create 10+ similar endpoints
  from visibly stable patterns, or onboarding cost demands it.
- Owner/next check: unknown; re-check when endpoint boilerplate pain is
  reported.

### 6. Collection helper extractions (was `backlog/collection-helper-extractions.md`)

- Prior status/priority/effort: Backlog, Medium, 4-6 hours.
- Source: contact-patch normalization lane (2026-08 consumer template
  convergence).
- Current meaning: three retained extractions — compose
  `createLoadedReorderSession` inside `EntityList.svelte` (deleting the
  duplicated ~lines 895-941 state machine), a retained
  `loadAllPages(fetchPage)` paged fan-out helper, and a retained
  `resolveReorderScope(query, { filterField })` reorder-eligibility helper.
  Proof consumers: contact-patch `book-reorder.ts` and dairy equivalents.
- Constraints/open questions: extraction boundaries sketched but not
  approved; adoption proof in two consumers required.
- Promotion condition: promote when the next consumer asks for
  subject/scope-gated reorder, or when `EntityList` reorder internals are
  next touched for other reasons.
- Owner/next check: unknown; re-check on the next reorder-related lane.

### 7. `createEntityListState` follow-ups (was `backlog/entity-list-state-followups.md`)

- Prior status/priority/effort: Backlog, Low, 2-3 hours.
- Source: dairy wrapper rollout (2026-08 consumer template convergence).
- Current meaning: four factory gaps that stayed hand-rolled in dairy —
  raw `reloadKey` override without the `:refreshVersion` suffix,
  `sourceContextFallback`, a pure `filterValueFromQuery` helper, and
  thunked `pageSize`.
- Constraints/open questions: API shapes sketched but not approved;
  adoption proof in dairy's remaining hand-rolled spots required.
- Promotion condition: promote when the next consumer wrapper rollout hits
  the same shapes, or when the factory is next touched for other reasons.
- Owner/next check: unknown; re-check on the next wrapper-rollout lane.

### 8. GraphQL support (was `backlog/graphql-support.md`)

- Prior status/priority/effort: Backlog (deferred indefinitely), Very Low,
  20-30 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: code-first GraphQL schema generation with async-graphql
  plus client tooling. Standing assessment: probably overkill — REST plus
  good API design is usually sufficient.
- Constraints/open questions: major paradigm shift, learning curve,
  unbounded-query footguns, harder caching. No requesting project known.
- Promotion condition: promote only if multiple projects need complex
  nested queries and over/under-fetching becomes a measured problem.
- Owner/next check: unknown; re-check only on concrete multi-project
  demand.

### 9. OpenTelemetry span integration (was `backlog/opentelemetry-integration.md`)

- Prior status/priority/effort: Backlog, Medium, 4-6 hours.
- Source: deferred from roadmap 009 (Quick Wins); created 2026-01-12.
- Current meaning: extend `RequestContext` in `underlay-http` and
  `underlay-observability` with trace/span IDs, a Tower `OtelLayer` for
  automatic spans, and W3C Trace Context propagation for OTLP backends
  (Jaeger, Tempo).
- Constraints/open questions: must be feature-gated to avoid bloating
  non-OTLP users; span-creation overhead must be measured; OTLP endpoint
  configuration varies by environment.
- Promotion condition: promote when distributed-trace correlation is
  requested by a concrete deployment.
- Owner/next check: unknown; re-check on the next observability ask.

### 10. Performance benchmarks (was `backlog/performance-benchmarks.md`)

- Prior status/priority/effort: Backlog, Low, 4-6 hours.
- Source: deferred from roadmap 009 (Quick Wins); created 2026-01-12.
- Current meaning: Criterion benchmarks for Rust crates, Vitest bench for
  TypeScript patterns, CI regression alerts (>10%), historical results in
  `docs/benchmarks/`, plus initial performance budgets (token verify <1ms,
  context extract <0.1ms, pagination wrap <0.5ms, storage get <0.1ms).
- Constraints/open questions: shared-CI-runner noise, regression
  thresholds need tuning, benchmarks need maintenance as APIs change.
- Promotion condition: promote when regression detection or objective
  implementation comparisons are requested.
- Owner/next check: unknown; re-check on the next performance-regression
  report.

### 11. Rate limiting (was `backlog/rate-limiting.md`)

- Prior status/priority/effort: Backlog, Low, 4-6 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: pluggable Axum rate-limit middleware (token bucket /
  sliding window; per-user, per-IP, global) with 429 responses, rate-limit
  headers, and in-memory plus Redis backends.
- Constraints/open questions: storage backend undecided (Redis needed for
  distributed); internal/admin bypass policy undecided.
- Promotion condition: promote when abuse is reported, endpoints are
  hammered, or billing tiers require it.
- Owner/next check: unknown; re-check on the next abuse report.

### 12. Real-time / WebSocket layer (was `backlog/realtime-websocket.md`)

- Prior status/priority/effort: Backlog, Low, 15-20 hours.
- Source: deferred from roadmap 011 (Advanced Features); created 2026-01-12.
- Current meaning: WebSocket infrastructure with channels, presence,
  broadcast/direct messaging, per-connection auth, and a Svelte client SDK.
- Constraints/open questions: server scaling strategy (sticky sessions vs
  Redis pub/sub) undecided; memory cost of many connections; fallback for
  non-WebSocket environments.
- Promotion condition: promote when users request live updates, polling
  becomes excessive, or collaboration features are required.
- Owner/next check: unknown; re-check on the next real-time feature ask.

### 13. Storage expiration support (was `backlog/storage-expiration.md`)

- Prior status/priority/effort: Backlog, Low, 2-3 hours.
- Source: deferred from roadmap 009 (Quick Wins); created 2026-01-12.
- Current meaning: optional TTL / `expiresAt` on `patterns/storage.ts`
  `set()` and `store()`, lazy expiration on access, SSR-safe timer
  handling, backwards compatible with existing stored values.
- Constraints/open questions: storage metadata format change needs a
  migration path; reactive expiration timer management; lazy vs background
  cleanup undecided.
- Promotion condition: promote when stale-cache cleanup or TTL-backed
  session data is requested by a concrete app.
- Owner/next check: unknown; re-check on the next storage-TTL ask.

### 14. TypeScript 7 adoption (was `backlog/ts-7-adoption.md`)

- Prior status: deferred; added 2026-08-03.
- Source: JS dependency survey 2026-08-03 (TypeScript 5.9.3 → 7.0.2).
- Current meaning: adopt TypeScript 7 across the family. Deliberately
  deferred: TS 7 is the native-port compiler era with a new performance
  and behavior profile, days old at survey time; TS 5.9.3 works
  everywhere and early adoption across ~20 packages has no driver.
- Constraints/open questions: at promotion, evaluate against
  svelte-check, tsc strict builds, and vitest across the family,
  underlay first.
- Promotion condition: promote when TypeScript 7.1 (first minor) is
  released, or when a concrete need appears (build-time wins, a required
  feature).
- Owner/next check: unknown; re-check on the TypeScript 7.1 release or
  the next toolchain-upgrade lane.

## Disposition

Keep open as the non-authoritative candidate record. Do not execute any
section without a promoted numbered roadmap task.
