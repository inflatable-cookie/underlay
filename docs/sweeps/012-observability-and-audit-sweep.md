# 012 - Observability and Audit Sweep

This sweep verifies that critical backend actions are observable and auditable with enough context for incident response and compliance needs.

## Problem this sweep targets

Common regressions:

- mutation failures not logged with actionable context
- important admin actions leave no durable audit trail
- inconsistent event/error codes across modules
- request correlation missing between logs and API responses

## Scope

```bash
export API_REPO="/path/to/myapp-api"
```

Acowtancy mapping: `farmyard`.

---

## Step 1 - Error observability baseline

```bash
rg -n "error_logging_middleware|ErrorLoggingConfig|request_id_layer|trace_layer" "$API_REPO/crates/api/src"
rg -n "to_api_error_with_context\(|with_context\(json!\(" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- global error logging middleware is active
- request correlation and structured context are present for important failures

---

## Step 2 - Mutation route audit coverage inventory

Identify high-value mutation areas:

```bash
rg -n "post,|put,|patch,|delete," "$API_REPO/crates/api/src/routes"
```

For each high-value mutation domain (content, learning, users, auth/security, billing/platform), check:

- does success/failure produce structured logs?
- is there a durable audit record where required?

---

## Step 3 - Structured event consistency

```bash
rg -n "event|audit|action|operation|error_code|request_id|correlation" "$API_REPO/crates" -g "*.rs"
```

Review for:

- stable event naming conventions
- consistent key fields (`actor_id`, `entity_id`, `operation`, `timestamp`)
- absence of sensitive payload leakage in logs

Pass criteria:

- event schema is consistent enough for dashboards/alerts
- sensitive data is redacted or excluded

---

## Step 4 - Durable audit trail checks

If project has audit tables/events, verify they capture:

- actor (who)
- action (what)
- target (which entity)
- time (when)
- optional diff/context (safe subset)

Search helpers:

```bash
rg -n "audit|audit_log|activity_log|event_log|platform\.error_log" "$API_REPO/migrations" "$API_REPO/crates" -g "*.sql" -g "*.rs"
```

Pass criteria:

- security/compliance-relevant operations produce durable records
- records are queryable and correlated with request/error metadata

---

## Step 5 - Alertability and operations readiness

Verify there is a practical way to alert on:

- spikes in 5xx rates
- repeated auth failures/forbidden attempts
- repeated validation failures on critical flows
- failed background jobs/scheduled tasks

Evidence sources:

- code-level metrics/event emission hooks
- runbook queries
- dashboard/alert definitions (if in repo)

---

## Step 6 - Runtime drill (recommended)

Perform a small controlled drill in non-prod:

1. execute one successful high-value mutation
2. execute one expected failure path
3. verify traces/logs/audit rows can be found quickly
4. verify correlation fields connect request -> error log -> audit entry (where applicable)

Capture time-to-diagnosis and missing context.

---

## Correction playbook

When findings are present:

1. add/normalize structured error context fields
2. ensure critical mutations emit durable audit events
3. standardize event/error code taxonomy
4. add redaction guards for sensitive fields
5. add runbook queries and alert definitions

---

## Severity rubric

- `high`: critical mutation path has no reliable audit/diagnostic trail
- `medium`: partial context or inconsistent event schema limits incident response
- `low`: minor observability taxonomy/hygiene gap
- `note`: monitoring enhancement opportunity

---

## Findings template

```md
### [SEVERITY] Observability/audit gap - <operation>

- **Location:** `crates/...` and/or `migrations/...`
- **Current visibility:**
- **Missing signal/context:**
- **Operational risk:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Observability/audit sweep summary

- Critical operations audited: N
- Missing audit trails: N
- Context gaps: N
- Alertability gaps: N
```

---

## Related docs

- [078-error-logging.md](../guides/078-error-logging.md)
- [007-error-diagnostics-and-logging-sweep.md](./007-error-diagnostics-and-logging-sweep.md)
- [001-security-sweep.md](./001-security-sweep.md)
