import { writable, type Readable } from "svelte/store";

import { UnderlayHttpError } from "./errors";
import type { AuthCommands, AuthSession, PasswordAuthParams, PassKeyAuthParams, RegisterParams } from "./auth";
import type { TokenStore } from "./http";

export type AuthStatus = "unknown" | "anonymous" | "authenticated";

export type AuthState = {
  status: AuthStatus;
  session: AuthSession | null;
  loading: boolean;
  error: UnderlayHttpError | null;
};

export type AuthStore = {
  state: Readable<AuthState>;

  init: () => Promise<void>;
  register: (params: RegisterParams) => Promise<AuthSession>;
  loginWithPassword: (params: PasswordAuthParams & { code?: string }) => Promise<AuthSession>;
  loginWithPasskey: (params: PassKeyAuthParams) => Promise<AuthSession>;
  refresh: () => Promise<AuthSession>;
  logout: () => Promise<void>;
};

export interface CreateAuthStoreOptions {
  commands: AuthCommands;
  tokenStore: TokenStore;

  /**
   * If true, `init()` will attempt a refresh on 401.
   *
   * Defaults to true.
   */
  refreshOnUnauthorized?: boolean;
}

export function createAuthStore(options: CreateAuthStoreOptions): AuthStore {
  const refreshOnUnauthorized = options.refreshOnUnauthorized ?? true;

  const state = writable<AuthState>({
    status: "unknown",
    session: null,
    loading: false,
    error: null,
  });

  function setAuthenticated(session: AuthSession) {
    state.set({ status: "authenticated", session, loading: false, error: null });
  }

  async function setTokens(session: AuthSession) {
    await options.tokenStore.setAccessToken(session.accessToken);
    await options.tokenStore.setRefreshToken(session.refreshToken);
  }

  async function clearTokens() {
    await options.tokenStore.clear();
  }

  function setAnonymous(error: UnderlayHttpError | null = null) {
    state.set({ status: "anonymous", session: null, loading: false, error });
  }

  async function init() {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      const session = await options.commands.session();
      await setTokens(session);
      setAuthenticated(session);
    } catch (err) {
      if (err instanceof UnderlayHttpError && err.status === 401) {
        if (refreshOnUnauthorized) {
          try {
            const refreshed = await options.commands.refresh();
            await setTokens(refreshed);
            setAuthenticated(refreshed);
            return;
          } catch (refreshErr) {
            await clearTokens();
            setAnonymous(refreshErr instanceof UnderlayHttpError ? refreshErr : null);
            return;
          }
        }

        await clearTokens();
        setAnonymous(err);
        return;
      }

      state.set({ status: "anonymous", session: null, loading: false, error: err as UnderlayHttpError });
    }
  }

  async function register(params: RegisterParams): Promise<AuthSession> {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      const session = await options.commands.register(params);
      await setTokens(session);
      setAuthenticated(session);
      return session;
    } catch (err) {
      const e = err instanceof UnderlayHttpError ? err : null;
      state.update((s) => ({ ...s, loading: false, error: e }));
      throw err;
    }
  }

  async function loginWithPassword(params: PasswordAuthParams & { code?: string }): Promise<AuthSession> {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      const session = await options.commands.loginWithPassword({
        email: params.email,
        password: params.password,
        ...(params.code ? { code: params.code } : {}),
      });
      await setTokens(session);
      setAuthenticated(session);
      return session;
    } catch (err) {
      const e = err instanceof UnderlayHttpError ? err : null;
      state.update((s) => ({ ...s, loading: false, error: e }));
      throw err;
    }
  }

  async function loginWithPasskey(params: PassKeyAuthParams): Promise<AuthSession> {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      const session = await options.commands.loginWithPasskey(params);
      await setTokens(session);
      setAuthenticated(session);
      return session;
    } catch (err) {
      const e = err instanceof UnderlayHttpError ? err : null;
      state.update((s) => ({ ...s, loading: false, error: e }));
      throw err;
    }
  }

  async function refresh(): Promise<AuthSession> {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      const session = await options.commands.refresh();
      await setTokens(session);
      setAuthenticated(session);
      return session;
    } catch (err) {
      if (err instanceof UnderlayHttpError && err.status === 401) {
        await clearTokens();
        setAnonymous(err);
      } else {
        state.update((s) => ({ ...s, loading: false, error: err as UnderlayHttpError }));
      }
      throw err;
    }
  }

  async function logout(): Promise<void> {
    state.update((s) => ({ ...s, loading: true, error: null }));

    try {
      await options.commands.logout();
    } finally {
      await clearTokens();
      setAnonymous(null);
    }
  }

  return {
    state,
    init,
    register,
    loginWithPassword,
    loginWithPasskey,
    refresh,
    logout,
  };
}
