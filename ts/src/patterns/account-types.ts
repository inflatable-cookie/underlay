/**
 * Account types - user profiles and identity/personalization.
 *
 * This establishes the Underlay pattern for identity:
 * - `auth.users` = authentication only (email, password hash, verification status)
 * - `account.user_profile` = identity and personalization (names, timezone, preferences)
 *
 * Name fields follow a culturally-inclusive pattern:
 * - `fullName`: User's full name as they wish to be known (legal name, preferred name, etc.)
 * - `displayName`: Short name for UI display (defaults to first word of fullName if not set)
 *
 * This replaces the Western-centric first_name/last_name pattern.
 */

/**
 * Baseline user profile for identity and personalization.
 *
 * Applications should extend this with domain-specific fields.
 */
export interface UserProfileBase {
  userId: string;

  /** User's full name as they wish to be known. */
  fullName: string | null;

  /** Short name for UI display (defaults to first word of fullName if not provided). */
  displayName: string | null;

  /** ISO 3166-1 alpha-2 country code (e.g., "GB", "US"). */
  countryCode: string | null;

  /** IANA time zone identifier (e.g., "Europe/London", "America/New_York"). */
  timeZone: string | null;

  /** BCP 47 language tag (e.g., "en", "en-GB", "zh-Hans"). */
  language: string | null;

  createdAt: string;
  updatedAt: string;
}

/**
 * Update payload for user profile.
 *
 * All fields are optional - only provided fields will be updated.
 * Use `null` to explicitly clear a field.
 */
export interface UserProfileUpdateBase {
  fullName?: string | null;
  displayName?: string | null;
  countryCode?: string | null;
  timeZone?: string | null;
  language?: string | null;
}

/**
 * Derive a display name from a full name.
 *
 * Uses the first word of the full name, handling various name formats:
 * - "Alice Smith" → "Alice"
 * - "李明" → "李明" (doesn't split CJK names)
 * - "María José García" → "María"
 *
 * @param fullName The user's full name
 * @returns The derived display name, or null if fullName is empty/null
 */
export function deriveDisplayName(fullName: string | null | undefined): string | null {
  if (!fullName?.trim()) {
    return null;
  }

  const trimmed = fullName.trim();

  // Check if the name contains CJK characters - if so, use the whole name
  // CJK names are typically short and shouldn't be split
  if (/[\u4e00-\u9fff\u3040-\u30ff\uac00-\ud7af]/.test(trimmed)) {
    return trimmed;
  }

  // For Western-style names, use the first word
  const firstSpace = trimmed.indexOf(' ');
  return firstSpace > 0 ? trimmed.substring(0, firstSpace) : trimmed;
}

/**
 * Get the effective display name for a user.
 *
 * Falls back through: displayName → fullName → email username → "User"
 *
 * @param profile The user profile (can be partial)
 * @param email Optional email for fallback
 * @returns The best available display name
 */
export function getEffectiveDisplayName(
  profile: { displayName?: string | null; fullName?: string | null } | null | undefined,
  email?: string | null
): string {
  if (profile?.displayName) {
    return profile.displayName;
  }

  const derived = deriveDisplayName(profile?.fullName);
  if (derived) {
    return derived;
  }

  if (email) {
    const atIndex = email.indexOf('@');
    return atIndex > 0 ? email.substring(0, atIndex) : email;
  }

  return 'User';
}
