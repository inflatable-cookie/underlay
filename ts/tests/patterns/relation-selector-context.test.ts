import { describe, expect, it, vi } from "vitest";

type Item = { id: string; label: string };

async function loadContextModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);
	(globalThis as any).$effect = (fn: () => void) => fn();

	const contextStore = new Map<symbol, unknown>();
	vi.doMock("svelte", () => ({
		setContext: (key: symbol, value: unknown) => {
			contextStore.set(key, value);
		},
		getContext: (key: symbol) => contextStore.get(key),
	}));

	const mod = await import("../../src/patterns/RelationSelector/context.svelte");
	return { mod, contextStore };
}

async function loadContextModuleWithDrillDownMock(drillDownActions: Record<string, unknown>) {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);
	(globalThis as any).$effect = (fn: () => void) => fn();

	vi.doMock("../../src/patterns/RelationSelector/drilldown-context.svelte.js", () => ({
		createDrillDownContext: () => ({
			state: { depth: 0, activeFilters: {}, searchQuery: "", suggestionItems: [] },
			actions: drillDownActions,
		}),
	}));
	const contextStore = new Map<symbol, unknown>();
	vi.doMock("svelte", () => ({
		setContext: (key: symbol, value: unknown) => {
			contextStore.set(key, value);
		},
		getContext: (key: symbol) => contextStore.get(key),
	}));

	const mod = await import("../../src/patterns/RelationSelector/context.svelte");
	return { mod };
}

describe("patterns/RelationSelector/context.svelte.ts", () => {
	it("runs primary actions and search/suggestions flows in single-select mode", async () => {
		const { mod } = await loadContextModule();
		const search = vi.fn(async (query: string) => ({
			items: [{ id: `id-${query}`, label: `L-${query}` }],
			total: 1,
		}));
		const suggestions = vi
			.fn()
			.mockResolvedValueOnce([{ id: "s1", label: "One" }])
			.mockResolvedValueOnce([{ id: "s2", label: "Two" }])
			.mockRejectedValueOnce(new Error("no suggestions"))
			.mockResolvedValue([]);
		const onchange = vi.fn();
		const onCreate = vi.fn();
		const history = {
			track: vi.fn(),
			getRecentIds: vi.fn(() => ["recent-1"]),
		};

		const errorSpy = vi.spyOn(console, "error").mockImplementation(() => undefined);
		const ctx = mod.createRelationSelectorContext<Item>({
			mode: "single",
			label: "Relations",
			value: null,
			initialSelection: { id: "init", label: "Initial" },
			search: search as any,
			suggestions: suggestions as any,
			onchange,
			onCreate,
			selectionHistory: history as any,
			filters: [{ key: "kind", label: "Kind", options: [], defaultValue: "all" }],
		});

		ctx.setSearchQuery("abc");
		expect(ctx.state.searchQuery).toBe("abc");
		ctx.clearSearch();
		expect(ctx.state.searchQuery).toBe("");
		expect(ctx.state.searchResults).toEqual([]);

		ctx.openPopover();
		await Promise.resolve();
		expect(ctx.state.popoverOpen).toBe(true);
		expect(suggestions).toHaveBeenCalled();

		ctx.closePopover();
		expect(ctx.state.popoverOpen).toBe(false);
		expect(ctx.state.searchQuery).toBe("");

		ctx.openModal();
		await Promise.resolve();
		expect(ctx.state.modalOpen).toBe(true);
		ctx.toggleCreateForm();
		expect(ctx.state.createFormOpen).toBe(true);
		ctx.closeCreateForm();
		expect(ctx.state.createFormOpen).toBe(false);
		ctx.closeModal();
		expect(ctx.state.modalOpen).toBe(false);

		ctx.openPopover();
		ctx.toggleCreateForm();
		expect(ctx.state.popoverOpen).toBe(false);
		expect(ctx.state.modalOpen).toBe(true);
		expect(ctx.state.createFormOpen).toBe(true);

		await ctx.performSearch("term");
		expect(search).toHaveBeenCalledWith("term", {
			limit: 20,
			offset: 0,
			filters: { kind: "all" },
		});
		expect(ctx.state.searchResults).toHaveLength(1);
		expect(ctx.state.searchTotal).toBe(1);

		search.mockRejectedValueOnce(new Error("search failed"));
		await ctx.performSearch("bad");
		expect(ctx.state.searchError).toBe("search failed");
		expect(ctx.state.searchResults).toEqual([]);
		expect(ctx.state.searchTotal).toBe(0);

		await ctx.retrySearch();
		expect(search).toHaveBeenCalledWith("bad", expect.any(Object));

		ctx.selectItem({ id: "picked", label: "Picked" });
		expect(onchange).toHaveBeenCalledWith("picked");
		expect(history.track).toHaveBeenCalledWith("picked");

		ctx.clearSelection();
		expect(onchange).toHaveBeenCalledWith(null);
		expect(ctx.isSelected("picked")).toBe(false);

		ctx.handleCreateSuccess({ id: "new", label: "New" });
		expect(onchange).toHaveBeenCalledWith("new");
		expect(onCreate).toHaveBeenCalledWith({ id: "new", label: "New" });

		await ctx.loadSuggestions();
		expect(history.getRecentIds).toHaveBeenCalled();
		expect(suggestions).toHaveBeenCalledWith({
			recentHints: ["recent-1"],
			filters: { kind: "all" },
		});

		await ctx.retrySuggestions();
		expect(ctx.state.suggestionItems).toEqual([]);
		errorSpy.mockRestore();
	});

	it("supports multi-select actions, filtering, and useRelationSelector retrieval/errors", async () => {
		const { mod, contextStore } = await loadContextModule();
		const search = vi.fn(async () => ({ items: [{ id: "a", label: "A" }], total: 1 }));
		const suggestions = vi.fn(async () => [{ id: "b", label: "B" }]);
		const onchangeMulti = vi.fn();

		const ctx = mod.createRelationSelectorContext<Item>({
			mode: "multi",
			label: "Relations",
			values: ["a"],
			initialSelections: [{ id: "a", label: "A" }],
			search: search as any,
			suggestions: suggestions as any,
			onchangeMulti,
		});

		ctx.selectItem({ id: "a", label: "A" });
		expect(onchangeMulti).toHaveBeenCalledWith([]);

		ctx.selectItem({ id: "b", label: "B" });
		expect(onchangeMulti).toHaveBeenCalledWith(["a", "b"]);

		ctx.deselectItem("a");
		expect(onchangeMulti).toHaveBeenCalledWith([]);

		ctx.clearSelection();
		expect(onchangeMulti).toHaveBeenCalledWith([]);
		expect(ctx.isSelected("a")).toBe(true);

		ctx.setSearchQuery("filtered");
		ctx.setFilter("kind", "x");
		await Promise.resolve();
		expect(search).toHaveBeenCalledWith("filtered", {
			limit: 20,
			offset: 0,
			filters: { kind: "x" },
		});

		ctx.clearSearch();
		ctx.setFilter("kind", undefined);
		await Promise.resolve();
		expect(suggestions).toHaveBeenCalled();

		const fromContext = mod.useRelationSelector<Item>();
		expect(fromContext).toBe(ctx);

		contextStore.clear();
		expect(() => mod.useRelationSelector<Item>()).toThrow(
			/useRelationSelector must be called within a RelationSelector component/
		);
	});

	it("integrates drill-down actions and merged filter behavior", async () => {
		const drillDownActions = {
			drillDownSelect: vi.fn(),
			drillDownBack: vi.fn(),
			drillDownNavigateTo: vi.fn(),
			setDrillDownSearch: vi.fn(),
			performDrillDownSearch: vi.fn(),
			loadDrillDownSuggestions: vi.fn(async () => undefined),
			setDrillDownFilter: vi.fn(),
			isDrillDownActive: true,
			currentDrillDownLevel: { key: "module" },
			drillDownBreadcrumbs: [],
			getDrillDownFilters: vi.fn(() => ({ module: "m1" })),
			finalLevelFilters: null,
			resetDrillDown: vi.fn(),
		};
		const { mod } = await loadContextModuleWithDrillDownMock(drillDownActions);
		const search = vi.fn(async () => ({ items: [{ id: "a", label: "A" }], total: 1 }));
		const suggestions = vi.fn(async () => [{ id: "a", label: "A" }]);

		const ctx = mod.createRelationSelectorContext<Item>({
			mode: "single",
			label: "Drilldown",
			value: null,
			search: search as any,
			suggestions: suggestions as any,
			drillDown: { levels: [{ key: "module", label: "Module", search: vi.fn() }] } as any,
			filters: [{ key: "status", label: "Status", options: [], defaultValue: "active" }],
		});

		ctx.openPopover();
		expect(drillDownActions.loadDrillDownSuggestions).toHaveBeenCalled();
		expect(suggestions).not.toHaveBeenCalled();

		ctx.closePopover();
		expect(drillDownActions.resetDrillDown).toHaveBeenCalled();

		await ctx.performSearch("term");
		expect(search).toHaveBeenCalledWith("term", {
			limit: 20,
			offset: 0,
			filters: { module: "m1", status: "active" },
		});

		await ctx.loadSuggestions();
		expect(suggestions).toHaveBeenCalledWith({
			recentHints: undefined,
			filters: { module: "m1", status: "active" },
		});

		expect(ctx.drillDown).toBe(drillDownActions);
	});
});
