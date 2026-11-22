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

const MAX_DECODE_ITERATIONS = 3;
// eslint-disable-next-line no-control-regex
const UNSAFE_REDIRECT_CHAR = /[\\\u0000-\u001f\u007f]/;

function decodeToFixpoint(value: string, maxIterations = MAX_DECODE_ITERATIONS): string | null {
	let decoded = value;
	for (let i = 0; i < maxIterations; i++) {
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

	// Reject inputs that still change (or become malformed) after max iterations.
	try {
		return decodeURIComponent(decoded) === decoded ? decoded : null;
	} catch {
		return null;
	}
}

function isSafeSuffix(suffix: string): boolean {
	let decoded: string;
	try {
		decoded = decodeURIComponent(suffix);
	} catch {
		return false;
	}

	// Once the wire encoding is validated, unwrap nested ASCII escapes only
	// for inspection. A literal percent in a query value is valid data.
	for (let i = 1; i <= MAX_DECODE_ITERATIONS; i++) {
		if (UNSAFE_REDIRECT_CHAR.test(decoded)) return false;
		const next = decoded.replace(/%([0-7][0-9a-f])/gi, (_, hex: string) =>
			String.fromCharCode(Number.parseInt(hex, 16)),
		);
		if (next === decoded) return true;
		decoded = next;
	}
	return false;
}

/**
 * Percent-decode and collapse `.`/`..` segments so prefix matching cannot be
 * bypassed with encodings like `%2e%2e` or embedded traversal.
 */
export function normalizePath(path: string): string {
	const decoded = decodeToFixpoint(path);
	if (decoded === null) {
		return "";
	}
	return normalizeDecodedPath(decoded);
}

function normalizeDecodedPath(decoded: string): string {
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

function stripTrailingSlash(path: string): string {
	return path.length > 1 && path.endsWith("/") ? path.slice(0, -1) : path;
}

/**
 * Resolve an untrusted redirect target (e.g. a `redirectTo` query param) to a
 * safe same-origin path. Accepts only single-leading-slash paths; rejects
 * protocol-relative targets (`//evil.com`), backslash variants, absolute URLs
 * with a scheme, control characters, and encoded traversal. Returns `fallback`
 * for anything unsafe.
 */
export function resolveRedirectTo(target: string | null | undefined, fallback = "/"): string {
	if (typeof target !== "string" || target.length === 0) {
		return fallback;
	}

	// Some callers pass an encoded redirect parameter. Unwrap it only until
	// we can see the path or the suffix, leaving the suffix's own encoding intact.
	let remainingDecodes = MAX_DECODE_ITERATIONS;
	while (!target.startsWith("/") && !/[?#]/.test(target) && remainingDecodes > 0) {
		try {
			const unwrapped = decodeURIComponent(target);
			if (unwrapped === target) return fallback;
			target = unwrapped;
			remainingDecodes--;
		} catch {
			return fallback;
		}
	}

	const suffixIndex = target.search(/[?#]/);
	const path = suffixIndex === -1 ? target : target.slice(0, suffixIndex);
	const suffix = suffixIndex === -1 ? "" : target.slice(suffixIndex);
	const decoded = decodeToFixpoint(path, remainingDecodes);
	if (decoded === null || UNSAFE_REDIRECT_CHAR.test(decoded) || !isSafeSuffix(suffix)) {
		return fallback;
	}
	if (!decoded.startsWith("/") || decoded.startsWith("//")) {
		return fallback;
	}
	if (decoded.split("/").some((segment) => segment === "..")) {
		return fallback;
	}

	const normalizedPath = normalizeDecodedPath(decoded).replace(/\?/g, "%3F").replace(/#/g, "%23");
	return `${normalizedPath}${suffix}`;
}

export function isPublicPath(path: string, publicPaths: string[]): boolean {
	const normalized = normalizePath(path);
	if (!normalized) {
		return false;
	}
	const normalizedNoTrailing = stripTrailingSlash(normalized);

	return publicPaths.some((publicPath) => {
		if (publicPath.endsWith("/*")) {
			const base = stripTrailingSlash(publicPath.slice(0, -2));
			if (base === "") {
				return normalizedNoTrailing.startsWith("/");
			}
			return normalizedNoTrailing === base || normalizedNoTrailing.startsWith(`${base}/`);
		}
		return normalizedNoTrailing === stripTrailingSlash(publicPath);
	});
}

export function shouldProtectRoute(path: string, publicPaths: string[]): boolean {
	return !isPublicPath(path, publicPaths);
}

export function createLoginRedirect(url: URL, config?: LoginRedirectOptions | string): Response {
	const {
		loginPath = "/login",
		useRedirectTo = true,
		redirectToParam = "redirectTo",
	} = typeof config === "string" ? { loginPath: config } : (config ?? {});

	const loginUrl = new URL(loginPath, url.origin);

	if (useRedirectTo && url.pathname !== loginPath) {
		// Guard the producer too: a protocol-relative pathname (`//evil.com`)
		// must never be written into the redirect param. Keep the query string so
		// post-login navigation returns to the same state (filters, pagination).
		const safeTarget = resolveRedirectTo(`${url.pathname}${url.search}`, "");
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
