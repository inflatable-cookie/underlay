import { goto } from "$app/navigation";
import { browser } from "$app/environment";
import {
  pushNavigationContext,
  popNavigationContext,
  deriveParentPath,
  storePageState,
  consumePageState,
  type NavigationContext,
} from "../patterns/navigation";

// Re-export types and state functions for convenience
export type { NavigationContext } from "../patterns/navigation";
export {
  storePageState,
  retrievePageState,
  consumePageState,
} from "../patterns/navigation";

/**
 * Navigate to a URL while pushing the current location as context.
 */
export async function gotoWithContext(
  targetHref: string,
  context: NavigationContext,
  options?: Parameters<typeof goto>[1],
): Promise<void> {
  if (context.state) {
    const origin = globalThis?.location?.origin;
    const pathname = context.href.startsWith("/")
      ? context.href.split("?")[0]
      : origin
        ? new URL(context.href, origin).pathname
        : context.href.split("?")[0];
    storePageState(pathname, context.state);
  }

  pushNavigationContext({
    ...context,
    targetHref,
  });
  await goto(targetHref, options);
}

/**
 * Navigate back using the navigation context stack.
 */
export function navigateBack(fallbackHref?: string): string {
  const context = popNavigationContext();

  if (context) {
    void goto(context.href);
    return context.href;
  }

  const target =
    fallbackHref ??
    (browser ? deriveParentPath(window.location.pathname) : "/");
  void goto(target);
  return target;
}

/**
 * Legacy cancel-button navigation helper.
 *
 * Prefer `navigateBack()` for context-aware navigation in new callers.
 */
export function navigateOnCancel(cancelHref: string | undefined): void {
  if (!browser) return;

  if (cancelHref) {
    window.location.href = cancelHref;
    return;
  }

  const parentPath = deriveParentPath(window.location.pathname);
  if (parentPath !== "/") {
    const url = new URL(window.location.href);
    url.pathname = parentPath;
    window.location.href = url.toString();
    return;
  }

  if (window.history.length > 1) {
    window.history.back();
  } else {
    window.location.href = "/";
  }
}

/**
 * Initialize page state from navigation context.
 * The state is consumed (removed) after retrieval, so it won't be
 * restored again on subsequent visits unless saved again.
 */
export function initPageState<T extends Record<string, unknown>>(
  defaults: T,
): T {
  if (!browser) return defaults;

  const restored = consumePageState<Partial<T>>();
  if (!restored) return defaults;

  const result = { ...defaults };
  for (const key of Object.keys(defaults) as (keyof T)[]) {
    if (key in restored && restored[key] !== undefined) {
      result[key] = restored[key] as T[keyof T];
    }
  }

  return result;
}

/** Capture page state for `gotoWithContext`. */
export function capturePageState<T extends Record<string, unknown>>(
  stateValues: T,
): T {
  return stateValues;
}
