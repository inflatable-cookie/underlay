import { describe, expect, it, vi } from "vitest";

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
});
