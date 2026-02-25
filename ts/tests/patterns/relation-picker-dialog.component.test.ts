// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationPickerDialogHarness from "../fixtures/RelationPickerDialogHarness.svelte";

describe("patterns/RelationPickerDialog.svelte", () => {
	it("filters local items by search query and emits selection callbacks", async () => {
		const onSelect = vi.fn();
		const items = [
			{ id: "a", label: "Alpha", description: "first" },
			{ id: "b", label: "Beta", description: "second" },
		];
		render(RelationPickerDialogHarness, {
			title: "Pick relation",
			items,
			onSelect,
		});

		expect(screen.getByText("Pick relation")).toBeTruthy();
		expect(screen.getByText("Alpha")).toBeTruthy();
		expect(screen.getByText("Beta")).toBeTruthy();

		const input = screen.getByRole("textbox") as HTMLInputElement;
		await fireEvent.input(input, { target: { value: "beta" } });
		expect(screen.queryByText("Alpha")).toBeNull();
		expect(screen.getByText("Beta")).toBeTruthy();

		await fireEvent.click(screen.getByText("Beta"));
		expect(onSelect).toHaveBeenCalledWith(items[1]);
	});

	it("uses external search mode when onSearch is provided", async () => {
		const onSearch = vi.fn();
		render(RelationPickerDialogHarness, {
			title: "Search relations",
			items: [
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Beta" },
			],
			onSearch,
		});

		const input = screen.getByRole("textbox") as HTMLInputElement;
		await fireEvent.input(input, { target: { value: "zzz" } });
		expect(onSearch).toHaveBeenCalledWith("zzz");
		expect(screen.getByText("Alpha")).toBeTruthy();
		expect(screen.getByText("Beta")).toBeTruthy();
	});

	it("renders create mode and forwards create success/cancel callbacks", async () => {
		const onCreateSuccess = vi.fn();
		const onCreateCancel = vi.fn();
		render(RelationPickerDialogHarness, {
			title: "Pick relation",
			createLabel: "Create relation",
			createFormOpen: true,
			useCreateForm: true,
			onCreateSuccess,
			onCreateCancel,
		});

		expect(screen.getByText("Create relation")).toBeTruthy();
		expect(screen.queryByRole("textbox")).toBeNull();
		expect(screen.getByTestId("create-form")).toBeTruthy();

		await fireEvent.click(screen.getByTestId("create-success"));
		await fireEvent.click(screen.getByTestId("create-cancel"));
		expect(onCreateSuccess).toHaveBeenCalledWith({ id: "new-1", label: "Created item" });
		expect(onCreateCancel).toHaveBeenCalledTimes(1);
	});

	it("renders clear/header/default footer actions and allows footer override", async () => {
		const onClear = vi.fn();
		const onCancel = vi.fn();
		const onConfirm = vi.fn();

		const view = render(RelationPickerDialogHarness, {
			title: "Pick relation",
			items: [{ id: "a", label: "Alpha" }],
			showClear: true,
			useHeaderExtra: true,
			multiSelect: true,
			selectedCount: 2,
			onClear,
			onCancel,
			onConfirm,
		});

		expect(screen.getByTestId("header-extra")).toBeTruthy();
		await fireEvent.click(screen.getByRole("button", { name: "Clear" }));
		expect(onClear).toHaveBeenCalledTimes(1);

		await fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
		await fireEvent.click(screen.getByRole("button", { name: "Confirm (2)" }));
		expect(onCancel).toHaveBeenCalledTimes(1);
		expect(onConfirm).toHaveBeenCalledTimes(1);
		view.unmount();

		render(RelationPickerDialogHarness, {
			title: "Pick relation",
			items: [{ id: "a", label: "Alpha" }],
			useFooterSnippet: true,
		});
		expect(screen.getByTestId("custom-footer")).toBeTruthy();
		expect(screen.queryByText("Confirm (0)")).toBeNull();
	});
});
