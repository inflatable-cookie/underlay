# User Profiles

This guide covers the Underlay pattern for user profiles, which handles identity and personalization separately from authentication.

## Principles

1. **Auth/Profile Separation** - Authentication (`auth.users`) and identity (`account.user_profile`) are separate concerns
2. **Culturally-Inclusive Names** - Use `fullName` + `displayName` instead of `firstName`/`lastName`
3. **Extensible Baseline** - Underlay provides types and helpers; apps add domain-specific fields

## Auth vs Profile

### Auth Layer (`auth.users`)

Handles authentication only:
- Email address
- Password hash
- Email verification status
- MFA credentials

The `display_name` field in `auth.users` is optional and deprecated in favor of profiles.

### Profile Layer (`account.user_profile`)

Handles identity and personalization:
- Names (full name, display name)
- Locale settings (timezone, language, country)
- Preferences (communication, regional)
- App-specific extensions (avatar, subscription, etc.)

## Name Fields

The traditional `firstName`/`lastName` pattern is Western-centric and doesn't work well for:
- CJK names (Chinese, Japanese, Korean) - typically one or two characters total
- Single-word names (common in some cultures)
- Names with multiple given names or family names
- Users who prefer a nickname

### Underlay Pattern

```typescript
interface UserProfileBase {
  /** User's full name as they wish to be known */
  fullName: string | null;

  /** Short name for UI display (e.g., in headers, greetings) */
  displayName: string | null;
}
```

**fullName**: The complete name the user wants to be known by. This might be:
- Legal name: "Alice Jane Smith"
- Preferred name: "A.J. Smith"
- CJK name: "李明"
- Single name: "Madonna"

**displayName**: A short name for UI elements like headers, greetings, avatars. If not provided, derive from `fullName`.

### Deriving Display Name

Use `deriveDisplayName()` to automatically derive a display name:

```typescript
import { deriveDisplayName } from "@decodelabs/underlay/patterns";

deriveDisplayName("Alice Smith");        // "Alice"
deriveDisplayName("李明");               // "李明" (CJK names kept whole)
deriveDisplayName("María José García");  // "María"
deriveDisplayName(null);                 // null
```

The function handles CJK names specially - it doesn't split them since they're typically short and splitting would be incorrect.

### Effective Display Name

Use `getEffectiveDisplayName()` for robust fallback:

```typescript
import { getEffectiveDisplayName } from "@decodelabs/underlay/patterns";

// Falls back through: displayName → fullName → email → "User"
getEffectiveDisplayName(profile, user.email);
```

## TypeScript Types

### Base Profile Type

```typescript
import { type UserProfileBase } from "@decodelabs/underlay/patterns";

// Underlay provides the baseline
interface UserProfileBase {
  userId: string;
  fullName: string | null;
  displayName: string | null;
  countryCode: string | null;   // ISO 3166-1 alpha-2
  timeZone: string | null;      // IANA identifier
  language: string | null;      // BCP 47 tag
  createdAt: string;
  updatedAt: string;
}
```

### Extending for Your App

```typescript
// cattle-grid/src/types/account-types.ts
import { type UserProfileBase } from "@decodelabs/underlay/patterns";

export interface UserProfile extends UserProfileBase {
  // App-specific fields
  avatarUrl: string | null;
  regionCode: string | null;
  currencyPreference: string | null;
  emailMarketingOptIn: boolean;
  emailTransactionalOptIn: boolean;
  emailFrequency: "low" | "normal" | "high";
}
```

### Update Type

```typescript
import { type UserProfileUpdateBase } from "@decodelabs/underlay/patterns";

// All fields optional - only provided fields are updated
// Use null to explicitly clear a field
interface UserProfileUpdateBase {
  fullName?: string | null;
  displayName?: string | null;
  countryCode?: string | null;
  timeZone?: string | null;
  language?: string | null;
}

// Extend for your app
export interface UserProfileUpdate extends UserProfileUpdateBase {
  avatarUrl?: string | null;
  currencyPreference?: string | null;
  emailMarketingOptIn?: boolean;
  // ...
}
```

## Database Schema

### SQL Schema Pattern

```sql
CREATE SCHEMA IF NOT EXISTS account;

CREATE TABLE account.user_profile (
    -- Link to auth user
    user_id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,

    -- Identity (culturally-inclusive naming)
    full_name TEXT CHECK (full_name IS NULL OR char_length(full_name) <= 256),
    display_name TEXT CHECK (display_name IS NULL OR char_length(display_name) <= 64),
    avatar_url TEXT CHECK (avatar_url IS NULL OR char_length(avatar_url) <= 512),

    -- Locale & Region
    country_code TEXT CHECK (country_code IS NULL OR char_length(country_code) = 2),
    time_zone TEXT CHECK (time_zone IS NULL OR char_length(time_zone) <= 64),
    language TEXT CHECK (language IS NULL OR char_length(language) <= 10),

    -- App-specific fields go here
    -- region_code TEXT,
    -- currency_preference TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Rust Database Layer

```rust
// Pattern for nullable fields that can be explicitly cleared
pub struct UserProfileUpdate {
    pub full_name: Option<Option<String>>,      // None = don't update, Some(None) = clear
    pub display_name: Option<Option<String>>,
    pub time_zone: Option<Option<String>>,
    // ...
}
```

## API Endpoints

### Get Profile

```
GET /v1/account/profile

Returns: UserProfile (auto-creates if doesn't exist)
```

### Update Profile

```
PATCH /v1/account/profile

Body: UserProfileUpdate
Returns: UserProfile
```

The GET endpoint should auto-create an empty profile if one doesn't exist, so clients don't need to handle the "no profile" case.

## Frontend Integration

### Auth Store Extension

Add profile to your auth store for app-wide access:

```typescript
// lib/stores/auth.ts
import { type UserProfile } from "@cattle-grid";

export interface AuthState {
  user: LoginUser | null;
  profile: UserProfile | null;        // Add profile
  profileLoading: boolean;            // Add loading state
  // ...
}

export interface AuthStore {
  // ...
  loadProfile: () => Promise<UserProfile | null>;
  clearProfile: () => void;
}
```

Load the profile automatically after authentication:

```typescript
async login(email: string, password: string): Promise<LoginUser> {
  // ... login logic ...

  // Load profile in background after successful login
  this.loadProfile();

  return user;
}

async loadProfile(): Promise<UserProfile | null> {
  const token = getToken();
  if (!token) return null;

  try {
    const profile = await accountCommands.getProfile(fetch, token);
    state.update(s => ({ ...s, profile, profileLoading: false }));
    return profile;
  } catch {
    state.update(s => ({ ...s, profile: null, profileLoading: false }));
    return null;
  }
}
```

### Profile Settings Page

```svelte
<script lang="ts">
  import { Field, TextInput, Select } from "@decodelabs/underlay/components";
  import { accountCommands, type UserProfileUpdate } from "@cattle-grid";
  import { auth } from "$lib/stores/auth";

  let fullName = $state("");
  let displayName = $state("");
  let timeZone = $state("");

  async function handleSubmit() {
    const updates: UserProfileUpdate = {
      fullName: fullName || null,
      displayName: displayName || null,
      timeZone: timeZone || null,
    };

    await accountCommands.updateProfile(updates, fetch, auth.getToken());
    await auth.loadProfile(); // Refresh cached profile
  }
</script>

<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <Field label="Full Name" hint="Your full name as you wish to be known">
    <TextInput bind:value={fullName} maxlength={256} />
  </Field>

  <Field label="Display Name" hint="Short name shown in the UI">
    <TextInput bind:value={displayName} maxlength={64} />
  </Field>

  <!-- More fields... -->
</form>
```

### Timezone Integration

See [080-timezone-handling.md](./080-timezone-handling.md) for complete timezone integration.

Initialize timezone from the profile:

```svelte
<script lang="ts">
  import { initTimezone, resetTimezone } from "@decodelabs/underlay/patterns";
  import { userProfile, currentUser } from "$lib/stores/auth";

  // Initialize timezone when profile loads
  $effect(() => {
    if ($userProfile && !timezoneStore.initialized) {
      initTimezone({
        profileTimezone: $userProfile.timeZone,
        onConflict: (profile, browser) => showConflictBanner(profile, browser),
        onAutoFill: async (browserTz) => {
          // Save browser timezone to profile for new users
          await updateProfile({ timeZone: browserTz });
        }
      });
    }
  });

  // Reset timezone on logout
  let prevUser = $state(null);
  $effect(() => {
    if (prevUser && !$currentUser) {
      resetTimezone();
    }
    prevUser = $currentUser;
  });
</script>
```

## Display Name in UI

### User Menu

```svelte
<script lang="ts">
  import { getEffectiveDisplayName } from "@decodelabs/underlay/patterns";
  import { userProfile, currentUser } from "$lib/stores/auth";

  const displayName = $derived(
    getEffectiveDisplayName($userProfile, $currentUser?.email)
  );
</script>

<span class="user-name">{displayName}</span>
```

### Avatar Initials

```typescript
function getInitials(displayName: string): string {
  return displayName.charAt(0).toUpperCase();
}

// Or for fuller initials:
function getInitials(fullName: string | null): string {
  if (!fullName) return "?";

  // For CJK, use first character
  if (/[\u4e00-\u9fff\u3040-\u30ff\uac00-\ud7af]/.test(fullName)) {
    return fullName.charAt(0);
  }

  // For Western names, use first letter of each word (max 2)
  return fullName
    .split(' ')
    .slice(0, 2)
    .map(n => n.charAt(0).toUpperCase())
    .join('');
}
```

## Validation Rules

| Field | Validation |
|-------|------------|
| `fullName` | Max 256 characters |
| `displayName` | Max 64 characters |
| `countryCode` | ISO 3166-1 alpha-2 (2 uppercase letters) |
| `timeZone` | Valid IANA identifier |
| `language` | BCP 47 tag (e.g., "en", "en-GB") |
| `currencyPreference` | ISO 4217 (3 uppercase letters) |

## Best Practices

### Do

- Store names as the user provides them (preserve case, diacritics)
- Use `displayName` for short UI elements (headers, greetings)
- Fall back gracefully when names aren't set
- Auto-derive display name from full name when not explicitly set
- Handle CJK names without splitting them

### Don't

- Split names into first/last (not culturally universal)
- Assume a maximum number of name parts
- Truncate names without user consent
- Require names during registration (allow setting later)
- Store display_name in auth.users (use profile instead)

## Migration from first_name/last_name

If migrating from the old pattern:

```sql
-- Add new columns
ALTER TABLE account.user_profile ADD COLUMN full_name TEXT;
ALTER TABLE account.user_profile ADD COLUMN temp_display_name TEXT;

-- Migrate data
UPDATE account.user_profile SET
    full_name = TRIM(COALESCE(first_name, '') || ' ' || COALESCE(last_name, '')),
    temp_display_name = COALESCE(display_name, first_name);

-- Clean up empty strings
UPDATE account.user_profile SET full_name = NULL WHERE full_name = '';
UPDATE account.user_profile SET temp_display_name = NULL WHERE temp_display_name = '';

-- Drop old columns and rename
ALTER TABLE account.user_profile DROP COLUMN first_name;
ALTER TABLE account.user_profile DROP COLUMN last_name;
ALTER TABLE account.user_profile DROP COLUMN display_name;
ALTER TABLE account.user_profile RENAME COLUMN temp_display_name TO display_name;
```

## See Also

- **[060-authentication.md](./060-authentication.md)** - Auth layer (separate from profiles)
- **[080-timezone-handling.md](./080-timezone-handling.md)** - Timezone integration with profiles
- **[090-ui-kit.md](./090-ui-kit.md)** - UI components for profile forms
