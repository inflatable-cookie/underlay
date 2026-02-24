import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { createHttpClient, MemoryTokenStore, type HttpClientOptions } from '../../src/client/http';
import { UnderlayHttpError } from '../../src/client/errors';
import {
	mockFetchSuccess,
	mockFetchError,
	mockFetchNoContent,
	mockFetchSequence,
	mockFetchWithDelay,
	FakeTokenStore,
	getFetchCallArgs,
	expectAuthHeader,
	expectNoAuthHeader
} from '../utils/http-mocks';

describe('createHttpClient', () => {
	let fetchMock: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		fetchMock = vi.fn();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	describe('basic requests', () => {
		it('should make GET request with correct URL and headers', async () => {
			fetchMock = mockFetchSuccess({ id: '123', name: 'Test' });

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.get<{ id: string; name: string }>('/users/123');

			expect(fetchMock).toHaveBeenCalledTimes(1);
			const { url, method, headers } = getFetchCallArgs(fetchMock);
			expect(url).toBe('https://api.example.com/users/123');
			expect(method).toBe('GET');
			expect(headers.Accept).toBe('application/json');
			expect(response).toEqual({ data: { id: '123', name: 'Test' } });
		});

		it('should make POST request with body', async () => {
			fetchMock = mockFetchSuccess({ id: '456', name: 'Created' }, 201);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const body = { name: 'New User', email: 'test@example.com' };
			const response = await client.post<{ id: string; name: string }>('/users', body);

			const { url, method, headers, body: sentBody } = getFetchCallArgs(fetchMock);
			expect(url).toBe('https://api.example.com/users');
			expect(method).toBe('POST');
			expect(headers['Content-Type']).toBe('application/json');
			expect(sentBody).toEqual(body);
			expect(response).toEqual({ data: { id: '456', name: 'Created' } });
		});

		it('should handle 204 No Content responses', async () => {
			fetchMock = mockFetchNoContent();

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.delete('/users/123');

			expect(response).toBeNull();
		});

		it('should add default headers to all requests', async () => {
			fetchMock = mockFetchSuccess({});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				defaultHeaders: {
					'X-Client-Version': '1.0.0',
					'X-Custom-Header': 'value'
				},
				fetch: fetchMock
			});

			await client.get('/test');

			const { headers } = getFetchCallArgs(fetchMock);
			expect(headers['X-Client-Version']).toBe('1.0.0');
			expect(headers['X-Custom-Header']).toBe('value');
		});

		it('should support PUT and PATCH helpers', async () => {
			fetchMock = mockFetchSuccess({ ok: true });

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			await client.put('/users/123', { name: 'Updated' });
			await client.patch('/users/123', { name: 'Partial' });

			expect(fetchMock).toHaveBeenCalledTimes(2);
			expect(fetchMock.mock.calls[0]?.[1]?.method).toBe('PUT');
			expect(fetchMock.mock.calls[1]?.[1]?.method).toBe('PATCH');
		});
	});

	describe('authentication', () => {
		it('should include access token in Authorization header', async () => {
			fetchMock = mockFetchSuccess({});

			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('access-token-123', 'refresh-token');

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await client.get('/protected');

			expectAuthHeader(fetchMock, 'access-token-123');
		});

		it('should not include Authorization header when no token', async () => {
			fetchMock = mockFetchSuccess({});

			const tokenStore = new FakeTokenStore();

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await client.get('/public');

			expectNoAuthHeader(fetchMock);
		});

		it('should refresh token on 401 and retry request', async () => {
			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('old-token', 'refresh-token');

			fetchMock = mockFetchSequence(
				// First call: 401 with old token
				{ ok: false, status: 401, error: { code: 'auth.token_expired', message: 'Token expired' } },
				// Refresh call: returns new tokens
				{ ok: true, status: 200, data: { accessToken: 'new-access-token', refreshToken: 'new-refresh-token' } },
				// Retry call: succeeds with new token
				{ ok: true, status: 200, data: { id: '123' } }
			);

			const refresh = vi.fn(async ({ rawRequest, getRefreshToken }) => {
				const refreshToken = await getRefreshToken();
				if (!refreshToken) return { success: false };

				const response = await rawRequest<{ data: { accessToken: string; refreshToken: string } }>({
					method: 'POST',
					path: '/auth/refresh',
					body: { refreshToken }
				});

				return {
					success: true,
					accessToken: response.data.accessToken,
					refreshToken: response.data.refreshToken
				};
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore, refresh },
				fetch: fetchMock
			});

			const result = await client.get<{ id: string }>('/protected');

			expect(refresh).toHaveBeenCalledTimes(1);
			expect(fetchMock).toHaveBeenCalledTimes(3);
			tokenStore.expectTokens('new-access-token', 'new-refresh-token');
			expect(result).toEqual({ data: { id: '123' } });
		});

		it('should clear tokens and throw error if refresh fails', async () => {
			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('old-token', 'invalid-refresh-token');

			fetchMock = mockFetchSequence(
				// First call: 401
				{ ok: false, status: 401, error: { code: 'auth.token_expired', message: 'Token expired' } },
				// Refresh call: fails
				{ ok: false, status: 401, error: { code: 'auth.refresh_invalid', message: 'Invalid refresh token' } }
			);

			const refresh = vi.fn(async () => ({ success: false }));

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore, refresh },
				fetch: fetchMock
			});

			await expect(client.get('/protected')).rejects.toThrow(UnderlayHttpError);

			tokenStore.expectTokens(null, null);
		});

		it('rethrows 401 errors when no refresh handler is configured', async () => {
			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('expired-token', 'refresh-token');
			fetchMock = mockFetchSequence(
				{ ok: false, status: 401, error: { code: 'auth.token_expired', message: 'Token expired' } }
			);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await expect(client.get('/protected')).rejects.toThrow(UnderlayHttpError);
			expect(fetchMock).toHaveBeenCalledTimes(1);
		});

		it('does not call refresh for non-401 auth errors', async () => {
			fetchMock = mockFetchSequence(
				{ ok: false, status: 403, error: { code: 'auth.forbidden', message: 'Forbidden' } }
			);
			const refresh = vi.fn(async () => ({ success: true }));
			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { refresh },
				fetch: fetchMock
			});

			await expect(client.get('/protected')).rejects.toThrow(UnderlayHttpError);
			expect(refresh).not.toHaveBeenCalled();
		});

		it('retries refresh-success requests even without token providers', async () => {
			fetchMock = mockFetchSequence(
				{ ok: false, status: 401, error: { code: 'auth.token_expired', message: 'Expired' } },
				{ ok: true, status: 200, data: { id: '123' } }
			);

			const refresh = vi.fn(async () => ({ success: true }));
			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { refresh },
				fetch: fetchMock
			});

			const result = await client.get<{ id: string }>('/protected');
			expect(result).toEqual({ data: { id: '123' } });
			expect(refresh).toHaveBeenCalledTimes(1);
		});

		it('should not override explicit Authorization headers', async () => {
			fetchMock = mockFetchSuccess({ ok: true });

			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('store-access-token', 'store-refresh-token');

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await client.get('/protected', { Authorization: 'Bearer explicit-token' });

			const { headers } = getFetchCallArgs(fetchMock);
			expect(headers.Authorization).toBe('Bearer explicit-token');
		});

		it('passes refresh-context token setters through to token store', async () => {
			const tokenStore = new FakeTokenStore();
			tokenStore.seedTokens('expired-token', 'refresh-token');

			fetchMock = mockFetchSequence(
				{ ok: false, status: 401, error: { code: 'auth.token_expired', message: 'Token expired' } },
				{ ok: true, status: 200, data: { id: '123' } }
			);

			const refresh = vi.fn(async ({ setAccessToken, setRefreshToken }) => {
				await setAccessToken('fresh-access-token');
				await setRefreshToken('fresh-refresh-token');
				return { success: true };
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore, refresh },
				fetch: fetchMock
			});

			await client.get('/protected');

			tokenStore.expectTokens('fresh-access-token', 'fresh-refresh-token');
			expect(refresh).toHaveBeenCalledTimes(1);
		});
	});

	describe('retry logic', () => {
		it('should retry on 502/503/504 for GET requests', async () => {
			fetchMock = mockFetchSequence(
				{ ok: false, status: 503, error: { code: 'server.unavailable', message: 'Service unavailable' } },
				{ ok: false, status: 503, error: { code: 'server.unavailable', message: 'Service unavailable' } },
				{ ok: true, status: 200, data: { id: '123' } }
			);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				maxRetries: 3,
				fetch: fetchMock
			});

			const result = await client.get<{ id: string }>('/resource');

			expect(fetchMock).toHaveBeenCalledTimes(3);
			expect(result).toEqual({ data: { id: '123' } });
		});

		it('should not retry on 502/503/504 for POST requests', async () => {
			fetchMock = mockFetchError('server.unavailable', 'Service unavailable', 503);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				maxRetries: 3,
				fetch: fetchMock
			});

			await expect(client.post('/resource', {})).rejects.toThrow(UnderlayHttpError);

			expect(fetchMock).toHaveBeenCalledTimes(1); // No retries for POST
		});

		it('should retry on custom retry statuses', async () => {
			fetchMock = mockFetchSequence(
				{ ok: false, status: 429, error: { code: 'rate_limit', message: 'Too many requests' } },
				{ ok: true, status: 200, data: { id: '123' } }
			);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				maxRetries: 3,
				retryStatuses: [429],
				fetch: fetchMock
			});

			const result = await client.get<{ id: string }>('/resource');

			expect(fetchMock).toHaveBeenCalledTimes(2);
			expect(result).toEqual({ data: { id: '123' } });
		});

		it('should respect maxRetries limit', async () => {
			// Mock will return 503 indefinitely
			fetchMock = vi.fn().mockResolvedValue({
				ok: false,
				status: 503,
				json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				maxRetries: 2,
				fetch: fetchMock
			});

			await expect(client.get('/resource')).rejects.toThrow(UnderlayHttpError);

			expect(fetchMock).toHaveBeenCalledTimes(3); // Original + 2 retries
		});
	});

	describe('timeout', () => {
		it('should timeout GET requests after specified time', async () => {
			vi.useFakeTimers();
			fetchMock = vi.fn().mockImplementation((_url, init?: RequestInit) => {
				return new Promise((_resolve, reject) => {
					init?.signal?.addEventListener('abort', () => reject(new Error('Aborted')));
				});
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				timeoutMs: 5000, // 5 seconds
				fetch: fetchMock
			});

			const promise = client.get('/slow-endpoint');
			const rejection = expect(promise).rejects.toMatchObject({
				status: 0,
				message: 'Aborted'
			});

			// Advance timers to trigger timeout
			await vi.advanceTimersByTimeAsync(5000);

			await rejection;
			vi.useRealTimers();
		});

		it.skip('should not timeout POST requests', async () => {
			fetchMock = mockFetchWithDelay({ id: '123' }, 10000);

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				timeoutMs: 5000,
				fetch: fetchMock
			});

			const promise = client.post('/upload', { large: 'data' });

			// Advance timers past timeout
			await vi.advanceTimersByTimeAsync(10000);

			const result = await promise;
			expect(result).toEqual({ data: { id: '123' } });
		});
	});

	describe('error handling', () => {
		it('should throw UnderlayHttpError with error envelope', async () => {
			fetchMock = mockFetchError('validation.failed', 'Validation failed', 400, { email: 'Invalid email' });

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			try {
				await client.post('/users', { email: 'invalid' });
				expect.fail('Should have thrown error');
			} catch (error) {
				expect(error).toBeInstanceOf(UnderlayHttpError);
				const httpError = error as UnderlayHttpError;
				expect(httpError.status).toBe(400);
				expect(httpError.code).toBe('validation.failed');
				expect(httpError.message).toBe('Validation failed');
				expect(httpError.fieldErrors).toEqual({ email: 'Invalid email' });
				}
			});

		it('should throw UnderlayHttpError for network errors', async () => {
			fetchMock.mockRejectedValueOnce(new Error('Network error'));

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			await expect(client.get('/resource')).rejects.toThrow(UnderlayHttpError);
		});

		it('should map non-Error throws to a generic network message', async () => {
			fetchMock.mockRejectedValueOnce('boom');

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			await expect(client.get('/resource')).rejects.toMatchObject({
				status: 0,
				message: 'Network error'
			});
		});
	});

	describe('debug logging', () => {
		it('should log requests when debug is enabled', async () => {
			const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
			fetchMock = mockFetchSuccess({});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				debug: true,
				fetch: fetchMock
			});

			await client.get('/test');

			expect(consoleSpy).toHaveBeenCalledWith(
				expect.stringContaining('[HTTP]'),
				expect.stringContaining('GET'),
				expect.stringContaining('/test')
			);

			consoleSpy.mockRestore();
		});
	});

	describe('response metadata', () => {
		it('should return status, headers, and body from getWithMeta', async () => {
			fetchMock = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 200,
				headers: new Headers({
					'content-type': 'application/json',
					etag: 'W/"abc123"'
				}),
				json: async () => ({ data: { id: '123' } })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.getWithMeta<{ data: { id: string } }>('/resource');
			expect(response.status).toBe(200);
			expect(response.headers.etag).toBe('W/"abc123"');
			expect(response.body).toEqual({ data: { id: '123' } });
		});

		it('should allow accepted 304 responses without throwing', async () => {
			fetchMock = vi.fn().mockResolvedValueOnce({
				ok: false,
				status: 304,
				headers: new Headers({
					etag: 'W/"abc123"'
				}),
				json: async () => {
					throw new Error('Not modified');
				}
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.getWithMeta('/resource', undefined, {
				acceptedStatuses: [304]
			});
			expect(response.status).toBe(304);
			expect(response.headers.etag).toBe('W/"abc123"');
			expect(response.body).toBeNull();
		});

		it('should parse text responses when content-type is non-json', async () => {
			fetchMock = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 200,
				headers: new Headers({
					'content-type': 'text/plain'
				}),
				text: async () => 'plain-text-response'
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.getWithMeta<string>('/resource');
			expect(response.status).toBe(200);
			expect(response.body).toBe('plain-text-response');
		});
	});
});

describe('MemoryTokenStore', () => {
	it('should store and retrieve access token', () => {
		const store = new MemoryTokenStore();
		expect(store.getAccessToken()).toBeNull();

		store.setAccessToken('test-token');
		expect(store.getAccessToken()).toBe('test-token');
	});

	it('should store and retrieve refresh token', () => {
		const store = new MemoryTokenStore();
		expect(store.getRefreshToken()).toBeNull();

		store.setRefreshToken('refresh-token');
		expect(store.getRefreshToken()).toBe('refresh-token');
	});

	it('should clear all tokens', () => {
		const store = new MemoryTokenStore();
		store.setAccessToken('access');
		store.setRefreshToken('refresh');

		store.clear();

		expect(store.getAccessToken()).toBeNull();
		expect(store.getRefreshToken()).toBeNull();
	});
});
