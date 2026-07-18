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

function decodeToFixpoint(value: string): string | null {
  let decoded = value;
  for (let i = 0; i < 3; i++) {
    let next: string;
    try {
      next = decodeURIComponent(decoded);
    } catch {
      return null;
    }
    if (next === decoded) {
      return decoded;
    }
    decoded = next;
  }
  return decoded;
}

/**
 * Percent-decode and collapse `.`/`..` segments so prefix matching cannot be
 * bypassed with encodings like `%2e%2e` or embedded traversal.
 */
export function normalizePath(path: string): string {
  const decoded = decodeToFixpoint(path);
  if (decoded === null) {
    return path;
  }

  const withForwardSlashes = decoded.replace(/\\/g, "/");
  const segments = withForwardSlashes.split("/");
  const out: string[] = [];
  for (const segment of segments) {
    if (segment === "" || segment === ".") {
      continue;
    }
    if (segment === "..") {
      out.pop();
      continue;
    }
    out.push(segment);
  }

  const normalized = `/${out.join("/")}`;
  return withForwardSlashes.endsWith("/") && normalized !== "/" ? `${normalized}/` : normalized;
}

/**
 * Resolve an untrusted redirect target (e.g. a `redirectTo` query param) to a
 * safe same-origin path. Accepts only single-leading-slash paths; rejects
 * protocol-relative targets (`//evil.com`), backslash variants, absolute URLs
 * with a scheme, control characters, and encoded traversal. Returns `fallback`
 * for anything unsafe.
 */
export function resolveRedirectTo(
  target: string | null | undefined,
  fallback = "/",
): string {
  if (typeof target !== "string" || target.length === 0) {
    return fallback;
  }

  const decoded = decodeToFixpoint(target);
  if (decoded === null) {
    return fallback;
  }

  // eslint-disable-next-line no-control-regex
  if (/[\u0000-\u001f\u007f]/.test(decoded)) {
    return fallback;
  }
  if (decoded.includes("\\")) {
    return fallback;
  }
  if (!decoded.startsWith("/") || decoded.startsWith("//")) {
    return fallback;
  }
  if (decoded.split("/").some((segment) => segment === "..")) {
    return fallback;
  }

  return target;
}

export function isPublicPath(path: string, publicPaths: string[]): boolean {
  const normalized = normalizePath(path);
  return publicPaths.some((publicPath) => {
    if (publicPath.endsWith("/*")) {
      const prefix = publicPath.slice(0, -1);
      return normalized.startsWith(prefix);
    }
    return normalized === publicPath;
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
    // Guard the producer too: a protocol-relative pathname (`//evil.com`)
    // must never be written into the redirect param.
    const safeTarget = resolveRedirectTo(url.pathname, "");
    if (safeTarget) {
      loginUrl.searchParams.set(redirectToParam, safeTarget);
    }
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
