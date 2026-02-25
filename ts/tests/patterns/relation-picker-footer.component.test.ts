// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationPickerFooterHarness from "../fixtures/RelationPickerFooterHarness.svelte";

describe("patterns/relation-picker/RelationPickerFooter.svelte", () => {
	it("renders custom footer snippet when provided", () => {
		const view = render(RelationPickerFooterHarness, {
			multiSelect: true,
			createFormOpen: false,
			selectedCount: 2,
			useFooterSnippet: true,
			onCancel: vi.fn(),
			onConfirm: vi.fn(),
		});

		expect(screen.getByTestId("custom-footer")).toBeTruthy();
		expect(screen.queryByText("Confirm (2)")).toBeNull();
		expect(view.container.querySelector(".relation-picker-dialog__footer")).toBeTruthy();
	});

	it("renders default multi-select footer buttons and wires callbacks", async () => {
		const onCancel = vi.fn();
		const onConfirm = vi.fn();

		render(RelationPickerFooterHarness, {
			multiSelect: true,
			createFormOpen: false,
			selectedCount: 3,
			onCancel,
			onConfirm,
		});

		const cancelButton = screen.getByRole("button", { name: "Cancel" });
		const confirmButton = screen.getByRole("button", { name: "Confirm (3)" });
		expect(cancelButton).toBeTruthy();
		expect(confirmButton).toBeTruthy();

		await fireEvent.click(cancelButton);
		await fireEvent.click(confirmButton);
		expect(onCancel).toHaveBeenCalledTimes(1);
		expect(onConfirm).toHaveBeenCalledTimes(1);
	});

	it("does not render footer when not multi-select or create form is open", () => {
		const singleSelect = render(RelationPickerFooterHarness, {
			multiSelect: false,
			createFormOpen: false,
			selectedCount: 1,
			onCancel: vi.fn(),
			onConfirm: vi.fn(),
		});
		expect(singleSelect.container.querySelector(".relation-picker-dialog__footer")).toBeNull();
		singleSelect.unmount();

		const createOpen = render(RelationPickerFooterHarness, {
			multiSelect: true,
			createFormOpen: true,
			selectedCount: 1,
			onCancel: vi.fn(),
			onConfirm: vi.fn(),
		});
		expect(createOpen.container.querySelector(".relation-picker-dialog__footer")).toBeNull();
	});
});
