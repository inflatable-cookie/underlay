import { flushSync } from "svelte";
import { describe, expect, it, vi } from "vitest";
import { useBatchActions } from "../../src/patterns/batch-actions.svelte";
import { createEffectRoot } from "./effect-root.svelte";

describe("patterns/batch-actions.svelte.ts", () => {
	it("handles selection operations and action registration lifecycle", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		expect(batch.count).toBe(0);
		expect(batch.hasSelection).toBe(false);

		batch.select("a");
		batch.select("a");
		batch.toggle("b", true);
		batch.toggle("b", false);
		batch.deselect("missing");
		batch.deselect("a");
		batch.selectAll(["x", "y"]);

		expect(batch.selectedIds).toEqual(["x", "y"]);
		expect(batch.count).toBe(2);
		expect(batch.hasSelection).toBe(true);
		expect(batch.isSelected("x")).toBe(true);

		batch.clear();
		expect(batch.selectedIds).toEqual([]);
		expect(batch.count).toBe(0);

		batch.set(["k"]);
		expect(batch.selectedIds).toEqual(["k"]);

		const noopAction = {
			id: "noop",
			label: "Noop",
			execute: vi.fn(async () => ({ success: true, affected: 1 })),
		};
		const replacementAction = {
			id: "noop",
			label: "Replaced",
			execute: vi.fn(async () => ({ success: true, affected: 1 })),
		};

		batch.registerAction(noopAction);
		batch.registerAction(replacementAction);
		expect(batch.actions).toHaveLength(1);
		expect(batch.actions[0]?.label).toBe("Replaced");

		batch.unregisterAction("noop");
		expect(batch.actions).toHaveLength(0);
	});

	it("executes available actions and handles confirm, errors, and unavailable actions", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		const denyAction = {
			id: "deny",
			label: "Deny",
			isAvailable: () => false,
			execute: vi.fn(async () => ({ success: true, affected: 1 })),
		};
		const confirmAction = {
			id: "confirm",
			label: "Confirm",
			confirm: {
				title: "Confirm action",
				description: (count: number) => `Selected ${count}`,
			},
			execute: vi.fn(async (ids: string[]) => ({ success: true, affected: ids.length })),
		};
		const failureAction = {
			id: "failure",
			label: "Failure",
			execute: vi.fn(async () => ({ success: false, affected: 0, message: "No-op failure" })),
		};
		const throwAction = {
			id: "throw",
			label: "Throw",
			execute: vi.fn(async () => {
				throw new Error("boom");
			}),
		};

		batch.registerAction(denyAction);
		batch.registerAction(confirmAction);
		batch.registerAction(failureAction);
		batch.registerAction(throwAction);
		batch.selectAll(["a", "b"]);

		expect(batch.availableActions.map((a) => a.id)).toEqual(["confirm", "failure", "throw"]);
		expect(await batch.requestAction("missing")).toBeNull();
		expect(await batch.requestAction("deny")).toBeNull();

		const pending = await batch.requestAction("confirm");
		expect(pending).toBeNull();
		expect(batch.pendingAction?.id).toBe("confirm");
		expect(batch.getConfirmDescription()).toBe("Selected 2");
		const confirmed = await batch.confirmPendingAction();
		expect(confirmed).toEqual({ success: true, affected: 2 });
		expect(batch.pendingAction).toBeNull();
		expect(batch.count).toBe(0);
		expect(confirmAction.execute).toHaveBeenCalledTimes(1);

		batch.select("z");
		const failureResult = await batch.requestAction("failure");
		expect(failureResult).toEqual({ success: false, affected: 0, message: "No-op failure" });
		expect(batch.error).toBe("No-op failure");
		expect(batch.executing).toBe(false);

		const thrownResult = await batch.requestAction("throw");
		expect(thrownResult).toEqual({ success: false, affected: 0, message: "boom" });
		expect(batch.error).toBe("boom");
		expect(batch.executing).toBe(false);

		batch.clearError();
		expect(batch.error).toBeNull();

		batch.cancelPendingAction();
		expect(batch.pendingAction).toBeNull();
		expect(batch.getConfirmDescription()).toBe("");
	});

	it("returns null when confirming with no pending action or executing with no selection", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		expect(await batch.confirmPendingAction()).toBeNull();

		const action = {
			id: "run",
			label: "Run",
			execute: vi.fn(async (ids: string[]) => ({ success: true, affected: ids.length })),
		};
		batch.registerAction(action);
		expect(await batch.requestAction("run")).toBeNull();
		expect(action.execute).not.toHaveBeenCalled();
	});

	it("ignores a second execution while one is in flight", async () => {
		vi.resetModules();
		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		let release: () => void = () => {};
		const slow = {
			id: "slow",
			label: "Slow",
			execute: vi.fn(
				() =>
					new Promise<{ success: boolean; affected: number }>((resolve) => {
						release = () => resolve({ success: true, affected: 1 });
					}),
			),
		};
		batch.registerAction(slow);
		batch.selectAll(["a"]);

		const first = batch.requestAction("slow");
		expect(batch.executing).toBe(true);
		expect(await batch.requestAction("slow")).toBeNull();
		expect(slow.execute).toHaveBeenCalledTimes(1);

		release();
		await expect(first).resolves.toEqual({ success: true, affected: 1 });
		expect(batch.executing).toBe(false);
	});

	it("reports a default error message when a failed result has no message", async () => {
		vi.resetModules();
		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		batch.registerAction({
			id: "silent",
			label: "Archive",
			execute: async () => ({ success: false, affected: 0 }),
		});
		batch.selectAll(["a"]);

		await batch.requestAction("silent");
		expect(batch.error).toBe("Archive failed");
	});

	it("clears pendingAction on clear() and unregisterAction(), and re-checks isAvailable on confirm", async () => {
		vi.resetModules();
		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		const execute = vi.fn(async (ids: string[]) => ({ success: true, affected: ids.length }));
		batch.registerAction({
			id: "delete",
			label: "Delete",
			isAvailable: (ids) => ids.includes("a"),
			confirm: { title: "Delete", description: "Delete selected?" },
			execute,
		});

		batch.selectAll(["a", "b"]);
		await batch.requestAction("delete");
		expect(batch.pendingAction?.id).toBe("delete");

		batch.clear();
		expect(batch.pendingAction).toBeNull();

		batch.selectAll(["a", "b"]);
		await batch.requestAction("delete");
		expect(batch.pendingAction?.id).toBe("delete");

		batch.unregisterAction("delete");
		expect(batch.pendingAction).toBeNull();

		batch.registerAction({
			id: "delete",
			label: "Delete",
			isAvailable: (ids) => ids.includes("a"),
			confirm: { title: "Delete", description: "Delete selected?" },
			execute,
		});
		batch.selectAll(["a", "b"]);
		await batch.requestAction("delete");
		expect(batch.pendingAction?.id).toBe("delete");

		// Deselect "a" so isAvailable becomes false before confirming.
		batch.deselect("a");
		expect(await batch.confirmPendingAction()).toBeNull();
		expect(batch.pendingAction).toBeNull();
		expect(execute).not.toHaveBeenCalled();
	});

	it("blocks opening a confirmation dialog via requestAction while another action is executing", async () => {
		vi.resetModules();
		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		let release!: () => void;
		batch.registerAction({
			id: "slow",
			label: "Slow",
			execute: () =>
				new Promise((resolve) => {
					release = () => resolve({ success: true, affected: 1 });
				}),
		});
		batch.registerAction({
			id: "confirmable",
			label: "Confirmable",
			confirm: { title: "Confirm", description: "Proceed?" },
			execute: vi.fn(async () => ({ success: true, affected: 1 })),
		});

		batch.selectAll(["a"]);
		const inFlight = batch.requestAction("slow");
		expect(batch.executing).toBe(true);

		expect(await batch.requestAction("confirmable")).toBeNull();
		expect(batch.pendingAction).toBeNull();

		release();
		await expect(inFlight).resolves.toEqual({ success: true, affected: 1 });
		expect(batch.executing).toBe(false);
		expect(batch.pendingAction).toBeNull();
	});

	it("removes only the executed IDs on success and preserves mid-flight selections", async () => {
		vi.resetModules();
		const { useBatchActions } = await import("../../src/patterns/batch-actions.svelte");
		const batch = useBatchActions<string>();

		let release!: () => void;
		const execute = vi.fn(
			() =>
				new Promise<{ success: boolean; affected: number }>((resolve) => {
					release = () => resolve({ success: true, affected: 2 });
				}),
		);
		batch.registerAction({
			id: "archive",
			label: "Archive",
			execute,
		});

		batch.selectAll(["a", "b"]);
		const inFlight = batch.requestAction("archive");
		expect(execute).toHaveBeenCalledWith(["a", "b"]);

		// User selects "c" and deselects "b" while "archive" is in flight.
		batch.select("c");
		batch.deselect("b");

		release();
		await expect(inFlight).resolves.toEqual({ success: true, affected: 2 });
		expect(batch.selectedIds).toEqual(["c"]);
		expect(batch.count).toBe(1);
	});

	it("ignores post-disposal state mutations when unmounted mid-execution", async () => {
		let resolveSuccess!: () => void;
		let rejectFailure!: (reason: Error) => void;
		const successBatchRef: { current?: ReturnType<typeof useBatchActions<string>> } = {};
		const failureBatchRef: { current?: ReturnType<typeof useBatchActions<string>> } = {};

		const disposeSuccess = createEffectRoot(() => {
			successBatchRef.current = useBatchActions<string>();
		});
		const disposeFailure = createEffectRoot(() => {
			failureBatchRef.current = useBatchActions<string>();
		});
		flushSync();

		const successBatch = successBatchRef.current!;
		successBatch.registerAction({
			id: "run",
			label: "Run",
			execute: () =>
				new Promise((resolve) => {
					resolveSuccess = () => resolve({ success: true, affected: 1 });
				}),
		});
		successBatch.selectAll(["a"]);

		const pendingSuccess = successBatch.requestAction("run");
		expect(successBatch.executing).toBe(true);

		disposeSuccess();
		flushSync();

		resolveSuccess();
		await pendingSuccess;

		// Disposed instance must not mutate selection or executing after unmount.
		expect(successBatch.selectedIds).toEqual(["a"]);
		expect(successBatch.executing).toBe(true);
		successBatch.select("b");
		expect(successBatch.selectedIds).toEqual(["a"]);

		const failureBatch = failureBatchRef.current!;
		failureBatch.registerAction({
			id: "fail",
			label: "Fail",
			execute: () =>
				new Promise((_, reject) => {
					rejectFailure = reject;
				}),
		});
		failureBatch.selectAll(["x"]);

		const pendingFailure = failureBatch.requestAction("fail");
		disposeFailure();
		flushSync();

		rejectFailure(new Error("late failure"));
		await pendingFailure;

		expect(failureBatch.error).toBeNull();
		expect(failureBatch.executing).toBe(true);
	});
});
