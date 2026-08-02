import { describe, expect, it, vi } from "vitest";

type PaginationModule = typeof import("../../src/patterns/pagination.svelte");

async function loadPaginationModule(options?: {
	globalConfig?: Record<string, unknown> | null;
}) {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);

	vi.doMock("../../src/patterns/auth", () => ({
		getAuthConfig: () => options?.globalConfig ?? null,
	}));

	return await import("../../src/patterns/pagination.svelte");
}

describe("patterns/pagination.svelte.ts", () => {
	it("defers missing getToken to the fetch path so setup stays SSR-safe", async () => {
		const modWithoutAuth = await loadPaginationModule();
		const controller = modWithoutAuth.createPaginationController(
			async () => ({
				data: [],
				nextCursor: null,
				prevCursor: null,
				hasMore: false,
				total: 0,
			}),
			{}
		);
		expect(controller.error).toBeNull();

		await controller.tryFetch(false, { id: "u1" });
		expect(controller.error).toMatch(/getToken is required/);
		expect(controller.loading).toBe(false);
	});

	it("supports server pagination fetch/refresh/setPageSize", async () => {
		const storage = {
			data: new Map<string, string>([["pagination:test", "11"]]),
			getItem: vi.fn((k: string) => storage.data.get(k) ?? null),
			setItem: vi.fn((k: string, v: string) => storage.data.set(k, v)),
		};
		(globalThis as any).localStorage = storage;

		const mod = await loadPaginationModule({
			globalConfig: {
				getToken: () => "token-1",
				onRefresh: vi.fn(async () => "token-2"),
			},
		});

		const onSuccess = vi.fn();
		const onError = vi.fn();
		const fetcher = vi
			.fn()
			.mockResolvedValueOnce({
				data: [{ id: "a" }],
				nextCursor: "next-1",
				prevCursor: null,
				hasMore: true,
				total: 3,
			})
			.mockResolvedValueOnce({
				data: [{ id: "b" }],
				nextCursor: null,
				prevCursor: "prev-1",
				hasMore: false,
				total: 3,
			})
			.mockResolvedValueOnce({
				data: [{ id: "c" }],
				nextCursor: null,
				prevCursor: null,
				hasMore: false,
				total: 1,
			});

		const controller = mod.createPaginationController(fetcher as any, {
			persistKey: "pagination:test",
			onSuccess,
			onError,
		});

		await controller.tryFetch(true, { id: "u1" });
		expect(fetcher).not.toHaveBeenCalled();

		await controller.tryFetch(false, null);
		expect(fetcher).not.toHaveBeenCalled();

		await controller.tryFetch(false, { id: "u1" });
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(controller.items).toEqual([{ id: "a" }]);
		expect(controller.total).toBe(3);
		expect(controller.pageSize).toBe(11);
		expect(controller.error).toBeNull();
		expect(onSuccess).toHaveBeenCalledTimes(1);
		expect(onError).not.toHaveBeenCalled();

		await controller.refresh();
		expect(fetcher).toHaveBeenCalledTimes(2);
		expect(controller.currentPage).toBe(1);

		controller.setPageSize(17);
		expect(storage.setItem).toHaveBeenCalledWith("pagination:test", "17");
		expect(fetcher).toHaveBeenCalledTimes(3);

		await controller.reset();
		expect(fetcher).toHaveBeenCalledTimes(3);

		delete (globalThis as any).localStorage;
	});

	it("handles server pagination error and refresh-retry branches", async () => {
		const onRefresh = vi
			.fn()
			.mockResolvedValueOnce("fresh")
			.mockResolvedValueOnce(null);
		const mod = await loadPaginationModule({
			globalConfig: {
				getToken: () => "token",
				onRefresh,
			},
		});

		const onError = vi.fn();
		const fetcher = vi
			.fn()
			.mockRejectedValueOnce({ status: 401 })
			.mockResolvedValueOnce({
				data: [{ id: "retry" }],
				nextCursor: null,
				prevCursor: null,
				hasMore: false,
				total: 1,
			})
			.mockRejectedValueOnce({ status: 401 })
			.mockRejectedValueOnce("boom");

		const controller = mod.createPaginationController(fetcher as any, { onError });
		await controller.tryFetch(false, { id: "u1" });
		expect(controller.items).toEqual([{ id: "retry" }]);
		expect(controller.error).toBeNull();
		expect(onRefresh).toHaveBeenCalledTimes(1);

		await controller.refresh();
		expect(controller.error).toBe("Session expired");
		expect(onRefresh).toHaveBeenCalledTimes(2);

		await controller.refresh();
		expect(controller.error).toBe("Failed to load data");
		expect(onError).toHaveBeenCalled();
	});

	it("supports client pagination with local data and persisted page size", async () => {
		const storage = {
			data: new Map<string, string>([
				["client:persist", "4"],
				["client:bad", "not-number"],
			]),
			getItem: vi.fn((k: string) => storage.data.get(k) ?? null),
			setItem: vi.fn((k: string, v: string) => storage.data.set(k, v)),
		};
		(globalThis as any).localStorage = storage;

		const mod = await loadPaginationModule();
		const all = [1, 2, 3, 4, 5, 6, 7];
		const pagination = mod.createClientPagination(() => all, {
			persistKey: "client:persist",
			initialPage: 2,
		});

		expect(pagination.pageSize).toBe(4);
		expect(pagination.total).toBe(7);
		expect(pagination.totalPages).toBe(2);
		expect(pagination.currentPage).toBe(2);
		expect(pagination.items).toEqual([5, 6, 7]);

		pagination.goToPage(999);
		expect(pagination.currentPage).toBe(2);
		pagination.goToPage(0);
		expect(pagination.currentPage).toBe(2);

		pagination.nextPage();
		pagination.prevPage();
		pagination.setPageSize(3);
		expect(storage.setItem).toHaveBeenCalledWith("client:persist", "3");

		await pagination.refresh();
		await pagination.reset();
		expect(pagination.currentPage).toBe(2);

		const fallback = mod.createClientPagination(() => [1, 2], {
			persistKey: "client:bad",
			pageSize: 9,
		});
		expect(fallback.pageSize).toBe(9);

		delete (globalThis as any).localStorage;
	});
});
