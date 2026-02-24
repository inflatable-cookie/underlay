import { describe, expect, it, vi } from "vitest";

async function loadAutonomousListModule(deps: {
	listController?: any;
	serverPagination?: any;
	clientPagination?: any;
	batch?: any;
	reorderController?: any;
}) {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);

	const createListController = vi.fn(() => deps.listController);
	const createPaginationController = vi.fn(() => deps.serverPagination);
	const createClientPagination = vi.fn(() => deps.clientPagination);
	const useBatchActions = vi.fn(() => deps.batch);
	const createReorderController = vi.fn(() => deps.reorderController);

	vi.doMock("../../src/patterns/list-controller.svelte", () => ({
		createListController,
	}));
	vi.doMock("../../src/patterns/pagination.svelte", () => ({
		createPaginationController,
		createClientPagination,
	}));
	vi.doMock("../../src/patterns/batch-actions.svelte", () => ({
		useBatchActions,
	}));
	vi.doMock("../../src/patterns/reorder-controller.svelte", () => ({
		createReorderController,
	}));

	const mod = await import("../../src/patterns/AutonomousList/autonomous-list-context.svelte");
	return {
		mod,
		spies: {
			createListController,
			createPaginationController,
			createClientPagination,
			useBatchActions,
			createReorderController,
		},
	};
}

describe("patterns/AutonomousList/autonomous-list-context.svelte.ts", () => {
	it("handles client mode flows including selection, reorder, refresh, and reset", async () => {
		const listController = {
			items: [
				{ id: "a", title: "Alpha" },
				{ id: "b", name: "Bravo" },
			],
			loading: false,
			error: null,
			filters: { kind: "x" },
			tryFetch: vi.fn(async () => undefined),
			refresh: vi.fn(async () => undefined),
		};
		const clientPagination = {
			items: [{ id: "paged" }],
			loading: false,
			error: null,
			total: null,
			reset: vi.fn(async () => undefined),
		};
		const batch = {
			clear: vi.fn(),
			registerAction: vi.fn(),
		};
		const reorderController = { pending: [] };
		const onDataChange = vi.fn();
		const reorderExecute = vi.fn(async () => undefined);

		const { mod, spies } = await loadAutonomousListModule({
			listController,
			clientPagination,
			batch,
			reorderController,
		});

		const state = mod.createAutonomousListState({
			fetcher: vi.fn(),
			pageSize: 20,
			persistKey: "list:one",
			batchActions: [{ id: "archive", label: "Archive", execute: vi.fn() }] as any,
			reorderable: {
				execute: reorderExecute,
				condition: (filters: Record<string, unknown>) => filters.kind === "x",
			},
			onDataChange,
		});

		expect(spies.createListController).toHaveBeenCalledTimes(1);
		expect(spies.createClientPagination).toHaveBeenCalledTimes(1);
		expect(state.items).toEqual([{ id: "paged" }]);
		expect(state.loading).toBe(false);
		expect(state.error).toBeNull();
		expect(state.total).toBe(2);
		expect(state.pagination).toBe(clientPagination);
		expect(state.canReorder).toBe(true);
		expect(batch.registerAction).toHaveBeenCalledTimes(1);

		await state.tryFetch(false, { id: "u1" });
		expect(listController.tryFetch).toHaveBeenCalledWith(false, { id: "u1" });

		await state.refresh();
		expect(listController.refresh).toHaveBeenCalledTimes(1);
		expect(onDataChange).toHaveBeenCalledTimes(1);

		state.toggleSelectionMode();
		expect(state.selectionMode).toBe(true);
		state.handleKeydown({ key: "Escape", preventDefault: vi.fn() } as any);
		expect(state.selectionMode).toBe(false);
		expect(batch.clear).toHaveBeenCalled();

		state.enterReorderMode();
		expect(state.reorderMode).toBe(true);
		expect(state.reorder).toBe(reorderController);
		expect(spies.createReorderController).toHaveBeenCalledWith(
			[
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Bravo" },
			],
			expect.any(Function)
		);
		const reorderSubmit = spies.createReorderController.mock.calls[0]?.[1];
		await reorderSubmit(["b", "a"]);
		expect(reorderExecute).toHaveBeenCalledWith(["b", "a"], fetch, "");

		await state.handleReorderSuccess();
		expect(state.reorderMode).toBe(false);
		expect(state.reorder).toBeNull();
		expect(listController.refresh).toHaveBeenCalledTimes(2);
		expect(onDataChange).toHaveBeenCalledTimes(2);

		await state.resetPagination();
		expect(clientPagination.reset).toHaveBeenCalledTimes(1);

		state.exitReorderMode();
		expect(state.reorderMode).toBe(false);
	});

	it("handles server mode flows and non-configured fallbacks", async () => {
		const serverPagination = {
			items: [{ custom_id: "s1", label: "Server 1" }],
			loading: true,
			error: "server error",
			total: 10,
			tryFetch: vi.fn(async () => undefined),
			refresh: vi.fn(async () => undefined),
			reset: vi.fn(async () => undefined),
		};
		const batch = { clear: vi.fn(), registerAction: vi.fn() };
		const reorderController = { pending: [] };

		const { mod, spies } = await loadAutonomousListModule({
			serverPagination,
			batch,
			reorderController,
		});

		const state = mod.createAutonomousListState({
			serverFetcher: vi.fn(async () => ({
				data: [],
				nextCursor: null,
				prevCursor: null,
				hasMore: false,
				total: 0,
			})),
			idField: "custom_id",
			reorderable: { execute: vi.fn(async () => undefined) },
		});

		expect(spies.createPaginationController).toHaveBeenCalledTimes(1);
		expect(state.items).toEqual([{ custom_id: "s1", label: "Server 1" }]);
		expect(state.loading).toBe(true);
		expect(state.error).toBe("server error");
		expect(state.total).toBe(10);
		expect(state.pagination).toBe(serverPagination);
		expect(state.allItemIds).toEqual(["s1"]);
		expect(state.canReorder).toBe(true);
		expect(typeof (state.pagination as any)?._setFilters).toBe("function");

		await state.tryFetch(false, { id: "u2" });
		expect(serverPagination.tryFetch).toHaveBeenCalledWith(false, { id: "u2" });

		await state.refresh();
		expect(serverPagination.refresh).toHaveBeenCalledTimes(1);
		await state.resetPagination();
		expect(serverPagination.reset).toHaveBeenCalledTimes(1);

		const esc = { key: "Escape", preventDefault: vi.fn() } as any;
		state.enterReorderMode();
		state.handleKeydown(esc);
		expect(esc.preventDefault).toHaveBeenCalledTimes(1);
		expect(state.reorderMode).toBe(false);

		const minimal = mod.createAutonomousListState({});
		expect(minimal.items).toEqual([]);
		expect(minimal.loading).toBe(false);
		expect(minimal.error).toBeNull();
		expect(minimal.total).toBeNull();
		expect(minimal.pagination).toBeNull();
		expect(minimal.canReorder).toBe(false);
		expect(minimal.allItemIds).toEqual([]);
		await minimal.tryFetch(false, {});
		await minimal.refresh();
		await minimal.resetPagination();
	});
});
