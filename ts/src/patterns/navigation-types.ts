/**
 * Navigation context representing where the user came from.
 */
export interface NavigationContext {
  /** Display label for back button (e.g., "Videos", "Module: Intro") */
  label: string;
  /** URL to navigate back to */
  href: string;
  /** Type of page - used for breadcrumb collapse rules */
  type: "list" | "detail" | "edit";
  /** Target URL this context is intended for (used for validation) */
  targetHref?: string;
  /** Optional page state snapshot (tabs, pagination, filters, etc.) */
  state?: Record<string, unknown>;
}

/**
 * Configuration options for the navigation context system.
 */
export interface NavigationContextConfig {
  /** Storage key for the context stack (default: "underlay:nav-context") */
  storageKey?: string;
  /** Maximum breadcrumb depth (default: 3) */
  maxDepth?: number;
}

/**
 * Return type for getBackButtonInfo().
 */
export interface BackButtonInfo {
  /** Label for the back button */
  label: string;
  /** Href for the back button */
  href: string;
  /** True when derived from stored navigation context (not fallback) */
  isContextual?: boolean;
}
