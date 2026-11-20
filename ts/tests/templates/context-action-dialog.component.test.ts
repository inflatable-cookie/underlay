import { fireEvent, render, screen } from "@testing-library/svelte";
import { expect, it, vi } from "vitest";
import ContextActionDialog from "../../src/templates/ContextActionDialog.svelte";
import type { ContextActionDefinition } from "../../src/templates/contextual-action.types";

const action: ContextActionDefinition = {
	id: "write",
	name: "Write summary",
	description: "Choose what to include.",
	resultMode: "client_prefill",
	defaultModelProviderKey: "provider",
	defaultModelId: "default",
	modelOptions: [
		{ providerKey: "provider", providerLabel: "Provider", modelId: "first", label: "First" },
		{ providerKey: "provider", providerLabel: "Provider", modelId: "default", label: "Default" },
	],
	fields: [
		{ id: "title", label: "Title", type: "text", required: true, defaultValue: "Summary" },
		{ id: "count", label: "Count", type: "number", defaultValue: 0 },
		{ id: "include", label: "Include history", type: "checkbox", defaultValue: false },
		{
			id: "style",
			label: "Style",
			type: "select",
			defaultValue: "short",
			options: [{ value: "short", label: "Short" }],
		},
	],
};

it("labels each input and submits default values through the dialog form", async () => {
	const onSubmit = vi.fn();
	render(ContextActionDialog, { action, onSubmit, open: true });

	expect(screen.getByRole("textbox", { name: "Title" })).toBeTruthy();
	expect(screen.getByRole("combobox", { name: "Model" })).toBeTruthy();
	expect(screen.getByRole("combobox", { name: "Style" })).toBeTruthy();
	expect(screen.getByRole("checkbox", { name: "Include history" })).toBeTruthy();
	expect(screen.getAllByText("Include history")).toHaveLength(1);

	await fireEvent.click(screen.getByRole("button", { name: "Generate" }));

	expect(onSubmit).toHaveBeenCalledExactlyOnceWith({
		action,
		values: { title: "Summary", count: 0, include: false, style: "short" },
		selectedProviderKey: "provider",
		selectedModelId: "default",
	});
});

it("commits a changed number and submits once when Enter is pressed", async () => {
	const onSubmit = vi.fn();
	render(ContextActionDialog, { action, onSubmit, open: true });

	const number = screen.getByRole("spinbutton", { name: "Count" });
	await fireEvent.focus(number);
	await fireEvent.input(number, { target: { value: "4" } });
	await fireEvent.keyDown(number, { key: "Enter" });

	expect(onSubmit).toHaveBeenCalledTimes(1);
	expect(onSubmit).toHaveBeenCalledWith(
		expect.objectContaining({
			values: { title: "Summary", count: 4, include: false, style: "short" },
		}),
	);
});

it("keeps an explicitly cleared required field empty and reports the validation error", async () => {
	const onSubmit = vi.fn();
	render(ContextActionDialog, { action, onSubmit, open: true });
	const title = screen.getByRole("textbox", { name: "Title" });

	await fireEvent.input(title, { target: { value: "" } });
	await fireEvent.click(screen.getByRole("button", { name: "Generate" }));

	expect((await screen.findByRole("alert")).textContent).toBe("Title is required.");
	expect(onSubmit).not.toHaveBeenCalled();
});
