use crate::types::{
    AccountSignalCounts, GlobalSignalCounts, LoginAttemptSignalCounts, SecurityAlertConfig,
    SecurityAlertType,
};

pub fn evaluate_alerts(
    counts: LoginAttemptSignalCounts,
    config: &SecurityAlertConfig,
) -> Vec<SecurityAlertType> {
    let mut alerts = Vec::new();

    if counts.failed_attempts >= config.failed_attempts_threshold() {
        alerts.push(SecurityAlertType::LoginFailuresFromIp);
    }

    if counts.distinct_users >= config.distinct_users_threshold() {
        alerts.push(SecurityAlertType::MultiAccountFailuresFromIp);
    }

    if counts.lockouts >= config.lockouts_threshold() {
        alerts.push(SecurityAlertType::LockoutsFromIp);
    }

    alerts
}

/// Evaluate per-account signals. These are keyed on the account, not the IP,
/// so an attacker rotating (spoofed or real) IPs cannot keep the counters
/// below threshold.
pub fn evaluate_account_alerts(
    counts: AccountSignalCounts,
    config: &SecurityAlertConfig,
) -> Vec<SecurityAlertType> {
    let mut alerts = Vec::new();

    if counts.failed_attempts >= config.account_failed_attempts_threshold() {
        alerts.push(SecurityAlertType::LoginFailuresForAccount);
    }

    if counts.distinct_ips >= config.account_distinct_ips_threshold() {
        alerts.push(SecurityAlertType::DistributedFailuresForAccount);
    }

    alerts
}

/// Evaluate the global failed-attempt surge signal across all accounts and
/// IPs in the window.
pub fn evaluate_global_alerts(
    counts: GlobalSignalCounts,
    config: &SecurityAlertConfig,
) -> Vec<SecurityAlertType> {
    let mut alerts = Vec::new();

    if counts.failed_attempts >= config.global_failed_attempts_threshold() {
        alerts.push(SecurityAlertType::GlobalLoginFailureSurge);
    }

    alerts
}
