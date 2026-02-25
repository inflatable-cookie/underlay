# Underlay Sweeps

Sweeps are prescripted cross-repository checks used to audit Underlay-based projects for consistency, security, and operational guardrails.

Unlike implementation guides, sweeps are runbooks for reviewing an existing codebase.

## When to use sweeps

- Before a release or major deploy.
- During onboarding to a new Underlay consumer project.
- As part of periodic platform health checks (for example, monthly or quarterly).
- After incidents to validate that key guardrails still hold.

## Sweep catalogue

| Sweep | Focus | Audience |
|-------|-------|----------|
| [001-security-sweep.md](./001-security-sweep.md) | Comprehensive security review across API, client, and frontends | Platform/security reviewers |
| [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md) | Detect and correct reimplementation of existing Underlay UI/pattern functionality | Frontend/platform reviewers |
| [003-frontend-consistency-sweep.md](./003-frontend-consistency-sweep.md) | Cross-site consistency checks for frontend architecture, API usage, state, and UX patterns | Frontend/platform reviewers |
| [004-tab-count-badges-sweep.md](./004-tab-count-badges-sweep.md) | Ensure tab badge counts come from detail DTO counts, not eager relation-list fetches | Frontend/API reviewers |
| [005-api-client-contract-drift-sweep.md](./005-api-client-contract-drift-sweep.md) | Detect DTO, endpoint, and type drift between Rust API and TypeScript client | API/client maintainers |
| [006-query-efficiency-sweep.md](./006-query-efficiency-sweep.md) | Detect avoidable API/DB over-fetching, fan-out requests, and N+1 query patterns | Backend/frontend reviewers |
| [007-error-diagnostics-and-logging-sweep.md](./007-error-diagnostics-and-logging-sweep.md) | Enforce Underlay db diagnostics and HTTP error logging with rich context | Backend/platform reviewers |
| [008-form-and-nightfire-validation-sweep.md](./008-form-and-nightfire-validation-sweep.md) | Validate API + client form validation coverage, async checks, and Nightfire emptiness rules | Backend/frontend reviewers |
| [009-rich-text-storage-alignment-sweep.md](./009-rich-text-storage-alignment-sweep.md) | Ensure `TEXT`/Markdown and `JSONB`/Nightfire are aligned end-to-end across DB, API, client, and UI | Backend/frontend reviewers |
| [010-authorization-boundary-sweep.md](./010-authorization-boundary-sweep.md) | Verify authn/authz boundaries are enforced in API extractors and route topology, not only in UI | Backend/security reviewers |
| [011-migration-safety-sweep.md](./011-migration-safety-sweep.md) | Enforce safe SQL migration practices (schema qualification, idempotency, and rollout discipline) | Backend/platform reviewers |
| [012-observability-and-audit-sweep.md](./012-observability-and-audit-sweep.md) | Validate error/event observability and audit trail coverage for critical mutations | Backend/platform reviewers |
| [013-background-jobs-and-scheduler-reliability-sweep.md](./013-background-jobs-and-scheduler-reliability-sweep.md) | Verify job queue and scheduler reliability (idempotency, retries, overlap control, and recovery) | Backend/platform reviewers |
| [014-accessibility-and-keyboard-sweep.md](./014-accessibility-and-keyboard-sweep.md) | Verify accessibility fundamentals: keyboard nav, focus management, ARIA semantics, and form/dialog usability | Frontend/platform reviewers |
| [015-test-coverage-and-critical-paths-sweep.md](./015-test-coverage-and-critical-paths-sweep.md) | Ensure critical workflows have meaningful automated coverage across API, client, and frontend boundaries | Backend/frontend reviewers |
| [016-api-versioning-and-backward-compat-sweep.md](./016-api-versioning-and-backward-compat-sweep.md) | Verify API version signaling and backward-compat discipline across routes, DTOs, and client commands | API/client maintainers |
| [017-dependency-and-supply-chain-hygiene-sweep.md](./017-dependency-and-supply-chain-hygiene-sweep.md) | Detect vulnerable, stale, or risky dependencies and enforce dependency hygiene across Rust and TypeScript stacks | Platform/security reviewers |
| [018-privacy-and-sensitive-data-handling-sweep.md](./018-privacy-and-sensitive-data-handling-sweep.md) | Verify PII/secret handling, redaction, retention, and safe data exposure across API, jobs, logs, and frontends | Security/platform reviewers |
| [019-pagination-contract-consistency-sweep.md](./019-pagination-contract-consistency-sweep.md) | Catch cursor/page pagination drift across API routes, client commands, and frontend list controllers | API/client/frontend maintainers |
| [022-api-endpoint-naming-convention-sweep.md](./022-api-endpoint-naming-convention-sweep.md) | Enforce consistent endpoint naming policy (`for-list`/`for-filter`, action routes, and alias cleanup) across API, client, and frontend consumers | API/client/frontend maintainers |
| [023-cache-contract-consistency-sweep.md](./023-cache-contract-consistency-sweep.md) | Verify validator headers, conditional requests, write preconditions, and cache directive consistency | API/client/frontend maintainers |
| [024-admin-fetch-and-caching-pressure-sweep.md](./024-admin-fetch-and-caching-pressure-sweep.md) | Identify repeated admin-fetch hotspots and ensure dedupe/microcache controls are applied proportionately | API/client/frontend maintainers |
| [025-html-sanitization-sinks-sweep.md](./025-html-sanitization-sinks-sweep.md) | Verify every `{@html}` sink has explicit sanitizer coverage and a documented trust boundary | Frontend/security reviewers |
| [026-auth-security-alerting-sweep.md](./026-auth-security-alerting-sweep.md) | Verify failed-login and lockout alerting uses shared Underlay thresholds, dedupe, persistence, and operator outputs | Backend/security reviewers |
| [028-layout-foundation-and-detail-page-css-sweep.md](./028-layout-foundation-and-detail-page-css-sweep.md) | Keep layout behavior in shared Underlay primitives and remove per-page CSS hacks from admin detail pages | Frontend/platform reviewers |

## How to run a sweep

1. Choose the sweep document.
2. Set project-specific repo paths (API, admin, web, client).
3. Run each command/check in order.
4. Record findings using the report template in that sweep.
5. Track remediation work in your project roadmap or issue tracker.

## Authoring new sweeps

When adding a new sweep:

1. Add a new numbered file in this folder (for example `002-accessibility-sweep.md`).
2. Keep commands runnable and copy/paste ready.
3. Distinguish "pass criteria" from "manual review required" cases.
4. Link to relevant Underlay guides/patterns for remediation.
5. Add the new sweep to the catalogue table above.
