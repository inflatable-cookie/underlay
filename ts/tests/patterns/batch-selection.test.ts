import { describe, expect, it, vi } from "vitest";

describe("patterns/batch-selection.svelte.ts", () => {
	it("manages selection lifecycle and no-op branches", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useBatchSelection } = await import("../../src/patterns/batch-selection.svelte");
		const selection = useBatchSelection<string>();

		expect(selection.selectedIds).toEqual([]);
		expect(selection.count).toBe(0);
		expect(selection.hasSelection).toBe(false);

		selection.select("a");
		expect(selection.isSelected("a")).toBe(true);
		expect(selection.count).toBe(1);

		selection.select("a");
		expect(selection.selectedIds).toEqual(["a"]);

		selection.toggle("b", true);
		expect(new Set(selection.selectedIds)).toEqual(new Set(["a", "b"]));

		selection.toggle("a", false);
		expect(selection.isSelected("a")).toBe(false);
		expect(selection.isSelected("b")).toBe(true);

		selection.deselect("a");
		expect(selection.selectedIds).toEqual(["b"]);

		selection.deselect("b");
		expect(selection.selectedIds).toEqual([]);

		selection.selectAll(["x", "y", "z"]);
		expect(selection.count).toBe(3);
		expect(selection.hasSelection).toBe(true);

		selection.set(["k"]);
		expect(selection.selectedIds).toEqual(["k"]);

		selection.clear();
		expect(selection.selectedIds).toEqual([]);
		expect(selection.count).toBe(0);
		expect(selection.hasSelection).toBe(false);
	});
});
