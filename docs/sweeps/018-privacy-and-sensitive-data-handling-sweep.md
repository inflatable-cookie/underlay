# 018 - Privacy and Sensitive Data Handling Sweep

This sweep verifies that sensitive data (PII, secrets, auth artifacts) is handled safely across storage, logs, APIs, jobs, and frontend surfaces.

## Problem this sweep targets

Common regressions:

- PII or secrets leak into logs/error contexts
- response payloads expose more user data than needed
- retention rules are inconsistent across tables and cleanup jobs
- export/delete workflows exist in policy but not in implementation
- client-side storage accidentally persists sensitive auth data

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Build sensitive-data inventory

Inventory likely sensitive entities/fields:

```bash
rg -n "email|phone|name|address|dob|session|token|secret|password|totp|webauthn|ip_address|user_agent" "$API_REPO/migrations" "$API_REPO/crates" -g "*.sql" -g "*.rs"
```

Classify each as:

- public
- internal operational
- sensitive (PII/auth/security)

Pass criteria:

- sensitive field inventory exists and is current

---

## Step 2 - API payload minimization

```bash
rg -n "pub struct .*Dto|pub struct .*Response" "$API_REPO/crates/api/src/dto"
rg -n "email|token|secret|password|hash|ip_address|user_agent" "$API_REPO/crates/api/src/dto" -g "*.rs"
```

Review each sensitive field in DTOs:

- is it necessary for the endpoint consumer?
- can it be masked/omitted?

Pass criteria:

- response payloads follow least-privilege data exposure

---

## Step 3 - Logging and error redaction checks

```bash
rg -n "with_context\(json!\(|error!\(|warn!\(|info!\(" "$API_REPO/crates" -g "*.rs"
rg -n "password|secret|token|authorization|cookie|set-cookie|email" "$API_REPO/crates" -g "*.rs"
```

Pass criteria:

- sensitive values are redacted or omitted in logs/error contexts
- error payloads do not leak secrets/raw auth material

---

## Step 4 - Client-side sensitive data storage checks

```bash
rg -n "localStorage|sessionStorage|document\.cookie|setItem\(|getItem\(" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- tokens/secrets are not stored in persistent browser storage unless explicitly approved
- sensitive values are not cached in unsafe client state by default

---

## Step 5 - Retention and cleanup policy enforcement

```bash
rg -n "purge|cleanup|archive|retention|expires_at|deleted_at" "$API_REPO/crates/jobs" "$API_REPO/crates/db/src" "$API_REPO/migrations" -g "*.rs" -g "*.sql"
```

Review whether sensitive tables have:

- explicit retention policy
- scheduled cleanup jobs
- audit-compliant archival where required

Pass criteria:

- retention behavior is explicit and implemented

---

## Step 6 - Export/delete pathway checks (if required)

Search for user-data export/delete implementation signals:

```bash
rg -n "export|erase|delete account|right to|privacy request|gdpr|ccpa" "$API_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "*.rs" -g "*.ts" -g "*.svelte" -g "*.md"
```

Pass criteria:

- policy-required user data operations are implemented or explicitly tracked as gaps

---

## Step 7 - Job/event payload sensitivity checks

```bash
rg -n "platform\.job|payload|domain_event|event_payload|error_history" "$API_REPO/crates" "$API_REPO/migrations" -g "*.rs" -g "*.sql"
```

Review for:

- sensitive fields inside long-lived job/event payloads
- unnecessary full-object snapshots

Pass criteria:

- queued/event payloads contain minimal required data
- sensitive payload content is controlled and lifecycle-managed

---

## Step 8 - Runtime spot checks

In non-prod, execute representative flows and verify:

1. API responses do not over-expose sensitive fields
2. error logs do not contain secrets/tokens
3. admin views only expose sensitive data to authorized roles

Capture examples for each confirmed issue.

---

## Correction playbook

When findings are present:

1. remove or mask sensitive fields from DTOs/logs
2. add redaction utilities and use them at logging boundaries
3. move sensitive client storage to safer mechanisms (or memory)
4. add/repair cleanup jobs and retention controls
5. document policy exceptions with owner + review date

---

## Severity rubric

- `critical`: direct secret/token leakage or broad unauthorized PII exposure
- `high`: significant sensitive-data overexposure in API/log paths
- `medium`: retention/redaction gaps with moderate compliance risk
- `low`: hygiene/documentation gap
- `note`: future hardening opportunity

---

## Findings template

```md
### [SEVERITY] Sensitive data handling gap - <area>

- **Location:** `crates/...`, `src/...`, `migrations/...`
- **Data type:** PII / auth secret / security metadata
- **Observed issue:**
- **Expected handling:**
- **Risk:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Privacy/sensitive-data sweep summary

- High/Critical issues: N
- API minimization issues: N
- Logging/redaction issues: N
- Retention/cleanup issues: N
```

---

## Related docs

- [001-security-sweep.md](./001-security-sweep.md)
- [007-error-diagnostics-and-logging-sweep.md](./007-error-diagnostics-and-logging-sweep.md)
- [012-observability-and-audit-sweep.md](./012-observability-and-audit-sweep.md)
- [013-background-jobs-and-scheduler-reliability-sweep.md](./013-background-jobs-and-scheduler-reliability-sweep.md)
