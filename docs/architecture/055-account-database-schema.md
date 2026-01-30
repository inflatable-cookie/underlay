# Account Database Schema

This document details the database schema for user profiles. The `account` schema is separate from `auth` to maintain a clean boundary between authentication (credentials/session) and user identity/personalization.

## Design Principles

1. **Separation from Auth**: The `auth` schema handles authentication only (email + credentials + sessions). User identity and personalization belong in `account`.

2. **Optional Profile**: Not every `auth.users` record needs a profile immediately. Profiles can be created lazily on first access.

3. **Timezone as IANA String**: Store timezones as IANA identifiers (e.g., `Europe/London`, `America/New_York`) rather than offsets, to handle DST correctly.

## Core Tables

### account.user_profile

Stores user preferences, locale settings, and consent data.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| user_id | UUID | PRIMARY KEY, FK → auth.users | User identifier (1:1 with auth.users) |
| full_name | TEXT | NULL | Full name as the user wishes to be known |
| display_name | TEXT | NULL | Short display name override (optional) |
| country_code | TEXT | NULL | ISO 3166-1 alpha-2 (e.g., `GB`, `US`) |
| time_zone | TEXT | NULL | IANA timezone (e.g., `Europe/London`) |
| language | TEXT | NULL | BCP 47 language tag (e.g., `en`, `en-GB`) |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Profile creation time |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update time |

**Notes:**
- `user_id` is both the primary key and foreign key - one profile per user
- All locale fields are optional; apps should fall back to browser detection
- Name fields use a culturally-inclusive pattern; avoid hardcoding first/last name.

## Relationships

```
auth.users (1) ─────── (0..1) account.user_profile
```

## Application Usage

### Creating a Profile

Profiles should be created lazily when a user first accesses profile-related features:

```rust
// Check if profile exists, create if not
let profile = match db::get_user_profile(user_id).await? {
    Some(p) => p,
    None => db::create_user_profile(user_id, defaults).await?
};
```

### Timezone Handling

1. Store timezone as IANA identifier (`Europe/London`)
2. Frontend sends timezone from browser if user hasn't set preference
3. Use timezone for displaying dates/times to user
4. Always store timestamps as UTC in database

## Application Extensions

Apps can add additional tables to the `account` schema for app-specific user data:

```sql
-- Example: Subscription tracking
CREATE TABLE account.subscription (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    tier TEXT NOT NULL,
    -- ...
);

-- Example: Saved/bookmarked items
CREATE TABLE account.saved_item (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    item_type TEXT NOT NULL,
    item_id UUID NOT NULL,
    -- ...
);
```

## Security Considerations

1. Profile data is PII - apply appropriate access controls
2. Only expose profile to the owning user or admins
