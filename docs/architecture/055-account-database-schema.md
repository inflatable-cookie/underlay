# Account Database Schema

Status: active reference
Owner: repo maintainers

This document describes the shared account/profile boundary that sits next to
the auth schema.

`account` exists to keep identity and personalization out of the shared auth
mechanics. `auth` handles authentication. `account` handles profile-oriented
user data.

## Boundary

`auth` owns:

- email
- credentials
- sessions
- workflow state
- auth-specific status and coarse role

`account` owns:

- full name
- display name override
- locale and timezone
- broader profile and personalization fields

The live shared auth user row does not carry a canonical `display_name`
column. That is why `account.user_profile` is the preferred home for identity
presentation fields.

## Core Table

### account.user_profile

Stores optional profile and personalization data keyed 1:1 to `auth.users`.

| Column | Type | Constraints | Description |
|---|---|---|---|
| `user_id` | `UUID` | PK, FK -> `auth.users` | Shared user id |
| `full_name` | `TEXT` | NULL | Full preferred name |
| `display_name` | `TEXT` | NULL | Short display name override |
| `country_code` | `TEXT` | NULL | ISO 3166-1 alpha-2 |
| `time_zone` | `TEXT` | NULL | IANA timezone id |
| `language` | `TEXT` | NULL | BCP 47 language tag |
| `created_at` | `TIMESTAMPTZ` | NOT NULL | Creation time |
| `updated_at` | `TIMESTAMPTZ` | NOT NULL | Last update time |

Relationship:

```
auth.users (1) -> (0..1) account.user_profile
```

## Design Rules

- profiles are optional and may be created lazily
- locale fields are optional and apps may fall back to browser defaults
- timezones should be stored as IANA ids, not offsets
- naming fields should stay culturally flexible; avoid forcing first/last-name
  assumptions

## Interaction With Shared Types

The shared auth `User` type still has an optional `display_name` field for
app-facing compatibility, but new shared identity data should prefer
`account.user_profile`.

That means:

- auth persistence does not need to own profile naming directly
- app service layers may join auth and account data into one caller-facing
  user/session payload
- the compatibility field in shared Rust/TS user types should not be treated as
  proof that the auth table owns the canonical display name

## Extension Rule

Apps may extend `account` with more profile-oriented tables such as:

- subscriptions
- preferences
- saved items
- onboarding state

They should keep those additions separate from the shared auth tables unless
the data is genuinely authentication-specific.
