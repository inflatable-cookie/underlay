# 074 - HTTP Caching and Freshness Contract

This guide defines the canonical caching and freshness strategy for Underlay-consuming APIs and clients.

It is optimized for admin correctness first, with optional targeted load reduction where safe.

## Policy Summary

- Admin APIs must prioritize correctness and concurrent edit safety.
- Use validator-based HTTP caching (`ETag` + conditional requests) as the baseline.
- Use optimistic concurrency (`If-Match`) for mutation safety where resource versioning exists.
- Use short-lived microcache only as an opt-in for known hot read paths.

## Audience Profiles

### Admin APIs

Default policy:

- `Cache-Control: private, no-cache, must-revalidate`
- emit `ETag`
- support `If-None-Match` (return `304` when unchanged)
- use `If-Match` on write endpoints where version semantics are available

Sensitive admin endpoints may use:

- `Cache-Control: no-store`

### Public/front APIs

Use stronger cache directives where product requirements allow (for example edge/browser caching with explicit TTL/SWR policy).

## Why Validator-First

Validator-first caching:

- prevents stale long-lived admin state,
- reduces response payload transfer,
- aligns with correctness expectations for shared back-office data.

Note: validator-only does not eliminate all DB work. For DB pressure relief, combine with short-horizon request dedupe and endpoint-level microcache.

## Required Server Contract

## 1) `ETag` on eligible GET routes

- `ETag` must represent the response version for the route/profile/filter shape.
- For list endpoints, include query-affecting dimensions in key derivation.
- Prefer stable weak validators unless strong validators are required.

## 2) Conditional GET support

- If request includes `If-None-Match` matching current validator, return:
  - `304 Not Modified`
  - no response body
  - relevant cache headers

## 3) Optimistic concurrency for writes

Where resources have version semantics:

- Mutations accept `If-Match`
- mismatch returns `412 Precondition Failed`
- return a consistent conflict payload shape for client UX handling

## 4) Cache directive consistency

- Admin default: `private, no-cache, must-revalidate`
- Explicitly document route-level exceptions (`no-store`, public cacheable endpoints, etc.)

## Optional DB-Load Controls

## A) Single-flight dedupe

Within process, coalesce concurrent identical read requests so only one backend query executes.

Underlay primitive: `underlay_http::SingleFlight` (keyed async coordinator for identical in-flight loads).

Use when:

- repeated bursts of identical reads are observed,
- endpoint is high traffic and response assembly is expensive.

## B) Bounded microcache (opt-in)

Use tiny TTL microcache (typically 1-3 seconds) for hot admin reads.

Guardrails:

- opt-in per endpoint/profile
- mutation-triggered invalidation for affected keys/tags
- narrow cache key dimensions (tenant/scope/resource/profile/query)
- avoid blanket global enablement

## Client Expectations (Admin)

- Deduplicate in-flight identical requests at client query layer.
- Use short in-memory reuse windows only when paired with quick revalidation.
- On `412`, show conflict UX and reload canonical latest server state before retry.

### TypeScript client pattern (`@inflatable-cookie/underlay/client/http`)

Admin clients should use conditional GET with response metadata access:

- call `getWithMeta(...)` with `acceptedStatuses: [304]`
- send cached validator via `If-None-Match`
- on `304`, reuse cached payload
- on `200`, replace payload and validator (`ETag`)

This keeps payload transfer low while preserving freshness semantics and enabling tiny short-lived memory caches at the query layer.

For shared command clients, prefer a thin utility wrapper (for example `getWithAdminEtagRevalidation` / `getWithAdminEtagRevalidationWithMeta`) so each command does not reimplement `304` handling, fallback behavior, ETag storage logic, and in-flight request dedupe.

## Decision Matrix

Use validator-only when:

- endpoint is correctness-sensitive,
- repeated burst volume is low/moderate,
- payload transfer is the main waste.

Use validator + single-flight when:

- many concurrent identical reads happen in small windows.

Use validator + single-flight + microcache when:

- endpoint remains hot after client dedupe,
- measured DB load justifies temporary cache windows,
- invalidation scope is clear and maintainable.

Use `no-store` when:

- data should never be persisted by intermediate/client caches.

## Rollout Sequence

1. Add validators and conditional GET handling.
2. Add write preconditions (`If-Match`) for conflict-prone edits.
3. Add single-flight for selected hotspots.
4. Add endpoint-level microcache where metrics justify it.
5. Enforce with sweeps.

## Verification Checklist

- Eligible admin GET routes emit `ETag`.
- Eligible routes support `If-None-Match` -> `304`.
- Conflict-prone writes enforce `If-Match` -> `412`.
- Cache-Control policy matches route audience and sensitivity.
- Hotspot endpoints are explicitly documented if microcache-enabled.

## Suggested Telemetry

To validate impact in live environments, emit counters for:

- conditional cache revalidation hits (`304`)
- microcache hits on hot admin reads
- DB-backed fetch executions on hot admin reads (for query-pressure trend)
- stale-write precondition rejections (`412`)

Example metric shapes:

- `*_admin_cache_events_total{endpoint,event}` where `event` includes `conditional_304`, `microcache_hit`, `db_fetch`
- `*_admin_write_precondition_total{endpoint,event}` where `event` includes `precondition_failed`

Interpretation note:

- for microcached endpoints, `db_fetch` trend over time is a practical proxy for backend query pressure; compare it against `microcache_hit` and `conditional_304` to validate load reduction.

These metrics complement static sweep checks and make before/after trend analysis possible during rollout.

## Related Docs

- [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md)
- [080-typescript-client.md](./080-typescript-client.md)
- [098-shared-admin-patterns.md](./098-shared-admin-patterns.md)
- [110-admin.md](./110-admin.md)
- [023-cache-contract-consistency-sweep.md](../sweeps/023-cache-contract-consistency-sweep.md)
- [024-admin-fetch-and-caching-pressure-sweep.md](../sweeps/024-admin-fetch-and-caching-pressure-sweep.md)
