// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import RelationSelectorHarness from "../fixtures/RelationSelectorHarness.svelte";

describe("patterns/RelationSelector/*.svelte", () => {
	it("renders single-select trigger state and clears selection", async () => {
		const onchange = vi.fn();
		render(RelationSelectorHarness, {
			mode: "single",
			placeholder: "Pick one",
			value: "a",
			initialSelection: { id: "a", label: "Alpha" },
			suggestionsItems: [{ id: "a", label: "Alpha" }],
			searchItems: [{ id: "a", label: "Alpha" }],
			onchange,
		});

		expect(screen.getByText("Alpha")).toBeTruthy();
		const clear = screen.getByRole("button", { name: "Clear selection" });
		await fireEvent.click(clear);

		expect(onchange).toHaveBeenCalledWith(null);
		await waitFor(() => {
			expect(screen.getByText("Pick one")).toBeTruthy();
		});
		expect(screen.getByTestId("single-value").textContent).toBe("null");
	});

	it("supports multi-select pill removal and shows overflow count", async () => {
		const onchangeMulti = vi.fn();
		render(RelationSelectorHarness, {
			mode: "multi",
			values: ["a", "b", "c", "d"],
			initialSelections: [
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Beta" },
				{ id: "c", label: "Gamma" },
				{ id: "d", label: "Delta" },
			],
			suggestionsItems: [],
			searchItems: [],
			onchangeMulti,
		});

		expect(screen.getByText("Alpha")).toBeTruthy();
		expect(screen.getByText("Beta")).toBeTruthy();
		expect(screen.getByText("Gamma")).toBeTruthy();
		expect(screen.getByText("+1 more")).toBeTruthy();

		await fireEvent.click(screen.getByRole("button", { name: "Remove Alpha" }));
		expect(onchangeMulti).toHaveBeenCalledWith(["b", "c", "d"]);
		await waitFor(() => {
			expect(screen.getByTestId("multi-values").textContent).toBe("b,c,d");
		});
	});

	it("opens popover, selects suggestions, and updates bound value", async () => {
		const onchange = vi.fn();
		render(RelationSelectorHarness, {
			mode: "single",
			label: "Pick relation",
			placeholder: "Select relation",
			suggestionsItems: [
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Beta" },
			],
			searchItems: [
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Beta" },
			],
			onchange,
		});

		await fireEvent.click(screen.getByRole("button", { name: "Select relation" }));
		await waitFor(() => {
			expect(screen.getByText("Suggestions")).toBeTruthy();
		});

		await fireEvent.click(screen.getByText("Beta"));
		expect(onchange).toHaveBeenCalledWith("b");
		await waitFor(() => {
			expect(screen.getByTestId("single-value").textContent).toBe("b");
		});
	});

	it("switches from popover to create modal and handles create success", async () => {
		const onchangeMulti = vi.fn();
		const onCreate = vi.fn();
		render(RelationSelectorHarness, {
			mode: "multi",
			label: "Pick relation",
			values: ["a"],
			initialSelections: [{ id: "a", label: "Alpha" }],
			suggestionsItems: [{ id: "a", label: "Alpha" }],
			searchItems: [{ id: "a", label: "Alpha" }],
			allowCreate: true,
			createLabel: "Add relation",
			useCreateForm: true,
			onchangeMulti,
			onCreate,
		});

		await fireEvent.click(screen.getByRole("button", { name: "Add relation" }));
		await waitFor(() => {
			expect(screen.getByTestId("selector-create-form")).toBeTruthy();
		});

		await fireEvent.click(screen.getByTestId("selector-create-success"));
		expect(onchangeMulti).toHaveBeenCalledWith(["a", "created"]);
		expect(onCreate).toHaveBeenCalledWith({ id: "created", label: "Created relation" });
		await waitFor(() => {
			expect(screen.getByTestId("multi-values").textContent).toBe("a,created");
		});

		await fireEvent.click(screen.getByRole("button", { name: "Close" }));
	});
});
