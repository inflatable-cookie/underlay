use crate::{LoginAttemptsTable, SecurityAlertEventsTable, SecurityAlertTables};

#[test]
fn alert_tables_accept_valid_names() {
    assert_eq!(
        LoginAttemptsTable::parse("auth.login_attempts")
            .unwrap()
            .quoted(),
        "\"auth\".\"login_attempts\""
    );
    assert_eq!(
        SecurityAlertEventsTable::parse("security_alert_events")
            .unwrap()
            .quoted(),
        "\"security_alert_events\""
    );
}

#[test]
fn alert_tables_reject_invalid_names() {
    assert!(LoginAttemptsTable::parse("auth.login-attempts").is_err());
    assert!(SecurityAlertEventsTable::parse("alerts; DROP TABLE alerts").is_err());
}

#[test]
fn alert_tables_group_keeps_both_locations() {
    let tables = SecurityAlertTables::new(
        LoginAttemptsTable::parse("auth.login_attempts").unwrap(),
        SecurityAlertEventsTable::parse("auth.security_alert_events").unwrap(),
    );

    assert_eq!(
        tables.login_attempts.quoted(),
        "\"auth\".\"login_attempts\""
    );
    assert_eq!(
        tables.alert_events.quoted(),
        "\"auth\".\"security_alert_events\""
    );
}
