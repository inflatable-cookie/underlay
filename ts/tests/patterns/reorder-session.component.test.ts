// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { createReorderController } from "../../src/patterns/reorder-controller.svelte";
import {
	createLoadedReorderSession,
	createLocalReorderSession,
} from "../../src/patterns/reorder-session.svelte";
import { createEffectRoot, createReactiveBox } from "./effect-root.svelte";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

describe("patterns/reorder-session.svelte.ts (compiled)", () => {
	it("cancels an in-flight createLoadedReorderSession.enter() when exit() is called", async () => {
		const loadGate = deferred<{ items: Array<{ rowId: string; label: string }>; error?: string }>();
		const pushError = vi.fn();

		let session!: ReturnType<
			typeof createLoadedReorderSession<
				{ rowId: string; label: string },
				{ id: string; label: string }
			>
		>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession({
				loadItems: () => loadGate.promise,
				mapItems: (items) => items.map((item) => ({ id: item.rowId, label: item.label })),
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError,
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			const entering = session.enter();
			// User cancels before loadItems() resolves
			session.exit();

			loadGate.resolve({
				items: [
					{ rowId: "a", label: "A" },
					{ rowId: "b", label: "B" },
				],
			});
			await entering;

			expect(session.reorderMode).toBe(false);
			expect(session.reorderItems).toEqual([]);
			expect(session.controller.pending).toEqual([]);
			expect(pushError).not.toHaveBeenCalled();
		} finally {
			dispose();
		}
	});

	it("assigns the exact actually-added IDs to highlightedIds when some conflict addedIds are unresolved", async () => {
		const pushInfo = vi.fn();
		const submitReorder = vi.fn(async () => undefined);

		let localSession!: ReturnType<typeof createLocalReorderSession<{ id: string; label: string }>>;
		let controller!: ReturnType<typeof createReorderController<{ id: string; label: string }>>;
		const dispose = createEffectRoot(() => {
			controller = createReorderController(
				[
					{ id: "a", label: "A" },
					{ id: "b", label: "B" },
				],
				submitReorder,
			);
			localSession = createLocalReorderSession({
				getController: () => controller,
				getLatestItems: () => [
					{ id: "a", label: "A" },
					{ id: "b", label: "B" },
					{ id: "real-2", label: "Real 2" },
				],
				entityLabel: "item",
				pushInfo,
				pushSuccess: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			localSession.enter();
			await localSession.handleError({
				status: 409,
				raw: {
					error: { message: "Items changed" },
					context: {
						// "missing-1" is not in latestItems; only "real-2" is resolved and merged.
						added_ids: ["missing-1", "real-2"],
						removed_ids: [],
					},
				},
			});

			expect(localSession.highlightedIds).toEqual(["real-2"]);
			expect(controller.pending.map((item) => item.id)).toEqual(["a", "b", "real-2"]);
		} finally {
			dispose();
		}
	});

	it("excludes IDs already present in controller.pending from addedIds and highlightedIds", async () => {
		let localSession!: ReturnType<typeof createLocalReorderSession<{ id: string; label: string }>>;
		let controller!: ReturnType<typeof createReorderController<{ id: string; label: string }>>;
		const dispose = createEffectRoot(() => {
			controller = createReorderController(
				[{ id: "already", label: "Already" }],
				vi.fn(async () => undefined),
			);
			localSession = createLocalReorderSession({
				getController: () => controller,
				getLatestItems: () => [
					{ id: "already", label: "Already" },
					{ id: "new-item", label: "New Item" },
				],
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushSuccess: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			localSession.enter();
			await localSession.handleError({
				status: 409,
				raw: {
					error: { message: "Items changed" },
					context: {
						added_ids: ["already", "new-item"],
						removed_ids: [],
					},
				},
			});

			expect(localSession.highlightedIds).toEqual(["new-item"]);
			expect(controller.pending.map((item) => item.id)).toEqual(["already", "new-item"]);
		} finally {
			dispose();
		}
	});

	it("creates a working controller outside $derived in createLoadedReorderSession", async () => {
		const submitReorder = vi.fn(async () => undefined);

		let session!: ReturnType<
			typeof createLoadedReorderSession<
				{ rowId: string; label: string },
				{ id: string; label: string }
			>
		>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession({
				loadItems: async () => ({
					items: [
						{ rowId: "a", label: "A" },
						{ rowId: "b", label: "B" },
					],
				}),
				mapItems: (items) => items.map((item) => ({ id: item.rowId, label: item.label })),
				submitReorder,
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await session.enter();
			expect(session.reorderMode).toBe(true);
			expect(session.controller.pending.map((item) => item.id)).toEqual(["a", "b"]);

			session.controller.move(0, 1);
			expect(session.controller.isDirty).toBe(true);
			await session.controller.submit();
			expect(submitReorder).toHaveBeenCalledWith(["b", "a"]);
		} finally {
			dispose();
		}
	});

	it("preserves reactive mapItems updates in createLoadedReorderSession after enter()", async () => {
		const labelBox = createReactiveBox("Old");

		let session!: ReturnType<
			typeof createLoadedReorderSession<{ rowId: string }, { id: string; label: string }>
		>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession({
				loadItems: async () => ({
					items: [{ rowId: "a" }],
				}),
				mapItems: (items) => items.map((item) => ({ id: item.rowId, label: labelBox.value })),
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await session.enter();
			expect(session.reorderItems).toEqual([{ id: "a", label: "Old" }]);

			labelBox.value = "New";
			expect(session.reorderItems).toEqual([{ id: "a", label: "New" }]);
		} finally {
			dispose();
		}
	});
});
