// @vitest-environment jsdom
import { flushSync } from "svelte";
import { describe, expect, it, vi } from "vitest";
import { createLoadedReorderSession } from "../../src/patterns/reorder-session.svelte";
import { createEffectRoot, createReactiveBox } from "../patterns/effect-root.svelte";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

describe("patterns/reorder-session.svelte.ts standalone reactivity & teardown", () => {
	it("reacts while entered, stops reacting after exit(), re-attaches on enter(), and tears down on destroy()", async () => {
		const labelBox = createReactiveBox("v1");
		const mapItems = vi.fn((items: Array<{ rowId: string }>) => {
			const suffix = labelBox.value;
			return items.map((item) => ({
				id: item.rowId,
				label: `${item.rowId}:${suffix}`,
			}));
		});

		const session = createLoadedReorderSession({
			loadItems: async () => ({
				items: [{ rowId: "a" }, { rowId: "b" }],
			}),
			mapItems,
			submitReorder: vi.fn(async () => undefined),
			entityLabel: "item",
			pushInfo: vi.fn(),
			pushError: vi.fn(),
			onRefresh: vi.fn(async () => undefined),
		});

		// Not yet entered: standalone session has not attached an active effect root.
		labelBox.value = "v1b";
		flushSync();
		expect(mapItems).not.toHaveBeenCalled();

		// Enter attaches the standalone effect root and reacts to reactive mapItems dependencies.
		await session.enter();
		flushSync();
		expect(session.reorderMode).toBe(true);
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:v1b" },
			{ id: "b", label: "b:v1b" },
		]);

		labelBox.value = "v2";
		flushSync();
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:v2" },
			{ id: "b", label: "b:v2" },
		]);

		// Exit tears down the standalone effect root so subsequent reactive updates do not run.
		session.exit();
		flushSync();
		expect(session.reorderMode).toBe(false);
		const mapCallsAfterExit = mapItems.mock.calls.length;

		labelBox.value = "v3";
		flushSync();
		expect(mapItems).toHaveBeenCalledTimes(mapCallsAfterExit);

		// Re-entering cleanly re-attaches the standalone effect root.
		await session.enter();
		flushSync();
		expect(session.reorderMode).toBe(true);
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:v3" },
			{ id: "b", label: "b:v3" },
		]);

		labelBox.value = "v4";
		flushSync();
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:v4" },
			{ id: "b", label: "b:v4" },
		]);

		// destroy() tears down the standalone effect root and prevents re-entry.
		session.destroy();
		flushSync();
		expect(session.reorderMode).toBe(false);
		expect(session.reorderItems).toEqual([]);
		expect(session.controller.pending).toEqual([]);
		const mapCallsAfterDestroy = mapItems.mock.calls.length;

		labelBox.value = "v5";
		flushSync();
		expect(mapItems).toHaveBeenCalledTimes(mapCallsAfterDestroy);

		await session.enter();
		expect(session.reorderMode).toBe(false);
	});

	it("cancels an in-flight standalone enter() when destroy() is called", async () => {
		const loadGate = deferred<{ items: Array<{ rowId: string }>; error?: string }>();
		const labelBox = createReactiveBox("v1");
		const mapItems = vi.fn((items: Array<{ rowId: string }>) => {
			const suffix = labelBox.value;
			return items.map((item) => ({ id: item.rowId, label: `${item.rowId}:${suffix}` }));
		});

		const session = createLoadedReorderSession({
			loadItems: () => loadGate.promise,
			mapItems,
			submitReorder: vi.fn(async () => undefined),
			entityLabel: "item",
			pushInfo: vi.fn(),
			pushError: vi.fn(),
			onRefresh: vi.fn(async () => undefined),
		});

		const entering = session.enter();
		session.destroy();

		loadGate.resolve({ items: [{ rowId: "a" }] });
		await entering;
		flushSync();

		expect(session.reorderMode).toBe(false);
		expect(mapItems).not.toHaveBeenCalled();

		labelBox.value = "v2";
		flushSync();
		expect(mapItems).not.toHaveBeenCalled();
	});

	it("supports destroy() on a session owned by an outer effect root", async () => {
		const labelBox = createReactiveBox("v1");
		const mapItems = vi.fn((items: Array<{ rowId: string }>) => {
			const suffix = labelBox.value;
			return items.map((item) => ({ id: item.rowId, label: `${item.rowId}:${suffix}` }));
		});

		let session!: ReturnType<
			typeof createLoadedReorderSession<{ rowId: string }, { id: string; label: string }>
		>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession({
				loadItems: async () => ({ items: [{ rowId: "a" }] }),
				mapItems,
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await session.enter();
			flushSync();
			expect(session.controller.pending).toEqual([{ id: "a", label: "a:v1" }]);

			session.destroy();
			expect(session.reorderMode).toBe(false);

			labelBox.value = "v2";
			flushSync();
			expect(session.controller.pending).toEqual([]);

			await session.enter();
			expect(session.reorderMode).toBe(false);
		} finally {
			dispose();
		}
	});
});
