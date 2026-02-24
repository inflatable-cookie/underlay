# 026 - Auth Security Alerting Sweep

Use this sweep to verify lockout/failure alerting is implemented consistently across Underlay-consuming API apps.

## Inputs

Set:

```bash
API_REPO="/path/to/api-repo"
UNDERLAY_REPO="/path/to/underlay"
```

## Step 1 - Shared crate adoption

```bash
rg -n "underlay-security-alerts" "$API_REPO/Cargo.toml" "$API_REPO/crates"
```

Pass criteria:

- API/auth crate depends on `underlay-security-alerts`.
- Detection path uses shared helpers, not ad-hoc duplicated SQL.

## Step 2 - Migration coverage

```bash
rg -n "security_alert_events|auth\\.security_alert_events" "$API_REPO/migrations"
```

Pass criteria:

- App migration defines a security alert events table.
- Table has lookup index on `(alert_type, ip_address, created_at DESC)`.

## Step 3 - Failed-login integration path

```bash
rg -n "record_failed_login|login_attempts|evaluate_alerts|has_recent_alert|insert_alert_event" "$API_REPO/crates"
```

Pass criteria:

- Failed login flow invokes alert evaluation.
- Cooldown dedupe is applied before event insertion.

## Step 4 - Operator-facing outputs

```bash
rg -n "security alert|login_failures_from_ip|lockouts_from_ip|append_.*audit|warn!|error!" "$API_REPO/crates"
```

Pass criteria:

- Emitted alerts generate structured logs.
- Alert events are reflected in audit trail or equivalent operator channel.

## Step 5 - Config and tests

```bash
rg -n "failed_attempts_threshold|distinct_users_threshold|lockouts_threshold|cooldown" "$API_REPO/crates"
rg -n "security.*alert|evaluate_alerts|cooldown" "$API_REPO/crates" -g '*test*.rs'
```

Pass criteria:

- Thresholds are configurable via typed app config.
- Tests cover threshold crossing and cooldown dedupe behavior.

## Report Template

```md
# Auth Security Alerting Sweep Report

Date: YYYY-MM-DD
Reviewer: <name>
Repo: <path>

## Findings
- [ ] Shared Underlay alerting helpers adopted.
- [ ] Security alert events migration present.
- [ ] Failed-login path emits deduped alerts.
- [ ] Operator outputs (logs/audit/notifications) present.
- [ ] Threshold/cooldown config + tests in place.

## Issues
- <file>:<line> - <issue>

## Follow-up
- <task>
```
