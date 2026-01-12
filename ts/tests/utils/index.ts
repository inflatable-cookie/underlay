/**
 * Underlay Testing Utilities
 * 
 * Reusable test helpers for HTTP clients, mocks, and assertions.
 * 
 * @example
 * ```typescript
 * import { mockFetchSuccess, FakeTokenStore, expectAuthHeader } from '@decodelabs/underlay/testing';
 * 
 * // Create a mock fetch with success response
 * const fetchMock = mockFetchSuccess({ id: '123' });
 * 
 * // Create a fake token store
 * const tokenStore = new FakeTokenStore();
 * tokenStore.seedTokens('access-token', 'refresh-token');
 * 
 * // Assert Authorization header was sent
 * expectAuthHeader(fetchMock, 'access-token');
 * ```
 */

export {
	mockFetchSuccess,
	mockFetchError,
	mockFetchNoContent,
	mockFetchNetworkError,
	mockFetchSequence,
	mockFetchWithDelay,
	FakeTokenStore,
	getFetchCallArgs,
	expectFetchHeaders,
	expectAuthHeader,
	expectNoAuthHeader
} from './http-mocks';
