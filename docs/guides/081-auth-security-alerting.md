# 081 - Auth Security Alerting (Failed Logins and Lockouts)

This guide standardises login-failure alerting across Underlay-consuming APIs.

## Purpose

Rate limiting and lockout prevent abuse, but operators still need signals when attacks are underway.

Use `underlay-security-alerts` to:

- evaluate suspicious login-attempt pressure from one IP,
- dedupe alerts with cooldown windows,
- persist alert events for auditability.

Use `underlay-jobs-postgres` auth maintenance to:

- suspend long-inactive accounts on a scheduled policy window,
- revoke active sessions when accounts are auto-suspended.

## Shared crate

Crate: `underlay-security-alerts`

Primary API:

- `SecurityAlertConfig` - threshold/cooldown/window settings
- `load_ip_signal_counts(...)` - query failed-attempt signals from login attempts
- `evaluate_alerts(...)` - map counts to alert types
- `has_recent_alert(...)` - cooldown dedupe check
- `insert_alert_event(...)` - persist emitted alert event

## Migration baseline

Copy and adapt:

- `underlay/rust/crates/underlay-security-alerts/migrations/0001__security_alert_events.sql`

Expected tables:

- login attempts table (for example `auth.login_attempts`)
- alert events table (for example `auth.security_alert_events`)

## Recommended alert policy

Baseline thresholds:

- 20+ failed attempts from one IP in 10 minutes
- 5+ distinct user accounts failed from one IP in 10 minutes
- 3+ lockouts from one IP in 10 minutes

Cooldown:

- 30 minutes per `(alert_type, ip)` before re-emitting

## Integration pattern

At failed-login write time:

1. Insert login attempt record.
2. Load recent signal counts for that IP.
3. Evaluate threshold breaches.
4. For each alert type:
   - skip if `has_recent_alert(...)` is true,
   - insert alert event with counts/details,
   - emit app-level log/audit/notification.

The shared crate intentionally does not send email/webhooks directly. Notification transport remains app-specific.

## Inactive Account Maintenance (3-year example)

`underlay-jobs-postgres` exposes `SuspendInactiveAccountsJob` in
`underlay_jobs_postgres::tasks`.

Example:

```rust,ignore
registry.register(
  tasks::SuspendInactiveAccountsJob::new(pool.clone())
    .with_inactivity_days(1095)
    .with_roles(vec!["student".to_string(), "tester".to_string()])
);
```

Recommended schedule:

- daily (`0 40 3 * * *`)

Behavior:

- candidate users are active users in configured roles,
- inactivity uses latest active session `last_used_at` (fallback to account creation),
- status transitions to `suspended` with `auth.users.suspension_reason = inactive_account_auto_suspend`,
- active sessions are revoked with reason `inactive_account_auto_suspend`.

Reactivation policy:

- self-service password reset should only unsuspend accounts with `suspension_reason = inactive_account_auto_suspend`,
- manually suspended accounts should remain admin-only to reactivate.

## App-level outputs

Each app should emit, at minimum:

- structured warning log with alert type, IP, counts, and event id,
- audit event in its platform audit log,
- optional email/webhook/pager integration.

## Validation checklist

- Alert table exists and is indexed by `(alert_type, ip_address, created_at DESC)`.
- Alert threshold config is loaded from typed app config (no magic constants).
- Cooldown dedupe prevents repeated alerts from flooding operators.
- Tests cover threshold crossing and cooldown behaviour.
