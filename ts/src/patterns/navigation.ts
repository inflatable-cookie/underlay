/**
 * Navigation context utilities for contextual back buttons.
 *
 * This module provides framework-agnostic utilities for tracking navigation
 * context, enabling edit forms to know where users came from and display
 * appropriate back button labels and redirect targets.
 *
 * For SvelteKit-specific helpers (gotoWithContext), see `@decodelabs/underlay/client`.
 *
 * @example
 * ```typescript
 * import { pushNavigationContext, getBackButtonInfo } from '@decodelabs/underlay/patterns';
 *
 * // When navigating to an edit page, push context first
 * pushNavigationContext({
 *   label: "Videos",
 *   href: "/content/videos",
 *   type: "list"
 * });
 *
 * // In edit form, get contextual back button info
 * const { label, href } = getBackButtonInfo("Back", "/fallback");
 * // label = "Back to Videos", href = "/content/videos"
 * ```
 *
 * @module
 */

import { matchesCurrentPath } from "./navigation-path";
import { buildPushedContextStack } from "./navigation-stack";
import type {
  BackButtonInfo,
  NavigationContext,
  NavigationContextConfig
} from "./navigation-types";
import {
  readNavigationContextStack,
  writeNavigationContextStack,
  popNavigationContextStack,
  peekNavigationContextStack,
  clearNavigationContextStack
} from "./navigation-context-store";
import {
  resolveBackButtonInfo,
  consumeBackNavigation,
  computeResolvedBackInfo
} from "./navigation-back-info";
import {
  getNavigationContextConfig,
  setNavigationContextConfig
} from "./navigation-config";
export type {
  NavigationContext,
  NavigationContextConfig,
  BackButtonInfo
} from "./navigation-types";
export {
  storePageState,
  retrievePageState,
  consumePageState,
  clearPageStates
} from "./navigation-state";

// ============================================================================
// Configuration
// ============================================================================

/**
 * Configure the navigation context system.
 *
 * Call this early in your app initialization if you need custom settings.
 *
 * @example
 * ```typescript
 * configureNavigationContext({
 *   storageKey: "myapp:nav-context",
 *   maxDepth: 5
 * });
 * ```
 */
export function configureNavigationContext(options: NavigationContextConfig): void {
  setNavigationContextConfig(options);
}

// ============================================================================
// Core Functions
// ============================================================================

/**
 * Push a navigation context onto the stack.
 *
 * Applies sanity rules:
 * - Max depth (default 3)
  * - Same-type collapse: list→list replaces top
 * - Deduplication: same href already in stack moves to top
 *
 * @example
 * ```typescript
 * // From a list page, before navigating to edit
 * pushNavigationContext({
 *   label: "Videos",
 *   href: "/content/videos",
 *   type: "list"
 * });
 * ```
 */
export function pushNavigationContext(context: NavigationContext): void {
  const config = getNavigationContextConfig();
  const stack = readNavigationContextStack<NavigationContext>(config.storageKey);
  const nextStack = buildPushedContextStack(stack, context, config.maxDepth);
  writeNavigationContextStack(config.storageKey, nextStack);
}

/**
 * Pop the most recent navigation context from the stack.
 *
 * Returns the context that was removed, or null if stack was empty.
 *
 * @example
 * ```typescript
 * const context = popNavigationContext();
 * if (context) {
 *   navigateTo(context.href);
 * }
 * ```
 */
export function popNavigationContext(): NavigationContext | null {
  const config = getNavigationContextConfig();
  return popNavigationContextStack<NavigationContext>(config.storageKey);
}

/**
 * Peek at the most recent navigation context without removing it.
 *
 * Returns null if the stack is empty.
 *
 * @example
 * ```typescript
 * const context = peekNavigationContext();
 * console.log(`User came from: ${context?.label}`);
 * ```
 */
export function peekNavigationContext(): NavigationContext | null {
  const config = getNavigationContextConfig();
  return peekNavigationContextStack<NavigationContext>(config.storageKey);
}

/**
 * Get the full navigation context stack.
 *
 * Useful for debugging or implementing full breadcrumb trails.
 */
export function getNavigationContextStack(): NavigationContext[] {
  const config = getNavigationContextConfig();
  return readNavigationContextStack<NavigationContext>(config.storageKey);
}

/**
 * Clear all navigation context.
 */
export function clearNavigationContext(): void {
  const config = getNavigationContextConfig();
  clearNavigationContextStack(config.storageKey);
}

/**
 * Get the return URL for form submissions.
 *
 * This is used to populate a hidden form field so the server action
 * knows where to redirect after a successful save.
 *
 * @param fallbackHref - Fallback URL if no context exists
 * @returns The URL to return to
 *
 * @example
 * ```typescript
 * const returnTo = getReturnUrl(`/items/${itemId}`);
 * // Use in hidden form field: <input type="hidden" name="returnTo" value={returnTo} />
 * ```
 */
export function getReturnUrl(fallbackHref: string): string {
  const context = peekNavigationContext();
  if (context?.targetHref && !matchesCurrentPath(context.targetHref)) {
    return fallbackHref;
  }
  return context?.href ?? fallbackHref;
}

/**
 * Get context info for displaying a back button.
 *
 * @param fallbackLabel - Default label if no context exists
 * @param fallbackHref - Default href if no context exists
 * @returns Object with label and href for the back button
 *
 * @example
 * ```typescript
 * const { label, href } = getBackButtonInfo("Back to item", `/items/${itemId}`);
 * // If context exists: { label: "Back to Videos", href: "/content/videos" }
 * // If no context:     { label: "Back to item", href: "/items/123" }
 * ```
 */
export function getBackButtonInfo(
  fallbackLabel: string,
  fallbackHref: string
): BackButtonInfo {
  return resolveBackButtonInfo(
    peekNavigationContext(),
    fallbackLabel,
    fallbackHref,
    matchesCurrentPath
  );
}

/**
 * Consume and return the navigation context for use in edit/create pages.
 *
 * This function **pops** the context from the stack (consuming it), then returns
 * both the back button info and return URL. This ensures the context is only
 * used once and doesn't persist across multiple page navigations.
 *
 * **Important**: If the context has a `targetHref` that doesn't match the current
 * URL pathname, the context is considered stale and discarded. This prevents
 * showing incorrect "Back to X" labels when users navigate to edit pages via
 * bookmarks, direct links, or from pages that don't push context.
 *
 * Call this once during page initialization, and use the returned values for
 * both the back button display and the hidden returnTo form field.
 *
 * @param fallbackLabel - Default label if no context exists
 * @param fallbackHref - Default href if no context exists
 * @returns Object with backInfo (label/href) and returnTo URL
 *
 * @example
 * ```typescript
 * // In edit/create page component
 * const { backInfo, returnTo } = consumeNavigationContext(
 *   "Back to videos",
 *   "/content/videos"
 * );
 *
 * // Use backInfo for PageHeader/CrudFormShell
 * // Use returnTo for hidden form field
 * ```
 */
export function consumeNavigationContext(
  fallbackLabel: string,
  fallbackHref: string
): { backInfo: BackButtonInfo; returnTo: string } {
  return consumeBackNavigation(
    popNavigationContext(),
    fallbackLabel,
    fallbackHref,
    matchesCurrentPath
  );
}

/**
 * Derive a sensible parent URL from the given path.
 *
 * Strips the last non-empty path segment:
 * - `/content/videos/new` → `/content/videos`
 * - `/learning/pathways/123/edit` → `/learning/pathways/123`
 * - `/learning/pathways/123` → `/learning/pathways`
 *
 * @param currentPath - The current URL path
 * @returns The derived parent path, or "/" if at root
 */
export function deriveParentPath(currentPath: string): string {
  const segments = currentPath.split("/").filter(Boolean);

  if (segments.length > 0) {
    segments.pop();
    return "/" + segments.join("/");
  }

  return "/";
}

// ============================================================================
// Back Info Helpers
// ============================================================================

/**
 * Compute back button info, respecting contextual navigation when available.
 *
 * This helper ensures that contextual back links (from navigation context)
 * take precedence over hardcoded fallbacks. Use this in edit/create pages
 * when you want to provide a sensible fallback but still respect the user's
 * actual navigation path.
 *
 * @param backInfo - The back info from consumeNavigationContext()
 * @param fallback - Optional fallback href and label when no contextual navigation
 * @returns BackButtonInfo that respects contextual navigation
 *
 * @example
 * ```typescript
 * // In a Svelte component
 * const { backInfo } = consumeNavigationContext("Back to module", defaultBackHref);
 *
 * // Compute final back info with data-dependent fallback
 * const computedBackInfo = $derived(
 *   computeBackInfo(backInfo, module ? {
 *     href: `/learning/modules/${module.moduleId}`,
 *     label: `Back to ${module.code}`
 *   } : undefined)
 * );
 * ```
 */
export function computeBackInfo(
  backInfo: BackButtonInfo,
  fallback?: { href: string; label: string }
): BackButtonInfo {
  return computeResolvedBackInfo(backInfo, fallback);
}
