# Backlog: Storage Expiration Support

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 2-3 hours  
**Source**: Deferred from roadmap 009 (Quick Wins)

---

## Problem Statement

The SSR-safe storage wrappers in `patterns/storage.ts` currently lack expiration support. This means:

- No automatic cleanup of stale cached data
- Manual expiration logic duplicated across consuming apps
- Session-like data (e.g., "remember for 24 hours") requires custom handling
- No way to invalidate cached API responses after TTL

---

## Proposed Solution

Add optional TTL (time-to-live) support to the storage API:

### 1. TTL on Set

```typescript
import { storage } from '@inflatable-cookie/underlay/patterns';

// Store with 1 hour TTL
storage.local.set('api-cache', data, { ttl: 3600 });

// Store with expiration date
storage.local.set('session-token', token, { 
  expiresAt: new Date('2026-01-13T00:00:00Z') 
});
```

### 2. Automatic Expiration Check

```typescript
// Returns undefined if expired (and removes the key)
const cached = storage.local.get('api-cache');

// Explicit check
if (storage.local.isExpired('api-cache')) {
  // refetch data
}
```

### 3. Reactive Store with TTL

```typescript
// Store automatically becomes undefined when expired
const $token = storage.local.store('session-token', null, { 
  ttl: 86400 // 24 hours
});
```

### 4. Implementation Details

Storage format with metadata:
```typescript
interface StoredValue<T> {
  value: T;
  expiresAt?: number; // Unix timestamp in ms
  version?: number;   // For future migration support
}
```

---

## Dependencies

- Existing `patterns/storage.ts`
- No external dependencies

---

## Success Criteria

- [ ] TTL option available on `set()` and `store()`
- [ ] Expired values return `undefined` (or default)
- [ ] Expired keys cleaned up on access
- [ ] Reactive stores update when values expire
- [ ] Backwards compatible with existing stored values
- [ ] No performance regression for non-TTL usage

---

## Risks & Considerations

- **Storage format change**: Need migration path for existing values
- **Timer management**: Reactive expiration requires `setTimeout`
- **SSR safety**: Timers shouldn't run during SSR
- **Cleanup strategy**: Active cleanup vs lazy cleanup on access

---

## Future Enhancements

- Background cleanup of expired keys
- Storage quota management
- LRU eviction when quota exceeded
- Compression for large values

---

## Related

- `ts/src/patterns/storage.ts` - Current implementation
- `docs/guides/100-frontend-bloom.md` - Storage documentation

---

**Created**: 2026-01-12
