// @vitest-environment jsdom
import { flushSync } from "svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createPaginationController } from "../../src/patterns/pagination-server.svelte";
import type {
	CursorPaginatedResponse,
	CursorPaginationParams,
} from "../../src/patterns/pagination-types";
import { createEffectRoot } from "./effect-root.svelte";

interface Item {
	id: string;
}

function deferred<T>() {
	let resolve!: (value: T) => void;
	let reject!: (reason?: unknown) => void;
	const promise = new Promise<T>((res, rej) => {
		resolve = res;
		reject = rej;
	});
	return { promise, resolve, reject };
}

function pageResponse(
	ids: string[],
	opts: {
		nextCursor?: string | null;
		prevCursor?: string | null;
		hasMore?: boolean;
		total?: number;
	} = {},
): CursorPaginatedResponse<Item> {
	return {
		data: ids.map((id) => ({ id })),
		nextCursor: opts.nextCursor ?? null,
		prevCursor: opts.prevCursor ?? null,
		hasMore: opts.hasMore ?? false,
		total: opts.total ?? ids.length,
	};
}

describe("patterns/pagination-server.svelte.ts (compiled)", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("rolls back currentPage and cursorHistory when nextPage or prevPage fails or getToken returns null", async () => {
		let token: string | null = "tok-1";
		const fetcher = vi.fn(
			async (_fetch: typeof fetch, _token: string, params: CursorPaginationParams) => {
				if (params.cursor === null) {
					return pageResponse(["a"], {
						nextCursor: "c2",
						prevCursor: null,
						hasMore: true,
						total: 3,
					});
				}
				if (params.cursor === "c2") {
					return pageResponse(["b"], {
						nextCursor: "c3",
						prevCursor: "p2",
						hasMore: true,
						total: 3,
					});
				}
				throw new Error("network failure");
			},
		);

		let controller!: ReturnType<typeof createPaginationController<Item>>;
		const dispose = createEffectRoot(() => {
			controller = createPaginationController<Item>(fetcher, {
				getToken: () => token,
				pageSize: 1,
			});
		});

		try {
			await controller.tryFetch(false, { id: "u1" });
			expect(controller.currentPage).toBe(1);

			// Advance to page 2 (succeeds)
			await controller.nextPage();
			expect(controller.currentPage).toBe(2);
			expect(controller.items).toEqual([{ id: "b" }]);

			// Advance to page 3 (fetcher throws -> rolls back to page 2)
			await controller.nextPage();
			expect(controller.currentPage).toBe(2);
			expect(controller.error).toBe("network failure");

			// Attempt nextPage with null token -> rolls back to page 2
			token = null;
			await controller.nextPage();
			expect(controller.currentPage).toBe(2);

			// Attempt prevPage with null token -> rolls back to page 2
			await controller.prevPage();
			expect(controller.currentPage).toBe(2);

			// Restore token and verify cursorHistory was preserved intact by navigating back to page 1
			token = "tok-1";
			await controller.prevPage();
			expect(controller.currentPage).toBe(1);
			expect(controller.items).toEqual([{ id: "a" }]);
		} finally {
			dispose();
		}
	});

	it("supersedes and queues setPageSize behind an active in-flight request without dropping the new size", async () => {
		const first = deferred<CursorPaginatedResponse<Item>>();
		const second = deferred<CursorPaginatedResponse<Item>>();
		const limits: number[] = [];

		const fetcher = vi.fn(
			async (_fetch: typeof fetch, _token: string, params: CursorPaginationParams) => {
				limits.push(params.limit);
				return limits.length === 1 ? first.promise : second.promise;
			},
		);

		let controller!: ReturnType<typeof createPaginationController<Item>>;
		const dispose = createEffectRoot(() => {
			controller = createPaginationController<Item>(fetcher, {
				getToken: () => "tok-1",
				pageSize: 10,
			});
		});

		try {
			const initialFetch = controller.tryFetch(false, { id: "u1" });
			expect(controller.loading).toBe(true);
			expect(limits).toEqual([10]);

			// Change page size while initial fetch is still in flight
			controller.setPageSize(25);
			expect(controller.pageSize).toBe(25);
			expect(controller.currentPage).toBe(1);

			// Resolve the superseded first request; its stale data must not overwrite items
			first.resolve(pageResponse(["stale-1"], { total: 1 }));
			await initialFetch;

			// Allow queued microtask to start the follow-up request
			await Promise.resolve();
			expect(limits).toEqual([10, 25]);
			expect(controller.items).toEqual([]);
			expect(controller.loading).toBe(true);

			// Resolve the follow-up request with the new page size
			second.resolve(pageResponse(["fresh-1", "fresh-2"], { total: 2 }));
			await new Promise((r) => setTimeout(r, 0));

			expect(controller.items).toEqual([{ id: "fresh-1" }, { id: "fresh-2" }]);
			expect(controller.loading).toBe(false);
		} finally {
			dispose();
		}
	});

	it("survives localStorage errors and ignores responses after unmount disposal", async () => {
		vi.spyOn(Storage.prototype, "getItem").mockImplementation(() => {
			throw new DOMException("SecurityError");
		});
		vi.spyOn(Storage.prototype, "setItem").mockImplementation(() => {
			throw new DOMException("QuotaExceededError");
		});

		const pending = deferred<CursorPaginatedResponse<Item>>();
		const onSuccess = vi.fn();
		const fetcher = vi.fn(async () => pending.promise);

		let controller!: ReturnType<typeof createPaginationController<Item>>;
		const dispose = createEffectRoot(() => {
			controller = createPaginationController<Item>(fetcher, {
				getToken: () => "tok-1",
				pageSize: 15,
				persistKey: "restricted-key",
				onSuccess,
			});
		});
		flushSync();

		expect(controller.pageSize).toBe(15);

		const inFlight = controller.tryFetch(false, { id: "u1" });
		expect(() => controller.setPageSize(20)).not.toThrow();
		expect(controller.pageSize).toBe(20);

		// Dispose before the response resolves
		dispose();
		flushSync();

		pending.resolve(pageResponse(["after-unmount"], { total: 1 }));
		await inFlight;
		await new Promise((r) => setTimeout(r, 0));

		expect(controller.items).toEqual([]);
		expect(onSuccess).not.toHaveBeenCalled();
	});
});
