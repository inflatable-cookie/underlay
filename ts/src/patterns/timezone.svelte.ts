/**
 * Timezone management for Underlay applications.
 *
 * Handles detecting browser timezone, comparing with user profile timezone,
 * resolving conflicts, and providing a reactive store for components.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { initTimezone, timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";
 *
 *   initTimezone({
 *     profileTimezone: userProfile?.time_zone ?? null,
 *     onConflict: (profile, browser) => showConflictDialog(profile, browser),
 *     onAutoFill: async (tz) => saveToProfile(tz)
 *   });
 * </script>
 *
 * <TimeAgo datetime={item.createdAt} timezone={$timezoneStore.effective} />
 * ```
 */

import { BROWSER } from "esm-env";

/**
 * Timezone store state.
 */
export interface TimezoneState {
  /** The resolved timezone to use for formatting (IANA identifier) */
  effective: string;
  /** Timezone from user profile (null if not set) */
  profile: string | null;
  /** Timezone detected from browser */
  browser: string;
  /** Whether there's an unresolved conflict between profile and browser */
  hasConflict: boolean;
  /** Whether the timezone has been initialized */
  initialized: boolean;
}

/**
 * Options for initializing the timezone system.
 */
export interface InitTimezoneOptions {
  /**
   * Timezone from user's profile (null if not set).
   * Should be an IANA identifier like "Europe/London".
   */
  profileTimezone: string | null;

  /**
   * Called when profile timezone differs from browser timezone.
   * Use this to show a conflict resolution UI.
   *
   * @param profileTz - The timezone stored in the user's profile
   * @param browserTz - The timezone detected from the browser
   */
  onConflict?: (profileTz: string, browserTz: string) => void;

  /**
   * Called when profile has no timezone and browser timezone is detected.
   * Use this to auto-save the browser timezone to the user's profile.
   *
   * @param browserTz - The timezone detected from the browser
   */
  onAutoFill?: (browserTz: string) => void | Promise<void>;
}

// Internal state
let state = $state<TimezoneState>({
  effective: "UTC",
  profile: null,
  browser: "UTC",
  hasConflict: false,
  initialized: false
});

/**
 * Reactive timezone store.
 *
 * Provides access to the effective timezone and conflict state.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { timezoneStore } from "@inflatable-cookie/underlay/runtime/browser";
 *
 *   // Reactive access
 *   const tz = $derived($timezoneStore.effective);
 * </script>
 * ```
 */
export const timezoneStore = {
  get effective() {
    return state.effective;
  },
  get profile() {
    return state.profile;
  },
  get browser() {
    return state.browser;
  },
  get hasConflict() {
    return state.hasConflict;
  },
  get initialized() {
    return state.initialized;
  }
};

/**
 * Detect the browser's timezone using Intl API.
 *
 * @returns IANA timezone identifier (e.g., "Europe/London")
 */
export function detectBrowserTimezone(): string {
  if (!BROWSER) return "UTC";

  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone;
  } catch {
    return "UTC";
  }
}

/**
 * Initialize the timezone system.
 *
 * Call this after authentication when the user profile is available.
 * Handles the following scenarios:
 *
 * 1. **No profile timezone**: Uses browser timezone, calls `onAutoFill`
 * 2. **Profile matches browser**: Uses profile timezone (confirmed)
 * 3. **Profile differs from browser**: Uses profile as default, calls `onConflict`
 *
 * @param options - Configuration options
 *
 * @example
 * ```typescript
 * initTimezone({
 *   profileTimezone: userProfile?.time_zone ?? null,
 *   onConflict: (profile, browser) => {
 *     showTimezoneConflictBanner(profile, browser);
 *   },
 *   onAutoFill: async (browserTz) => {
 *     await api.updateProfile({ time_zone: browserTz });
 *   }
 * });
 * ```
 */
export function initTimezone(options: InitTimezoneOptions): void {
  const browserTz = detectBrowserTimezone();
  const profileTz = options.profileTimezone;

  state.browser = browserTz;
  state.profile = profileTz;

  if (!profileTz) {
    // No profile timezone - use browser, auto-fill profile
    state.effective = browserTz;
    state.hasConflict = false;
    state.initialized = true;

    if (options.onAutoFill) {
      // Fire and forget - don't block initialization
      void Promise.resolve(options.onAutoFill(browserTz));
    }
  } else if (profileTz === browserTz) {
    // Profile matches browser - use profile (confirmed)
    state.effective = profileTz;
    state.hasConflict = false;
    state.initialized = true;
  } else {
    // Profile differs from browser - use profile as default, flag conflict
    state.effective = profileTz;
    state.hasConflict = true;
    state.initialized = true;

    if (options.onConflict) {
      options.onConflict(profileTz, browserTz);
    }
  }
}

/**
 * Resolve a timezone conflict by choosing which timezone to use.
 *
 * @param choice - Which timezone to use: "profile" or "browser"
 *
 * @example
 * ```svelte
 * <button onclick={() => resolveTimezoneConflict("browser")}>
 *   Use browser timezone
 * </button>
 * ```
 */
export function resolveTimezoneConflict(choice: "profile" | "browser"): void {
  if (choice === "profile" && state.profile) {
    state.effective = state.profile;
  } else {
    state.effective = state.browser;
  }
  state.hasConflict = false;
}

/**
 * Manually set the effective timezone.
 *
 * Use this when the user explicitly changes their timezone preference.
 *
 * @param timezone - IANA timezone identifier
 */
export function setEffectiveTimezone(timezone: string): void {
  state.effective = timezone;
  state.profile = timezone;
  state.hasConflict = false;
}

/**
 * Reset the timezone store to uninitialized state.
 *
 * Call this on logout to clear timezone state.
 */
export function resetTimezone(): void {
  state = {
    effective: "UTC",
    profile: null,
    browser: detectBrowserTimezone(),
    hasConflict: false,
    initialized: false
  };
}

/**
 * Format a date in a specific timezone.
 *
 * @param date - Date to format (ISO string or Date object)
 * @param timezone - IANA timezone identifier
 * @param options - Intl.DateTimeFormat options
 * @returns Formatted date string
 *
 * @example
 * ```typescript
 * const formatted = formatInTimezone(
 *   "2026-01-30T14:30:00Z",
 *   "Europe/London",
 *   { dateStyle: "long", timeStyle: "short" }
 * );
 * // "30 January 2026 at 14:30"
 * ```
 */
export function formatInTimezone(
  date: string | Date,
  timezone: string,
  options: Intl.DateTimeFormatOptions = {}
): string {
  const dateObj = typeof date === "string" ? new Date(date) : date;

  if (isNaN(dateObj.getTime())) {
    return "Invalid date";
  }

  try {
    return dateObj.toLocaleString(undefined, {
      ...options,
      timeZone: timezone
    });
  } catch {
    // Fallback if timezone is invalid
    return dateObj.toLocaleString(undefined, options);
  }
}

/**
 * Format a date for display, using the effective timezone from the store.
 *
 * Convenience wrapper around `formatInTimezone` that uses the current
 * effective timezone.
 *
 * @param date - Date to format (ISO string or Date object)
 * @param options - Intl.DateTimeFormat options
 * @returns Formatted date string
 */
export function formatDate(
  date: string | Date,
  options: Intl.DateTimeFormatOptions = {}
): string {
  return formatInTimezone(date, state.effective, options);
}
