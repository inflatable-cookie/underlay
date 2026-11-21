import { describe, expect, it, vi } from "vitest";
import { createReorderController } from "../../src/patterns/reorder-controller.svelte";

describe("patterns/reorder-controller.svelte.ts", () => {
	it("handles move/reset/update/merge/remove operations", () => {
		const submitFn = vi.fn(async () => undefined);
		const controller = createReorderController(
			[
				{ id: "a", label: "A" },
				{ id: "b", label: "B" },
				{ id: "c", label: "C" },
			],
			submitFn,
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
		expect(controller.isDirty).toBe(true);

		controller.updatePending([{ id: "x", label: "X" }]);
		expect(controller.pending.map((x) => x.id)).toEqual(["x"]);
		controller.pending = [{ id: "m", label: "M" }];
		expect(controller.pending.map((x) => x.id)).toEqual(["m"]);

		controller.mergeNewItems([
			{ id: "x", label: "X2" },
			{ id: "y", label: "Y" },
		]);
		expect(controller.pending.map((x) => x.id)).toEqual(["m", "x", "y"]);

		controller.removeItems(["x"]);
		expect(controller.pending.map((x) => x.id)).toEqual(["m", "y"]);

		controller.reset();
		expect(controller.pending.map((x) => x.id)).toEqual(["a", "b", "c"]);
		expect(controller.isDirty).toBe(false);
		expect(controller.error).toBeNull();
	});

	it("skips submit when clean and handles submit success/error when dirty", async () => {
		const cleanSubmit = vi.fn(async () => undefined);
		const clean = createReorderController([{ id: "a" }], cleanSubmit);
		await clean.submit();
		expect(cleanSubmit).not.toHaveBeenCalled();
		expect(clean.isPending).toBe(false);
		expect(clean.error).toBeNull();

		const successSubmit = vi.fn(async () => undefined);
		const dirtySuccess = createReorderController(
			[
				{ id: "k", label: "K" },
				{ id: "m", label: "M" },
			],
			successSubmit,
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
		const dirtyFail = createReorderController(
			[
				{ id: "z", label: "Z" },
				{ id: "q", label: "Q" },
			],
			failingSubmit,
		);
		dirtyFail.move(0, 1);
		await expect(dirtyFail.submit()).rejects.toThrow("submit failed");
		expect(dirtyFail.isPending).toBe(false);
		expect(dirtyFail.error?.message).toBe("submit failed");
		expect(dirtyFail.original.map((x) => x.id)).toEqual(["z", "q"]);
		expect(dirtyFail.isDirty).toBe(true);
	});

	it("guards re-entrant submit and preserves concurrent edits against the submitted snapshot", async () => {
		let resolveSubmit!: () => void;
		const submitFn = vi.fn(
			() =>
				new Promise<void>((resolve) => {
					resolveSubmit = resolve;
				}),
		);
		const controller = createReorderController(
			[
				{ id: "a", label: "A" },
				{ id: "b", label: "B" },
				{ id: "c", label: "C" },
			],
			submitFn,
		);

		controller.move(0, 1); // pending: [b, a, c]
		const inFlight = controller.submit();
		expect(controller.isPending).toBe(true);

		// Second overlapping submit is ignored.
		await controller.submit();
		expect(submitFn).toHaveBeenCalledTimes(1);
		expect(submitFn).toHaveBeenCalledWith(["b", "a", "c"]);

		// User performs another reorder while the first submit is in flight.
		controller.move(2, 0); // pending: [c, b, a]

		resolveSubmit();
		await inFlight;

		expect(controller.isPending).toBe(false);
		// original reflects the submitted snapshot [b, a, c], NOT the concurrent edit [c, b, a].
		expect(controller.original.map((x) => x.id)).toEqual(["b", "a", "c"]);
		expect(controller.pending.map((x) => x.id)).toEqual(["c", "b", "a"]);
		expect(controller.isDirty).toBe(true);
	});
});
