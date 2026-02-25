// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import RelationSelectorPopoverFilters from "../../src/patterns/RelationSelector/RelationSelectorPopoverFilters.svelte";

describe("patterns/RelationSelector/RelationSelectorPopoverFilters.svelte", () => {
	it("renders filter trigger labels and toggles dropdown callbacks", async () => {
		const onToggleFilter = vi.fn();
		const view = render(RelationSelectorPopoverFilters, {
			filters: [
				{
					key: "status",
					label: "Status",
					options: [{ id: "active", label: "Active" }],
				},
			],
			activeFilters: {},
			openFilterKey: null,
			getActiveFilterLabel: () => "All",
			onToggleFilter,
			onSelectFilter: vi.fn(),
		});

		expect(screen.getByText("Status:")).toBeTruthy();
		await fireEvent.click(screen.getByRole("button", { name: /All/i }));
		expect(onToggleFilter).toHaveBeenCalledWith("status");
		expect(view.container.querySelector(".relation-selector-popover__filter-menu")).toBeNull();
	});

	it("renders options when open and forwards select callback for all and concrete options", async () => {
		const onSelectFilter = vi.fn();
		const view = render(RelationSelectorPopoverFilters, {
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
			activeFilters: { status: "active" },
			openFilterKey: "status",
			getActiveFilterLabel: () => "Active",
			onToggleFilter: vi.fn(),
			onSelectFilter,
		});

		expect(view.container.querySelector(".relation-selector-popover__filter-menu")).toBeTruthy();
		const menu = view.container.querySelector(".relation-selector-popover__filter-menu") as HTMLElement;
		expect(within(menu).getByRole("button", { name: "Any" })).toBeTruthy();
		expect(within(menu).getByRole("button", { name: "Active" })).toBeTruthy();
		expect(within(menu).getByRole("button", { name: "Archived" })).toBeTruthy();
		expect(view.container.querySelector(".relation-selector-popover__filter-option--selected")).toBeTruthy();

		await fireEvent.click(within(menu).getByRole("button", { name: "Any" }));
		await fireEvent.click(within(menu).getByRole("button", { name: "Archived" }));
		expect(onSelectFilter).toHaveBeenNthCalledWith(1, "status", undefined);
		expect(onSelectFilter).toHaveBeenNthCalledWith(2, "status", "archived");
	});

	it("omits 'All' option when includeAll is false", () => {
		render(RelationSelectorPopoverFilters, {
			filters: [
				{
					key: "category",
					label: "Category",
					includeAll: false,
					options: [{ id: "x", label: "X" }],
				},
			],
			activeFilters: {},
			openFilterKey: "category",
			getActiveFilterLabel: () => "X",
			onToggleFilter: vi.fn(),
			onSelectFilter: vi.fn(),
		});

		expect(screen.queryByRole("button", { name: "All" })).toBeNull();
		const menu = document.querySelector(".relation-selector-popover__filter-menu") as HTMLElement;
		expect(within(menu).getByRole("button", { name: "X" })).toBeTruthy();
	});
});
