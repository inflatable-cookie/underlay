#[cfg(test)]
mod tests {
    use crate::{
        evaluate_account_alerts, evaluate_alerts, evaluate_global_alerts, AccountSignalCounts,
        GlobalSignalCounts, LoginAttemptSignalCounts, SecurityAlertConfig, SecurityAlertType,
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

    #[test]
    fn account_alert_fires_under_ip_rotation() {
        // Attacker rotates IPs so every per-IP counter stays at 1; the
        // per-account counters still accumulate.
        let config = SecurityAlertConfig::default();
        let alerts = evaluate_account_alerts(
            AccountSignalCounts {
                failed_attempts: 10,
                distinct_ips: 10,
            },
            &config,
        );

        assert!(alerts.contains(&SecurityAlertType::LoginFailuresForAccount));
        assert!(alerts.contains(&SecurityAlertType::DistributedFailuresForAccount));
    }

    #[test]
    fn account_alert_silent_below_thresholds() {
        let config = SecurityAlertConfig::default();
        let alerts = evaluate_account_alerts(
            AccountSignalCounts {
                failed_attempts: 2,
                distinct_ips: 2,
            },
            &config,
        );

        assert!(alerts.is_empty());
    }

    #[test]
    fn global_surge_alert_fires_at_threshold() {
        let config = SecurityAlertConfig::default();
        let alerts = evaluate_global_alerts(
            GlobalSignalCounts {
                failed_attempts: 200,
                distinct_ips: 150,
            },
            &config,
        );

        assert!(alerts.contains(&SecurityAlertType::GlobalLoginFailureSurge));

        let quiet = evaluate_global_alerts(
            GlobalSignalCounts {
                failed_attempts: 10,
                distinct_ips: 8,
            },
            &config,
        );
        assert!(quiet.is_empty());
    }

    #[test]
    fn security_alert_type_serde_uses_snake_case_matching_as_str() {
        for alert_type in [
            SecurityAlertType::LoginFailuresFromIp,
            SecurityAlertType::MultiAccountFailuresFromIp,
            SecurityAlertType::LockoutsFromIp,
            SecurityAlertType::DormantAccountAccessAttempt,
            SecurityAlertType::LoginFailuresForAccount,
            SecurityAlertType::DistributedFailuresForAccount,
            SecurityAlertType::GlobalLoginFailureSurge,
        ] {
            let serialized = serde_json::to_string(&alert_type).expect("serialize alert type");
            assert_eq!(serialized, format!("\"{}\"", alert_type.as_str()));

            let deserialized: SecurityAlertType =
                serde_json::from_str(&serialized).expect("deserialize alert type");
            assert_eq!(deserialized, alert_type);
        }
    }

    #[test]
    fn security_alert_config_validate_rejects_non_positive_values() {
        use chrono::Duration;

        assert!(SecurityAlertConfig::default().validate().is_ok());

        for bad in [
            SecurityAlertConfig::default().with_window(Duration::zero()),
            SecurityAlertConfig::default().with_window(Duration::seconds(-1)),
            SecurityAlertConfig::default().with_cooldown(Duration::zero()),
            SecurityAlertConfig::default().with_cooldown(Duration::seconds(-5)),
            SecurityAlertConfig::default().with_failed_attempts_threshold(0),
            SecurityAlertConfig::default().with_distinct_users_threshold(-1),
            SecurityAlertConfig::default().with_lockouts_threshold(0),
            SecurityAlertConfig::default().with_account_failed_attempts_threshold(0),
            SecurityAlertConfig::default().with_account_distinct_ips_threshold(-2),
            SecurityAlertConfig::default().with_global_failed_attempts_threshold(0),
        ] {
            assert!(
                bad.validate().is_err(),
                "expected invalid config to fail validation: {bad:?}"
            );
        }
    }
}
