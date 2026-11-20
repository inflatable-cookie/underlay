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
fn test_fixtures_unique_within_same_millisecond() {
    use std::collections::HashSet;

    let count = 64;
    let emails: HashSet<String> = (0..count).map(|_| Fixtures::email("user")).collect();
    let usernames: HashSet<String> = (0..count).map(|_| Fixtures::username("user")).collect();
    let passwords: HashSet<String> = (0..count).map(|_| Fixtures::password()).collect();

    assert_eq!(emails.len(), count, "emails collided within tight loop");
    assert_eq!(
        usernames.len(),
        count,
        "usernames collided within tight loop"
    );
    assert_eq!(
        passwords.len(),
        count,
        "passwords collided within tight loop"
    );
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
