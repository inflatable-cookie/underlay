import { describe, expect, it, vi } from "vitest";

async function loadModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	return await import("../../src/patterns/selection-mode-controller.svelte");
}

describe("patterns/selection-mode-controller.svelte.ts", () => {
	it("toggles selection mode and clears selection when exiting", async () => {
		const { createSelectionModeController } = await loadModule();
		const clearSelection = vi.fn();
		const exitReorderMode = vi.fn();
		const controller = createSelectionModeController({ clearSelection, exitReorderMode });

		expect(controller.selectionMode).toBe(false);

		controller.toggleSelectionMode(false);
		expect(controller.selectionMode).toBe(true);
		expect(clearSelection).not.toHaveBeenCalled();

		controller.toggleSelectionMode(false);
		expect(controller.selectionMode).toBe(false);
		expect(clearSelection).toHaveBeenCalledTimes(1);
	});

	it("exits reorder mode before entering selection and handles Escape", async () => {
		const { createSelectionModeController } = await loadModule();
		const clearSelection = vi.fn();
		const exitReorderMode = vi.fn();
		const controller = createSelectionModeController({ clearSelection, exitReorderMode });

		controller.toggleSelectionMode(true);
		expect(exitReorderMode).toHaveBeenCalledTimes(1);
		expect(controller.selectionMode).toBe(true);

		controller.handleKeydown({ key: "Escape" } as KeyboardEvent, false);
		expect(controller.selectionMode).toBe(false);
		expect(clearSelection).toHaveBeenCalledTimes(1);

		controller.handleKeydown({ key: "Escape" } as KeyboardEvent, true);
		expect(exitReorderMode).toHaveBeenCalledTimes(2);
	});
});
