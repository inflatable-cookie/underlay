import { describe, expect, it } from 'vitest';
import { createMockHttpClient } from '../../src/testing/http-client-mock';

describe('createMockHttpClient', () => {
	it('tracks calls and returns nextResponse', async () => {
		const client = createMockHttpClient();
		client.nextResponse = { data: { ok: true } };

		const result = await client.get('/v1/admin/content/summaries');

		expect(client.calls).toEqual([
			{ method: 'GET', path: '/v1/admin/content/summaries', headers: undefined }
		]);
		expect(result).toEqual({ data: { ok: true } });
	});

	it('supports path-specific responses', async () => {
		const client = createMockHttpClient({ nextResponse: { data: 'fallback' } });
		client.setResponse('GET', '/v1/a', { data: 'A' });
		client.setResponse('GET', '/v1/b', (call) => ({ data: call.path }));

		const first = await client.get('/v1/a');
		const second = await client.get('/v1/b');
		const third = await client.get('/v1/c');

		expect(first).toEqual({ data: 'A' });
		expect(second).toEqual({ data: '/v1/b' });
		expect(third).toEqual({ data: 'fallback' });
	});

	it('returns request metadata responses via getWithMeta', async () => {
		const client = createMockHttpClient();
		client.setResponse('GET', '/v1/modules', {
			status: 304,
			headers: { etag: 'W/"modules-v1"' },
			body: null
		});

		const response = await client.getWithMeta('/v1/modules', { 'If-None-Match': 'W/"modules-v1"' }, {
			acceptedStatuses: [200, 304]
		});

		expect(client.calls[0]).toEqual({
			method: 'GET',
			path: '/v1/modules',
			headers: { 'If-None-Match': 'W/"modules-v1"' },
			options: { acceptedStatuses: [200, 304] }
		});
		expect(response).toEqual({
			status: 304,
			headers: { etag: 'W/"modules-v1"' },
			body: null
		});
	});

	it('resets call and response state', async () => {
		const client = createMockHttpClient();
		client.nextResponse = { ok: true };
		await client.get('/v1/check');

		client.reset();

		expect(client.calls).toEqual([]);
		expect(client.nextResponse).toBeUndefined();
		expect(client.responses.size).toBe(0);
	});
});
