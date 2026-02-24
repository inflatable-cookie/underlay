import { describe, expect, it, vi } from "vitest";

type ListModule = typeof import("../../src/patterns/list-controller.svelte");

async function loadListControllerModule(options?: { mockAuthNull?: boolean }): Promise<ListModule> {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;

	if (options?.mockAuthNull) {
		vi.doMock("../../src/patterns/auth", () => ({
			getAuthConfig: () => null,
		}));
	}

	return await import("../../src/patterns/list-controller.svelte");
}

async function flush(): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 0));
}

describe("patterns/list-controller.svelte.ts", () => {
	it("requires getToken via options or global auth config", async () => {
		const { createListController } = await loadListControllerModule({ mockAuthNull: true });
		expect(() =>
			createListController(
				async () => [],
				{}
			)
		).toThrow(/getToken is required/);
	});

	it("gates fetch by auth readiness and token presence", async () => {
		const { createListController } = await loadListControllerModule();
		const fetcher = vi.fn(async () => [{ id: "1", name: "A" }]);
		const onSuccess = vi.fn();
		const onItemsChange = vi.fn();

		let token: string | null = null;
		const controller = createListController(fetcher as any, {
			getToken: () => token,
			onSuccess,
			onItemsChange,
		});

		await controller.tryFetch(true, { id: "u1" });
		expect(fetcher).not.toHaveBeenCalled();

		await controller.tryFetch(false, null);
		expect(fetcher).not.toHaveBeenCalled();

		await controller.tryFetch(false, { id: "u1" });
		expect(fetcher).not.toHaveBeenCalled();
		expect(controller.loading).toBe(false);

		token = "token-1";
		await controller.tryFetch(false, { id: "u1" });
		expect(fetcher).toHaveBeenCalledTimes(1);
		expect(controller.fetched).toBe(true);
		expect(controller.items).toEqual([{ id: "1", name: "A" }]);
		expect(onSuccess).toHaveBeenCalledTimes(1);
		expect(onItemsChange).toHaveBeenCalledTimes(1);
		expect(controller.loading).toBe(false);
		expect(controller.refetching).toBe(false);
	});

	it("supports refresh, filter updates, resetFilters, and local item updates", async () => {
		const { createListController } = await loadListControllerModule();
		const fetcher = vi.fn(async (_fetch: typeof fetch, _token: string, filters: Record<string, unknown>) => [
			{ id: "a", filters: { ...filters } },
		]);
		const onItemsChange = vi.fn();
		const onSuccess = vi.fn();

		const controller = createListController(fetcher as any, {
			getToken: () => "token-1",
			initialFilters: { query: "alpha", archived: false } as any,
			onItemsChange,
			onSuccess,
		});

		await controller.tryFetch(false, { id: "u1" });
		expect(controller.filters).toEqual({ query: "alpha", archived: false });
		expect(fetcher).toHaveBeenCalledTimes(1);

		await controller.refresh();
		expect(fetcher).toHaveBeenCalledTimes(2);

		controller.setFilters({ query: "beta" } as any);
		await flush();
		expect(fetcher).toHaveBeenCalledTimes(3);
		expect(controller.filters).toEqual({ query: "beta", archived: false });

		controller.setFilter("archived" as any, true as any);
		await flush();
		expect(fetcher).toHaveBeenCalledTimes(4);
		expect(controller.filters).toEqual({ query: "beta", archived: true });

		controller.updateItems((items) => [...items, { id: "local", filters: {} } as any]);
		expect(controller.items.some((item: any) => item.id === "local")).toBe(true);

		controller.removeItem("local");
		expect(controller.items.some((item: any) => item.id === "local")).toBe(false);

		controller.updateItems(() => [{ slug: "x-1" } as any, { slug: "x-2" } as any]);
		controller.removeItem("x-1", "slug");
		expect(controller.items).toEqual([{ slug: "x-2" }]);

		await controller.resetFilters();
		expect(controller.filters).toEqual({ query: "alpha", archived: false });
		expect(fetcher).toHaveBeenCalledTimes(5);
		expect(onSuccess).toHaveBeenCalledTimes(5);
		expect(onItemsChange).toHaveBeenCalled();
	});

	it("supports manual filter mode when autoFetchOnFilterChange is disabled", async () => {
		const { createListController } = await loadListControllerModule();
		const fetcher = vi.fn(async () => [{ id: "1" }]);

		const controller = createListController(fetcher as any, {
			getToken: () => "token-1",
			initialFilters: { term: "a" } as any,
			autoFetchOnFilterChange: false,
		});

		await controller.tryFetch(false, { id: "u1" });
		expect(fetcher).toHaveBeenCalledTimes(1);

		controller.setFilters({ term: "b" } as any);
		await flush();
		expect(fetcher).toHaveBeenCalledTimes(1);

		await controller.refresh();
		expect(fetcher).toHaveBeenCalledTimes(2);

		await controller.resetFilters();
		expect(controller.filters).toEqual({ term: "a" });

		const fallbackController = createListController(fetcher as any, {
			getToken: () => "token-1",
			autoFetchOnFilterChange: false,
		});
		await fallbackController.tryFetch(false, { id: "u1" });
		fallbackController.setFilters({ term: "x" } as any);
		await fallbackController.resetFilters();
		expect(fallbackController.filters).toEqual({});
	});

	it("handles 401 refresh success and failure paths", async () => {
		const { createListController } = await loadListControllerModule();

		const fetcher = vi
			.fn()
			.mockRejectedValueOnce({ status: 401 })
			.mockResolvedValueOnce([{ id: "recovered" }]);
		const onRefresh = vi.fn(async () => "fresh-token");
		const onError = vi.fn();
		const onSuccess = vi.fn();

		const controller = createListController(fetcher as any, {
			getToken: () => "expired",
			onRefresh,
			onError,
			onSuccess,
		});

		await controller.tryFetch(false, { id: "u1" });
		expect(onRefresh).toHaveBeenCalledTimes(1);
		expect(onSuccess).toHaveBeenCalledTimes(1);
		expect(onError).not.toHaveBeenCalled();
		expect(controller.error).toBeNull();
		expect(controller.items).toEqual([{ id: "recovered" }]);
	});

	it("handles refresh retry errors, session expiry, and non-401 errors", async () => {
		const { createListController } = await loadListControllerModule();

		const fetcherRetryError = vi
			.fn()
			.mockRejectedValueOnce({ status: 401 })
			.mockRejectedValueOnce("retry failed");
		const onErrorRetry = vi.fn();
		const controllerRetry = createListController(fetcherRetryError as any, {
			getToken: () => "expired",
			onRefresh: async () => "fresh-token",
			onError: onErrorRetry,
		});
		await controllerRetry.tryFetch(false, { id: "u1" });
		expect(controllerRetry.error).toBe("Failed to load data");
		expect(onErrorRetry).toHaveBeenCalledTimes(1);

		const fetcherSessionExpired = vi.fn().mockRejectedValue({ status: 401 });
		const onErrorSession = vi.fn();
		const controllerSession = createListController(fetcherSessionExpired as any, {
			getToken: () => "expired",
			onRefresh: async () => null,
			onError: onErrorSession,
		});
		await controllerSession.tryFetch(false, { id: "u1" });
		expect(controllerSession.error).toBe("Session expired");
		expect(onErrorSession).toHaveBeenCalledTimes(1);

		const fetcherGenericError = vi.fn().mockRejectedValue("boom");
		const onErrorGeneric = vi.fn();
		const controllerGeneric = createListController(fetcherGenericError as any, {
			getToken: () => "token",
			onError: onErrorGeneric,
		});
		await controllerGeneric.tryFetch(false, { id: "u1" });
		expect(controllerGeneric.error).toBe("Failed to load data");
		expect(onErrorGeneric).toHaveBeenCalledTimes(1);

		const fetcherRetryErrorInstance = vi
			.fn()
			.mockRejectedValueOnce({ status: 401 })
			.mockRejectedValueOnce(new Error("retry boom"));
		const controllerRetryErrorInstance = createListController(fetcherRetryErrorInstance as any, {
			getToken: () => "expired",
			onRefresh: async () => "fresh-token",
			onError: vi.fn(),
		});
		await controllerRetryErrorInstance.tryFetch(false, { id: "u1" });
		expect(controllerRetryErrorInstance.error).toBe("retry boom");

		const fetcherGenericErrorInstance = vi.fn().mockRejectedValue(new Error("direct boom"));
		const controllerGenericErrorInstance = createListController(fetcherGenericErrorInstance as any, {
			getToken: () => "token",
			onError: vi.fn(),
		});
		await controllerGenericErrorInstance.tryFetch(false, { id: "u1" });
		expect(controllerGenericErrorInstance.error).toBe("direct boom");
	});
});
