// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationSelectorPopoverListSectionHarness from "../fixtures/RelationSelectorPopoverListSectionHarness.svelte";

describe("patterns/RelationSelector/RelationSelectorPopoverListSection.svelte", () => {
	it("renders list with selected/focused/disabled states and interaction handlers", async () => {
		const onItemClick = vi.fn();
		const onListKeyDown = vi.fn();
		const onListRef = vi.fn();
		const items = [
			{ id: "a", label: "Alpha", description: "A desc" },
			{ id: "b", label: "Beta", disabled: true },
		];

		const view = render(RelationSelectorPopoverListSectionHarness, {
			label: "Suggestions",
			items,
			focusedIndex: 0,
			isSelected: (id: string) => id === "a",
			onItemClick,
			onListKeyDown,
			onListRef,
		});

		expect(screen.getByText("Suggestions")).toBeTruthy();
		const list = view.container.querySelector("#relation-selector-popover-list");
		expect(list).toBeTruthy();
		expect(onListRef).toHaveBeenCalledWith(list);

		const rows = view.container.querySelectorAll(".relation-selector-popover__item");
		expect(rows.length).toBe(2);
		expect(rows[0]?.classList.contains("relation-selector-popover__item--selected")).toBe(true);
		expect(rows[0]?.classList.contains("relation-selector-popover__item--focused")).toBe(true);
		expect(rows[0]?.getAttribute("tabindex")).toBe("0");
		expect(rows[1]?.classList.contains("relation-selector-popover__item--disabled")).toBe(true);
		expect(rows[1]?.getAttribute("tabindex")).toBe("-1");
		expect(view.container.querySelector(".relation-selector-popover__item-check")).toBeTruthy();

		await fireEvent.click(rows[0] as Element);
		await fireEvent.keyDown(rows[0] as Element, { key: "Enter" });
		await fireEvent.keyDown(rows[0] as Element, { key: " " });
		expect(onItemClick).toHaveBeenCalledTimes(3);
		expect(onItemClick).toHaveBeenCalledWith(items[0]);

		onListKeyDown.mockClear();
		await fireEvent.keyDown(list as Element, { key: "ArrowDown" });
		expect(onListKeyDown).toHaveBeenCalledTimes(1);
		expect(onListKeyDown.mock.calls[0]?.[1]).toEqual(items);
	});

	it("supports custom renderItem snippet path", async () => {
		const onItemClick = vi.fn();
		const view = render(RelationSelectorPopoverListSectionHarness, {
			label: "Results (1)",
			items: [{ id: "x", label: "Xray" }],
			focusedIndex: 0,
			useRenderItem: true,
			isSelected: () => false,
			onItemClick,
			onListKeyDown: vi.fn(),
			onListRef: vi.fn(),
		});

		expect(screen.getByTestId("custom-item-x").textContent).toContain("Xray::idle");
		expect(view.container.querySelector(".relation-selector-popover__item-content")).toBeNull();

		const row = view.container.querySelector(".relation-selector-popover__item") as Element;
		await fireEvent.click(row);
		expect(onItemClick).toHaveBeenCalledWith({ id: "x", label: "Xray" });
	});
});
