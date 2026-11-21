import { UnderlayHttpError, isErrorEnvelope } from "./errors";
import { headersToRecord, hasHeader, setHeaderIfMissing } from "./http-headers.js";
import type {
	HttpClient,
	HttpClientOptions,
	HttpRequest,
	HttpRequestOptions,
	HttpResponse,
	RefreshResult,
} from "./http-types.js";

export type {
	HttpAuthOptions,
	HttpClient,
	HttpClientOptions,
	HttpMethod,
	HttpRequest,
	HttpRequestOptions,
	HttpResponse,
	RefreshContext,
	RefreshResult,
	TokenStore,
} from "./http-types.js";
export { MemoryTokenStore } from "./http-token-store.js";

/**
 * Lowercased media type of a response, or `null` when the server sent no
 * usable `Content-Type`. Parameters such as `; charset=utf-8` are dropped.
 */
function responseMediaType(res: Response): string | null {
	const raw = res.headers?.get?.("content-type");
	if (typeof raw !== "string") {
		return null;
	}
	const mediaType = raw.split(";", 1)[0]?.trim().toLowerCase() ?? "";
	return mediaType === "" ? null : mediaType;
}

/**
 * JSON and structured-suffix types (`application/json`,
 * `application/problem+json`, `application/ld+json`, ...).
 */
function isJsonMediaType(mediaType: string | null): boolean {
	if (mediaType === null) {
		return false;
	}
	return mediaType.endsWith("/json") || mediaType.endsWith("+json");
}

/**
 * Read a response whose `Content-Type` is missing. Untyped bodies are read as
 * text and only parsed as JSON when they actually are JSON, so empty and
 * non-JSON responses stay plain values instead of becoming parse errors.
 */
async function readUntypedBody(res: Response): Promise<unknown> {
	if (typeof res.text === "function") {
		const text = await res.text();
		if (text.trim() === "") {
			return null;
		}
		try {
			return JSON.parse(text) as unknown;
		} catch {
			return text;
		}
	}
	if (typeof res.json === "function") {
		return await res.json();
	}
	return null;
}

/**
 * Refresh failures that may clear on their own: network errors and 5xx.
 * Stored tokens are preserved for these so a recoverable outage does not
 * silently sign the user out.
 */
export function isTransientRefreshError(err: unknown): boolean {
	return (
		err instanceof TypeError ||
		(err instanceof UnderlayHttpError && (err.status === 0 || err.status >= 500))
	);
}

export function createHttpClient(options: HttpClientOptions): HttpClient {
	const fetchImpl = options.fetch ?? globalThis.fetch;
	const timeoutMs = options.timeoutMs ?? 8000;
	const maxRetries = options.maxRetries ?? 3;
	const retryStatuses = new Set([502, 503, 504, ...(options.retryStatuses ?? [])]);
	const debug = options.debug ?? false;
	const credentials = options.credentials ?? "omit";

	function log(...args: unknown[]): void {
		if (debug) {
			console.log("[HTTP]", ...args);
		}
	}

	const tokenStore = options.auth?.tokenStore;

	const getAccessToken =
		options.auth?.getAccessToken ?? (tokenStore ? () => tokenStore.getAccessToken() : undefined);
	const setAccessToken =
		options.auth?.setAccessToken ??
		(tokenStore ? (t: string | null) => tokenStore.setAccessToken(t) : undefined);

	const getRefreshToken =
		options.auth?.getRefreshToken ??
		(tokenStore ? () => tokenStore.getRefreshToken() : async () => null);
	const setRefreshToken =
		options.auth?.setRefreshToken ??
		(tokenStore ? (t: string | null) => tokenStore.setRefreshToken(t) : undefined);

	let refreshInFlight: Promise<RefreshResult> | null = null;

	// Tokens must only be attached to requests that resolve to the configured
	// base origin; an absolute req.path can otherwise leak them cross-origin.
	let baseOrigin: string | null = null;
	try {
		baseOrigin = new URL(options.baseUrl).origin;
	} catch {
		baseOrigin = null;
	}

	function isBaseOrigin(path: string): boolean {
		if (baseOrigin === null) {
			return false;
		}
		try {
			return new URL(path, options.baseUrl).origin === baseOrigin;
		} catch {
			return false;
		}
	}

	async function rawRequest<T>(
		req: HttpRequest,
		opts?: { skipRetry?: boolean; acceptedStatuses?: number[] },
	): Promise<HttpResponse<T>> {
		const url = new URL(req.path, options.baseUrl).toString();
		const headers: Record<string, string> = {
			...options.defaultHeaders,
			...req.headers,
		};

		setHeaderIfMissing(headers, "Accept", "application/json");

		let body: BodyInit | undefined;
		if (req.body !== undefined) {
			setHeaderIfMissing(headers, "Content-Type", "application/json");
			body = JSON.stringify(req.body);
		}

		// Determine if this request is idempotent (can be retried)
		const isIdempotent = req.method === "GET" || req.method === "DELETE";
		const shouldRetry = isIdempotent && !opts?.skipRetry;

		let retries = 0;

		while (true) {
			const controller = typeof AbortController !== "undefined" ? new AbortController() : undefined;

			const timeout =
				controller != null ? setTimeout(() => controller.abort(), timeoutMs) : undefined;

			let retryStatus: number | undefined;

			try {
				log(req.method, req.path);

				const res = await fetchImpl(url, {
					method: req.method,
					headers,
					body,
					signal: controller?.signal,
					credentials,
				});

				const mediaType = responseMediaType(res);
				const hasJson = isJsonMediaType(mediaType);

				const acceptedStatuses = new Set(opts?.acceptedStatuses ?? []);
				const isAccepted = res.ok || acceptedStatuses.has(res.status);

				if (!isAccepted) {
					let parsed: unknown;
					if (hasJson || mediaType === null) {
						try {
							parsed = hasJson ? await res.json() : await readUntypedBody(res);
						} catch (parseErr) {
							if (
								parseErr instanceof Error &&
								(parseErr.name === "AbortError" || controller?.signal.aborted === true)
							) {
								throw parseErr;
							}
							parsed = undefined;
						}
					}
					const envelope = isErrorEnvelope(parsed) ? parsed : undefined;
					const message = envelope?.error.message ?? `HTTP ${res.status}`;

					log(`Error response (${res.status}):`, envelope);

					// Check if we should retry this error
					const canRetry = shouldRetry && retryStatuses.has(res.status) && retries < maxRetries;

					if (canRetry) {
						retryStatus = res.status;
					} else {
						throw new UnderlayHttpError(res.status, message, envelope);
					}
				} else {
					const responseHeaders = headersToRecord(res.headers);

					if (res.status === 204 || res.status === 304) {
						return { status: res.status, headers: responseHeaders, body: null };
					}

					if (!hasJson) {
						if (mediaType === null) {
							return {
								status: res.status,
								headers: responseHeaders,
								body: (await readUntypedBody(res)) as T,
							};
						}
						return {
							status: res.status,
							headers: responseHeaders,
							body: (await res.text()) as unknown as T,
						};
					}

					try {
						return {
							status: res.status,
							headers: responseHeaders,
							body: (await res.json()) as T,
						};
					} catch (parseErr) {
						if (
							parseErr instanceof Error &&
							(parseErr.name === "AbortError" || controller?.signal.aborted === true)
						) {
							throw parseErr;
						}
						const message =
							parseErr instanceof Error
								? `Invalid JSON response: ${parseErr.message}`
								: "Invalid JSON response";
						throw new UnderlayHttpError(res.status, message, {
							error: {
								code: "invalid_json",
								message,
							},
						});
					}
				}
			} catch (err) {
				// If it's an HTTP error (not network/timeout), don't retry - we already handled that above
				if (err instanceof UnderlayHttpError) {
					throw err;
				}

				const isTimeout =
					err instanceof Error &&
					(err.name === "AbortError" || controller?.signal.aborted === true);
				if (isTimeout) {
					throw new UnderlayHttpError(0, "Request timed out", {
						error: {
							code: "timeout",
							message: "Request timed out",
						},
					});
				}

				const message = err instanceof Error ? err.message : "Network error";
				throw new UnderlayHttpError(0, message);
			} finally {
				if (timeout != null) {
					clearTimeout(timeout);
				}
			}

			if (retryStatus !== undefined) {
				retries += 1;
				log("Retrying due to status", retryStatus);
				// Exponential backoff: 100ms, 200ms, 400ms, etc.
				const backoffMs = Math.min(100 * Math.pow(2, retries - 1), 3000);
				await new Promise((resolve) => setTimeout(resolve, backoffMs));
			}
		}
	}

	async function rawRequestBody<T>(req: HttpRequest): Promise<T> {
		const response = await rawRequest<T>(req);
		return response.body as T;
	}

	async function rawRequestWithMeta<T>(
		req: HttpRequest,
		requestOptions?: HttpRequestOptions,
	): Promise<HttpResponse<T>> {
		return rawRequest<T>(req, requestOptions);
	}

	async function clearStoredTokens(): Promise<void> {
		if (tokenStore) {
			await tokenStore.clear();
		}
		if (options.auth?.setAccessToken) {
			await options.auth.setAccessToken(null);
		}
		if (options.auth?.setRefreshToken) {
			await options.auth.setRefreshToken(null);
		}
	}

	function authHeaders(req: HttpRequest, token: string | null): Record<string, string> {
		const headers: Record<string, string> = { ...(req.headers ?? {}) };
		if (token && isBaseOrigin(req.path)) {
			setHeaderIfMissing(headers, "Authorization", `Bearer ${token}`);
		}
		return headers;
	}

	/**
	 * Run the configured refresh handler at most once across concurrent waiters.
	 * Token store writes and clears happen inside this promise so every waiter
	 * observes one rotation instead of repeating it (rotating refresh tokens
	 * revoke each other when written twice).
	 */
	async function runRefreshOnce(): Promise<RefreshResult> {
		if (refreshInFlight === null) {
			const refreshFn = options.auth?.refresh;
			if (!refreshFn) {
				throw new Error("No refresh handler configured");
			}
			refreshInFlight = (async () => {
				try {
					const refreshed = await refreshFn({
						rawRequest: rawRequestBody,
						rawRequestWithMeta,
						tokenStore,
						getRefreshToken: async () => (await getRefreshToken?.()) ?? null,
						setAccessToken: async (t) => {
							await setAccessToken?.(t);
						},
						setRefreshToken: async (t) => {
							await setRefreshToken?.(t);
						},
					});

					if (!refreshed.success) {
						await clearStoredTokens();
						return refreshed;
					}

					if (refreshed.accessToken !== undefined) {
						await setAccessToken?.(refreshed.accessToken);
					}
					if (refreshed.refreshToken !== undefined) {
						await setRefreshToken?.(refreshed.refreshToken);
					}
					return refreshed;
				} catch (refreshErr) {
					// A transient failure (5xx or network) must not discard a session
					// that may still be valid once the server recovers.
					if (!isTransientRefreshError(refreshErr)) {
						await clearStoredTokens();
					}
					throw refreshErr;
				}
			})().finally(() => {
				refreshInFlight = null;
			});
		}

		return await refreshInFlight;
	}

	async function requestWithMeta<T>(
		req: HttpRequest,
		requestOptions?: HttpRequestOptions,
	): Promise<HttpResponse<T>> {
		const hasExplicitAuth =
			hasHeader(req.headers ?? {}, "Authorization") ||
			hasHeader(options.defaultHeaders ?? {}, "Authorization");

		// Remember the token this attempt was built with. A 401 that lands after
		// another request already rotated the token is stale: retry with the
		// current token instead of refreshing (and revoking) the new family again.
		const startedWithToken = hasExplicitAuth ? null : ((await getAccessToken?.()) ?? null);

		const attempt = (token: string | null) =>
			rawRequest<T>({ ...req, headers: authHeaders(req, token) }, requestOptions);

		try {
			return await attempt(startedWithToken);
		} catch (err) {
			if (!(err instanceof UnderlayHttpError)) throw err;

			if (err.status !== 401 || hasExplicitAuth || !options.auth?.refresh) {
				throw err;
			}

			if (startedWithToken !== null) {
				const currentToken = (await getAccessToken?.()) ?? null;
				if (currentToken !== null && currentToken !== startedWithToken) {
					try {
						return await attempt(currentToken);
					} catch (retryErr) {
						if (!(retryErr instanceof UnderlayHttpError) || retryErr.status !== 401) {
							throw retryErr;
						}
					}
				}
			}

			const refreshed = await runRefreshOnce();

			if (!refreshed.success) {
				throw err;
			}

			const retryToken = (await getAccessToken?.()) ?? null;
			return await attempt(retryToken);
		}
	}

	async function request<T>(req: HttpRequest): Promise<T> {
		const response = await requestWithMeta<T>(req);
		return response.body as T;
	}

	return {
		request,
		requestWithMeta,
		get: (path, headers) => request({ method: "GET", path, headers }),
		getWithMeta: (path, headers, options) =>
			requestWithMeta({ method: "GET", path, headers }, options),
		post: (path, body, headers) => request({ method: "POST", path, body, headers }),
		put: (path, body, headers) => request({ method: "PUT", path, body, headers }),
		patch: (path, body, headers) => request({ method: "PATCH", path, body, headers }),
		delete: (path, headers) => request({ method: "DELETE", path, headers }),
	};
}
