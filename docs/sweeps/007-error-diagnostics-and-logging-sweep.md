# 007 - Error Diagnostics and Logging Sweep

This sweep enforces two linked Underlay standards:

1. Database failures are mapped through Underlay diagnostic helpers (`map_db_error_ref`/`map_db_error`)
2. HTTP errors are captured by Underlay error logging middleware with structured context

## Problem this sweep targets

Common regressions:

- generic "Database error" messages with no actionable diagnostics
- ad-hoc `e.to_string()` responses leaking raw internals
- handlers returning raw status responses instead of `ApiError`
- missing or misordered error logging middleware
- poor context on high-value failures (no operation/entity metadata)

## Scope

Backend repo only.

```bash
export API_REPO="/path/to/myapp-api"
```

Acowtancy mapping: `farmyard`.

---

## Step 1 - Verify error logging middleware is configured

```bash
rg -n "ErrorLoggingConfig|error_logging_middleware|with_client_errors|with_server_errors" "$API_REPO/crates/api/src"
```

Pass criteria:

- `underlay-http` error logging feature is in use
- middleware is configured with database pool/state
- client/server error capture policy is explicit

### 1.1 Verify middleware order

```bash
rg -n "trace_layer|request_id_layer|error_logging_middleware|cors_layer" "$API_REPO/crates/api/src/main.rs"
```

Pass criteria:

- order follows: tracing -> request ID -> error logging -> CORS

---

## Step 2 - Verify DB diagnostic mapping standard

```bash
rg -n "map_db_error_ref\(|map_db_error\(" "$API_REPO/crates/api/src" "$API_REPO/crates/domain/src"
rg -n "underlay_db::map_db_error_ref|underlay_db::map_db_error|describe_db_error" "$API_REPO/crates"
```

Pass criteria:

- SQLx errors map through Underlay helpers
- operation strings are specific and actionable (for example, "Database error updating module")
- legacy per-feature wrappers are thin delegates to Underlay helpers

---

## Step 3 - Detect likely error leakage or degraded diagnostics

### 3.1 Raw error text in response context

```bash
rg -n "\"db_error\"\s*:\s*(e|err|msg)\.to_string\(|AppError::new\([^\n]*e\.to_string\(" "$API_REPO/crates/api/src/routes"
```

### 3.2 Raw status responses bypassing `ApiError`

```bash
rg -n "StatusCode::(BAD_REQUEST|UNAUTHORIZED|FORBIDDEN|NOT_FOUND|CONFLICT|INTERNAL_SERVER_ERROR).*into_response\(" "$API_REPO/crates/api/src/routes"
rg -n "error_response\(" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- no user-facing error paths depend on raw `to_string()` leakage
- handlers prefer `ApiError`/`to_api_error(...)` helpers

---

## Step 4 - Verify structured context on high-value failures

```bash
rg -n "to_api_error_with_context\(|with_context\(json!\(" "$API_REPO/crates/api/src/routes"
```

For mutation and critical read paths, verify context includes:

- operation name
- primary entity ID(s)
- safe filter/query metadata (never secrets)

Pass criteria:

- high-value failures include minimal structured context for operators
- context does not include sensitive values

---

## Step 5 - Verify error code stability and taxonomy

```bash
rg -n "AppError::new\(|ApiError::(bad_request|unauthorized|forbidden|not_found|conflict|internal)" "$API_REPO/crates/api/src"
```

Review for:

- stable dotted error codes (`domain.action_failed`, `validation.invalid_id`)
- no random, per-handler ad-hoc code naming

Pass criteria:

- code namespace is consistent and predictable
- equivalent failures across modules reuse aligned error codes

---

## Step 6 - Runtime verification in local/dev

Trigger representative known-failure paths and verify `platform.error_log` capture.

Checklist:

1. Trigger 4xx validation error
2. Trigger 5xx db failure path (safe controlled case)
3. Verify logged row includes endpoint, status, error code, correlation/request ID, and handler context

Optional query check (adjust to project schema):

```sql
SELECT occurred_at, endpoint, status_code, error_code, context
FROM platform.error_log
ORDER BY occurred_at DESC
LIMIT 20;
```

---

## Step 7 - Correction playbook

When findings are present:

1. Replace direct SQLx error responses with `map_db_error_ref` or wrapper that delegates to it
2. Convert raw status branches to `ApiError` helpers
3. Add `with_context(...)` for high-value failures
4. Remove raw `e.to_string()` from user-facing payloads
5. Ensure middleware ordering and feature flags are correct

---

## Severity rubric

- `critical`: sensitive error data leakage to clients
- `high`: missing db diagnostics or missing error logging middleware in production path
- `medium`: inconsistent context and operator diagnostics
- `low`: taxonomy cleanup and non-critical consistency gaps
- `note`: informational hardening opportunity

---

## Findings template

```md
### [SEVERITY] Error diagnostics/logging gap - <area>

- **Location:** `crates/api/src/...`
- **Observed issue:**
- **Expected Underlay pattern:**
- **Operator impact:**
- **User impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Error diagnostics sweep summary

- Middleware gaps: N
- DB mapping gaps: N
- Context enrichment gaps: N
- Leakage risks: N
```

---

## Related docs

- [078-error-logging.md](../guides/078-error-logging.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
- [068-security.md](../guides/068-security.md)
- [200-project-sync.md](../guides/200-project-sync.md)
