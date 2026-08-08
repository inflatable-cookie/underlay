# Timezone Handling

This guide covers how to handle timezones correctly in Underlay applications, including storing user preferences, detecting browser timezone, resolving conflicts, and formatting dates.

## Principles

1. **Store timestamps as UTC** - All `TIMESTAMPTZ` columns store UTC. Never store local times.
2. **Store timezone as IANA identifier** - Use `Europe/London`, not `+00:00`. IANA handles DST automatically.
3. **Render in user's timezone** - Convert UTC to user's preferred timezone for display.
4. **Detect and resolve conflicts** - Browser timezone may differ from profile; handle gracefully.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Application                               │
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────┐ │
│  │   Profile   │    │   Browser   │    │  Effective Timezone │ │
│  │  Timezone   │───▶│  Timezone   │───▶│       Store         │ │
│  │  (database) │    │  (detected) │    │   (resolved)        │ │
│  └─────────────┘    └─────────────┘    └─────────────────────┘ │
│                                                 │               │
│                                                 ▼               │
│                                        ┌─────────────────┐     │
│                                        │  TimeAgo, etc.  │     │
│                                        │  (components)   │     │
│                                        └─────────────────┘     │
└─────────────────────────────────────────────────────────────────┘
```

## The Timezone Store

Underlay provides a timezone management system via `useTimezone()`.

### Setup

In your app's root layout, initialize the timezone system after authentication:

```svelte
<script lang="ts">
  import { initTimezone, timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";
  import { currentUser } from "$lib/stores/auth";

  // Initialize when user profile is available
  $effect(() => {
    if ($currentUser) {
      initTimezone({
        profileTimezone: $currentUser.profile?.time_zone ?? null,
        onConflict: (profile, browser) => {
          // Show conflict resolution UI (app-specific)
          showTimezoneConflictDialog(profile, browser);
        },
        onAutoFill: async (browserTimezone) => {
          // Save browser timezone to profile (app-specific)
          await updateUserProfile({ time_zone: browserTimezone });
        }
      });
    }
  });
</script>
```

### Resolution Logic

The `initTimezone()` function follows this logic:

1. **No profile timezone** → Use browser timezone, call `onAutoFill` to save it
2. **Profile matches browser** → Use profile timezone (confirmed correct)
3. **Profile differs from browser** → Use profile as default, call `onConflict`

### Reading the Effective Timezone

Components can read the resolved timezone:

```svelte
<script lang="ts">
  import { timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";

  // Reactive access to effective timezone
  const tz = $derived($timezoneStore.effective);
</script>

<p>Your timezone: {tz}</p>
```

### Store Shape

```typescript
interface TimezoneState {
  /** The resolved timezone to use for formatting */
  effective: string;
  /** Timezone from user profile (if set) */
  profile: string | null;
  /** Timezone detected from browser */
  browser: string;
  /** Whether there's an unresolved conflict */
  hasConflict: boolean;
}
```

## Using Timezone in Components

### TimeAgo Component

Poodle `TimeAgo` accepts an explicit `timezone` prop when you want tooltip
output to follow the effective timezone:

```svelte
<TimeAgo datetime={item.createdAt} timezone={$timezoneStore.effective} />
```

Without `timezone`, the tooltip uses the browser locale timezone.

### Manual Formatting

For custom date formatting, use the `formatInTimezone` utility:

```typescript
import { formatInTimezone, timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";
import { get } from "svelte/store";

const tz = get(timezoneStore).effective;
const formatted = formatInTimezone(someDate, tz, {
  dateStyle: "long",
  timeStyle: "short"
});
```

Or reactively:

```svelte
<script lang="ts">
  import { formatInTimezone, timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";

  let { date } = $props();

  const formatted = $derived(
    formatInTimezone(date, $timezoneStore.effective, {
      dateStyle: "long",
      timeStyle: "short"
    })
  );
</script>

<span>{formatted}</span>
```

## Conflict Resolution UI

When the profile timezone differs from the browser, you should show a conflict resolution UI. This is app-specific, but here's a pattern:

```svelte
<script lang="ts">
  import { timezoneStore, resolveTimezoneConflict } from "@inflatable-cookie/underlay/runtime/browser";

  function useProfile() {
    resolveTimezoneConflict("profile");
  }

  function useBrowser() {
    resolveTimezoneConflict("browser");
    // Optionally update profile to match
    updateUserProfile({ time_zone: $timezoneStore.browser });
  }
</script>

{#if $timezoneStore.hasConflict}
  <div class="timezone-conflict-banner">
    <p>
      Your profile timezone is <strong>{$timezoneStore.profile}</strong>
      but your browser reports <strong>{$timezoneStore.browser}</strong>.
    </p>
    <button onclick={useProfile}>Use {$timezoneStore.profile}</button>
    <button onclick={useBrowser}>Use {$timezoneStore.browser}</button>
  </div>
{/if}
```

## Date Input Handling

### Date-Only Fields

For date-only fields (release dates, expiry dates), use a date-type `TextInput` bound to a `YYYY-MM-DD` string. These are timezone-agnostic:

```svelte
<TextInput id="releaseAt" name="releaseAt" type="date" bind:value={releaseDate} />
```

The database stores these as `DATE` type (no time component), avoiding timezone issues entirely.

### DateTime Fields

For full datetime fields, the input should capture local time and convert to UTC for storage:

```typescript
// User enters: "2026-01-30 14:30" in their local timezone
// Convert to UTC ISO string for API:
const localDate = new Date(`2026-01-30T14:30:00`);
const utcString = localDate.toISOString(); // Sends UTC to server
```

When displaying, convert back using the effective timezone.

## Best Practices

### Do

- ✅ Store all timestamps as UTC in the database
- ✅ Store timezone preferences as IANA identifiers (`Europe/London`)
- ✅ Use `TimeAgo` component for relative times with timezone-aware tooltips
- ✅ Initialize timezone early in app lifecycle (after auth)
- ✅ Handle conflicts gracefully with clear UI
- ✅ Auto-fill profile timezone from browser when not set

### Don't

- ❌ Store timezone as offset (`+05:30`) - doesn't handle DST
- ❌ Store local times in the database
- ❌ Assume browser timezone is always correct (VPNs, travel)
- ❌ Ignore timezone conflicts silently
- ❌ Format dates without considering user's timezone preference

## Common Timezones

For reference, common IANA timezone identifiers:

| Region | Timezone ID |
|--------|-------------|
| UK | `Europe/London` |
| US East | `America/New_York` |
| US West | `America/Los_Angeles` |
| Central Europe | `Europe/Paris` |
| India | `Asia/Kolkata` |
| Japan | `Asia/Tokyo` |
| Australia East | `Australia/Sydney` |
| UTC | `UTC` |

## API Considerations

### Profile Endpoints

Your API should support:

```
GET /api/account/profile
  → { time_zone: "Europe/London", ... }

PATCH /api/account/profile
  ← { time_zone: "America/New_York" }
```

### Timestamp Format

All timestamps in API responses should be ISO 8601 with timezone indicator:

```json
{
  "createdAt": "2026-01-30T14:30:00Z",
  "updatedAt": "2026-01-30T15:45:00Z"
}
```

The `Z` suffix indicates UTC. Frontend converts to local display.
