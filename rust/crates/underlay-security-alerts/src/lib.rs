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
mod detector_tests;
mod error;
mod store;
mod types;

pub use crate::detector::evaluate_alerts;
pub use crate::error::{SecurityAlertError, SecurityAlertResult};
pub use crate::store::{has_recent_alert, insert_alert_event, load_ip_signal_counts};
pub use crate::types::{
    LoginAttemptSignalCounts, SecurityAlertConfig, SecurityAlertEventInput, SecurityAlertType,
};
