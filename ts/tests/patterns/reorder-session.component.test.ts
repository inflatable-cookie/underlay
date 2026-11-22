// @vitest-environment jsdom
import { flushSync } from "svelte";
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

	it("preserves reactive mapItems updates in createLoadedReorderSession and active controller after enter()", async () => {
		const labelBox = createReactiveBox("Old");

		let session!: ReturnType<
			typeof createLoadedReorderSession<{ rowId: string }, { id: string; label: string }>
		>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession({
				loadItems: async () => ({
					items: [{ rowId: "a" }, { rowId: "b" }],
				}),
				mapItems: (items) =>
					items.map((item) => ({ id: item.rowId, label: `${item.rowId}:${labelBox.value}` })),
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await session.enter();
			expect(session.reorderItems).toEqual([
				{ id: "a", label: "a:Old" },
				{ id: "b", label: "b:Old" },
			]);

			// Reorder pending items before updating the reactive label
			session.controller.move(0, 1);
			expect(session.controller.pending).toEqual([
				{ id: "b", label: "b:Old" },
				{ id: "a", label: "a:Old" },
			]);

			labelBox.value = "New";
			flushSync();
			expect(session.reorderItems).toEqual([
				{ id: "a", label: "a:New" },
				{ id: "b", label: "b:New" },
			]);
			// Active controller retains pending ID order [b, a] while reflecting updated labels
			expect(session.controller.pending).toEqual([
				{ id: "b", label: "b:New" },
				{ id: "a", label: "a:New" },
			]);
			expect(session.controller.original).toEqual([
				{ id: "a", label: "a:New" },
				{ id: "b", label: "b:New" },
			]);
			expect(session.controller.isDirty).toBe(true);
		} finally {
			dispose();
		}
	});

	it("preserves in-place pending mutations and caller-supplied row flags in createLoadedReorderSession", async () => {
		const submitReorder = vi.fn(async () => undefined);
		const labelBox = createReactiveBox("Old");

		type Row = { id: string; label: string; isDndShadowItem?: boolean };
		let session!: ReturnType<typeof createLoadedReorderSession<{ rowId: string }, Row>>;
		const dispose = createEffectRoot(() => {
			session = createLoadedReorderSession<{ rowId: string }, Row>({
				loadItems: async () => ({
					items: [{ rowId: "a" }, { rowId: "b" }],
				}),
				mapItems: (items) =>
					items.map((item) => ({ id: item.rowId, label: `${item.rowId}:${labelBox.value}` })),
				submitReorder,
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await session.enter();

			// Caller-supplied row flags via updatePending must remain visible
			session.controller.updatePending([
				{ id: "a", label: "a:Old", isDndShadowItem: true },
				{ id: "b", label: "b:Old" },
			]);
			expect(session.controller.pending[0].isDndShadowItem).toBe(true);

			// Reactive mapper update preserves the caller-supplied flag while updating label
			labelBox.value = "New";
			flushSync();
			expect(session.controller.pending[0]).toEqual({
				id: "a",
				label: "a:New",
				isDndShadowItem: true,
			});

			// In-place pending array mutations (e.g. reverse()) mark controller dirty and submit
			session.controller.pending.reverse();
			expect(session.controller.isDirty).toBe(true);
			await session.controller.submit();
			expect(submitReorder).toHaveBeenCalledWith(["b", "a"]);
		} finally {
			dispose();
		}
	});

	it("preserves accessor-backed IDs, class methods, and drops omitted mapped fields on sync", async () => {
		class MappedRow {
			readonly #id: string;
			readonly label: string;

			constructor(id: string, label: string) {
				this.#id = id;
				this.label = label;
			}

			get id(): string {
				return this.#id;
			}

			format(): string {
				return `${this.#id}:${this.label}`;
			}
		}

		const labelBox = createReactiveBox("Old");

		let classSession!: ReturnType<typeof createLoadedReorderSession<{ rowId: string }, MappedRow>>;
		const disposeClass = createEffectRoot(() => {
			classSession = createLoadedReorderSession({
				loadItems: async () => ({
					items: [{ rowId: "a" }, { rowId: "b" }],
				}),
				mapItems: (items) => items.map((item) => new MappedRow(item.rowId, labelBox.value)),
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await classSession.enter();
			flushSync();

			// First effect flush must preserve accessor-backed id, class prototype, and methods
			expect(classSession.controller.pending[0]).toBeInstanceOf(MappedRow);
			expect(classSession.controller.pending[0].id).toBe("a");
			expect(classSession.controller.pending[0].format()).toBe("a:Old");

			// Mutate extra caller metadata in-place onto the class-backed row
			(
				classSession.controller.pending[0] as MappedRow & { isDndShadowItem?: boolean }
			).isDndShadowItem = true;

			labelBox.value = "New";
			flushSync();

			// Class instance with private #id + in-place caller metadata survives sync and submits
			expect(classSession.controller.pending[0]).toBeInstanceOf(MappedRow);
			expect(classSession.controller.pending[0].id).toBe("a");
			expect(classSession.controller.pending[0].format()).toBe("a:New");
			expect(
				(classSession.controller.pending[0] as MappedRow & { isDndShadowItem?: boolean })
					.isDndShadowItem,
			).toBe(true);
			expect("isDndShadowItem" in classSession.controller.original[0]).toBe(false);

			classSession.controller.move(0, 1);
			await expect(classSession.controller.submit()).resolves.toBeUndefined();
		} finally {
			disposeClass();
		}

		type PlainRow = { id: string; label: string; subtitle?: string; isDndShadowItem?: boolean };
		const includeSubtitle = createReactiveBox(true);
		const plainLabel = createReactiveBox("Old");
		let plainSession!: ReturnType<typeof createLoadedReorderSession<{ rowId: string }, PlainRow>>;
		const disposePlain = createEffectRoot(() => {
			plainSession = createLoadedReorderSession<{ rowId: string }, PlainRow>({
				loadItems: async () => ({
					items: [{ rowId: "a" }],
				}),
				mapItems: (items) =>
					items.map((item) =>
						includeSubtitle.value
							? { id: item.rowId, label: plainLabel.value, subtitle: "Temp subtitle" }
							: { id: item.rowId, label: plainLabel.value },
					),
				submitReorder: vi.fn(async () => undefined),
				entityLabel: "item",
				pushInfo: vi.fn(),
				pushError: vi.fn(),
				onRefresh: vi.fn(async () => undefined),
			});
		});

		try {
			await plainSession.enter();
			flushSync();
			expect(plainSession.controller.pending[0].subtitle).toBe("Temp subtitle");

			// Add a caller-supplied flag via updatePending
			plainSession.controller.updatePending([
				{ id: "a", label: "Old", subtitle: "Temp subtitle", isDndShadowItem: true },
			]);

			// Omitting subtitle on reactive update drops subtitle while retaining isDndShadowItem
			includeSubtitle.value = false;
			plainLabel.value = "New";
			flushSync();

			expect(plainSession.controller.pending[0]).toEqual({
				id: "a",
				label: "New",
				isDndShadowItem: true,
			});
			expect("subtitle" in plainSession.controller.pending[0]).toBe(false);
			expect("subtitle" in plainSession.controller.original[0]).toBe(false);
		} finally {
			disposePlain();
		}
	});

	it("allows direct createLoadedReorderSession construction outside a component or effect root with reactive mapping", async () => {
		const submitReorder = vi.fn(async () => undefined);
		const labelBox = createReactiveBox("Old");
		const session = createLoadedReorderSession({
			loadItems: async () => ({
				items: [{ rowId: "a" }, { rowId: "b" }],
			}),
			mapItems: (items) =>
				items.map((item) => ({ id: item.rowId, label: `${item.rowId}:${labelBox.value}` })),
			submitReorder,
			entityLabel: "item",
			pushInfo: vi.fn(),
			pushError: vi.fn(),
			onRefresh: vi.fn(async () => undefined),
		});

		await session.enter();
		expect(session.reorderMode).toBe(true);
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:Old" },
			{ id: "b", label: "b:Old" },
		]);

		labelBox.value = "New";
		flushSync();
		expect(session.reorderItems).toEqual([
			{ id: "a", label: "a:New" },
			{ id: "b", label: "b:New" },
		]);
		expect(session.controller.pending).toEqual([
			{ id: "a", label: "a:New" },
			{ id: "b", label: "b:New" },
		]);
		expect(session.controller.original).toEqual([
			{ id: "a", label: "a:New" },
			{ id: "b", label: "b:New" },
		]);

		session.controller.move(0, 1);
		await session.controller.submit();
		expect(submitReorder).toHaveBeenCalledWith(["b", "a"]);
	});
});
