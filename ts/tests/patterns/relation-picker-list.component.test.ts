// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationPickerListHarness from "../fixtures/RelationPickerListHarness.svelte";

describe("patterns/relation-picker/RelationPickerList.svelte", () => {
	it("renders flat list mode with selection/focus/disabled states and interaction handlers", async () => {
		const onItemClick = vi.fn();
		const onListKeyDown = vi.fn();
		const onListRef = vi.fn();
		const displayItems = [
			{ id: "a", label: "Alpha", description: "A desc" },
			{ id: "b", label: "Beta", disabled: true },
		];

		const view = render(RelationPickerListHarness, {
			displaySections: null,
			displayItems,
			sectionLabel: "Results",
			selectedIds: ["a"],
			focusedIndex: 0,
			onItemClick,
			onListKeyDown,
			getGlobalIndex: (_section: number, idx: number) => idx,
			onListRef,
		});

		expect(screen.getByText("Results")).toBeTruthy();
		const list = view.container.querySelector("#relation-picker-list");
		expect(list).toBeTruthy();
		expect(onListRef).toHaveBeenCalledWith(list);

		const items = view.container.querySelectorAll(".relation-picker-dialog__item");
		expect(items.length).toBe(2);
		expect(items[0]?.classList.contains("relation-picker-dialog__item--selected")).toBe(true);
		expect(items[0]?.classList.contains("relation-picker-dialog__item--focused")).toBe(true);
		expect(items[0]?.getAttribute("tabindex")).toBe("0");
		expect(items[1]?.classList.contains("relation-picker-dialog__item--disabled")).toBe(true);
		expect(items[1]?.getAttribute("tabindex")).toBe("-1");
		expect(view.container.querySelector(".relation-picker-dialog__item-check")).toBeTruthy();

		await fireEvent.click(items[0] as Element);
		expect(onItemClick).toHaveBeenCalledWith(displayItems[0]);

		await fireEvent.keyDown(items[0] as Element, { key: "Enter" });
		await fireEvent.keyDown(items[0] as Element, { key: " " });
		expect(onItemClick).toHaveBeenCalledTimes(3);

		onListKeyDown.mockClear();
		await fireEvent.keyDown(list as Element, { key: "ArrowDown" });
		expect(onListKeyDown).toHaveBeenCalledTimes(1);
	});

	it("renders sectioned mode and custom renderItem snippet path", async () => {
		const onItemClick = vi.fn();
		const onListKeyDown = vi.fn();
		const onListRef = vi.fn();
		const displaySections = [
			{
				label: "Suggestions",
				items: [{ id: "s1", label: "Suggestion 1" }],
			},
			{
				label: "Empty",
				items: [],
			},
		];

		const view = render(RelationPickerListHarness, {
			displaySections,
			displayItems: [],
			selectedIds: ["s1"],
			focusedIndex: 5,
			useRenderItem: true,
			onItemClick,
			onListKeyDown,
			getGlobalIndex: (section: number, idx: number) => section * 10 + idx + 5,
			onListRef,
		});

		expect(screen.getByText("Suggestions")).toBeTruthy();
		expect(screen.queryByText("Empty")).toBeNull();
		expect(screen.getByTestId("custom-item-s1").textContent).toContain("selected");
		expect(view.container.querySelector(".relation-picker-dialog__item-content")).toBeNull();
		expect(view.container.querySelector(".relation-picker-dialog__item--focused")).toBeTruthy();

		const item = view.container.querySelector(".relation-picker-dialog__item") as Element;
		await fireEvent.click(item);
		expect(onItemClick).toHaveBeenCalledWith(displaySections[0].items[0]);
	});
});
