# 039 - Storage Expiration Support

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add optional TTL and expiration support to the SSR-safe storage wrappers in `patterns/storage.ts` so consuming apps can store short-lived client state without hand-rolled cleanup logic.

## Research Basis

- `docs/roadmaps/backlog/storage-expiration.md`
- `docs/guides/100-frontend-web.md`
- `ts/src/patterns/storage.ts`

## Likely Implementation Surface

- `ts/src/patterns/storage.ts`
- `ts/tests/patterns/storage.test.ts`
- `docs/guides/100-frontend-web.md`

## Phase 39.1 - Expiration Model

- [x] Add optional `ttl` / `expiresAt` support to storage set and store operations.
- [x] Keep existing stored payloads readable so upgrades do not invalidate current client state.
- [x] Add explicit expiration inspection semantics for consumers that need to probe stale keys.

## Phase 39.2 - Reactive Store Expiry

- [x] Make storage-backed Svelte stores reset to their default values when the stored item expires.
- [x] Keep browser-only timers SSR-safe and avoid changing behavior for non-expiring stores.
- [x] Keep cross-tab sync behavior aligned with the expiration envelope.

## Phase 39.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/100-frontend-web.md` with TTL and expiration examples.
- [x] Document the wire format and behavior boundary clearly enough that consuming apps know existing raw values still work.

## Deferred

- Background scavenging of expired keys.
- Storage quota management and eviction policies.
- Compression or envelope migration beyond the first metadata version.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing raw storage values must continue to deserialize without migration work.
- Expiration must stay opt-in per key; existing `set()` and `store()` calls should keep their current persistence behavior.
- Upgrade guidance must call out that expired values are removed lazily on access and actively reset only for stores created in the current page session.

## Validation

```bash
bun x vitest run ts/tests/patterns/storage.test.ts
effigy validate
```

## Completion

Current active roadmap set is complete. Promote the next backlog item into `g01` only when the next reusable batch is ready for execution.
