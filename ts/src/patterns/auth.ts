import type { Readable } from "svelte/store";

import type { AuthSession } from "../client/auth";
import type { AuthState, AuthStore } from "../client/useAuth";

export type {
  AuthFieldErrors,
  PassKeyStartPayload,
  SessionListItem
} from "./auth-workflows/types";

// ============================================================================
// Global Auth Configuration
// ============================================================================

/**
 * Configuration for global auth handlers used by useAuthenticatedData.
 */
export interface AuthConfig {
  /**
   * Function to get the current access token synchronously.
   * Should return null if not authenticated.
   */
  getToken: () => string | null;

  /**
   * Function to refresh the access token.
   * Should return the new token on success, or null if refresh failed.
   */
  onRefresh: (fetchFn: typeof fetch) => Promise<string | null>;

  /**
   * Reactive getter for whether auth is still initializing.
   * When provided (along with getCurrentUser), enables automatic initial
   * fetch in useAuthenticatedData without manual $effect wiring.
   *
   * Must be called from a reactive context (e.g. inside $effect) to
   * track changes. Typically: () => $authLoading
   */
  getAuthLoading?: () => boolean;

  /**
   * Reactive getter for the current user (null if not authenticated).
   * When provided (along with getAuthLoading), enables automatic initial
   * fetch in useAuthenticatedData without manual $effect wiring.
   *
   * Must be called from a reactive context (e.g. inside $effect) to
   * track changes. Typically: () => $currentUser
   */
  getCurrentUser?: () => unknown;
}

let globalAuthConfig: AuthConfig | null = null;

/**
 * Configure global auth handlers for useAuthenticatedData.
 * Call this once at app startup (e.g., in +layout.svelte).
 *
 * @example
 * ```svelte
 * <script>
 *   import { configureAuth } from '@decodelabs/underlay/patterns';
 *   import { auth } from '$lib/stores/auth';
 *
 *   configureAuth({
 *     getToken: () => auth.getToken(),
 *     onRefresh: auth.getRefreshHandler()
 *   });
 * </script>
 * ```
 */
export function configureAuth(config: AuthConfig): void {
  globalAuthConfig = config;
}

/**
 * Get the global auth configuration.
 * Returns null if not configured.
 */
export function getAuthConfig(): AuthConfig | null {
  return globalAuthConfig;
}

export function isAuthenticated(state: AuthState): state is AuthState & {
  status: "authenticated";
  session: AuthSession;
} {
  return state.status === "authenticated" && state.session !== null;
}

export function requireAuth(state: AuthState): AuthSession {
  if (!isAuthenticated(state)) {
    throw new Error("Authentication required");
  }
  return state.session;
}

export type HasRoleFn = (role: string) => boolean;

export function requireRole(hasRole: HasRoleFn, role: string): void {
  if (!hasRole(role)) {
    throw new Error("Forbidden");
  }
}

export function getAuthState(store: AuthStore): Readable<AuthState> {
  return store.state;
}
