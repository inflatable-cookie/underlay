/**
 * Slug utilities for generating and validating URL-friendly identifiers.
 */

/** List of reserved slugs that could conflict with routes */
export const RESERVED_SLUGS = [
  "new",
  "edit",
  "delete",
  "create",
  "update",
  "list",
  "admin",
  "api",
  "auth",
  "login",
  "logout",
  "register",
  "settings",
  "profile",
  "dashboard",
  "search",
] as const;

/**
 * Convert a string to a URL-friendly slug.
 *
 * @example
 * slugify("Hello World!")     // "hello-world"
 * slugify("FA1 2024")         // "fa1-2024"
 * slugify("  Spaces  ")       // "spaces"
 * slugify("Über Café")        // "uber-cafe"
 */
export function slugify(input: string): string {
  return (
    input
      .normalize("NFD") // Decompose accents
      .replace(/[\u0300-\u036f]/g, "") // Remove accent marks
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9\s-]/g, "") // Remove non-alphanumeric
      .replace(/[\s_]+/g, "-") // Spaces/underscores to hyphens
      .replace(/-+/g, "-") // Collapse multiple hyphens
      .replace(/^-|-$/g, "") // Trim leading/trailing hyphens
  );
}

/**
 * Validate slug format without checking availability.
 * A valid slug:
 * - Contains only lowercase letters, numbers, and hyphens
 * - Starts and ends with a letter or number
 * - Is between 2 and maxLength characters (default 100)
 * - Has no consecutive hyphens
 */
export function isValidSlugFormat(slug: string, maxLength: number = 100): boolean {
  return (
    /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug) &&
    slug.length >= 2 &&
    slug.length <= maxLength
  );
}

/**
 * Check if a slug is reserved (would conflict with routes).
 */
export function isReservedSlug(slug: string): boolean {
  return (RESERVED_SLUGS as readonly string[]).includes(slug);
}

/**
 * Validate a slug and return detailed error information.
 */
export type SlugValidationError =
  | "too_short"
  | "too_long"
  | "invalid_format"
  | "reserved";

export function validateSlug(
  slug: string
): { valid: true } | { valid: false; reason: SlugValidationError } {
  if (slug.length < 2) {
    return { valid: false, reason: "too_short" };
  }
  if (slug.length > 100) {
    return { valid: false, reason: "too_long" };
  }
  if (!isValidSlugFormat(slug)) {
    return { valid: false, reason: "invalid_format" };
  }
  if (isReservedSlug(slug)) {
    return { valid: false, reason: "reserved" };
  }
  return { valid: true };
}

/**
 * Result from the async slug validation function.
 */
export interface SlugValidationResult {
  available: boolean;
  reason?: "already_exists" | "invalid_format" | "reserved" | "too_short" | "too_long" | "missing_prerequisite";
  /** Custom message to display (overrides default reason-based message) */
  message?: string;
  suggestion?: string;
}
