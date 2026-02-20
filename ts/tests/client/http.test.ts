import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { createHttpClient, MemoryTokenStore, type HttpClientOptions } from '../../src/client/http';
import { UnderlayHttpError } from '../../src/client/errors';

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
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: { id: '123', name: 'Test' } })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.get<{ id: string; name: string }>('/users/123');

			expect(fetchMock).toHaveBeenCalledTimes(1);
			expect(fetchMock).toHaveBeenCalledWith(
				'https://api.example.com/users/123',
				expect.objectContaining({
					method: 'GET',
					headers: expect.objectContaining({
						Accept: 'application/json'
					})
				})
			);
			expect(response).toEqual({ data: { id: '123', name: 'Test' } });
		});

		it('should make POST request with body', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 201,
				json: async () => ({ data: { id: '456', name: 'Created' } })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const body = { name: 'New User', email: 'test@example.com' };
			const response = await client.post<{ id: string; name: string }>('/users', body);

			expect(fetchMock).toHaveBeenCalledWith(
				'https://api.example.com/users',
				expect.objectContaining({
					method: 'POST',
					headers: expect.objectContaining({
						'Content-Type': 'application/json',
						Accept: 'application/json'
					}),
					body: JSON.stringify(body)
				})
			);
			expect(response).toEqual({ data: { id: '456', name: 'Created' } });
		});

		it('should handle 204 No Content responses', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 204,
				json: async () => {
					throw new Error('No content');
				}
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				fetch: fetchMock
			});

			const response = await client.delete('/users/123');

			expect(response).toBeNull();
		});

		it('should add default headers to all requests', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: {} })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				defaultHeaders: {
					'X-Client-Version': '1.0.0',
					'X-Custom-Header': 'value'
				},
				fetch: fetchMock
			});

			await client.get('/test');

			expect(fetchMock).toHaveBeenCalledWith(
				expect.any(String),
				expect.objectContaining({
					headers: expect.objectContaining({
						'X-Client-Version': '1.0.0',
						'X-Custom-Header': 'value'
					})
				})
			);
		});
	});

	describe('authentication', () => {
		it('should include access token in Authorization header', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: {} })
			});

			const tokenStore = new MemoryTokenStore();
			tokenStore.setAccessToken('access-token-123');

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await client.get('/protected');

			expect(fetchMock).toHaveBeenCalledWith(
				expect.any(String),
				expect.objectContaining({
					headers: expect.objectContaining({
						Authorization: 'Bearer access-token-123'
					})
				})
			);
		});

		it('should not include Authorization header when no token', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: {} })
			});

			const tokenStore = new MemoryTokenStore();

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore },
				fetch: fetchMock
			});

			await client.get('/public');

			const callArgs = fetchMock.mock.calls[0];
			const headers = callArgs[1]?.headers as Record<string, string>;
			expect(headers['Authorization']).toBeUndefined();
		});

		it('should refresh token on 401 and retry request', async () => {
			const tokenStore = new MemoryTokenStore();
			tokenStore.setAccessToken('old-token');
			tokenStore.setRefreshToken('refresh-token');

			// First call: 401 with old token
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					error: { code: 'auth.token_expired', message: 'Token expired' }
				})
			});

			// Refresh call: returns new tokens
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({
					data: {
						accessToken: 'new-access-token',
						refreshToken: 'new-refresh-token'
					}
				})
			});

			// Retry call: succeeds with new token
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: { id: '123' } })
			});

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
			expect(fetchMock).toHaveBeenCalledTimes(3); // Original + refresh + retry
			expect(tokenStore.getAccessToken()).toBe('new-access-token');
			expect(tokenStore.getRefreshToken()).toBe('new-refresh-token');
			expect(result).toEqual({ data: { id: '123' } });
		});

		it('should clear tokens and throw error if refresh fails', async () => {
			const tokenStore = new MemoryTokenStore();
			tokenStore.setAccessToken('old-token');
			tokenStore.setRefreshToken('invalid-refresh-token');

			// First call: 401
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					error: { code: 'auth.token_expired', message: 'Token expired' }
				})
			});

			// Refresh call: fails with 401
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: async () => ({
					error: { code: 'auth.refresh_invalid', message: 'Invalid refresh token' }
				})
			});

			const refresh = vi.fn(async () => ({
				success: false
			}));

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				auth: { tokenStore, refresh },
				fetch: fetchMock
			});

			await expect(client.get('/protected')).rejects.toThrow(UnderlayHttpError);

			expect(tokenStore.getAccessToken()).toBeNull();
			expect(tokenStore.getRefreshToken()).toBeNull();
		});
	});

	describe('retry logic', () => {
		it('should retry on 502/503/504 for GET requests', async () => {
			// First attempt: 503
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 503,
				json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
			});

			// Second attempt: 503
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 503,
				json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
			});

			// Third attempt: success
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: { id: '123' } })
			});

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
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 503,
				json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				maxRetries: 3,
				fetch: fetchMock
			});

			await expect(client.post('/resource', {})).rejects.toThrow(UnderlayHttpError);

			expect(fetchMock).toHaveBeenCalledTimes(1); // No retries for POST
		});

		it('should retry on custom retry statuses', async () => {
			// First attempt: 429
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 429,
				json: async () => ({ error: { code: 'rate_limit', message: 'Too many requests' } })
			});

			// Second attempt: success
			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: { id: '123' } })
			});

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
			fetchMock.mockResolvedValue({
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
		it.skip('should timeout GET requests after specified time', async () => {
			fetchMock.mockImplementation(() => {
				return new Promise((resolve) => {
					setTimeout(() => {
						resolve({
							ok: true,
							status: 200,
							json: async () => ({ data: {} })
						});
					}, 10000); // 10 seconds
				});
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				timeoutMs: 5000, // 5 seconds
				fetch: fetchMock
			});

			const promise = client.get('/slow-endpoint');

			// Advance timers to trigger timeout
			await vi.advanceTimersByTimeAsync(5000);

			await expect(promise).rejects.toThrow();
		});

		it.skip('should not timeout POST requests', async () => {
			fetchMock.mockImplementation(() => {
				return new Promise((resolve) => {
					setTimeout(() => {
						resolve({
							ok: true,
							status: 200,
							json: async () => ({ data: { id: '123' } })
						});
					}, 10000);
				});
			});

			const client = createHttpClient({
				baseUrl: 'https://api.example.com',
				timeoutMs: 5000,
				fetch: fetchMock
			});

			const promise = client.post('/upload', { large: 'data' });

			// Advance timers past timeout
			await vi.advanceTimersByTimeAsync(10000);

			const result = await promise;
			expect(result).toEqual({ id: '123' });
		});
	});

	describe('error handling', () => {
		it('should throw UnderlayHttpError with error envelope', async () => {
			fetchMock.mockResolvedValueOnce({
				ok: false,
				status: 400,
				json: async () => ({
					error: {
						code: 'validation.failed',
						message: 'Validation failed',
						fieldErrors: { email: 'Invalid email' }
					}
				})
			});

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
	});

	describe('debug logging', () => {
		it('should log requests when debug is enabled', async () => {
			const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});

			fetchMock.mockResolvedValueOnce({
				ok: true,
				status: 200,
				json: async () => ({ data: {} })
			});

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
			fetchMock.mockResolvedValueOnce({
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
			fetchMock.mockResolvedValueOnce({
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
