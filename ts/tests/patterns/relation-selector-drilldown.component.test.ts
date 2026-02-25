// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

const { ctx } = vi.hoisted(() => ({
	ctx: {
		state: {
			drillDown: {
				depth: 1,
				searchQuery: "",
				searchResults: [] as Array<{ id: string; label: string; description?: string; disabled?: boolean; count?: number }>,
				searchTotal: 0,
				searching: false,
				suggestionItems: [] as Array<{ id: string; label: string; description?: string; disabled?: boolean; count?: number }>,
				suggestionsLoading: false,
				searchError: null as string | null,
				activeFilters: {} as Record<string, string | undefined>,
			},
		},
		drillDown: {
			currentDrillDownLevel: {
				key: "module",
				label: "Module",
				searchPlaceholder: "Search modules",
				filters: [
					{
						key: "status",
						label: "Status",
						allLabel: "Any",
						options: [
							{ id: "active", label: "Active" },
							{ id: "archived", label: "Archived" },
						],
					},
				],
			},
			drillDownBreadcrumbs: [] as Array<{ key: string; itemLabel: string; depth: number }>,
			setDrillDownFilter: vi.fn(),
			setDrillDownSearch: vi.fn(),
			performDrillDownSearch: vi.fn(async () => undefined),
			drillDownSelect: vi.fn(),
			drillDownBack: vi.fn(),
			drillDownNavigateTo: vi.fn(),
		},
		closePopover: vi.fn(),
	},
}));

vi.mock("../../src/patterns/RelationSelector/context.svelte.js", () => ({
	useRelationSelector: () => ctx,
}));

import RelationSelectorDrillDown from "../../src/patterns/RelationSelector/RelationSelectorDrillDown.svelte";

describe("patterns/RelationSelector/RelationSelectorDrillDown.svelte", () => {
	beforeEach(() => {
		ctx.state.drillDown = {
			depth: 1,
			searchQuery: "",
			searchResults: [],
			searchTotal: 0,
			searching: false,
			suggestionItems: [
				{ id: "a", label: "Alpha", description: "First", count: 3 },
				{ id: "b", label: "Beta", disabled: true },
			],
			suggestionsLoading: false,
			searchError: null,
			activeFilters: { status: undefined },
		};
		ctx.drillDown.currentDrillDownLevel = {
			key: "module",
			label: "Module",
			searchPlaceholder: "Search modules",
			filters: [
				{
					key: "status",
					label: "Status",
					allLabel: "Any",
					options: [
						{ id: "active", label: "Active" },
						{ id: "archived", label: "Archived" },
					],
				},
			],
		};
		ctx.drillDown.drillDownBreadcrumbs = [
			{ key: "k1", itemLabel: "Products", depth: 0 },
			{ key: "k2", itemLabel: "SaaS", depth: 1 },
		];
		ctx.drillDown.setDrillDownFilter.mockClear();
		ctx.drillDown.setDrillDownSearch.mockClear();
		ctx.drillDown.performDrillDownSearch.mockClear();
		ctx.drillDown.drillDownSelect.mockClear();
		ctx.drillDown.drillDownBack.mockClear();
		ctx.drillDown.drillDownNavigateTo.mockClear();
		ctx.closePopover.mockClear();
	});

	it("renders breadcrumbs/filters/list and wires back, breadcrumb, filter and item interactions", async () => {
		render(RelationSelectorDrillDown);

		expect(screen.getByText("Module")).toBeTruthy();
		expect(screen.getByRole("button", { name: "Go back" })).toBeTruthy();
		expect(screen.getByRole("button", { name: "Products" })).toBeTruthy();
		expect(screen.getByRole("button", { name: "SaaS" })).toBeTruthy();
		expect(screen.getByText("Alpha")).toBeTruthy();
		expect(screen.getByText("3")).toBeTruthy();

		await fireEvent.click(screen.getByRole("button", { name: "Go back" }));
		expect(ctx.drillDown.drillDownBack).toHaveBeenCalledTimes(1);

		await fireEvent.click(screen.getByRole("button", { name: "Products" }));
		expect(ctx.drillDown.drillDownNavigateTo).toHaveBeenCalledWith(0);

		await fireEvent.click(screen.getByRole("button", { name: /Any/i }));
		const menu = document.querySelector(".relation-selector-popover__filter-menu") as HTMLElement;
		await fireEvent.click(within(menu).getByRole("button", { name: "Archived" }));
		expect(ctx.drillDown.setDrillDownFilter).toHaveBeenCalledWith("status", "archived");

		await fireEvent.click(screen.getByText("Alpha"));
		expect(ctx.drillDown.drillDownSelect).toHaveBeenCalledWith({
			id: "a",
			label: "Alpha",
			description: "First",
			count: 3,
		});

		await fireEvent.click(screen.getByText("Beta"));
		expect(ctx.drillDown.drillDownSelect).toHaveBeenCalledTimes(1);
	});

	it("handles debounced search, escape/backspace behavior, and loading/error/empty states", async () => {
		vi.useFakeTimers();
		try {
			ctx.state.drillDown = {
				...ctx.state.drillDown!,
				searchQuery: "zz",
				searchResults: [],
				searchTotal: 0,
				searching: false,
				suggestionItems: [],
				suggestionsLoading: false,
				searchError: "Lookup failed",
			};
			render(RelationSelectorDrillDown);

			expect(screen.getByText("Lookup failed")).toBeTruthy();
			expect(screen.getByText("No results found")).toBeTruthy();

			await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
			expect(ctx.drillDown.performDrillDownSearch).toHaveBeenCalledWith("zz");

			const input = screen.getByRole("textbox") as HTMLInputElement;
			await fireEvent.input(input, { target: { value: "alp" } });
			expect(ctx.drillDown.setDrillDownSearch).toHaveBeenCalledWith("alp");
			vi.advanceTimersByTime(300);
			await waitFor(() => {
				expect(ctx.drillDown.performDrillDownSearch).toHaveBeenCalledWith("alp");
			});

			await fireEvent.keyDown(input, { key: "Escape" });
			expect(ctx.drillDown.drillDownBack).toHaveBeenCalled();

			ctx.state.drillDown!.searchQuery = "";
			await fireEvent.keyDown(input, { key: "Backspace" });
			expect(ctx.drillDown.drillDownBack).toHaveBeenCalledTimes(2);
		} finally {
			vi.useRealTimers();
		}
	});

	it("closes popover on escape when there are no drilldown breadcrumbs", async () => {
		ctx.drillDown.drillDownBreadcrumbs = [];
		render(RelationSelectorDrillDown);
		const input = screen.getByRole("textbox") as HTMLInputElement;
		await fireEvent.keyDown(input, { key: "Escape" });
		expect(ctx.closePopover).toHaveBeenCalledTimes(1);
	});
});
