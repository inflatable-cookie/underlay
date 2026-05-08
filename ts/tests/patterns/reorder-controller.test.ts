import { describe, expect, it, vi } from "vitest";

async function loadReorderModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	return await import("../../src/patterns/reorder-controller.svelte");
}

describe("patterns/reorder-controller.svelte.ts", () => {
	it("handles move/reset/update/merge/remove operations", async () => {
		const { createReorderController } = await loadReorderModule();
		const submitFn = vi.fn(async () => undefined);
		const controller = createReorderController(
			[
				{ id: "a", label: "A" },
				{ id: "b", label: "B" },
				{ id: "c", label: "C" },
			],
			submitFn
		);

		expect(controller.pending.map((x) => x.id)).toEqual(["a", "b", "c"]);
		expect(controller.original.map((x) => x.id)).toEqual(["a", "b", "c"]);
		expect(controller.isDirty).toBe(false);

		controller.move(0, 0);
		expect(controller.pending.map((x) => x.id)).toEqual(["a", "b", "c"]);
		controller.move(-1, 1);
		controller.move(0, 99);
		expect(controller.pending.map((x) => x.id)).toEqual(["a", "b", "c"]);

		controller.move(0, 2);
		expect(controller.pending.map((x) => x.id)).toEqual(["b", "c", "a"]);

		controller.updatePending([{ id: "x", label: "X" }]);
		expect(controller.pending.map((x) => x.id)).toEqual(["x"]);
		controller.pending = [{ id: "m", label: "M" }];
		expect(controller.pending.map((x) => x.id)).toEqual(["m"]);

		controller.mergeNewItems([{ id: "x", label: "X2" }, { id: "y", label: "Y" }]);
		expect(controller.pending.map((x) => x.id)).toEqual(["m", "x", "y"]);

		controller.removeItems(["x"]);
		expect(controller.pending.map((x) => x.id)).toEqual(["m", "y"]);

		controller.reset();
		expect(controller.pending.map((x) => x.id)).toEqual(["a", "b", "c"]);
		expect(controller.error).toBeNull();
	});

	it("skips submit when clean and handles submit success/error when dirty", async () => {
		const cleanModule = await loadReorderModule();
		const cleanSubmit = vi.fn(async () => undefined);
		const clean = cleanModule.createReorderController([{ id: "a" }], cleanSubmit);
		await clean.submit();
		expect(cleanSubmit).not.toHaveBeenCalled();
		expect(clean.isPending).toBe(false);
		expect(clean.error).toBeNull();

		const dirtyModule = await loadReorderModule();
		const successSubmit = vi.fn(async () => undefined);
		const dirtySuccess = dirtyModule.createReorderController(
			[
				{ id: "k", label: "K" },
				{ id: "m", label: "M" },
			],
			successSubmit
		);
		dirtySuccess.move(0, 1);
		await dirtySuccess.submit();
		expect(successSubmit).toHaveBeenCalledWith(["m", "k"]);
		expect(dirtySuccess.isPending).toBe(false);
		expect(dirtySuccess.error).toBeNull();
		expect(dirtySuccess.original.map((x) => x.id)).toEqual(["m", "k"]);
		expect(dirtySuccess.isDirty).toBe(false);
		dirtySuccess.reset();
		expect(dirtySuccess.pending.map((x) => x.id)).toEqual(["m", "k"]);

		const failingSubmit = vi.fn(async () => {
			throw new Error("submit failed");
		});
		const dirtyFail = dirtyModule.createReorderController(
			[
				{ id: "z", label: "Z" },
				{ id: "q", label: "Q" },
			],
			failingSubmit
		);
		dirtyFail.move(0, 1);
		await expect(dirtyFail.submit()).rejects.toThrow("submit failed");
		expect(dirtyFail.isPending).toBe(false);
		expect(dirtyFail.error?.message).toBe("submit failed");
		expect(dirtyFail.original.map((x) => x.id)).toEqual(["z", "q"]);
		expect(dirtyFail.isDirty).toBe(true);
	});
});
