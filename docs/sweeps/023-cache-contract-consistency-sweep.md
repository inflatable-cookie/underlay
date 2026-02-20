# 023 - Cache Contract Consistency Sweep

This sweep checks whether API routes follow the Underlay caching/freshness contract.

Focus areas:

- validator headers (`ETag`),
- conditional request handling (`If-None-Match` -> `304`),
- optimistic concurrency (`If-Match` -> `412`),
- cache directive consistency by audience.

## Problem this sweep targets

Teams may adopt caching piecemeal, leading to mixed behavior across equivalent endpoints:

- some admin routes emit validators, others do not,
- conditional requests are partially wired,
- concurrent edit safety is inconsistent,
- cache directives drift from policy.

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

## Step 1 - Detect validator/conditional handling in API routes

```bash
rg -n "etag|if-none-match|not_modified|304" "$API_REPO" --glob '!target/**'
```

Review:

- Are eligible admin GET handlers covered?
- Is route-level handling centralized or duplicated ad hoc?

## Step 2 - Detect optimistic concurrency handling on writes

```bash
rg -n "if-match|precondition failed|412" "$API_REPO" --glob '!target/**'
```

Review:

- Do conflict-prone mutation routes enforce preconditions?
- Is conflict response shape consistent?

## Step 3 - Detect cache-control policy drift

```bash
rg -n "cache-control|no-store|must-revalidate|s-maxage|stale-while-revalidate" "$API_REPO" --glob '!target/**'
```

Review:

- Admin defaults should match policy unless documented exception.
- Public cache directives should not leak into admin routes unintentionally.

## Step 4 - Spot client-side conditional request usage

```bash
rg -n "If-None-Match|If-Match|ETag" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" --glob '!node_modules/**'
```

Review:

- Do high-frequency admin reads revalidate using validators?
- Is `412` handling implemented for edit flows where required?

## Step 5 - Capture findings

Report template:

```md
## Cache Contract Findings

### Missing validators
- Route:
- Location:
- Impact:
- Suggested fix:

### Missing conditional handling
- Route:
- Location:
- Impact:
- Suggested fix:

### Missing write preconditions
- Route:
- Location:
- Impact:
- Suggested fix:

### Cache-Control drift
- Route:
- Current header policy:
- Expected policy:
- Suggested fix:
```

## Pass criteria

- Eligible admin GET routes emit validators.
- Conditional request handling is present and consistent.
- Conflict-prone mutations use `If-Match`/`412` semantics.
- Cache-Control directives match documented policy by route audience.
