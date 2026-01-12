import type { Handle, RequestEvent } from "@sveltejs/kit";

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
          accessToken: session.accessToken,
          refreshToken: session.refreshToken,
          retry: true,
        };
      } catch {
        await tokenStore.clear();
        return { retry: false, accessToken: null, refreshToken: null };
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
            get: (path: string, headers?: Record<string, string>) =>
              ctx.rawRequest({ method: "GET", path, headers }),
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
              retry: true,
              accessToken: session.accessToken,
              refreshToken: session.refreshToken,
            };
          } catch {
            return { retry: false, accessToken: null, refreshToken: null };
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
