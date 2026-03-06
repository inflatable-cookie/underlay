# Test Utilities Patterns Analysis

**Date**: 2026-01-12  
**Status**: Patterns Identified  
**Next Step**: Complete Phase 8.6 Extraction

## Overview

After writing comprehensive tests for `ts/src/client/http.ts` (~450 lines of tests), **clear reusable patterns have emerged**. This document identifies the patterns and proposes extraction to reusable test utilities.

---

## Patterns Identified

### 1. Mock Fetch Builders

**Pattern**: Creating fetch mocks for different scenarios is repetitive

**Before**:
```typescript
fetchMock.mockResolvedValueOnce({
	ok: true,
	status: 200,
	json: async () => ({ data: { id: '123' } })
});
```

**After (with utility)**:
```typescript
fetchMock = mockFetchSuccess({ id: '123' });
```

**Utilities Created**:
- `mockFetchSuccess(data, status)` - Success response
- `mockFetchError(code, message, status, fieldErrors)` - Error response
- `mockFetchNoContent()` - 204 No Content
- `mockFetchNetworkError(message)` - Network failure
- `mockFetchSequence(...responses)` - Multi-step flows (retry logic, auth refresh)
- `mockFetchWithDelay(data, delayMs)` - Timeout testing

**LOC Reduction**: ~50% in mock setup code

---

### 2. Fake Token Store

**Pattern**: Testing auth requires token storage with spies

**Before**:
```typescript
const tokenStore = new MemoryTokenStore();
tokenStore.setAccessToken('token');
// ... later ...
expect(tokenStore.getAccessToken()).toBe('new-token');
```

**After (with utility)**:
```typescript
const tokenStore = new FakeTokenStore();
tokenStore.seedTokens('access', 'refresh');
// ... later ...
tokenStore.expectTokens('new-access', 'new-refresh');
```

**Features**:
- Extends `MemoryTokenStore` with vi.fn() spies on all methods
- `seedTokens(access, refresh)` - Set initial state
- `expectTokens(access, refresh)` - Assert final state
- All method calls are trackable with `expect(tokenStore.setAccessToken).toHaveBeenCalled()`

**LOC Reduction**: ~30% in auth test setup

---

### 3. Fetch Call Assertions

**Pattern**: Extracting and asserting fetch call arguments

**Before**:
```typescript
const callArgs = fetchMock.mock.calls[0];
const [url, options] = callArgs;
const headers = options?.headers as Record<string, string>;
expect(headers['Authorization']).toBe('Bearer token');
```

**After (with utility)**:
```typescript
expectAuthHeader(fetchMock, 'token');
```

**Utilities Created**:
- `getFetchCallArgs(mock, index)` - Extract { url, method, headers, body }
- `expectFetchHeaders(mock, headers, index)` - Assert specific headers
- `expectAuthHeader(mock, token, index)` - Assert Authorization header
- `expectNoAuthHeader(mock, index)` - Assert no Authorization header

**LOC Reduction**: ~40% in assertion code

---

## Test Code Comparison

### Original Test (Without Utilities)

```typescript
it('should retry on 502/503/504 for GET requests', async () => {
	// Setup: 3 fetch mocks
	fetchMock.mockResolvedValueOnce({
		ok: false,
		status: 503,
		json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
	});
	fetchMock.mockResolvedValueOnce({
		ok: false,
		status: 503,
		json: async () => ({ error: { code: 'server.unavailable', message: 'Service unavailable' } })
	});
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
	expect(result).toEqual({ id: '123' });
});
```

**Lines**: 25

### Refactored Test (With Utilities)

```typescript
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
	expect(result).toEqual({ id: '123' });
});
```

**Lines**: 16  
**Reduction**: 36%

---

## Code Metrics

### Original Tests
- **File**: `http.test.ts`
- **Lines**: ~450
- **Mock setup**: ~180 lines
- **Assertions**: ~100 lines
- **Test logic**: ~170 lines

### Refactored Tests
- **File**: `http-refactored.test.ts`
- **Lines**: ~320
- **Mock setup**: ~80 lines (↓ 56%)
- **Assertions**: ~60 lines (↓ 40%)
- **Test logic**: ~180 lines (↑ 6%, more readable)

### Test Utilities
- **File**: `test-utils.ts`
- **Lines**: ~220
- **Reusable across all client tests**

### Net Result
- **Original**: 450 lines
- **Refactored + Utilities**: 320 + 220 = 540 lines
- **Extra lines**: 90 lines (20% increase)

**BUT**:
- **Reusability**: Utilities usable across 10+ test files
- **Readability**: Tests 40% more concise and readable
- **Maintainability**: Mock logic centralized
- **Future tests**: ~50% less boilerplate

**Projected savings** (10 test files):
- Without utilities: ~4,500 lines
- With utilities: ~3,200 + 220 = ~3,420 lines
- **Savings**: ~24% (~1,080 lines)

---

## Phase 8.6 Extraction Plan

### Files to Create

#### 1. TypeScript Testing Utilities

**Location**: `underlay/ts/src/testing/http-mocks.ts`

**Exports**:
```typescript
// Mock builders
export function mockFetchSuccess<T>(data: T, status?: number)
export function mockFetchError(code: string, message: string, status?: number, fieldErrors?: Record<string, string>)
export function mockFetchNoContent()
export function mockFetchNetworkError(message?: string)
export function mockFetchSequence(...responses: Response[])
export function mockFetchWithDelay<T>(data: T, delayMs: number, status?: number)

// Fake implementations
export class FakeTokenStore implements TokenStore
export class FakeHttpClient // (future, if needed)

// Assertion helpers
export function getFetchCallArgs(mock: MockFn, index?: number)
export function expectFetchHeaders(mock: MockFn, headers: Record<string, string>, index?: number)
export function expectAuthHeader(mock: MockFn, token: string, index?: number)
export function expectNoAuthHeader(mock: MockFn, index?: number)
```

**File size**: ~220 lines (already written!)

#### 2. Documentation

**Location**: `underlay/docs/guides/testing/http-client-testing.md`

**Sections**:
- Overview of testing HTTP clients
- Mock builders guide
- Fake implementations guide
- Assertion helpers guide
- Common patterns (retry, auth, errors)
- Examples

**File size**: ~150 lines

#### 3. Update vitest.config.ts

Add test utilities to coverage exclusions:
```typescript
exclude: [
	'ts/src/testing/**' // Test utilities
]
```

---

## Benefits

### 1. Reduced Boilerplate

**Mock setup**: 50% reduction
```typescript
// Before: 6 lines
fetchMock.mockResolvedValueOnce({
	ok: true,
	status: 200,
	json: async () => ({ data: { id: '123' } })
});

// After: 1 line
fetchMock = mockFetchSuccess({ id: '123' });
```

### 2. Improved Readability

**Tests are more declarative**:
```typescript
// Before: Implementation details visible
const callArgs = fetchMock.mock.calls[0];
const headers = callArgs[1]?.headers;
expect(headers['Authorization']).toBe('Bearer token');

// After: Intent is clear
expectAuthHeader(fetchMock, 'token');
```

### 3. Easier Maintenance

**Change mock structure once, not in every test**:
- If error envelope structure changes, update `mockFetchError()`
- All tests automatically updated

### 4. Consistency

**All tests use same patterns**:
- No variations in how mocks are created
- No missed assertions
- Easier to review

### 5. Faster Test Writing

**Utilities enable rapid test creation**:
- Copy-paste template test
- Replace mock with appropriate utility
- Add assertions with helpers
- Done in ~2 minutes vs ~5 minutes

---

## Comparison to Original Phase 8.6 Plan

### Original Plan (Deferred)

**Proposed**:
- Extract `FakeHttpClient` from cattle-grid tests (didn't exist)
- Create `underlay-testing` package (premature)
- Unknown patterns (no tests written)

**Status**: ⏸️ Deferred (nothing to extract)

### New Plan (After Writing Tests)

**Reality**:
- ✅ **Real patterns identified** from actual test code
- ✅ **220 lines of utilities** already written
- ✅ **Proven value** (36% LOC reduction in tests)
- ✅ **Ready to extract** to proper location

**Status**: ✅ Ready to proceed

---

## Recommendation

**Proceed with Phase 8.6 extraction** using the patterns identified:

1. ✅ **Test utilities exist and are proven valuable**
2. ✅ **Clear patterns emerged from real test code**
3. ✅ **Significant code reduction demonstrated** (~36%)
4. ✅ **Reusable across all client tests** (auth, http, sveltekit)

**Next Steps**:
1. Move `test-utils.ts` to `ts/src/testing/http-mocks.ts`
2. Create `ts/src/testing/index.ts` barrel export
3. Update `package.json` exports to include `"./testing"`
4. Write documentation (`docs/guides/testing/http-client-testing.md`)
5. Create examples showing before/after
6. Update Phase 8.6 roadmap with completion details

---

## Future Extensions

### Additional Utilities (As Patterns Emerge)

1. **SvelteKit Test Helpers**:
   - `mockSvelteKitEvent()`
   - `FakeCookies`
   - `mockFormData()`

2. **Auth Test Helpers**:
   - `mockLoginResponse()`
   - `mockSessionResponse()`
   - `mockRefreshFlow()`

3. **Component Test Helpers** (if we add component tests):
   - `renderWithAuth()`
   - `mockToastStore()`

**Approach**: Add as we write more tests, not speculatively.

---

## Conclusion

**Phase 8.6 is now ready to proceed** with real, battle-tested patterns extracted from actual test code.

**Value Proposition**:
- **Immediate**: 36% reduction in test boilerplate
- **Future**: ~24% reduction across all client tests
- **Quality**: More readable, maintainable, consistent tests

**Effort**: ~1 hour to move files and write documentation

**ROI**: Extremely high - patterns proven, code written, just needs organization.
