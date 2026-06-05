//! Shared login security alerting helpers for Underlay consumers.
//!
//! This crate provides:
//! - alert thresholds/config (`SecurityAlertConfig`)
//! - signal evaluation from login-attempt counts (`evaluate_alerts`)
//! - SQL helpers for loading counts and persisting deduped alert events
//!
//! Consuming applications own:
//! - where login attempts are written
//! - notification transport (email/webhook/pager)
//! - audit/event emission policy
//!
//! ## Example migration
//!
//! See `migrations/0001__security_alert_events.sql` in this crate for a
//! copy-paste-ready baseline table definition.

mod detector;
mod error;
mod store;
mod tables;
mod types;

#[cfg(test)]
#[path = "tests/detector_tests.rs"]
mod detector_tests;

pub use crate::detector::evaluate_alerts;
pub use crate::error::{SecurityAlertError, SecurityAlertResult};
#[allow(deprecated)]
pub use crate::store::{
    has_recent_alert, has_recent_alert_in_table, insert_alert_event, insert_alert_event_into_table,
    load_ip_signal_counts, load_ip_signal_counts_from_table,
};
pub use crate::tables::{LoginAttemptsTable, SecurityAlertEventsTable, SecurityAlertTables};
pub use crate::types::{
    LoginAttemptSignalCounts, SecurityAlertConfig, SecurityAlertEventInput, SecurityAlertType,
};
