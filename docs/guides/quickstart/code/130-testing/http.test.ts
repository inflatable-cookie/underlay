import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { HttpClient } from './http.js';

describe('HttpClient', () => {
  let client: HttpClient;
  let fetchMock: any;

  beforeEach(() => {
    client = new HttpClient({ baseUrl: 'http://localhost:3000' });
    fetchMock = vi.fn();
    global.fetch = fetchMock;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('makes GET requests', async () => {
    fetchMock.mockResolvedValue({
      ok: true,
      text: async () => JSON.stringify({ data: { test: true } }),
    });

    const result = await client.get<{ test: boolean }>('/test');
    expect(result).toEqual({ test: true });
    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:3000/test',
      expect.objectContaining({ method: 'GET' })
    );
  });

  it('includes auth header when token is set', async () => {
    client.setAuthTokenGetter(() => 'test-token');
    fetchMock.mockResolvedValue({
      ok: true,
      text: async () => JSON.stringify({ data: {} }),
    });

    await client.get('/test');
    expect(fetchMock).toHaveBeenCalledWith(
      'http://localhost:3000/test',
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: 'Bearer test-token',
        }),
      })
    );
  });

  it('throws ApiError on HTTP error', async () => {
    fetchMock.mockResolvedValue({
      ok: false,
      json: async () => ({
        code: 'test.error',
        message: 'Test error',
        status_code: 400,
      }),
    });

    await expect(client.get('/test')).rejects.toThrow();
  });
});
