// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

const { ctx } = vi.hoisted(() => ({
	ctx: {
		props: {
			label: "Select relation",
			emptyMessage: "No results",
			searchPlaceholder: "Search relations",
			suggestionsLabel: "Suggestions",
			required: false,
			renderItem: undefined as unknown,
			allowCreate: true,
			createLabel: "Add relation",
			createForm: {} as unknown,
		},
		state: {
			modalOpen: true,
			searchQuery: "",
			searchResults: [] as Array<{ id: string; label: string }>,
			searchTotal: 0,
			suggestionItems: [] as Array<{ id: string; label: string }>,
			isSearching: false,
			isSuggestionsLoading: false,
			createFormOpen: false,
			searchError: null as string | null,
		},
		isMultiSelect: true,
		selectedItem: null as { id: string; label: string } | null,
		selectedItems: [] as Array<{ id: string; label: string }>,
		setSearchQuery: vi.fn(),
		performSearch: vi.fn(async () => undefined),
		clearSearch: vi.fn(),
		selectItem: vi.fn(),
		closeModal: vi.fn(),
		clearSelection: vi.fn(),
		toggleCreateForm: vi.fn(),
		handleCreateSuccess: vi.fn(),
		closeCreateForm: vi.fn(),
		retrySearch: vi.fn(async () => undefined),
	},
}));

vi.mock("../../src/patterns/RelationSelector/context.svelte.js", () => ({
	useRelationSelector: () => ctx,
}));

import RelationSelectorModal from "../../src/patterns/RelationSelector/RelationSelectorModal.svelte";

describe("patterns/RelationSelector/RelationSelectorModal.svelte", () => {
	beforeEach(() => {
		ctx.props.emptyMessage = "No results";
		ctx.props.searchPlaceholder = "Search relations";
		ctx.props.suggestionsLabel = "Suggestions";
		ctx.props.required = false;
		ctx.props.allowCreate = true;
		ctx.props.createLabel = "Add relation";
		ctx.props.createForm = {};
		ctx.state.modalOpen = true;
		ctx.state.searchQuery = "";
		ctx.state.searchResults = [];
		ctx.state.searchTotal = 0;
		ctx.state.suggestionItems = [];
		ctx.state.isSearching = false;
		ctx.state.isSuggestionsLoading = false;
		ctx.state.createFormOpen = false;
		ctx.state.searchError = null;
		ctx.isMultiSelect = true;
		ctx.selectedItem = { id: "a", label: "Alpha" };
		ctx.selectedItems = [{ id: "a", label: "Alpha" }];
		ctx.setSearchQuery.mockClear();
		ctx.performSearch.mockClear();
		ctx.clearSearch.mockClear();
		ctx.selectItem.mockClear();
		ctx.closeModal.mockClear();
		ctx.clearSelection.mockClear();
		ctx.toggleCreateForm.mockClear();
		ctx.handleCreateSuccess.mockClear();
		ctx.closeCreateForm.mockClear();
		ctx.retrySearch.mockClear();
	});

	it("renders suggestion mode and wires select/clear/create/retry/confirm/cancel handlers", async () => {
		ctx.state.suggestionItems = [{ id: "a", label: "Alpha" }];
		ctx.state.searchError = "Failed";

		render(RelationSelectorModal);

		expect(screen.getByText("Select relation")).toBeTruthy();
		expect(screen.getByText("Suggestions")).toBeTruthy();
		expect(screen.getByText("Failed")).toBeTruthy();

		await fireEvent.click(screen.getByText("Alpha"));
		await fireEvent.click(screen.getByRole("button", { name: "Clear" }));
		await fireEvent.click(screen.getByRole("button", { name: "Retry" }));
		await fireEvent.click(screen.getByRole("button", { name: "Add relation" }));
		await fireEvent.click(screen.getByRole("button", { name: "Confirm (1)" }));
		await fireEvent.click(screen.getByRole("button", { name: "Cancel" }));

		expect(ctx.selectItem).toHaveBeenCalledWith({ id: "a", label: "Alpha" });
		expect(ctx.clearSelection).toHaveBeenCalledTimes(1);
		expect(ctx.retrySearch).toHaveBeenCalledTimes(1);
		expect(ctx.toggleCreateForm).toHaveBeenCalledTimes(1);
		expect(ctx.closeModal).toHaveBeenCalledTimes(2);
	});

	it("shows result sections and debounces search handler", async () => {
		vi.useFakeTimers();
		ctx.state.searchQuery = "be";
		ctx.state.searchResults = [{ id: "b", label: "Beta" }];
		ctx.state.searchTotal = 1;

		try {
			render(RelationSelectorModal);
			expect(screen.getByText("Results (1)")).toBeTruthy();
			expect(screen.getByText("Beta")).toBeTruthy();

			const input = screen.getByRole("textbox") as HTMLInputElement;
			await fireEvent.input(input, { target: { value: "bet" } });
			expect(ctx.setSearchQuery).toHaveBeenCalledWith("bet");
			vi.advanceTimersByTime(300);
			await waitFor(() => {
				expect(ctx.performSearch).toHaveBeenCalledWith("bet");
			});

			await fireEvent.input(input, { target: { value: "" } });
			expect(ctx.clearSearch).toHaveBeenCalledTimes(1);
		} finally {
			vi.useRealTimers();
		}
	});
});
