# 024 - Admin Fetch and Caching Pressure Sweep

This sweep identifies high-churn admin fetch behavior and verifies dedupe/caching controls are applied proportionately.

## Problem this sweep targets

Admin UIs often trigger repeated identical reads in short windows:

- route remount/refetch loops,
- sibling components calling the same resource,
- missing in-flight dedupe,
- no short-horizon server protections on known hotspot routes.

Result: avoidable API and DB load.

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`.

## Step 1 - Find repeated-fetch trigger patterns in admin UI

```bash
rg -n "tryFetch\(|refetch\(|invalidate\(|onMount\(" "$ADMIN_REPO/src" --glob '*.svelte'
```

Review likely duplicates:

- effects that trigger unconditionally on mount/navigation,
- duplicate calls across sibling tabs/components,
- calls lacking guard conditions.

## Step 2 - Verify request-level dedupe usage

```bash
rg -n "dedupeInFlight|singleFlight|inFlight|getWithAdminEtagRevalidation" "$ADMIN_REPO/src" "$CLIENT_REPO/src" --glob '!node_modules/**'
```

Review:

- hot list/detail callsites should have in-flight dedupe or shared data hoisting.

## Step 3 - Identify backend single-flight/microcache controls

```bash
rg -n "single[-_ ]?flight|microcache|cache key|invalidate" "$API_REPO" --glob '!target/**'
```

Review:

- Are hotspot routes protected?
- Are invalidation hooks wired for relevant writes?

## Step 4 - Check candidate hotspots against endpoint list

Maintain an explicit endpoint table in your roadmap/report:

- endpoint
- read churn symptom
- selected strategy (validator-only / +single-flight / +microcache)
- invalidation trigger(s)

## Step 5 - Capture findings

Report template:

```md
## Admin Fetch/Caching Pressure Findings

### Client duplicate-fetch hotspots
- Route/component:
- Trigger pattern:
- Suggested fix:

### Missing in-flight dedupe
- Callsite:
- Current behavior:
- Suggested fix:

### Backend hotspot missing protection
- Endpoint:
- Suggested strategy:
- Invalidation notes:
```

## Pass criteria

- Hot admin routes are identified and documented.
- Client duplicate-fetch hotspots have dedupe or guard remediation.
- Backend hotspot routes have explicit chosen strategy.
- No uncontrolled microcache sprawl (endpoint opt-in list is explicit).
