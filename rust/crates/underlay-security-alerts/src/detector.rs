use crate::types::{LoginAttemptSignalCounts, SecurityAlertConfig, SecurityAlertType};

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
