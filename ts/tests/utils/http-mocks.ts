/**
 * Test utilities for Underlay HTTP client tests
 * 
 * Common patterns extracted from client tests:
 * - Mock fetch responses
 * - Mock HTTP clients with predefined behaviors
 * - Assertion helpers for HTTP errors
 */

import { vi } from 'vitest';
import type { TokenStore } from '../http';

/**
 * Create a mock Headers object that works like the Fetch API Headers
 */
function createMockHeaders(contentType = 'application/json') {
	const headers = new Map([['content-type', contentType]]);
	return {
		get: (key: string) => headers.get(key.toLowerCase()) ?? null,
		has: (key: string) => headers.has(key.toLowerCase()),
		set: (key: string, value: string) => headers.set(key.toLowerCase(), value),
		delete: (key: string) => headers.delete(key.toLowerCase()),
		forEach: (callback: (value: string, key: string) => void) => headers.forEach(callback),
		entries: () => headers.entries(),
		keys: () => headers.keys(),
		values: () => headers.values()
	};
}

/**
 * Create a mock fetch function that returns a successful JSON response
 */
export function mockFetchSuccess<T>(data: T, status = 200) {
	return vi.fn().mockResolvedValue({
		ok: true,
		status,
		headers: createMockHeaders('application/json'),
		json: async () => ({ data })
	});
}

/**
 * Create a mock fetch function that returns an error response
 */
export function mockFetchError(code: string, message: string, status = 400, fieldErrors?: Record<string, string>) {
	return vi.fn().mockResolvedValue({
		ok: false,
		status,
		headers: createMockHeaders('application/json'),
		json: async () => ({
			error: {
				code,
				message,
				...(fieldErrors && { fieldErrors })
			}
		})
	});
}

/**
 * Create a mock fetch function that returns 204 No Content
 */
export function mockFetchNoContent() {
	return vi.fn().mockResolvedValue({
		ok: true,
		status: 204,
		headers: createMockHeaders(''),
		json: async () => {
			throw new Error('No content');
		}
	});
}

/**
 * Create a mock fetch function that fails with a network error
 */
export function mockFetchNetworkError(message = 'Network error') {
	return vi.fn().mockRejectedValue(new Error(message));
}

/**
 * Create a sequence of mock fetch responses
 * Useful for testing retry logic or multi-step flows
 */
export function mockFetchSequence(...responses: Array<{
	ok: boolean;
	status: number;
	data?: unknown;
	error?: { code: string; message: string; fieldErrors?: Record<string, string> };
}>) {
	const mock = vi.fn();
	
	responses.forEach((response, index) => {
		mock.mockResolvedValueOnce({
			ok: response.ok,
			status: response.status,
			headers: createMockHeaders('application/json'),
			json: async () => {
				if (response.error) {
					return { error: response.error };
				}
				return { data: response.data };
			}
		});
	});
	
	return mock;
}

/**
 * Create a mock fetch that delays before resolving
 * Useful for testing timeouts
 */
export function mockFetchWithDelay<T>(data: T, delayMs: number, status = 200) {
	return vi.fn().mockImplementation(() => {
		return new Promise((resolve) => {
			setTimeout(() => {
				resolve({
					ok: true,
					status,
					headers: createMockHeaders('application/json'),
					json: async () => ({ data })
				});
			}, delayMs);
		});
	});
}

/**
 * Fake token store for testing
 * Provides a simple in-memory implementation with spies
 */
export class FakeTokenStore implements TokenStore {
	private accessToken: string | null = null;
	private refreshToken: string | null = null;

	getAccessToken = vi.fn((): string | null => this.accessToken);
	setAccessToken = vi.fn((token: string | null): void => {
		this.accessToken = token;
	});

	getRefreshToken = vi.fn((): string | null => this.refreshToken);
	setRefreshToken = vi.fn((token: string | null): void => {
		this.refreshToken = token;
	});

	clear = vi.fn((): void => {
		this.accessToken = null;
		this.refreshToken = null;
	});

	// Helper to set initial tokens
	seedTokens(accessToken: string, refreshToken: string): void {
		this.accessToken = accessToken;
		this.refreshToken = refreshToken;
	}

	// Helper to verify tokens were set
	expectTokens(accessToken: string | null, refreshToken: string | null): void {
		if (this.accessToken !== accessToken) {
			throw new Error(`Expected access token to be ${accessToken}, got ${this.accessToken}`);
		}
		if (this.refreshToken !== refreshToken) {
			throw new Error(`Expected refresh token to be ${refreshToken}, got ${this.refreshToken}`);
		}
	}
}

/**
 * Extract fetch call arguments for assertions
 */
export function getFetchCallArgs(mockFetch: ReturnType<typeof vi.fn>, callIndex = 0) {
	const call = mockFetch.mock.calls[callIndex];
	if (!call) {
		throw new Error(`No call at index ${callIndex}`);
	}

	const [url, options] = call;
	return {
		url: url as string,
		method: options?.method as string,
		headers: options?.headers as Record<string, string>,
		body: options?.body ? JSON.parse(options.body as string) : undefined
	};
}

/**
 * Assert that fetch was called with specific headers
 */
export function expectFetchHeaders(
	mockFetch: ReturnType<typeof vi.fn>,
	expectedHeaders: Record<string, string>,
	callIndex = 0
) {
	const { headers } = getFetchCallArgs(mockFetch, callIndex);
	
	for (const [key, value] of Object.entries(expectedHeaders)) {
		if (headers[key] !== value) {
			throw new Error(`Expected header ${key} to be ${value}, got ${headers[key]}`);
		}
	}
}

/**
 * Assert that fetch was called with Authorization header
 */
export function expectAuthHeader(
	mockFetch: ReturnType<typeof vi.fn>,
	token: string,
	callIndex = 0
) {
	expectFetchHeaders(mockFetch, { Authorization: `Bearer ${token}` }, callIndex);
}

/**
 * Assert that fetch was NOT called with Authorization header
 */
export function expectNoAuthHeader(
	mockFetch: ReturnType<typeof vi.fn>,
	callIndex = 0
) {
	const { headers } = getFetchCallArgs(mockFetch, callIndex);
	if (headers.Authorization) {
		throw new Error(`Expected no Authorization header, but found: ${headers.Authorization}`);
	}
}
