// @vitest-environment jsdom
import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";

const { ctx } = vi.hoisted(() => ({
	ctx: {
		props: {
			required: false,
			disabled: false,
			error: undefined as string | undefined,
			placeholder: "Pick one",
		},
		state: {
			modalOpen: false,
		},
		isMultiSelect: false,
		selectedItem: null as { id: string; label: string } | null,
		selectedItems: [] as Array<{ id: string; label: string }>,
		clearSelection: vi.fn(),
		deselectItem: vi.fn(),
	},
}));

vi.mock("../../src/patterns/RelationSelector/context.svelte.js", () => ({
	useRelationSelector: () => ctx,
}));

import RelationSelectorTrigger from "../../src/patterns/RelationSelector/RelationSelectorTrigger.svelte";

describe("patterns/RelationSelector/RelationSelectorTrigger.svelte", () => {
	beforeEach(() => {
		ctx.props.required = false;
		ctx.props.disabled = false;
		ctx.props.error = undefined;
		ctx.props.placeholder = "Pick one";
		ctx.state.modalOpen = false;
		ctx.isMultiSelect = false;
		ctx.selectedItem = null;
		ctx.selectedItems = [];
		ctx.clearSelection.mockClear();
		ctx.deselectItem.mockClear();
	});

	it("renders placeholder/default state and error/disabled classes", () => {
		ctx.props.error = "Field error";
		ctx.props.disabled = true;
		const view = render(RelationSelectorTrigger, { class: "extra-trigger" });

		const root = view.container.querySelector(".relation-selector-trigger");
		expect(root).toBeTruthy();
		expect(root?.classList.contains("extra-trigger")).toBe(true);
		expect(root?.classList.contains("relation-selector-trigger--disabled")).toBe(true);
		expect(root?.classList.contains("relation-selector-trigger--error")).toBe(true);
		expect(root?.getAttribute("aria-label")).toBe("Pick one");
		expect(screen.getByText("Pick one")).toBeTruthy();
		expect(view.container.querySelector(".relation-selector-trigger__clear")).toBeNull();
	});

	it("shows clear button in single-select mode and invokes clear handler", async () => {
		ctx.selectedItem = { id: "a", label: "Alpha" };
		const view = render(RelationSelectorTrigger);

		expect(screen.getByText("Alpha")).toBeTruthy();
		const clear = screen.getByRole("button", { name: "Clear selection" });
		await fireEvent.click(clear);
		expect(ctx.clearSelection).toHaveBeenCalledTimes(1);
		expect(view.container.querySelector(".relation-selector-trigger__clear")).toBeTruthy();
	});

	it("renders multi-select pills with overflow and remove actions", async () => {
		ctx.isMultiSelect = true;
		ctx.selectedItems = [
			{ id: "a", label: "Alpha" },
			{ id: "b", label: "Beta" },
			{ id: "c", label: "Gamma" },
			{ id: "d", label: "Delta" },
		];
		const view = render(RelationSelectorTrigger);

		expect(view.container.querySelector(".relation-selector-trigger--multi")).toBeTruthy();
		expect(screen.getByText("Alpha")).toBeTruthy();
		expect(screen.getByText("Beta")).toBeTruthy();
		expect(screen.getByText("Gamma")).toBeTruthy();
		expect(screen.getByText("+1 more")).toBeTruthy();

		await fireEvent.click(screen.getByRole("button", { name: "Remove Beta" }));
		expect(ctx.deselectItem).toHaveBeenCalledWith("b");
	});
});
