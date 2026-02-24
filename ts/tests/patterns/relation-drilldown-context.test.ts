import { describe, expect, it, vi } from "vitest";

type DrillDownItem = { id: string; label: string };

function createRunes() {
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = Object.assign(
		<T>(value: T) => value,
		{ by: <T>(fn: () => T) => fn() }
	);
}

describe("patterns/RelationSelector/drilldown-context.svelte.ts", () => {
	it("handles suggestions, search, selection, navigation, filters, and reset", async () => {
		vi.resetModules();
		createRunes();

		const levelOneSuggestions = vi.fn(async () => [{ id: "m1", label: "Module 1" }] as DrillDownItem[]);
		const levelTwoSuggestions = vi.fn(async (ctx: Record<string, string>) => [
			{ id: `s-${ctx.module ?? "unknown"}`, label: "Section" },
		] as DrillDownItem[]);

		const levelOneSearch = vi.fn(async (query: string) => ({
			items: [{ id: `m-${query}`, label: `M ${query}` }],
			total: 1,
		}));
		const levelTwoSearch = vi.fn(async (query: string, ctx: Record<string, string>) => ({
			items: [{ id: `s-${ctx.module}-${query}`, label: `S ${query}` }],
			total: 1,
		}));

		const config = {
			levels: [
				{
					key: "module",
					label: "Module",
					search: levelOneSearch,
					suggestions: levelOneSuggestions,
					filters: [{ key: "status", label: "Status", options: [], defaultValue: "active" }],
				},
				{
					key: "section",
					label: "Section",
					search: levelTwoSearch,
					suggestions: levelTwoSuggestions,
				},
			],
			finalLevelFilters: (ctx: Record<string, string>) => [
				{ key: "module", label: "Module", options: [{ id: ctx.module ?? "none", label: "Scoped" }] },
			],
		};

		const { createDrillDownContext } = await import(
			"../../src/patterns/RelationSelector/drilldown-context.svelte"
		);
		const ctx = createDrillDownContext(() => config as any);

		expect(ctx.actions.isDrillDownActive).toBe(true);
		expect(ctx.actions.currentDrillDownLevel?.key).toBe("module");
		expect(ctx.state.depth).toBe(0);

		await ctx.actions.loadDrillDownSuggestions();
		expect(levelOneSuggestions).toHaveBeenCalledTimes(1);
		expect(ctx.state.suggestionItems).toEqual([{ id: "m1", label: "Module 1" }]);

		await ctx.actions.performDrillDownSearch("");
		expect(ctx.state.searchResults).toEqual([]);
		expect(ctx.state.searchTotal).toBe(0);
		expect(ctx.state.searchError).toBeNull();

		ctx.actions.setDrillDownSearch("abc");
		expect(ctx.state.searchQuery).toBe("abc");
		await ctx.actions.performDrillDownSearch("abc");
		expect(levelOneSearch).toHaveBeenCalledWith("abc", {});
		expect(ctx.state.searchResults[0]?.id).toBe("m-abc");

		ctx.actions.drillDownSelect({ id: "mod-1", label: "Module 1" });
		expect(ctx.state.depth).toBe(1);
		expect(ctx.state.slideDirection).toBe("forward");
		expect(ctx.state.selections.module).toEqual({ id: "mod-1", label: "Module 1" });
		expect(ctx.actions.getDrillDownFilters()).toEqual({ module: "mod-1" });
		expect(ctx.actions.finalLevelFilters).toEqual([
			{ key: "module", label: "Module", options: [{ id: "mod-1", label: "Scoped" }] },
		]);

		await ctx.actions.loadDrillDownSuggestions();
		expect(levelTwoSuggestions).toHaveBeenCalledWith({ module: "mod-1" });

		ctx.actions.setDrillDownSearch("section");
		ctx.actions.setDrillDownFilter("kind", "docs");
		await Promise.resolve();
		expect(levelTwoSearch).toHaveBeenCalledWith("section", { module: "mod-1", kind: "docs" });

		ctx.actions.drillDownNavigateTo(0);
		expect(ctx.state.depth).toBe(0);
		expect(ctx.state.slideDirection).toBe("back");
		expect(ctx.actions.getDrillDownFilters()).toEqual({});

		ctx.actions.drillDownBack();
		expect(ctx.state.depth).toBe(0);

		ctx.actions.drillDownSelect({ id: "mod-2", label: "Module 2" });
		ctx.actions.drillDownSelect({ id: "sec-9", label: "Section 9" });
		expect(ctx.state.depth).toBe(2);

		ctx.actions.resetDrillDown();
		expect(ctx.state.depth).toBe(0);
		expect(ctx.state.selections).toEqual({});
		expect(ctx.state.searchQuery).toBe("");
		expect(ctx.state.activeFilters).toEqual({});
	});

	it("handles back-navigation mutation branches and guard branches", async () => {
		vi.resetModules();
		createRunes();

		const levelOneSuggestions = vi.fn(async () => [{ id: "m1", label: "Module 1" }] as DrillDownItem[]);
		const levelTwoSuggestions = vi.fn(async () => [{ id: "s1", label: "Section 1" }] as DrillDownItem[]);
		const config = {
			levels: [
				{
					key: "module",
					label: "Module",
					search: vi.fn(async () => ({ items: [], total: 0 })),
					suggestions: levelOneSuggestions,
				},
				{
					key: "section",
					label: "Section",
					search: vi.fn(async () => ({ items: [], total: 0 })),
					suggestions: levelTwoSuggestions,
				},
			],
		};

		const { createDrillDownContext } = await import(
			"../../src/patterns/RelationSelector/drilldown-context.svelte"
		);
		const ctx = createDrillDownContext(() => config as any);

		await ctx.actions.loadDrillDownSuggestions();
		expect(levelOneSuggestions).toHaveBeenCalledTimes(1);

		ctx.actions.drillDownSelect({ id: "m1", label: "Module 1" });
		expect(ctx.state.depth).toBe(1);
		expect(levelTwoSuggestions).toHaveBeenCalledTimes(1);

		ctx.actions.drillDownBack();
		expect(ctx.state.depth).toBe(0);
		expect(ctx.state.slideDirection).toBe("back");
		expect(ctx.state.selections).toEqual({});
		expect(levelOneSuggestions).toHaveBeenCalledTimes(2);

		const before = ctx.state.depth;
		ctx.actions.drillDownNavigateTo(10);
		expect(ctx.state.depth).toBe(before);
		ctx.actions.drillDownNavigateTo(-1);
		expect(ctx.state.depth).toBe(before);
	});

	it("handles missing config and error branches safely", async () => {
		vi.resetModules();
		createRunes();

		const { createDrillDownContext } = await import(
			"../../src/patterns/RelationSelector/drilldown-context.svelte"
		);
		const ctx = createDrillDownContext(() => undefined);

		expect(ctx.actions.isDrillDownActive).toBe(false);
		expect(ctx.actions.currentDrillDownLevel).toBeNull();
		expect(ctx.actions.drillDownBreadcrumbs).toEqual([]);
		expect(ctx.actions.finalLevelFilters).toBeNull();
		expect(ctx.actions.getDrillDownFilters()).toEqual({});

		await ctx.actions.loadDrillDownSuggestions();
		await ctx.actions.performDrillDownSearch("query");
		ctx.actions.drillDownSelect({ id: "x", label: "X" });
		ctx.actions.drillDownBack();
		ctx.actions.drillDownNavigateTo(0);
		ctx.actions.setDrillDownSearch("q");
		ctx.actions.setDrillDownFilter("any", "v");
		ctx.actions.resetDrillDown();
		expect(ctx.state.depth).toBe(0);

		const badConfig = {
			levels: [
				{
					key: "k",
					label: "K",
					search: vi.fn(async () => {
						throw "search-failed";
					}),
					suggestions: vi.fn(async () => {
						throw "suggestions-failed";
					}),
				},
			],
		};
		const errCtx = createDrillDownContext(() => badConfig as any);
		const errSpy = vi.spyOn(console, "error").mockImplementation(() => undefined);

		await errCtx.actions.loadDrillDownSuggestions();
		expect(errCtx.state.suggestionItems).toEqual([]);
		await errCtx.actions.performDrillDownSearch("q");
		expect(errCtx.state.searchError).toBe("Search failed");
		expect(errSpy).toHaveBeenCalled();
		errSpy.mockRestore();
	});
});
