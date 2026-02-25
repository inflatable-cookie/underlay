// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

const { ctx } = vi.hoisted(() => ({
	ctx: {
		props: {
			label: "Select relation",
			required: false,
			searchPlaceholder: "Search relations",
			suggestionsLabel: "Suggestions",
			emptyMessage: "No results",
			allowCreate: true,
			createLabel: "Add relation",
			createForm: {} as unknown,
			filters: undefined as
				| Array<{ key: string; label: string; options: Array<{ id: string; label: string }>; allLabel?: string }>
				| undefined,
			drillDown: undefined as unknown,
			renderItem: undefined as unknown,
		},
		state: {
			popoverOpen: true,
			searchQuery: "",
			searchResults: [] as Array<{ id: string; label: string; disabled?: boolean }>,
			searchTotal: 0,
			suggestionItems: [] as Array<{ id: string; label: string; disabled?: boolean }>,
			isSearching: false,
			isSuggestionsLoading: false,
			searchError: null as string | null,
			activeFilters: {} as Record<string, string | undefined>,
		},
		isMultiSelect: false,
		selectedItem: null as { id: string; label: string } | null,
		selectedItems: [] as Array<{ id: string; label: string }>,
		drillDown: {
			isDrillDownActive: false,
			drillDownBreadcrumbs: [] as Array<{ key: string; itemLabel: string; depth: number }>,
			finalLevelFilters: null as unknown,
			drillDownBack: vi.fn(),
			drillDownNavigateTo: vi.fn(),
		},
		setFilter: vi.fn(),
		setSearchQuery: vi.fn(),
		performSearch: vi.fn(async () => undefined),
		clearSearch: vi.fn(),
		selectItem: vi.fn(),
		retrySearch: vi.fn(async () => undefined),
		closePopover: vi.fn(),
		toggleCreateForm: vi.fn(),
		clearSelection: vi.fn(),
		isSelected: vi.fn((id: string) => id === "a"),
	},
}));

vi.mock("../../src/patterns/RelationSelector/context.svelte.js", () => ({
	useRelationSelector: () => ctx,
}));

import RelationSelectorPopoverHarness from "../fixtures/RelationSelectorPopoverHarness.svelte";

describe("patterns/RelationSelector/RelationSelectorPopover.svelte", () => {
	beforeEach(() => {
		ctx.props.required = false;
		ctx.props.searchPlaceholder = "Search relations";
		ctx.props.suggestionsLabel = "Suggestions";
		ctx.props.emptyMessage = "No results";
		ctx.props.allowCreate = true;
		ctx.props.createLabel = "Add relation";
		ctx.props.createForm = {};
		ctx.props.filters = undefined;
		ctx.props.drillDown = undefined;
		ctx.state.popoverOpen = true;
		ctx.state.searchQuery = "";
		ctx.state.searchResults = [];
		ctx.state.searchTotal = 0;
		ctx.state.suggestionItems = [];
		ctx.state.isSearching = false;
		ctx.state.isSuggestionsLoading = false;
		ctx.state.searchError = null;
		ctx.state.activeFilters = {};
		ctx.isMultiSelect = false;
		ctx.selectedItem = null;
		ctx.selectedItems = [];
		ctx.drillDown.isDrillDownActive = false;
		ctx.drillDown.drillDownBreadcrumbs = [];
		ctx.drillDown.finalLevelFilters = null;
		ctx.setFilter.mockClear();
		ctx.setSearchQuery.mockClear();
		ctx.performSearch.mockClear();
		ctx.clearSearch.mockClear();
		ctx.selectItem.mockClear();
		ctx.retrySearch.mockClear();
		ctx.closePopover.mockClear();
		ctx.toggleCreateForm.mockClear();
		ctx.clearSelection.mockClear();
		ctx.isSelected.mockClear();
		ctx.isSelected.mockImplementation((id: string) => id === "a");
	});

	it("renders suggestions and wires clear/retry/create/item/done interactions", async () => {
		ctx.isMultiSelect = true;
		ctx.selectedItems = [{ id: "x", label: "X" }, { id: "y", label: "Y" }];
		ctx.selectedItem = { id: "x", label: "X" };
		ctx.state.suggestionItems = [
			{ id: "a", label: "Alpha" },
			{ id: "b", label: "Beta" },
		];
		ctx.state.searchError = "Failed";

		render(RelationSelectorPopoverHarness);

		expect(screen.getByText("Select relation")).toBeTruthy();
		expect(screen.getByText("Suggestions")).toBeTruthy();
		expect(screen.getByText("Failed")).toBeTruthy();

		await fireEvent.click(screen.getByRole("button", { name: "Clear" }));
		await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
		await fireEvent.click(screen.getByRole("button", { name: "Add relation" }));
		await fireEvent.click(screen.getByText("Alpha"));
		await fireEvent.click(screen.getByRole("button", { name: "Done (2)" }));

		expect(ctx.clearSelection).toHaveBeenCalledTimes(1);
		expect(ctx.retrySearch).toHaveBeenCalledTimes(1);
		expect(ctx.toggleCreateForm).toHaveBeenCalledTimes(1);
		expect(ctx.selectItem).toHaveBeenCalledWith({ id: "a", label: "Alpha" });
		expect(ctx.closePopover).toHaveBeenCalledTimes(1);
	});

	it("debounces search, clears empty query, and applies filter selection", async () => {
		vi.useFakeTimers();
		ctx.props.filters = [
			{
				key: "status",
				label: "Status",
				allLabel: "Any",
				options: [
					{ id: "active", label: "Active" },
					{ id: "archived", label: "Archived" },
				],
			},
		];
		ctx.state.activeFilters = { status: undefined };
		ctx.state.searchQuery = "";
		ctx.state.suggestionItems = [{ id: "a", label: "Alpha" }];

		try {
			render(RelationSelectorPopoverHarness);

			await fireEvent.click(screen.getByRole("button", { name: /Any/i }));
			await fireEvent.click(screen.getByRole("button", { name: "Archived" }));
			expect(ctx.setFilter).toHaveBeenCalledWith("status", "archived");

			const input = screen.getByRole("textbox") as HTMLInputElement;
			await fireEvent.input(input, { target: { value: "alp" } });
			expect(ctx.setSearchQuery).toHaveBeenCalledWith("alp");
			vi.advanceTimersByTime(300);
			await waitFor(() => {
				expect(ctx.performSearch).toHaveBeenCalledWith("alp");
			});

			await fireEvent.input(input, { target: { value: "   " } });
			expect(ctx.clearSearch).toHaveBeenCalledTimes(1);
		} finally {
			vi.useRealTimers();
		}
	});

	it("switches to drilldown branch when active and hides normal popover header", () => {
		ctx.props.drillDown = { levels: [] } as unknown;
		ctx.drillDown.isDrillDownActive = true;
		(ctx.state as Record<string, unknown>).drillDown = {
			depth: 1,
			searchQuery: "",
			searchResults: [],
			searchTotal: 0,
			searching: false,
			suggestionItems: [{ id: "a", label: "Alpha" }],
			suggestionsLoading: false,
			searchError: null,
			activeFilters: {},
		};
		(ctx.drillDown as Record<string, unknown>).currentDrillDownLevel = {
			key: "module",
			label: "Module",
			searchPlaceholder: "Search modules",
			filters: [],
		};

		render(RelationSelectorPopoverHarness);

		expect(screen.getByText("Module")).toBeTruthy();
		expect(screen.queryByText("Select relation")).toBeNull();
		expect(screen.queryByRole("button", { name: "Add relation" })).toBeNull();
	});
});
