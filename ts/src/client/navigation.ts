/**
 * SvelteKit-specific navigation context helpers.
 *
 * This module provides SvelteKit integration for the navigation context system,
 * including goto wrappers and browser history utilities.
 *
 * For framework-agnostic utilities, see `@decodelabs/underlay/patterns`.
 *
 * @example
 * ```typescript
 * import { gotoWithContext, navigateBack } from '@decodelabs/underlay/client';
 *
 * // Navigate to edit while remembering where user came from
 * await gotoWithContext(`/items/${id}/edit`, {
 *   label: "Items",
 *   href: "/items",
 *   type: "list"
 * });
 *
 * // Navigate back using stored context
 * navigateBack("/fallback-url");
 * ```
 *
 * @module
 */

import { goto } from "$app/navigation";
import { browser } from "$app/environment";
import {
  pushNavigationContext,
  popNavigationContext,
  deriveParentPath,
  type NavigationContext
} from "../patterns/navigation";

// Re-export types for convenience
export type { NavigationContext } from "../patterns/navigation";

/**
 * Navigate to a URL while pushing the current location as context.
 *
 * This is the primary way to navigate to edit pages - it captures
 * where the user came from so the edit page can show a contextual
 * back button.
 *
 * @param targetHref - The URL to navigate to
 * @param context - The context to push (label/href/type of current page)
 * @param options - Optional goto options
 *
 * @example
 * ```typescript
 * // From a list page, navigating to edit:
 * await gotoWithContext(`/items/${id}/edit`, {
 *   label: "Items",
 *   href: "/items",
 *   type: "list"
 * });
 *
 * // From a detail page:
 * await gotoWithContext(`/items/${id}/edit`, {
 *   label: item.name,
 *   href: `/items/${id}`,
 *   type: "detail"
 * });
 * ```
 */
export async function gotoWithContext(
  targetHref: string,
  context: NavigationContext,
  options?: Parameters<typeof goto>[1]
): Promise<void> {
  // Store the target URL with the context so we can validate it when consuming
  pushNavigationContext({
    ...context,
    targetHref
  });
  await goto(targetHref, options);
}

/**
 * Navigate back using the navigation context stack.
 *
 * If context exists, navigates to the stored href.
 * Otherwise falls back to the provided default or derives one from the current URL.
 *
 * @param fallbackHref - Optional fallback URL if no context exists
 * @returns The href that was navigated to
 *
 * @example
 * ```typescript
 * // Simple usage - falls back to derived parent URL
 * navigateBack();
 *
 * // With explicit fallback
 * navigateBack(`/items/${itemId}`);
 * ```
 */
export function navigateBack(fallbackHref?: string): string {
  const context = popNavigationContext();

  if (context) {
    void goto(context.href);
    return context.href;
  }

  // Fall back to provided href or derive from current URL
  const target = fallbackHref ?? (browser ? deriveParentPath(window.location.pathname) : "/");
  void goto(target);
  return target;
}

/**
 * Legacy function for cancel button navigation.
 *
 * Navigates to the provided href, or derives a sensible parent URL,
 * or uses browser history as a last resort.
 *
 * @deprecated Use navigateBack() for context-aware navigation.
 *
 * @param cancelHref - Optional explicit URL to navigate to
 */
export function navigateOnCancel(cancelHref: string | undefined): void {
  if (!browser) return;

  if (cancelHref) {
    window.location.href = cancelHref;
    return;
  }

  // Derive a sensible parent URL from the current location
  const parentPath = deriveParentPath(window.location.pathname);
  if (parentPath !== "/") {
    const url = new URL(window.location.href);
    url.pathname = parentPath;
    window.location.href = url.toString();
    return;
  }

  // Fall back to browser history
  if (window.history.length > 1) {
    window.history.back();
  } else {
    window.location.href = "/";
  }
}
