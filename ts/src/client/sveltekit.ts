import type { Cookies, Handle, RequestEvent } from "@sveltejs/kit";

import type { AuthCommands, AuthRoutes, AuthSession } from "./auth";
import { createAuthCommands } from "./auth";
import { UnderlayHttpError } from "./errors";
import { createHttpClient, type HttpClient, type RefreshResult, type TokenStore } from "./http";
import type { SingleResponse } from "./types";

export interface CookieTokenStoreOptions {
  accessTokenCookie: string;
  refreshTokenCookie: string;
  cookie?: Parameters<RequestEvent["cookies"]["set"]>[2];
}

/**
 * Configuration for auth cookie helpers.
 */
export interface AuthCookieConfig {
  /** Cookie name for the access token (e.g., "bloom_access_token") */
  accessTokenName: string;
  /** Cookie name for the refresh token (e.g., "bloom_refresh_token") */
  refreshTokenName: string;
  /** Cookie options (path, httpOnly, sameSite, maxAge, etc.) */
  options?: {
    path?: string;
    httpOnly?: boolean;
    sameSite?: "lax" | "strict" | "none";
    maxAge?: number;
    secure?: boolean;
  };
}

/**
 * Helper functions for reading and writing auth cookies.
 */
export interface AuthCookieHelpers {
  /** Read the access token from cookies (returns null if empty/missing) */
  readAccessToken(cookies: Cookies): string | null;
  /** Read the refresh token from cookies (returns null if empty/missing) */
  readRefreshToken(cookies: Cookies): string | null;
  /** Write both access and refresh tokens to cookies */
  writeAuthTokens(
    cookies: Cookies,
    tokens: { accessToken: string; refreshToken: string }
  ): void;
  /** Clear both auth cookies */
  clearAuthTokens(cookies: Cookies): void;
}

/**
 * Creates auth cookie helper functions with the given configuration.
 *
 * Use this to create a set of cookie utilities for your app that can be
 * used in route actions (login, logout, etc.) where you have direct access
 * to the Cookies object but not the token store.
 *
 * @example
 * ```ts
 * // src/lib/auth.ts
 * import { createAuthCookieHelpers } from "@decodelabs/underlay/client";
 *
 * export const authCookies = createAuthCookieHelpers({
 *   accessTokenName: "bloom_access_token",
 *   refreshTokenName: "bloom_refresh_token",
 *   options: { maxAge: 60 * 60 * 24 * 7 }
 * });
 *
 * // In routes:
 * import { authCookies } from "$lib/auth";
 * authCookies.writeAuthTokens(cookies, { accessToken, refreshToken });
 * ```
 */
export function createAuthCookieHelpers(config: AuthCookieConfig): AuthCookieHelpers {
  const defaultOptions = {
    path: "/",
    httpOnly: true,
    sameSite: "lax" as const,
    maxAge: 60 * 60 * 24 * 7, // 7 days
    ...config.options,
  };

  return {
    readAccessToken(cookies: Cookies): string | null {
      const value = cookies.get(config.accessTokenName);
      return value?.trim() || null;
    },

    readRefreshToken(cookies: Cookies): string | null {
      const value = cookies.get(config.refreshTokenName);
      return value?.trim() || null;
    },

    writeAuthTokens(
      cookies: Cookies,
      tokens: { accessToken: string; refreshToken: string }
    ): void {
      cookies.set(config.accessTokenName, tokens.accessToken, defaultOptions);
      cookies.set(config.refreshTokenName, tokens.refreshToken, defaultOptions);
    },

    clearAuthTokens(cookies: Cookies): void {
      cookies.delete(config.accessTokenName, { path: "/" });
      cookies.delete(config.refreshTokenName, { path: "/" });
    },
  };
}

export function createCookieTokenStore(
  event: RequestEvent,
  options: CookieTokenStoreOptions
): TokenStore {
  return {
    getAccessToken: () => event.cookies.get(options.accessTokenCookie) ?? null,
    setAccessToken: async (token) => {
      if (!token) {
        event.cookies.delete(options.accessTokenCookie, { path: "/" });
        return;
      }
      event.cookies.set(options.accessTokenCookie, token, {
        path: "/",
        ...options.cookie,
      });
    },

    getRefreshToken: () => event.cookies.get(options.refreshTokenCookie) ?? null,
    setRefreshToken: async (token) => {
      if (!token) {
        event.cookies.delete(options.refreshTokenCookie, { path: "/" });
        return;
      }
      event.cookies.set(options.refreshTokenCookie, token, {
        path: "/",
        ...options.cookie,
      });
    },

    clear: async () => {
      event.cookies.delete(options.accessTokenCookie, { path: "/" });
      event.cookies.delete(options.refreshTokenCookie, { path: "/" });
    },
  };
}

export interface SvelteKitAuthOptions {
  baseUrl: string;
  routes: AuthRoutes;

  /**
   * Cookie names used for access/refresh tokens.
   *
   * You likely want httpOnly cookies for refresh tokens.
   */
  cookies: CookieTokenStoreOptions;

  /**
   * If provided, protects requests where this returns true.
   *
   * Use `onUnauthenticated` to redirect/deny.
   */
  shouldProtect?: (event: RequestEvent) => boolean;

  /**
   * Called when a protected request has no valid session.
   */
  onUnauthenticated?: (event: RequestEvent) => Response | Promise<Response>;

  /**
   * Optional hook to customise refresh request shape.
   *
   * Default calls `POST routes.refresh` with `{ refreshToken }` when available.
   */
  refreshRequest?: (ctx: {
    rawHttp: HttpClient;
    routes: AuthRoutes;
    refreshToken: string | null;
  }) => Promise<AuthSession>;
}

export interface SvelteKitAuthLocals {
  http: HttpClient;
  commands: AuthCommands;
  getSession: () => Promise<AuthSession | null>;
  clearTokens: () => Promise<void>;
}

export function createAuthHandle(options: SvelteKitAuthOptions): Handle {
  return async ({ event, resolve }) => {
    const tokenStore = createCookieTokenStore(event, options.cookies);

    const rawHttp = createHttpClient({
      baseUrl: options.baseUrl,
      fetch: event.fetch,
    });

    const refresh = async (): Promise<RefreshResult> => {
      const refreshToken = await tokenStore.getRefreshToken();

      try {
        const session = options.refreshRequest
          ? await options.refreshRequest({ rawHttp, routes: options.routes, refreshToken })
          : await defaultRefreshRequest(rawHttp, options.routes, refreshToken);

        await tokenStore.setAccessToken(session.accessToken);
        await tokenStore.setRefreshToken(session.refreshToken);

        return {
          success: true,
          accessToken: session.accessToken,
          refreshToken: session.refreshToken,
        };
      } catch {
        await tokenStore.clear();
        return { success: false, accessToken: null, refreshToken: null };
      }
    };

    const http = createHttpClient({
      baseUrl: options.baseUrl,
      fetch: event.fetch,
      auth: {
        tokenStore,
        refresh: async (ctx) => {
          // Ensure refresh requests do not include Authorization.
          const rawHttp = {
            request: ctx.rawRequest,
            requestWithMeta: async <T>(req: {
              method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
              path: string;
              headers?: Record<string, string>;
              body?: unknown;
            }, _options?: { acceptedStatuses?: number[] }) => {
              const body = await ctx.rawRequest<T>(req);
              return { status: 200, headers: {}, body };
            },
            get: (path: string, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "GET", path, headers }),
            getWithMeta: async <T>(
              path: string,
              headers?: Record<string, string>,
              _options?: { acceptedStatuses?: number[] }
            ) => {
              const body = await ctx.rawRequest<T>({ method: "GET", path, headers });
              return { status: 200, headers: {}, body };
            },
            post: (path: string, body?: unknown, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "POST", path, body, headers }),
            put: (path: string, body?: unknown, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "PUT", path, body, headers }),
            patch: (path: string, body?: unknown, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "PATCH", path, body, headers }),
            delete: (path: string, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "DELETE", path, headers }),
          } satisfies HttpClient;

          const refreshToken = await ctx.getRefreshToken();

          try {
            const session = options.refreshRequest
              ? await options.refreshRequest({ rawHttp, routes: options.routes, refreshToken })
              : await defaultRefreshRequest(rawHttp, options.routes, refreshToken);

            return {
              success: true,
              accessToken: session.accessToken,
              refreshToken: session.refreshToken,
            };
          } catch {
            return { success: false, accessToken: null, refreshToken: null };
          }
        },
      },
    });

    const commands = createAuthCommands(http, options.routes);

    async function getSession(): Promise<AuthSession | null> {
      try {
        return await commands.session();
      } catch (err) {
        if (err instanceof UnderlayHttpError && err.status === 401) {
          await tokenStore.clear();
          return null;
        }
        throw err;
      }
    }

    const locals: SvelteKitAuthLocals = {
      http,
      commands,
      getSession,
      clearTokens: async () => {
        await tokenStore.clear();
      },
    };

    // App can module-augment Locals to include this.
    (event.locals as unknown as Record<string, unknown>).auth = locals;

    if (options.shouldProtect?.(event)) {
      const session = await getSession();
      if (!session) {
        if (options.onUnauthenticated) {
          return await options.onUnauthenticated(event);
        }

        return new Response("Unauthorized", { status: 401 });
      }
    }

    return resolve(event);
  };
}

async function defaultRefreshRequest(
  http: HttpClient,
  routes: AuthRoutes,
  refreshToken: string | null
): Promise<AuthSession> {
  const body = refreshToken ? { refreshToken } : undefined;
  const res = await http.post<SingleResponse<AuthSession>>(routes.refresh, body);
  return res.data;
}
