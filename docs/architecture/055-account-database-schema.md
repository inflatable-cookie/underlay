# Account Database Schema

This document details the database schema for user profiles, subscriptions, and saved items. The `account` schema is separate from `auth` to maintain a clean boundary between authentication (identity/credentials) and user personalization.

## Schema Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              account schema                                  │
│                                                                             │
│  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐       │
│  │   user_profile    │  │   subscription    │  │    saved_item     │       │
│  │                   │  │                   │  │                   │       │
│  │ - locale settings │  │ - tier & status   │  │ - bookmarks       │       │
│  │ - preferences     │  │ - Stripe IDs      │  │ - notes           │       │
│  │ - consent         │  │ - billing periods │  │                   │       │
│  └───────────────────┘  └───────────────────┘  └───────────────────┘       │
│           │                      │                      │                   │
│           └──────────────────────┼──────────────────────┘                   │
│                                  ▼                                          │
│                         auth.users (FK)                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Design Principles

1. **Separation from Auth**: The `auth` schema handles identity and credentials only. User preferences, locale, and personalization belong in `account`.

2. **Optional Profile**: Not every `auth.users` record needs a profile immediately. Profiles can be created lazily on first access.

3. **Timezone as IANA String**: Store timezones as IANA identifiers (e.g., `Europe/London`, `America/New_York`) rather than offsets, to handle DST correctly.

4. **Subscription Flexibility**: Support multiple subscription models (Stripe, manual, trials) with clear status tracking.

## Core Tables

### account.user_profile

Stores user preferences, locale settings, and consent data.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| user_id | UUID | PRIMARY KEY, FK → auth.users | User identifier (1:1 with auth.users) |
| display_name | TEXT | NULL, ≤128 chars | Override display name (if different from auth.users) |
| first_name | TEXT | NULL, ≤64 chars | First/given name |
| last_name | TEXT | NULL, ≤64 chars | Last/family name |
| avatar_url | TEXT | NULL, ≤512 chars | Profile picture URL |
| country_code | TEXT | NULL, ≤2 chars | ISO 3166-1 alpha-2 (e.g., `GB`, `US`) |
| time_zone | TEXT | NULL, ≤64 chars | IANA timezone (e.g., `Europe/London`) |
| language | TEXT | NULL, ≤10 chars | BCP 47 language tag (e.g., `en`, `en-GB`) |
| region_code | TEXT | NULL, ≤8 chars | Sub-national region (e.g., `ENG`, `CA`) |
| currency_preference | TEXT | NULL, ≤3 chars | ISO 4217 currency (e.g., `GBP`, `USD`) |
| email_marketing_opt_in | BOOLEAN | NOT NULL DEFAULT FALSE | Marketing email consent |
| email_transactional_opt_in | BOOLEAN | NOT NULL DEFAULT TRUE | Transactional email consent |
| email_frequency | TEXT | NOT NULL DEFAULT 'normal' | Email frequency preference |
| cookie_consent | JSONB | NULL | Cookie consent details |
| data_processing_consent_version | TEXT | NULL, ≤32 chars | GDPR/privacy consent version |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Profile creation time |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update time |

**Notes:**
- `user_id` is both the primary key and foreign key - one profile per user
- All locale fields are optional; apps should fall back to browser detection
- `email_frequency` enum: `low`, `normal`, `high`

### account.subscription

Tracks user subscription tiers and billing status.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY | Subscription identifier |
| user_id | UUID | NOT NULL, FK → auth.users, UNIQUE | Owning user (one active subscription per user) |
| tier | TEXT | NOT NULL DEFAULT 'free' | Subscription tier |
| status | TEXT | NOT NULL DEFAULT 'active' | Subscription status |
| stripe_subscription_id | TEXT | NULL, ≤128 chars | Stripe subscription ID |
| stripe_customer_id | TEXT | NULL, ≤128 chars | Stripe customer ID |
| current_period_start | TIMESTAMPTZ | NULL | Current billing period start |
| current_period_end | TIMESTAMPTZ | NULL | Current billing period end |
| trial_ends_at | TIMESTAMPTZ | NULL | Trial expiration (if trialing) |
| cancelled_at | TIMESTAMPTZ | NULL | When subscription was cancelled |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Subscription creation time |
| updated_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Last update time |

**Tier values:** `free`, `premium`, `premium_plus`

**Status values:** `active`, `past_due`, `cancelled`, `expired`, `trialing`

### account.saved_item

User bookmarks/saved content for later review.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY | Saved item identifier |
| user_id | UUID | NOT NULL, FK → auth.users | Owning user |
| item_type | TEXT | NOT NULL | Type of saved item |
| item_id | UUID | NOT NULL | Reference to the saved item |
| note | TEXT | NULL, ≤1000 chars | User's personal note |
| created_at | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | When item was saved |

**Item type values:** `activity`, `summary`, `video`, `question`

**Unique constraint:** `(user_id, item_type, item_id)` - can't save same item twice

## Indexes

| Table | Index | Purpose |
|-------|-------|---------|
| user_profile | PK on user_id | Profile lookup by user |
| subscription | idx_subscription_user_id | Subscription lookup by user |
| subscription | idx_subscription_stripe_id | Webhook processing |
| subscription | idx_subscription_status | Find active/expired subscriptions |
| saved_item | idx_saved_item_user_id | User's saved items |
| saved_item | idx_saved_item_user_type | User's saved items by type |
| saved_item | unique on (user_id, item_type, item_id) | Prevent duplicates |

## Relationships

```
auth.users (1) ─────── (0..1) account.user_profile
     │
     ├──────────────── (0..1) account.subscription
     │
     └──────────────── (0..n) account.saved_item
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

### Subscription Webhooks

Stripe webhooks should update subscription status:

```
customer.subscription.created → create subscription record
customer.subscription.updated → update tier/status/periods
customer.subscription.deleted → set status to 'cancelled'
invoice.payment_failed → set status to 'past_due'
```

## Security Considerations

1. Profile data is PII - apply appropriate access controls
2. Only expose profile to the owning user or admins
3. Subscription tier affects feature access - validate server-side
4. Saved items should only be visible to the owning user
