export interface RouteProtectionConfig {
  publicPaths: string[];
  loginPath?: string;
  useRedirectTo?: boolean;
  redirectToParam?: string;
}

export interface LoginRedirectOptions {
  loginPath?: string;
  useRedirectTo?: boolean;
  redirectToParam?: string;
}

export function isPublicPath(path: string, publicPaths: string[]): boolean {
  return publicPaths.some((publicPath) => {
    if (publicPath.endsWith("/*")) {
      const prefix = publicPath.slice(0, -1);
      return path.startsWith(prefix);
    }
    return path === publicPath;
  });
}

export function shouldProtectRoute(
  path: string,
  publicPaths: string[],
): boolean {
  return !isPublicPath(path, publicPaths);
}

export function createLoginRedirect(
  url: URL,
  config?: LoginRedirectOptions | string,
): Response {
  const {
    loginPath = "/login",
    useRedirectTo = true,
    redirectToParam = "redirectTo",
  } = typeof config === "string" ? { loginPath: config } : (config ?? {});

  const loginUrl = new URL(loginPath, url.origin);

  if (useRedirectTo && url.pathname !== loginPath) {
    loginUrl.searchParams.set(redirectToParam, url.pathname);
  }

  return Response.redirect(loginUrl, 302);
}

export function createRouteProtection(config: RouteProtectionConfig) {
  const { publicPaths, ...redirectConfig } = config;

  return function protectRoute(url: URL): Response | null {
    if (shouldProtectRoute(url.pathname, publicPaths)) {
      return createLoginRedirect(url, redirectConfig);
    }
    return null;
  };
}
