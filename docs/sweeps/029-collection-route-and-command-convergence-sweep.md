# 029 - Collection Route and Command Convergence Sweep

This sweep verifies that resource-backed collection consumers converge on:

- one canonical route family per resource
- one shared query/profile vocabulary
- an intentional command-wrapper posture

It is the practical audit companion to:

- [073-api-profiles-and-query-contract.md](../guides/073-api-profiles-and-query-contract.md)
- [116-canonical-collection-routes-and-query-profiles.md](../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Problem This Sweep Targets

After route naming is cleaned up, a second kind of drift remains:

- duplicate client commands that still hide different route families
- selector-specific or page-specific helper endpoints for the same resource
- broad compatibility facades that flatten the same resource route into
  multiple app-local shapes
- wrappers that are harmless in isolation but accumulate into unclear API
  posture

The point of this sweep is to separate:

- **good thin wrappers**
- **acceptable temporary compatibility wrappers**
- **real route/query drift**

Do not assume every `*ForList*` / `*ForFilter*` pair is bad. The route family
is the hard boundary. Wrapper names are a softer judgment call.

## Policy Baseline

See:

- [116-canonical-collection-routes-and-query-profiles.md](../contracts/116-canonical-collection-routes-and-query-profiles.md)

### Hard rules

- one canonical route family per resource collection
- `profile` varies projection, not route identity
- selector and page consumers for the same resource should normally hit the
  same route family
- route/query divergence is drift

### Soft rules

- thin typed wrappers like `listModulesForListAdmin()` and
  `listModulesForFilterAdmin()` are acceptable when they:
  - hit the same route family
  - use the same query vocabulary
  - improve DTO typing or caller intent
- broad compatibility facades are suspect when they:
  - flatten different envelopes inconsistently
  - preserve old query vocabulary
  - hide multiple route families behind one friendly name

## Scope

Run across:

- API routes
- TS client commands
- frontend callsites where those commands are used

```bash
export API_REPO="/path/to/app-api"
export CLIENT_REPO="/path/to/app-client"
export ADMIN_REPO="/path/to/app-admin"
export WEB_REPO="/path/to/app-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

## Step 1 - Inventory resource collection command families

```bash
rg -n "ForListAdmin|ForFilterAdmin|PageAdmin|profile=\\\"list\\\"|profile=\\\"filter\\\"" "$CLIENT_REPO/src/commands" --type ts
```

Build a table for each resource family:

- resource
- route path
- command names
- profile usage
- response envelope
- primary callers

Pass criteria:

- every resource family is grouped by canonical route, not by command name alone

## Step 2 - Detect route-family duplication

```bash
rg -n "\"/v1/.+(for-list|for-filter|selector|dropdown|paginated)\"" "$CLIENT_REPO/src/commands" --type ts
rg -n 'path\\s*=\\s*"/v1/.+(for-list|for-filter|selector|dropdown|paginated)"' "$API_REPO/crates/api/src/routes" --type rust
```

Pass criteria:

- no resource family uses helper-specific path variants when a canonical route
  already exists

## Step 3 - Classify wrapper quality

For each `*ForList*` / `*ForFilter*` or similar pair, classify it as one of:

### A. Good thin wrapper

All are true:

- both wrappers hit the same canonical route family
- difference is only `profile` and DTO typing
- query vocabulary stays aligned
- no extra flattening or compatibility remapping beyond `response.data`

### B. Acceptable temporary compatibility wrapper

Mostly good, but still carrying one temporary compromise:

- older caller naming
- temporary alias to a newer canonical command
- envelope adaptation during an active migration
- a legacy cursor/list profile that still exists beside the preferred page
  profile on the same canonical route family

Must have a plausible cleanup path.

### C. Real drift

Any of these are true:

- wrappers hit different routes for the same resource family
- one wrapper uses old query terms or cursor/page semantics that do not match
  the canonical route posture
- one wrapper flattens a page-shaped route into a second public shape for no
  good reason
- one compatibility facade duplicates a newer explicit command module and hides
  the real contract

## Step 4 - Cross-check frontend dependence

```bash
rg -n "ForListAdmin|ForFilterAdmin|PageAdmin" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

For each drift candidate, verify whether:

- callers genuinely benefit from separate typed wrappers
- callers would be clearer on one canonical command plus `profile`
- the wrapper is only kept alive for historical reasons

Pass criteria:

- wrapper status is justified by caller needs, not inertia

## Step 5 - Record findings

Use this rubric:

- `good-wrapper`
- `temp-wrapper`
- `drift`

## Reference Classification: Cattle Grid

### Good thin wrappers

- `learning/modules.ts`
  - `listModulesForListAdmin(...)`
  - `listModulesForFilterAdmin(...)`
  - same `/v1/admin/learning/modules` route
  - same filter/search query vocabulary
  - different DTOs and typed return shapes

- `learning/pathways.ts`
  - `listPathwaysForListAdmin(...)`
  - `listPathwaysForFilterAdmin(...)`
  - same `/v1/admin/learning/pathways` route
  - clean `profile=list|filter` split

- `learning/sections.ts`
  - `listSectionsPageAdmin(...)`
  - `listSectionsForListAdmin(...)` as a thin unwrap wrapper
  - `listSectionsForFilterAdmin(...)`
  - same `/v1/admin/learning/sections` route family

- `learning/areas.ts`
  - same pattern as sections

- `learning/outcomes.ts`
  - `listOutcomesPageAdmin(...)`
  - `listOutcomesForListAdmin(...)`
  - `listOutcomesForFilterAdmin(...)`
  - same `/v1/admin/learning/outcomes` route family

### Acceptable temporary compatibility wrappers

- `exams-commands.ts`
  - route family is already converged:
    - `/v1/admin/exams/schedules`
    - `/v1/admin/exams/editions`
  - `profile=filter` is clean and canonical
  - the remaining split is between:
    - older cursor-style `profile=list`
    - newer page-shaped `profile=page`
  - client wrappers currently mirror that backend compatibility debt with:
    - `ExamsListResponse<T>`
    - `PagedListResponse<T>`
  - this is not route drift, but it is not the target end state either

### Drift or cleanup targets

- `learning-commands.ts`
  - broad compatibility facade
  - the real public namespace still points at this file in `@cattle-grid`
  - primary implementation owners already live under `commands/learning/*`
  - stale type ownership can and should move off this file first
  - the barrel can remain as stable namespace scaffolding, but it should stop
    pretending to be the primary owner surface
  - `createLearningCommands(...)` is acceptable test-helper scaffolding, not a
    second public owner model

- `content-commands.ts`
  - same basic pattern as `learning-commands.ts`
  - real owners already live under `commands/content/*`
  - the barrel is acceptable as grouped namespace scaffolding
  - `createContentCommands(...)` is test-helper compatibility scaffolding, not
    the target public owner surface

- `exams-commands.ts`
  - not the same cleanup category yet
  - there is no split `commands/exams/*` owner set behind it today
  - the real issue is the explicit list/page compatibility split on one
    canonical route family, not a fake barrel facade

#### Exams split prerequisites

Do not split `exams-commands.ts` on file-shape instinct alone.

The clean split point needs these first:

1. backend convergence is explicit
   - either retire cursor-style `profile=list`
   - or document a durable long-term owner split between:
     - compatibility list/cursor consumers
     - page-shaped admin consumers
2. client envelope posture is reduced
   - retire `ExamsListResponse<T>` where possible
   - stop new callers from depending on `offset -> cursor` compatibility
3. page-shell and tab callers are already on the preferred path
   - `PageAdmin` usage should be the norm for page-shaped surfaces
4. only then split by owner concern
   - schedules
   - editions
   - documents
   - shared compatibility helpers

Before that point, splitting the file would mostly rearrange compatibility debt.

### Current exams caller classification

Page-shaped callers already on the preferred path:

- Dairy root pages:
  - exam schedules
  - exam editions
  - mock exams

Current compatibility holdouts:

- shared tab/list components that still use cursor/list posture:
  - `ExamSchedulesList.svelte`
  - `ExamEditionsList.svelte`
- those are reused under pathway/module/detail tabs, so they still have a real
  reason to sit on the compatibility path for now
- they should be treated as tab-only compatibility components, not mixed
  page/tab loaders

### Summary judgment

`cattle-grid` proves the important point:

- separate `ForList` / `ForFilter` wrappers can be perfectly fine
- not every mixed command surface is the same problem:
  - exams are temporary backend/client compatibility posture
  - `learning-commands.ts` is broader facade drift
- the real smell is duplicated route families or client facades that hide the
  real contract longer than needed

## Correction Playbook

When drift is found:

1. normalize the API onto one canonical route family
2. normalize query vocabulary and profile usage
3. keep thin typed wrappers if they still add clarity
4. remove compatibility facades or envelope forks that no longer add value
5. only then consider collapsing wrapper names if the result is actually
   clearer

## Severity Rubric

- `high`: same resource family still split across multiple route families
- `medium`: one canonical route exists, but command/envelope drift still hides
  the contract
- `low`: wrapper naming clutter with otherwise clean route/query convergence
- `note`: acceptable wrapper pattern worth preserving

## Findings Template

```md
### [CLASS] <resource family>

- **Client file:** `src/commands/...`
- **API route family:** `/v1/...`
- **Wrapper posture:**
- **Why this classification:**
- **Caller impact:**
- **Fix plan:** keep / thin-wrapper cleanup / route convergence / facade removal
```

## Related Docs

- [073-api-profiles-and-query-contract.md](../guides/073-api-profiles-and-query-contract.md)
- [080-typescript-client.md](../guides/080-typescript-client.md)
- [100-frontend-web.md](../guides/100-frontend-web.md)
- [116-canonical-collection-routes-and-query-profiles.md](../contracts/116-canonical-collection-routes-and-query-profiles.md)
