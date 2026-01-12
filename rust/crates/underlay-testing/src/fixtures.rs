//! Common test fixtures and data builders
//!
//! Provides utilities for generating test data and working with timestamps.

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// Collection of fixture utilities for tests
pub struct Fixtures;

impl Fixtures {
    /// Generate a unique test ID using UUID v7
    pub fn id() -> Uuid {
        Uuid::now_v7()
    }

    /// Generate a unique test ID as a string
    pub fn id_string() -> String {
        Uuid::now_v7().to_string()
    }

    /// Generate a unique email address for testing
    pub fn email(prefix: &str) -> String {
        let id = Uuid::now_v7().to_string()[..8].to_string();
        format!("{prefix}-{id}@test.example.com")
    }

    /// Generate a unique username for testing
    pub fn username(prefix: &str) -> String {
        let id = Uuid::now_v7().to_string()[..8].to_string();
        format!("{prefix}_{id}")
    }

    /// Generate a test password (not for production use!)
    pub fn password() -> String {
        format!("TestPassword123!_{}", &Uuid::now_v7().to_string()[..8])
    }
}

/// Auth token fixtures for testing authentication flows
pub struct AuthFixtures;

impl AuthFixtures {
    /// Generate a fake access token for testing.
    ///
    /// This is NOT a valid JWT - it's a simple identifier for test purposes.
    /// Your test auth middleware should recognize tokens with this prefix.
    pub fn access_token() -> String {
        format!("test_access_{}", Uuid::now_v7())
    }

    /// Generate a fake refresh token for testing.
    pub fn refresh_token() -> String {
        format!("test_refresh_{}", Uuid::now_v7())
    }

    /// Generate a fake API key for testing.
    pub fn api_key(prefix: &str) -> String {
        format!("{prefix}_{}", Uuid::now_v7())
    }

    /// Generate a session ID for testing.
    pub fn session_id() -> String {
        Uuid::now_v7().to_string()
    }

    /// Generate a CSRF token for testing.
    pub fn csrf_token() -> String {
        format!("csrf_{}", Uuid::now_v7())
    }

    /// Generate an OAuth state parameter for testing.
    pub fn oauth_state() -> String {
        format!("state_{}", Uuid::now_v7())
    }

    /// Create a test user payload for seeding.
    ///
    /// Returns (id, email, username, password_hash_placeholder)
    pub fn user_seed(prefix: &str) -> (Uuid, String, String, String) {
        let id = Uuid::now_v7();
        let email = Fixtures::email(prefix);
        let username = Fixtures::username(prefix);
        // Placeholder - in real tests you'd use argon2 or similar
        let password_hash = format!("$test$hash${}", Uuid::now_v7());
        (id, email, username, password_hash)
    }
}

/// Timestamp fixtures for testing time-dependent logic
pub struct TimestampFixtures;

impl TimestampFixtures {
    /// Get the current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Get a timestamp in the past
    pub fn past_days(days: i64) -> DateTime<Utc> {
        Utc::now() - Duration::days(days)
    }

    /// Get a timestamp in the future
    pub fn future_days(days: i64) -> DateTime<Utc> {
        Utc::now() + Duration::days(days)
    }

    /// Get a timestamp in the past (hours)
    pub fn past_hours(hours: i64) -> DateTime<Utc> {
        Utc::now() - Duration::hours(hours)
    }

    /// Get a timestamp in the future (hours)
    pub fn future_hours(hours: i64) -> DateTime<Utc> {
        Utc::now() + Duration::hours(hours)
    }

    /// Get a timestamp in the past (minutes)
    pub fn past_minutes(minutes: i64) -> DateTime<Utc> {
        Utc::now() - Duration::minutes(minutes)
    }

    /// Get a timestamp in the future (minutes)
    pub fn future_minutes(minutes: i64) -> DateTime<Utc> {
        Utc::now() + Duration::minutes(minutes)
    }

    /// Get an expired timestamp (1 hour ago)
    pub fn expired() -> DateTime<Utc> {
        Self::past_hours(1)
    }

    /// Get a not-yet-expired timestamp (1 hour from now)
    pub fn not_expired() -> DateTime<Utc> {
        Self::future_hours(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_id_is_unique() {
        let id1 = Fixtures::id();
        let id2 = Fixtures::id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_fixtures_email_format() {
        let email = Fixtures::email("user");
        assert!(email.starts_with("user-"));
        assert!(email.ends_with("@test.example.com"));
    }

    #[test]
    fn test_fixtures_username_format() {
        let username = Fixtures::username("test");
        assert!(username.starts_with("test_"));
    }

    #[test]
    fn test_timestamp_past_days() {
        let past = TimestampFixtures::past_days(7);
        let now = Utc::now();
        assert!(past < now);
        // Should be roughly 7 days ago (within a few seconds)
        let diff = now - past;
        assert!(diff.num_days() == 7 || diff.num_days() == 6);
    }

    #[test]
    fn test_timestamp_future_days() {
        let future = TimestampFixtures::future_days(7);
        let now = Utc::now();
        assert!(future > now);
    }

    #[test]
    fn test_timestamp_expired() {
        let expired = TimestampFixtures::expired();
        let now = Utc::now();
        assert!(expired < now);
    }

    #[test]
    fn test_timestamp_not_expired() {
        let not_expired = TimestampFixtures::not_expired();
        let now = Utc::now();
        assert!(not_expired > now);
    }
}
