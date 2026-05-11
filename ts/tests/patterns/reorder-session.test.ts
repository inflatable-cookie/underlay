import { describe, expect, it, vi } from "vitest";

async function loadModule() {
	vi.resetModules();
	(globalThis as any).$state = <T>(initial: T) => initial;
	(globalThis as any).$derived = ((value: unknown) => value) as typeof $derived;
	return await import("../../src/patterns/reorder-session.svelte");
}

describe("patterns/reorder-session.svelte.ts", () => {
	it("handles local constrained reorder lifecycle and conflict recovery", async () => {
		const { createLocalReorderSession } = await loadModule();
		const clearSelection = vi.fn();
		const clearSelectionMode = vi.fn();
		const pushInfo = vi.fn();
		const pushSuccess = vi.fn();
		const onRefresh = vi.fn(async () => undefined);
		const onDataChange = vi.fn();

		const controller = {
			pending: [
				{ id: "a", label: "A" },
				{ id: "b", label: "B" }
			],
			mergeNewItems(items: Array<{ id: string; label: string }>) {
				const existing = new Set(this.pending.map((item) => item.id));
				this.pending = [...this.pending, ...items.filter((item) => !existing.has(item.id))];
			},
			removeItems(idsToRemove: string[]) {
				const removeSet = new Set(idsToRemove);
				this.pending = this.pending.filter((item) => !removeSet.has(item.id));
			}
		};

		const session = createLocalReorderSession({
			getController: () => controller as any,
			getLatestItems: () => [
				{ id: "a", label: "A" },
				{ id: "c", label: "C" }
			],
			entityLabel: "item",
			pushInfo,
			pushSuccess,
			onRefresh,
			onDataChange,
			clearSelection,
			clearSelectionMode
		});

		session.enter();
		expect(session.reorderMode).toBe(true);
		expect(clearSelection).toHaveBeenCalledTimes(1);
		expect(clearSelectionMode).toHaveBeenCalledTimes(1);

		const message = await session.handleError({
			status: 409,
			raw: {
				error: { message: "Items changed" },
				context: {
					added_ids: ["c"],
					removed_ids: ["b"]
				}
			}
		});

		expect(message).toBe("Items changed");
		expect(session.highlightedIds).toEqual(["c"]);
		expect(controller.pending.map((item) => item.id)).toEqual(["a", "c"]);
		expect(pushInfo).toHaveBeenCalledWith("Items changed");

		await session.handleSuccess("Saved order");
		expect(session.reorderMode).toBe(false);
		expect(session.highlightedIds).toEqual([]);
		expect(onRefresh).toHaveBeenCalledTimes(1);
		expect(onDataChange).toHaveBeenCalledTimes(1);
		expect(pushSuccess).toHaveBeenCalledWith("Saved order");
	});

	it("handles loaded reorder lifecycle, load failure, and loaded conflict recovery", async () => {
		const { createLoadedReorderSession } = await loadModule();
		const pushInfo = vi.fn();
		const pushError = vi.fn();
		const pushSuccess = vi.fn();
		const onRefresh = vi.fn(async () => undefined);
		const onDataChange = vi.fn();
		const clearSelection = vi.fn();
		const clearSelectionMode = vi.fn();
		const submitReorder = vi.fn(async () => undefined);

		const failure = createLoadedReorderSession({
			loadItems: async () => ({ items: [], error: "load failed" }),
			mapItems: (items: Array<{ rowId: string; label: string }>) =>
				items.map((item) => ({ ...item, id: item.rowId })),
			submitReorder,
			entityLabel: "item",
			pushInfo,
			pushSuccess,
			pushError,
			onRefresh,
			onDataChange,
			clearSelection,
			clearSelectionMode
		});

		await failure.enter();
		expect(failure.reorderMode).toBe(false);
		expect(pushError).toHaveBeenCalledWith("load failed");

		const loaded = createLoadedReorderSession({
			loadItems: async () => ({
				items: [
					{ rowId: "a", label: "A" },
					{ rowId: "b", label: "B" }
				]
			}),
			mapItems: (items: Array<{ rowId: string; label: string }>) =>
				items.map((item) => ({ ...item, id: item.rowId })),
			submitReorder,
			entityLabel: "item",
			pushInfo,
			pushSuccess,
			pushError,
			onRefresh,
			onDataChange,
			clearSelection,
			clearSelectionMode
		});

		await loaded.enter();
		expect(loaded.reorderMode).toBe(true);

		const message = await loaded.handleError({
			status: 409,
			raw: {
				error: { message: "Items changed" },
				context: {
					added_ids: ["c"],
					removed_ids: ["b"]
				}
			}
		});

		expect(message).toBe("Items changed");
		expect(loaded.highlightedIds).toEqual([]);
		expect(pushInfo).toHaveBeenCalledWith("Items changed");

		await loaded.handleSuccess("Saved order");
		expect(loaded.reorderMode).toBe(false);
		expect(loaded.reorderItems).toEqual([]);
		expect(onRefresh).toHaveBeenCalled();
		expect(onDataChange).toHaveBeenCalled();
		expect(pushSuccess).toHaveBeenCalledWith("Saved order");
	});
});
