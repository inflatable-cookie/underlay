import type { Readable } from "svelte/store";

import type { AuthSession } from "../client/auth";
import type { AuthState, AuthStore } from "../client/useAuth";

export { createAuthStore } from "../client/useAuth";
export type { AuthState, AuthStore } from "../client/useAuth";

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
