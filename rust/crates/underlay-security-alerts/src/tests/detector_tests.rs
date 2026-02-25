#[cfg(test)]
mod tests {
    use crate::{
        evaluate_alerts, LoginAttemptSignalCounts, SecurityAlertConfig, SecurityAlertType,
    };

    #[test]
    fn emits_alerts_when_thresholds_met() {
        let config = SecurityAlertConfig::default();
        let alerts = evaluate_alerts(
            LoginAttemptSignalCounts {
                failed_attempts: 20,
                distinct_users: 5,
                lockouts: 3,
            },
            &config,
        );

        assert!(alerts.contains(&SecurityAlertType::LoginFailuresFromIp));
        assert!(alerts.contains(&SecurityAlertType::MultiAccountFailuresFromIp));
        assert!(alerts.contains(&SecurityAlertType::LockoutsFromIp));
    }

    #[test]
    fn emits_nothing_below_thresholds() {
        let config = SecurityAlertConfig::default();
        let alerts = evaluate_alerts(
            LoginAttemptSignalCounts {
                failed_attempts: 2,
                distinct_users: 1,
                lockouts: 0,
            },
            &config,
        );

        assert!(alerts.is_empty());
    }
}
