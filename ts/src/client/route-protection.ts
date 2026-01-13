/**
 * Route protection utilities for SvelteKit apps.
 *
 * These utilities provide a simple, reusable pattern for protecting routes
 * and redirecting unauthenticated users to a login page.
 *
 * @example
 * ```typescript
 * // In hooks.server.ts
 * import { isPublicPath, createLoginRedirect } from "@decodelabs/underlay/client";
 *
 * const PUBLIC_PATHS = ["/login", "/register", "/auth/callback", "/health"];
 *
 * export const handle: Handle = async ({ event, resolve }) => {
 *   // ... token refresh logic ...
 *
 *   if (!isAuthenticated) {
 *     if (!isPublicPath(event.url.pathname, PUBLIC_PATHS)) {
 *       return createLoginRedirect(event.url, "/login");
 *     }
 *   }
 *
 *   return resolve(event);
 * };
 * ```
 */

export interface RouteProtectionConfig {
  /**
   * Paths that don't require authentication.
   * Supports exact matches and prefix matches (paths ending with `*`).
   */
  publicPaths: string[];

  /**
   * Path to redirect unauthenticated users to.
   * @default "/login"
   */
  loginPath?: string;

  /**
   * Whether to include a `redirectTo` query parameter with the original path.
   * @default true
   */
  useRedirectTo?: boolean;

  /**
   * Query parameter name for the redirect target.
   * @default "redirectTo"
   */
  redirectToParam?: string;
}

/**
 * Options for creating a login redirect.
 */
export interface LoginRedirectOptions {
  /**
   * Path to redirect unauthenticated users to.
   * @default "/login"
   */
  loginPath?: string;

  /**
   * Whether to include a `redirectTo` query parameter with the original path.
   * @default true
   */
  useRedirectTo?: boolean;

  /**
   * Query parameter name for the redirect target.
   * @default "redirectTo"
   */
  redirectToParam?: string;
}

/**
 * Check if a path matches any of the public paths.
 *
 * Supports:
 * - Exact matches: `/login` matches only `/login`
 * - Prefix matches: `/auth/*` matches `/auth/callback`, `/auth/google`, etc.
 * - Simple prefix: `/api` matches `/api`, `/api/users`, etc. (startsWith behavior)
 */
export function isPublicPath(path: string, publicPaths: string[]): boolean {
  return publicPaths.some((publicPath) => {
    if (publicPath.endsWith("/*")) {
      // Wildcard: match prefix (e.g., `/auth/*` matches `/auth/callback`)
      const prefix = publicPath.slice(0, -1); // Remove the `*`
      return path.startsWith(prefix);
    }
    // Default: prefix match (startsWith)
    return path.startsWith(publicPath);
  });
}

/**
 * Check if a route should be protected (i.e., requires authentication).
 *
 * Returns `true` if the path is NOT in the public paths list.
 */
export function shouldProtectRoute(path: string, publicPaths: string[]): boolean {
  return !isPublicPath(path, publicPaths);
}

/**
 * Create a redirect response to the login page.
 *
 * Optionally includes the original path as a `redirectTo` query parameter
 * so the user can be redirected back after login.
 */
export function createLoginRedirect(
  url: URL,
  config?: LoginRedirectOptions | string
): Response {
  const {
    loginPath = "/login",
    useRedirectTo = true,
    redirectToParam = "redirectTo"
  } = typeof config === "string" ? { loginPath: config } : (config ?? {});

  const loginUrl = new URL(loginPath, url.origin);

  if (useRedirectTo && url.pathname !== loginPath) {
    loginUrl.searchParams.set(redirectToParam, url.pathname);
  }

  return Response.redirect(loginUrl, 302);
}

/**
 * Create a complete route protection handler.
 *
 * Returns a function that checks if the current request should be protected
 * and returns a redirect response if the user is not authenticated.
 *
 * @example
 * ```typescript
 * const protectRoute = createRouteProtection({
 *   publicPaths: ["/login", "/register", "/health"],
 *   loginPath: "/login"
 * });
 *
 * // In your handle function:
 * if (!isAuthenticated) {
 *   const redirect = protectRoute(event.url);
 *   if (redirect) return redirect;
 * }
 * ```
 */
export function createRouteProtection(config: RouteProtectionConfig) {
  const { publicPaths, ...redirectConfig } = config;

  return function protectRoute(url: URL): Response | null {
    if (shouldProtectRoute(url.pathname, publicPaths)) {
      return createLoginRedirect(url, redirectConfig);
    }
    return null;
  };
}
