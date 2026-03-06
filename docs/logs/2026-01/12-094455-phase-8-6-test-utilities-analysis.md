# Phase 8.6 Analysis: Test Utilities

**Date**: 2026-01-12  
**Status**: Analysis Complete  
**Recommendation**: **DEFER** - Insufficient reusable patterns; wait for more test coverage

## Executive Summary

After analyzing Acowtancy's test patterns, **there are insufficient reusable test utilities to justify extraction at this time**. The existing test code is either:
1. **Already in Underlay** (testcontainers integration in `underlay-db`)
2. **Highly domain-specific** (Farmyard's learning graph seeding)
3. **Non-existent** (No TypeScript/frontend tests found)

**Recommendation**: **Mark Phase 8.6 as deferred** and revisit when more test coverage exists.

---

## What Exists in Acowtancy

### Rust Tests

#### 1. Farmyard DB Tests (`farmyard/crates/db/tests/learning_soft_delete.rs`)

**Lines**: 367  
**Purpose**: Integration test for soft-delete cascade and restore

**Components**:
| Function | Purpose | Reusable? |
|----------|---------|-----------|
| `test_database_url()` | Read test DB URL from env | ❌ Trivial (1 line) |
| `setup_db()` | Create pool + run migrations | ⚠️ Farmyard-specific |
| `seed_learning_graph()` | Insert test data (pathway, module, etc.) | ❌ Domain-specific |
| `assert_row_marked_deleted()` | Assert soft-delete flags set | ✅ **Potentially reusable** |
| `assert_row_restored()` | Assert soft-delete flags cleared | ✅ **Potentially reusable** |

**Analysis**:
- **Most code is domain-specific seeding** (~150 lines inserting learning.pathway, learning.module, etc.)
- Only 2 helpers are reusable: `assert_row_marked_deleted` and `assert_row_restored`
- These are ~30 lines total and only useful for soft-delete testing

#### 2. Underlay DB Tests (`underlay/rust/crates/underlay-db/tests/postgres_integration.rs`)

**Lines**: 171  
**Purpose**: Integration tests with testcontainers

**Components**:
| Function | Purpose | Status |
|----------|---------|--------|
| `docker_client()` | Create testcontainers CLI client | ✅ Already in Underlay |
| `postgres_database_url()` | Get DB URL from container | ✅ Already in Underlay |
| Tests for `create_pool`, `drop_schemas` | Various DB operations | ✅ Already in Underlay |

**Analysis**:
- **All infrastructure is already in Underlay**
- Uses `testcontainers` crate for Postgres containers
- No extraction needed

### TypeScript Tests

#### 1. Underlay Client Tests

**Status**: ❌ **No test files found**

Searched:
- `underlay/ts/src/client/` - No `.test.ts` or `.spec.ts` files
- `underlay/ts/tests/` - Directory doesn't exist

#### 2. Cattle-grid Tests

**Status**: ❌ **No test files found in src/**

Searched:
- `cattle-grid/src/` - No test files

**Note**: The original proposal mentioned extracting `FakeHttpClient` from cattle-grid tests, but **no such tests exist yet**.

#### 3. Frontend Tests (Cream/Dairy)

**Status**: ❌ **Not evaluated** (would need to search, but low priority)

---

## Proposed Extraction (Original Plan)

### Rust: `underlay-testing` Crate

**Proposed utilities**:
- `test_db_pool()` - Create test pool with migrations
- `assert_row_exists()` - Generic row assertion
- `assert_row_deleted()` - Soft delete assertion
- `assert_row_restored()` - Restore assertion

**Reality Check**:
| Utility | Status | Notes |
|---------|--------|-------|
| `test_db_pool()` | ⚠️ Partially exists | `underlay-db` already has `create_pool` + testcontainers setup |
| `assert_row_exists()` | ❌ Doesn't exist | Not found in Farmyard tests |
| `assert_row_deleted()` | ✅ Exists | In `learning_soft_delete.rs` (~15 lines) |
| `assert_row_restored()` | ✅ Exists | In `learning_soft_delete.rs` (~15 lines) |

**Total extractable code**: ~30 lines (2 soft-delete assertion helpers)

**Problem**: Not enough value to justify a new crate for 30 lines of code.

### TypeScript: Test Utilities

**Proposed utilities**:
- `FakeHttpClient` - Mock HTTP client
- `expectCall()` - Assert HTTP call
- `expectNoCalls()` - Assert no calls
- `resetCalls()` - Clear history

**Reality Check**: ❌ **None of these exist** in Acowtancy codebase

---

## Why Defer Phase 8.6?

### 1. Insufficient Reusable Patterns

**Rust**:
- Only ~30 lines of reusable soft-delete assertions
- Most test code is domain-specific (learning graph seeding)
- Testcontainers integration already in `underlay-db`

**TypeScript**:
- **Zero test files found** in Underlay client
- **Zero test utilities** in cattle-grid
- Nothing to extract

### 2. Premature Extraction

Creating test utilities before having sufficient test coverage leads to:
- **Over-engineered helpers** that don't match real needs
- **Unused utilities** that accumulate technical debt
- **Churn** as patterns evolve during actual testing

**Better approach**: Write more tests first, identify common patterns, then extract.

### 3. Low ROI

**Estimated effort**: 3-4 days (original)  
**Actual extractable value**: ~30 lines of Rust code + 0 lines of TypeScript

**Cost/benefit**: Not worth the overhead of:
- New crate setup
- Documentation
- Maintenance
- Versioning

---

## Recommendation

### Immediate: Mark Phase 8.6 as Deferred

**Reason**: Insufficient patterns to extract

**Action**: Update roadmap to mark Phase 8.6 as deferred, with note:
> Deferred until more test coverage exists. Revisit after writing tests for:
> - Underlay HTTP client (TypeScript)
> - Underlay auth hooks (TypeScript)
> - Additional domain tests in Farmyard/Cream/Dairy

### Future: When to Revisit

Reconsider Phase 8.6 when:

1. **TypeScript test coverage exists**:
   - Tests for `ts/src/client/http.ts` (HTTP client)
   - Tests for `ts/src/client/sveltekit.ts` (auth hooks)
   - Tests for `ts/src/patterns/` (UI patterns)

2. **Common patterns emerge**:
   - Mock factories repeated across 3+ test files
   - Assertion helpers used in multiple places
   - Setup/teardown patterns duplicated

3. **Clear value proposition**:
   - At least 100+ lines of reusable code identified
   - Helpers used in at least 5+ test files
   - Reduction in test boilerplate of 30%+

---

## Alternative: Soft-Delete Assertions in `underlay-soft-delete`

**Observation**: The only reusable patterns found are soft-delete assertions.

**Option**: Add test helpers to existing `underlay-soft-delete` crate instead of new `underlay-testing` crate.

**File**: `underlay/rust/crates/underlay-soft-delete/src/testing.rs`

```rust
#[cfg(test)]
pub mod testing {
    use sqlx::PgPool;
    use uuid::Uuid;
    use sqlx::Row;

    /// Assert that a row in the given table is marked as soft-deleted.
    pub async fn assert_row_deleted(pool: &PgPool, table: &str, id: Uuid, batch_id: Uuid) {
        let sql = format!("SELECT deleted_at, delete_batch_id FROM {} WHERE id = $1", table);
        let row = sqlx::query(&sql).bind(id).fetch_one(pool).await
            .unwrap_or_else(|_| panic!("failed to fetch row from {}", table));

        let deleted_at: Option<chrono::DateTime<chrono::Utc>> = row.get("deleted_at");
        let batch: Option<Uuid> = row.get("delete_batch_id");

        assert!(deleted_at.is_some(), "expected {} row to have deleted_at set", table);
        assert_eq!(batch, Some(batch_id), "expected {} row delete_batch_id to match", table);
    }

    /// Assert that a row in the given table has been restored (soft-delete flags cleared).
    pub async fn assert_row_restored(pool: &PgPool, table: &str, id: Uuid) {
        let sql = format!("SELECT deleted_at, delete_batch_id FROM {} WHERE id = $1", table);
        let row = sqlx::query(&sql).bind(id).fetch_one(pool).await
            .unwrap_or_else(|_| panic!("failed to fetch row from {}", table));

        let deleted_at: Option<chrono::DateTime<chrono::Utc>> = row.get("deleted_at");
        let batch: Option<Uuid> = row.get("delete_batch_id");

        assert!(deleted_at.is_none(), "expected {} row deleted_at cleared", table);
        assert!(batch.is_none(), "expected {} row batch cleared", table);
    }
}
```

**Pros**:
- Keeps test helpers with related functionality
- No new crate overhead
- Immediately useful for soft-delete tests

**Cons**:
- Limited to soft-delete use case
- Doesn't address broader test utility needs

**Recommendation**: ⚠️ **Optional** - Can add if we expect more soft-delete tests, but low priority.

---

## Summary

| Component | Status | Recommendation |
|-----------|--------|----------------|
| Rust test utilities | ~30 lines extractable | **Defer** - Not enough value |
| TypeScript test utilities | 0 lines exist | **Defer** - Nothing to extract |
| Soft-delete assertions | Optional add to existing crate | **Low priority** |
| Phase 8.6 overall | Insufficient patterns | **DEFER until more tests exist** |

**Next Steps**:
1. Mark Phase 8.6 as deferred in roadmap
2. Continue with Phase 8.7 (Dev Seeds CLI) or complete Phase 8
3. Revisit 8.6 after writing more tests in Acowtancy

---

## Lessons Learned

1. **Test coverage first, utilities second**: Extracting test utilities before having sufficient test coverage is premature
2. **Domain-specific vs reusable**: Most test code is inherently domain-specific (seeding, fixtures)
3. **Underlay already has infrastructure**: Testcontainers integration already exists, don't need to extract
4. **Small patterns don't justify new crates**: ~30 lines doesn't warrant a new `underlay-testing` crate

---

## Updated Phase 8.6 Roadmap Entry

**Priority**: P2  
**Effort**: N/A (deferred)  
**Status**: ⏸️ **DEFERRED**

### Reason for Deferral

Insufficient reusable test patterns exist in Acowtancy to justify extraction:
- **Rust**: Only ~30 lines of soft-delete assertions found (low ROI)
- **TypeScript**: Zero test files found in Underlay/cattle-grid (nothing to extract)
- **Infrastructure**: Testcontainers integration already exists in `underlay-db`

### When to Revisit

Reconsider when:
- TypeScript test coverage exists for client/auth/patterns
- Common patterns repeated across 5+ test files
- At least 100+ lines of reusable code identified

### Alternatives Considered

- Add soft-delete assertions to existing `underlay-soft-delete` crate (optional, low priority)
- Skip test utilities entirely (tests can use inline helpers)
