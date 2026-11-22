// @vitest-environment jsdom
import { render } from "@testing-library/svelte";
import { flushSync } from "svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import { createPaginationController } from "../../src/patterns/pagination-server.svelte";
import type {
	CursorPaginatedResponse,
	CursorPaginationParams,
} from "../../src/patterns/pagination-types";
import { createEffectRoot } from "./effect-root.svelte";
import ImmediateUnmountHarness from "./ImmediateUnmountHarness.svelte";

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

	it("rolls back currentPage and cursorHistory when getToken() throws during nextPage or prevPage", async () => {
		let shouldThrow = false;
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
				return pageResponse(["b"], {
					nextCursor: "c3",
					prevCursor: "p2",
					hasMore: true,
					total: 3,
				});
			},
		);

		let controller!: ReturnType<typeof createPaginationController<Item>>;
		const dispose = createEffectRoot(() => {
			controller = createPaginationController<Item>(fetcher, {
				getToken: () => {
					if (shouldThrow) {
						throw new Error("token store unavailable");
					}
					return "tok-1";
				},
				pageSize: 1,
			});
		});

		try {
			await controller.tryFetch(false, { id: "u1" });
			expect(controller.currentPage).toBe(1);

			// Throwing getToken() on nextPage rejects and rolls back to page 1
			shouldThrow = true;
			await expect(controller.nextPage()).rejects.toThrow("token store unavailable");
			expect(controller.currentPage).toBe(1);
			expect(controller.items).toEqual([{ id: "a" }]);

			// Advance to page 2 normally
			shouldThrow = false;
			await controller.nextPage();
			expect(controller.currentPage).toBe(2);

			// Throwing getToken() on prevPage rejects and rolls back to page 2 with cursorHistory intact
			shouldThrow = true;
			await expect(controller.prevPage()).rejects.toThrow("token store unavailable");
			expect(controller.currentPage).toBe(2);

			shouldThrow = false;
			await controller.prevPage();
			expect(controller.currentPage).toBe(1);
			expect(controller.items).toEqual([{ id: "a" }]);
		} finally {
			dispose();
		}
	});

	it("survives an opaque-origin localStorage getter that throws on property access", async () => {
		const originalDescriptor = Object.getOwnPropertyDescriptor(globalThis, "localStorage");
		Object.defineProperty(globalThis, "localStorage", {
			configurable: true,
			get() {
				throw new DOMException("The operation is insecure.", "SecurityError");
			},
		});

		const limits: number[] = [];
		const fetcher = vi.fn(
			async (_fetch: typeof fetch, _token: string, params: CursorPaginationParams) => {
				limits.push(params.limit);
				return pageResponse(["ok"], { total: 1 });
			},
		);

		let controller!: ReturnType<typeof createPaginationController<Item>>;
		let dispose = () => {};
		try {
			dispose = createEffectRoot(() => {
				controller = createPaginationController<Item>(fetcher, {
					getToken: () => "tok-1",
					pageSize: 12,
					persistKey: "opaque-origin-key",
				});
			});

			expect(controller.pageSize).toBe(12);
			expect(() => controller.setPageSize(24)).not.toThrow();
			expect(controller.pageSize).toBe(24);
			await new Promise((r) => setTimeout(r, 0));
			expect(limits).toEqual([24]);
		} finally {
			dispose();
			if (originalDescriptor) {
				Object.defineProperty(globalThis, "localStorage", originalDescriptor);
			} else {
				delete (globalThis as Record<string, unknown>).localStorage;
			}
		}
	});

	it("ignores responses when a component unmounts immediately before the initial effect flush", async () => {
		const pending = deferred<CursorPaginatedResponse<Item>>();
		let controller!: ReturnType<typeof createPaginationController<Item>>;

		const { unmount } = render(ImmediateUnmountHarness, {
			paginationFetcher: vi.fn(() => pending.promise),
			onPaginationReady: (c) => {
				controller = c;
			},
		});

		const inFlight = controller.tryFetch(false, { id: "u1" });
		// Unmount immediately without flushing effects
		unmount();

		pending.resolve(pageResponse(["post-unmount"], { total: 1 }));
		await inFlight;
		await new Promise((r) => setTimeout(r, 0));

		expect(controller.items).toEqual([]);
	});
});
