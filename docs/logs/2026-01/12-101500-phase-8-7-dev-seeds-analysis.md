# Phase 8.7 Analysis: Dev Seeds CLI

**Date**: 2026-01-12  
**Status**: Analysis Complete  
**Recommendation**: **DEFER** - Pattern is too app-specific; low ROI

## Executive Summary

After examining Farmyard's `migrations_dev/` pattern, **this is highly app-specific test data** that doesn't warrant generic tooling in Underlay. The pattern is simple enough to document without adding CLI commands.

**Recommendation**: **Defer Phase 8.7** - Document the pattern in guides, but don't add CLI tooling.

---

## What Exists in Acowtancy

### Farmyard migrations_dev/

**Location**: `farmyard/migrations_dev/`

**Files**:
- `202512051300__seed_learning_modules.sql` (12KB) - Pathways, modules, areas
- `202512071510__seed_learning_activities.sql` (2.6KB) - Learning activities
- `202601091620__seed_tom_admin.sql` (1.2KB) - Test admin user

**Pattern**:
```sql
-- Standard SQL INSERT statements
INSERT INTO learning.pathway (id, name, slug, year, title, ...)
VALUES
  ('018f2a3b-3c4d-7e8f-8a9b-000000000001'::uuid, 'ACCA', 'acca', 2016, ...),
  ('018f2a3b-3c4d-7e8f-8a9b-000000000002'::uuid, 'CIMA', 'cima', 2023, ...)
ON CONFLICT (slug) DO NOTHING;
```

**Naming Convention**: `YYYYMMDDHHmm__description.sql`

**Usage**:
```bash
# Manually run dev seeds
psql $DATABASE_URL -f migrations_dev/202512051300__seed_learning_modules.sql
```

---

## Analysis

### What is migrations_dev/?

**Purpose**: Development-only test data separate from migrations

**Key Characteristics**:
1. **Not versioned like migrations** - Can be re-run, modified freely
2. **Idempotent** - Uses `ON CONFLICT DO NOTHING` or similar
3. **Domain-specific** - Tailored to each app's schema and needs
4. **Git-tracked** - Shared across dev team

**vs Migrations**:
| Aspect | migrations/ | migrations_dev/ |
|--------|-------------|-----------------|
| **Purpose** | Schema changes | Test data |
| **Run automatically** | Yes (on startup/deploy) | No (manual) |
| **Reversible** | Yes (with down migrations) | No |
| **Production** | Required | Never used |
| **Idempotent** | Should be | Must be |

---

## Proposed CLI Command (Original Plan)

**Command**:
```bash
underlay-devtools init-dev-seeds --target ./migrations_dev
```

**Would create**:
```
migrations_dev/
├── .gitignore              (empty - keep all files)
├── README.md               (explains pattern)
└── example__seed_users.sql (template)
```

---

## Why Defer?

### 1. Too App-Specific

**Problem**: Seed data is inherently domain-specific

**Examples**:
- Farmyard seeds: `learning.pathway`, `learning.module`, `learning.activity`
- Different app seeds: `products`, `categories`, `inventory`
- Each app's schema is unique

**Generic template would be**:
```sql
-- TODO: Add your domain-specific seed data here
INSERT INTO your_schema.your_table (columns...)
VALUES (...);
```

This provides **zero value** over just documenting the pattern.

### 2. Minimal CLI Value

**What the CLI would do**:
1. Create `migrations_dev/` directory
2. Add README.md explaining pattern
3. Add example SQL file

**Alternative**: Document the pattern in quickstart guides

**Comparison**:
| Approach | Effort | Value |
|----------|--------|-------|
| CLI command | 1-2 days to implement + maintain | Low - just creates directory + docs |
| Documentation | 1 hour to write guide | Same outcome for users |

### 3. Pattern is Already Simple

**Current workflow**:
```bash
mkdir migrations_dev
echo "# Dev Seeds" > migrations_dev/README.md
# Create seed files as needed
```

**With CLI**:
```bash
underlay-devtools init-dev-seeds --target ./migrations_dev
# Still need to create actual seed files
```

**Saved effort**: ~30 seconds  
**CLI maintenance cost**: Ongoing

### 4. underlay-devtools Focus

**Current command**: `sync-migrations`
- **Purpose**: Sync Underlay's migrations to app
- **Reusable**: Yes - all apps need Underlay migrations
- **Value**: High - automates repetitive task

**Proposed command**: `init-dev-seeds`
- **Purpose**: Create empty directory + README
- **Reusable**: No - each app's seeds are unique
- **Value**: Low - barely saves time

**Philosophy mismatch**: `underlay-devtools` should provide **Underlay-specific** tooling, not generic scaffolding.

---

## Alternative: Documentation-Only Approach

### Update Quickstart Guide 050 (Database)

**Add section**: "Development Seed Data"

```markdown
## Development Seed Data

For local development, you may want test data separate from migrations.

### Pattern: migrations_dev/

Create a `migrations_dev/` directory with SQL files:

```
migrations_dev/
├── 202601121000__seed_users.sql
├── 202601121030__seed_products.sql
└── README.md
```

### Naming Convention

`YYYYMMDDHHmm__description.sql` (same as migrations)

### Writing Seed Files

Use idempotent INSERT statements:

```sql
-- migrations_dev/202601121000__seed_users.sql

INSERT INTO auth.users (id, email, name, role)
VALUES
  ('018f2a3b-3c4d-7e8f-8a9b-000000000001'::uuid, 'admin@example.com', 'Admin User', 'admin'),
  ('018f2a3b-3c4d-7e8f-8a9b-000000000002'::uuid, 'user@example.com', 'Test User', 'user')
ON CONFLICT (email) DO NOTHING;
```

**Important**: Use `ON CONFLICT DO NOTHING` or `DO UPDATE` to make seeds rerunnable.

### Running Seeds

```bash
# Run all dev seeds
psql $DATABASE_URL -f migrations_dev/*.sql

# Run specific seed
psql $DATABASE_URL -f migrations_dev/202601121000__seed_users.sql
```

### vs Migrations

- **migrations/**: Schema changes (run automatically)
- **migrations_dev/**: Test data (run manually, dev-only)

**Git**: Commit `migrations_dev/` to share across team.

### Tips

1. **Fixed UUIDs**: Use predictable UUIDs for easy reference in tests
2. **Idempotent**: Always safe to re-run
3. **Minimal**: Only seed data you actively use
4. **Separate files**: One file per domain/schema for clarity
```

**Total effort**: 1 hour to write  
**User value**: Same as CLI approach

---

## Recommendation

### Immediate: Defer Phase 8.7

**Reason**: Low ROI - documentation provides same value as CLI

**Action**: 
1. Mark Phase 8.7 as deferred in roadmap
2. Add dev seeds section to Guide 050 (Database)
3. Note in roadmap: "Pattern documented in guides; CLI unnecessary"

### Future: When to Revisit

Only reconsider if:

1. **Generic seed framework emerges**:
   - Multiple apps share seed file structure
   - Clear abstraction for reusable seeds
   - Templating system provides real value

2. **Advanced features needed**:
   - Seed versioning/rollback
   - Seed dependencies (ordering)
   - Seed validation
   - Database-agnostic seed format

3. **User demand**:
   - Multiple teams request CLI tooling
   - Clear pain points identified

**Likelihood**: Low - seeds are inherently app-specific

---

## Summary

| Aspect | Assessment |
|--------|------------|
| **Pattern value** | ✅ Useful (separate test data from migrations) |
| **CLI value** | ❌ Low (just creates directory + README) |
| **Documentation value** | ✅ High (explains pattern clearly) |
| **Maintenance cost** | ⚠️ Medium (CLI needs ongoing support) |
| **User time saved** | ❌ Minimal (~30 seconds) |
| **Recommendation** | **DEFER** - Document, don't build CLI |

---

## Phase 8.7 Updated Status

**Priority**: P3 → **Deferred**  
**Effort**: Low (~1 hour for docs instead of 1-2 days for CLI)  
**Status**: ⏸️ **DEFERRED** - Pattern documented in guides

### Reason for Deferral

Seed data is too app-specific for generic CLI tooling. Documentation provides same value with less maintenance overhead.

### What Was Done Instead

- [x] Analyzed existing migrations_dev/ pattern
- [x] Evaluated CLI vs documentation approach
- [x] Recommend documentation-only (Guide 050 update)

### What Would Be Done (If Approved)

- [ ] Add dev seeds section to Guide 050 (Database)
- [ ] Include examples from Farmyard
- [ ] Document idempotent pattern (ON CONFLICT DO NOTHING)
- [ ] Show usage with psql

**Effort**: ~1 hour (vs 1-2 days for CLI)

---

## Lessons Learned

1. **Not everything needs CLI tooling**: Sometimes documentation is better
2. **App-specific patterns are hard to genericize**: Seed data varies too much
3. **underlay-devtools should be Underlay-specific**: Don't bloat with generic scaffolding
4. **ROI matters**: 30 seconds saved doesn't justify maintenance burden

---

## Conclusion

**Phase 8.7 should be deferred** in favor of simple documentation. The pattern is useful, but CLI tooling provides minimal value over a clear guide.

**Alternative approach** (documentation-only) achieves the same outcome with 90% less effort and zero maintenance cost.
