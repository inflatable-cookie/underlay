# 005 - API/Client Contract Drift Sweep

This sweep catches contract drift between Rust API DTO/routes and TypeScript client commands/types.

It is designed for Underlay-style stacks where:

- Rust API defines DTOs and envelopes
- TypeScript client wraps endpoints as typed commands
- Admin/web apps rely on client types for compile-time safety

## Problem this sweep targets

Common drift symptoms:

- API adds/renames fields but client types lag behind
- endpoint path/query names drift between routes and command functions
- response envelope shape drift (`SingleResponse`, `ListResponse`, paginated)
- snake_case payload fields not mapped predictably to camelCase usage
- client code compensates with `as any`/`as never`

## Scope

Run this across backend + client repos.

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Inventory API routes and DTOs

```bash
rg -n "#\[utoipa::path\(|path = \"/v1/" "$API_REPO/crates/api/src/routes"
rg -n "pub struct .*Dto|pub struct .*Response|serde\(rename_all = \"snake_case\"\)" "$API_REPO/crates/api/src/dto"
```

Capture for each operation:

- method + path
- expected envelope shape
- DTO type name
- required query/path/body fields

Pass criteria:

- every public endpoint has an explicit DTO/response shape
- route inventory is complete enough to diff against client commands

---

## Step 2 - Inventory client command surface

```bash
rg -n "export async function" "$CLIENT_REPO/src/commands"
rg -n "http\.(get|post|put|patch|delete)<" "$CLIENT_REPO/src/commands"
rg -n "SingleResponse|ListResponse|PaginatedResponse" "$CLIENT_REPO/src/commands" "$CLIENT_REPO/src/types"
```

Create a command map row per operation:

- command function
- HTTP method + path string
- request type
- response type

Pass criteria:

- each API route has one command or justified exception
- command names align with domain actions (`getX`, `listX`, `createX`, `updateX`, `deleteX`)

---

## Step 3 - Route-to-command parity check

### 3.1 Missing commands for existing routes

```bash
rg -n "path = \"/v1/" "$API_REPO/crates/api/src/routes" > /tmp/api-routes.txt
rg -n "\"/v1/" "$CLIENT_REPO/src/commands" > /tmp/client-routes.txt
```

Manually compare or script compare unique paths.

### 3.2 Dead commands for removed routes

Look for command paths that no longer exist in API route definitions.

Pass criteria:

- no missing command for active endpoint families consumed by apps
- no stale commands calling retired paths

---

## Step 4 - DTO/type field parity

### 4.1 Count and status fields parity

```bash
rg -n "_count|is_live|status|created_at|updated_at" "$API_REPO/crates/api/src/dto"
rg -n "Count|isLive|status|createdAt|updatedAt" "$CLIENT_REPO/src/types"
```

Focus on known drift-prone fields:

- count badges (`*_count`)
- timestamps
- status/live booleans
- enum-like fields

### 4.2 ID and UUID field parity

```bash
rg -n "_id: String|Uuid|RawUuid" "$API_REPO/crates/api/src/dto"
rg -n "Id: string|id: string" "$CLIENT_REPO/src/types"
```

Pass criteria:

- API IDs are exposed consistently as strings
- client keeps matching string ID fields with stable names

---

## Step 5 - Envelope and pagination parity

```bash
rg -n "SingleResponse|ListResponse|PaginatedResponseDto" "$API_REPO/crates/api/src"
rg -n "SingleResponse|ListResponse|PaginatedResponse" "$CLIENT_REPO/src"
```

Pass criteria:

- command return types match endpoint envelope shape
- paginated endpoints use paginated types in both API and client
- no endpoint silently switched from list to paginated (or reverse) without client update

---

## Step 6 - Query/filter key parity

```bash
rg -n "filter\[|sort|limit|offset|cursor|direction|includeTotal" "$API_REPO/crates/api/src/routes"
rg -n "filter\[|sort|limit|offset|cursor|direction|includeTotal" "$CLIENT_REPO/src/commands"
```

Pass criteria:

- command query param keys match route expectations exactly
- bracketed filter keys are preserved correctly (`filter[field]`)
- no client-only alias keys that API does not parse

---

## Step 7 - Drift smell detection

Detect temporary cast workarounds that usually indicate contract mismatch:

```bash
rg -n "as any|as unknown as|as never|@ts-ignore|TODO.*type|FIXME.*type" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- no type-system escape hatches in API command call paths
- callsites do not need unsafe casts to satisfy command signatures

---

## Step 8 - Verification checks

Run typechecks in client and consumers:

```bash
cd "$CLIENT_REPO" && bun check
cd "$ADMIN_REPO" && bun check
cd "$WEB_REPO" && bun check
```

Optional backend check:

```bash
cd "$API_REPO" && cargo check -p api --all-features
```

---

## Correction playbook

When drift is found, fix in this order:

1. API DTO and route response shape (source of truth)
2. Client type definitions
3. Client command signatures + endpoint path/query builders
4. Consumer callsites (`dairy`/`cream`)
5. Regression tests and type checks

If the change is intentional and breaking:

- document in release notes/changelog
- bump client version according to SemVer policy

---

## Severity rubric

- `high`: runtime break risk (wrong endpoint path/envelope/type mismatch causing failures)
- `medium`: compile-time drift requiring casts/workarounds or missing fields
- `low`: naming or minor consistency drift with low immediate runtime impact
- `note`: cleanup opportunity

---

## Findings template

```md
### [SEVERITY] Contract drift - <operation>

- **API location:** `crates/api/src/routes/...`
- **Client location:** `src/commands/...` and `src/types/...`
- **Observed drift:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Contract drift sweep summary

- Endpoints audited: N
- Missing commands: N
- Type mismatches: N
- Envelope mismatches: N
- Query key mismatches: N
```

---

## Related docs

- [080-typescript-client.md](../guides/080-typescript-client.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
- [071-json-naming.md](../guides/071-json-naming.md)
- [200-project-sync.md](../guides/200-project-sync.md)
