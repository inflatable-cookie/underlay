# 011 - Migration Safety Sweep

This sweep checks whether database migrations are safe, repeatable, and aligned with Underlay standards.

## Problem this sweep targets

Common migration regressions:

- use of `SET search_path` in migration SQL
- unqualified table references causing environment-dependent behavior
- non-idempotent DDL/data backfills that fail on retries
- risky rollouts mixing schema and behavior changes without compatibility window

## Scope

```bash
export API_REPO="/path/to/myapp-api"
```

Acowtancy mapping: `farmyard`.

---

## Step 1 - Search path and schema qualification compliance

### 1.1 Forbid `SET search_path`

```bash
rg -n "SET\s+search_path" "$API_REPO/migrations" "$API_REPO/migrations_dev" "$API_REPO/crates"
```

### 1.2 Spot unqualified object references

```bash
rg -n "\b(FROM|JOIN|UPDATE|INTO|TABLE|REFERENCES)\s+[a-z_][a-z0-9_]*\b" "$API_REPO/migrations" -g "*.sql"
```

Manual review:

- ensure intended objects are schema-qualified (`content.qa_item`, `learning.module`, etc.)

Pass criteria:

- no executable migration uses `SET search_path`
- object references are schema-qualified unless intentionally global

---

## Step 2 - Idempotency and rerun safety

```bash
rg -n "CREATE TABLE|CREATE INDEX|ALTER TABLE|DROP|INSERT INTO|UPDATE" "$API_REPO/migrations" -g "*.sql"
```

Review for safety constructs where appropriate:

- `IF NOT EXISTS` / `IF EXISTS`
- guarded backfills
- deterministic data updates

Pass criteria:

- migration behavior is predictable on retries or partial runs
- data backfills are bounded and safe

---

## Step 3 - Backfill and locking risk review

```bash
rg -n "UPDATE .* SET|DELETE FROM|CREATE INDEX|ALTER TABLE .* ADD COLUMN .* NOT NULL" "$API_REPO/migrations" -g "*.sql"
```

Check for:

- large-table updates without batching/guarding
- `NOT NULL` additions without staged default/backfill strategy
- lock-heavy operations during peak windows

Pass criteria:

- potentially expensive operations include staged rollout plan
- migration comments/docs describe operational expectations where needed

---

## Step 4 - API compatibility window

When schema changes affect API contracts, verify rollout sequence:

1. additive schema changes first
2. dual-read/dual-write app compatibility if required
3. backfill
4. remove deprecated columns/paths in a later step

Checks:

```bash
rg -n "deprecated|legacy|backfill|rollout|compat" "$API_REPO/crates" "$API_REPO/migrations" -g "*.rs" -g "*.sql"
```

Pass criteria:

- no breaking schema change lands without app compatibility strategy

---

## Step 5 - Naming and ordering conventions

```bash
ls "$API_REPO/migrations"
```

Review for:

- stable chronological naming
- descriptive migration names
- no out-of-order insertion without clear reason

Pass criteria:

- migration history remains readable and deterministic

---

## Step 6 - Local migration verification

Run migration flow in clean local/dev DB:

1. fresh apply all migrations
2. apply latest migration on near-current DB state
3. verify app starts and key smoke tests pass

Record elapsed time and lock-sensitive operations.

---

## Correction playbook

When findings are present:

1. remove `search_path` and schema-qualify references
2. split risky migration into staged steps
3. add backfill guards and compatibility windows
4. document operational runbook for heavy migrations
5. add migration verification notes to PR description

---

## Severity rubric

- `critical`: migration can corrupt data or cause prolonged outage
- `high`: production rollout risk from lock-heavy or incompatible change
- `medium`: reliability/idempotency gap likely to cause deploy friction
- `low`: convention/hygiene issue
- `note`: improvement suggestion

---

## Findings template

```md
### [SEVERITY] Migration safety issue - <migration file>

- **Location:** `migrations/...sql`
- **Observed risk:**
- **Expected pattern:**
- **Operational impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Migration safety sweep summary

- Migrations audited: N
- Search path violations: N
- Qualification gaps: N
- Rollout-risk issues: N
```

---

## Related docs

- [050-database.md](../guides/050-database.md)
- [120-configuration.md](../guides/120-configuration.md)
- [001-security-sweep.md](./001-security-sweep.md)
