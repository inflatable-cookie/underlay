/**
 * Check if a target href matches the current browser pathname.
 *
 * Compares pathnames only, ignoring query strings and fragments.
 * Returns true in non-browser environments for SSR safety.
 */
export function matchesCurrentPath(targetHref: string): boolean {
  if (typeof window === "undefined") return true;

  try {
    const targetUrl = new URL(targetHref, window.location.origin);
    return window.location.pathname === targetUrl.pathname;
  } catch {
    return window.location.pathname === targetHref;
  }
}
