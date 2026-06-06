import { matchesCurrentPath } from "./navigation-path";
import { buildPushedContextStack } from "./navigation-stack";
import type {
  BackButtonInfo,
  NavigationContext,
  NavigationContextConfig,
} from "./navigation-types";
import {
  readNavigationContextStack,
  writeNavigationContextStack,
  popNavigationContextStack,
  peekNavigationContextStack,
  clearNavigationContextStack,
} from "./navigation-context-store";
import {
  resolveBackButtonInfo,
  consumeBackNavigation,
  computeResolvedBackInfo,
} from "./navigation-back-info";
import {
  getNavigationContextConfig,
  setNavigationContextConfig,
} from "./navigation-config";
export type {
  NavigationContext,
  NavigationContextConfig,
  BackButtonInfo,
} from "./navigation-types";
export {
  storePageState,
  retrievePageState,
  consumePageState,
  clearPageStates,
} from "./navigation-state";

/**
 * Configure the navigation context system.
 */
export function configureNavigationContext(
  options: NavigationContextConfig,
): void {
  setNavigationContextConfig(options);
}

/**
 * Push a navigation context onto the stack.
 *
 * Applies max-depth, same-type collapse, and href deduplication.
 */
export function pushNavigationContext(context: NavigationContext): void {
  const config = getNavigationContextConfig();
  const stack = readNavigationContextStack<NavigationContext>(
    config.storageKey,
  );
  const nextStack = buildPushedContextStack(stack, context, config.maxDepth);
  writeNavigationContextStack(config.storageKey, nextStack);
}

/** Pop the most recent navigation context from the stack. */
export function popNavigationContext(): NavigationContext | null {
  const config = getNavigationContextConfig();
  return popNavigationContextStack<NavigationContext>(config.storageKey);
}

/** Peek at the most recent navigation context without removing it. */
export function peekNavigationContext(): NavigationContext | null {
  const config = getNavigationContextConfig();
  return peekNavigationContextStack<NavigationContext>(config.storageKey);
}

/** Get the full navigation context stack. */
export function getNavigationContextStack(): NavigationContext[] {
  const config = getNavigationContextConfig();
  return readNavigationContextStack<NavigationContext>(config.storageKey);
}

/** Clear all navigation context. */
export function clearNavigationContext(): void {
  const config = getNavigationContextConfig();
  clearNavigationContextStack(config.storageKey);
}

/** Get the return URL for form submissions. */
export function getReturnUrl(fallbackHref: string): string {
  const context = peekNavigationContext();
  if (context?.targetHref && !matchesCurrentPath(context.targetHref)) {
    return fallbackHref;
  }
  return context?.href ?? fallbackHref;
}

/** Get context info for displaying a back button. */
export function getBackButtonInfo(
  fallbackLabel: string,
  fallbackHref: string,
): BackButtonInfo {
  return resolveBackButtonInfo(
    peekNavigationContext(),
    fallbackLabel,
    fallbackHref,
    matchesCurrentPath,
  );
}

/**
 * Consume and return the navigation context for use in edit/create pages.
 *
 * This function **pops** the context from the stack (consuming it), then returns
 * both the back button info and return URL. This ensures the context is only
 * used once and doesn't persist across multiple page navigations.
 *
 * If the context has a `targetHref` that doesn't match the current URL
 * pathname, the context is considered stale and discarded. This prevents
 * showing incorrect contextual labels when users navigate to edit pages via
 * bookmarks, direct links, or from pages that don't push context.
 */
export function consumeNavigationContext(
  fallbackLabel: string,
  fallbackHref: string,
): { backInfo: BackButtonInfo; returnTo: string } {
  return consumeBackNavigation(
    popNavigationContext(),
    fallbackLabel,
    fallbackHref,
    matchesCurrentPath,
  );
}

/**
 * Derive a sensible parent URL from the given path.
 *
 * Strips the last non-empty path segment:
 * - `/content/videos/new` → `/content/videos`
 * - `/learning/pathways/123/edit` → `/learning/pathways/123`
 * - `/learning/pathways/123` → `/learning/pathways`
 */
export function deriveParentPath(currentPath: string): string {
  const segments = currentPath.split("/").filter(Boolean);

  if (segments.length > 0) {
    segments.pop();
    return "/" + segments.join("/");
  }

  return "/";
}

/**
 * Compute back button info, respecting contextual navigation when available.
 */
export function computeBackInfo(
  backInfo: BackButtonInfo,
  fallback?: { href: string; label: string },
): BackButtonInfo {
  return computeResolvedBackInfo(backInfo, fallback);
}
