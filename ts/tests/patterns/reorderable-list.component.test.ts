// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";

vi.mock("svelte-dnd-action", () => ({
	dndzone: () => ({
		update: () => undefined,
		destroy: () => undefined,
	}),
}));

import ReorderableListHarness from "../fixtures/ReorderableListHarness.svelte";

interface Item {
	id: string;
	label: string;
}

function createController(options?: {
	items?: Item[];
	isDirty?: boolean;
	isPending?: boolean;
	submitImpl?: () => Promise<void>;
}) {
	let pending = options?.items ?? [];
	const submit = vi.fn(
		options?.submitImpl ??
			(async () => {
				return undefined;
			})
	);
	const reset = vi.fn();
	const updatePending = vi.fn((items: Item[]) => {
		pending = items;
	});

	return {
		get pending() {
			return pending;
		},
		set pending(value: Item[]) {
			pending = value;
		},
		get isDirty() {
			return options?.isDirty ?? true;
		},
		get isPending() {
			return options?.isPending ?? false;
		},
		submit,
		reset,
		updatePending,
	};
}

afterEach(() => cleanup());

describe("patterns/ReorderableList.svelte", () => {
	it("renders rows and wires dnd consider/finalize events to controller updates", async () => {
		const controller = createController({
			items: [
				{ id: "a", label: "Alpha" },
				{ id: "b", label: "Beta" },
			],
		});
		const view = render(ReorderableListHarness, { controller });

		expect(screen.getByTestId("row-a").textContent).toContain("Alpha");
		expect(screen.getByTestId("row-b").textContent).toContain("Beta");

		const items = view.container.querySelector(".underlay-reorderable-list__items");
		expect(items).toBeTruthy();
		items?.dispatchEvent(
			new CustomEvent("consider", {
				detail: { items: [{ id: "x", label: "X" }] },
			})
		);
		items?.dispatchEvent(
			new CustomEvent("finalize", {
				detail: { items: [{ id: "y", label: "Y" }] },
			})
		);

		expect(controller.updatePending).toHaveBeenCalledTimes(2);
		expect(controller.updatePending).toHaveBeenNthCalledWith(1, [{ id: "x", label: "X" }]);
		expect(controller.updatePending).toHaveBeenNthCalledWith(2, [{ id: "y", label: "Y" }]);
	});

	it("handles cancel, submit success, submit error transform, and empty state", async () => {
		const oncancel = vi.fn();
		const onsuccess = vi.fn();
		const controller = createController({
			items: [],
			isDirty: false,
			submitImpl: async () => undefined,
		});
		const first = render(ReorderableListHarness, {
			controller,
			oncancel,
			onsuccess,
			saveLabel: "Save",
			cancelLabel: "Abort",
		});

		expect(screen.getByTestId("empty-list").textContent).toContain("No rows");
		expect(
			first.container.querySelector(".underlay-reorderable-list__header button:last-child")?.hasAttribute("disabled")
		).toBe(true);

		const cancelButton = Array.from(first.container.querySelectorAll("button")).find((b) =>
			b.textContent?.includes("Abort")
		);
		await fireEvent.click(cancelButton!);
		expect(controller.reset).toHaveBeenCalledTimes(1);
		expect(oncancel).toHaveBeenCalledTimes(1);

		const successController = createController({
			items: [{ id: "a", label: "Alpha" }],
			isDirty: true,
		});
		first.unmount();
		const success = render(ReorderableListHarness, {
			controller: successController,
			onsuccess,
		});
		const saveButton = Array.from(success.container.querySelectorAll("button")).find((b) =>
			b.textContent?.includes("Save Order")
		);
		await fireEvent.click(saveButton!);
		await Promise.resolve();
		expect(successController.submit).toHaveBeenCalledTimes(1);
		expect(onsuccess).toHaveBeenCalledTimes(1);
		expect(success.container.querySelector(".underlay-reorderable-list__error")).toBeNull();

		const failingController = createController({
			items: [{ id: "z", label: "Zulu" }],
			isDirty: true,
			submitImpl: async () => {
				throw new Error("raw submit failure");
			},
		});
		const onsubmiterror = vi.fn(async () => "friendly error");
		success.unmount();
		const fail = render(ReorderableListHarness, {
			controller: failingController,
			onsubmiterror,
			disabled: true,
		});
		expect(
			fail.container
				.querySelector(".underlay-reorderable-list")
				?.classList.contains("underlay-reorderable-list--disabled")
		).toBe(true);
		const failSave = Array.from(fail.container.querySelectorAll("button")).find((b) =>
			b.textContent?.includes("Save Order")
		);
		await fireEvent.click(failSave!);
		await Promise.resolve();
		expect(failingController.submit).toHaveBeenCalledTimes(1);
		expect(onsubmiterror).toHaveBeenCalledTimes(1);
		expect(fail.container.querySelector(".underlay-reorderable-list__error")?.textContent).toContain("friendly error");
	});
});
