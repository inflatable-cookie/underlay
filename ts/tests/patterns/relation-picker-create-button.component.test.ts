// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationPickerCreateButtonHarness from "../fixtures/RelationPickerCreateButtonHarness.svelte";

describe("patterns/relation-picker/RelationPickerCreateButton.svelte", () => {
	it("renders create button when allowed and createForm is available", async () => {
		const onCreate = vi.fn();

		const view = render(RelationPickerCreateButtonHarness, {
			allowCreate: true,
			createFormOpen: false,
			createLabel: "Create relation",
			useCreateForm: true,
			onCreate,
		});

		const button = screen.getByRole("button", { name: "Create relation" });
		expect(button).toBeTruthy();
		expect(view.container.querySelector(".relation-picker-dialog__create")).toBeTruthy();

		await fireEvent.click(button);
		expect(onCreate).toHaveBeenCalledTimes(1);
	});

	it("hides create button when create is disallowed, form is open, or createForm is missing", () => {
		const disallowed = render(RelationPickerCreateButtonHarness, {
			allowCreate: false,
			createFormOpen: false,
			createLabel: "Create relation",
			useCreateForm: true,
			onCreate: vi.fn(),
		});
		expect(disallowed.container.querySelector(".relation-picker-dialog__create")).toBeNull();
		disallowed.unmount();

		const createOpen = render(RelationPickerCreateButtonHarness, {
			allowCreate: true,
			createFormOpen: true,
			createLabel: "Create relation",
			useCreateForm: true,
			onCreate: vi.fn(),
		});
		expect(createOpen.container.querySelector(".relation-picker-dialog__create")).toBeNull();
		createOpen.unmount();

		const noCreateForm = render(RelationPickerCreateButtonHarness, {
			allowCreate: true,
			createFormOpen: false,
			createLabel: "Create relation",
			useCreateForm: false,
			onCreate: vi.fn(),
		});
		expect(noCreateForm.container.querySelector(".relation-picker-dialog__create")).toBeNull();
	});
});
