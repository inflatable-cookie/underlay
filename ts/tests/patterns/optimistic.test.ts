/**
 * Tests for optimistic update utilities.
 */

import { describe, it, expect } from "vitest";
import { get } from "svelte/store";
import {
	createOptimisticList,
	createOptimisticToggle,
	createOptimisticValue,
	createOptimisticCounter
} from "../../src/patterns/optimistic";

// ============================================================================
// createOptimisticList
// ============================================================================

describe("createOptimisticList", () => {
	interface Item {
		id: string;
		name: string;
	}

	describe("initialization", () => {
		it("starts with empty array by default", () => {
			const list = createOptimisticList<Item>();
			expect(get(list)).toEqual([]);
		});

		it("starts with initial items", () => {
			const list = createOptimisticList<Item>([
				{ id: "1", name: "First" },
				{ id: "2", name: "Second" }
			]);
			expect(get(list)).toHaveLength(2);
			expect(get(list)[0].name).toBe("First");
		});
	});

	describe("add", () => {
		it("adds item optimistically with temp ID", () => {
			const list = createOptimisticList<Item>([]);
			list.add({ name: "New Item" });

			expect(get(list)).toHaveLength(1);
			expect(get(list)[0].name).toBe("New Item");
			expect(get(list)[0].id).toMatch(/^temp-/);
		});

		it("marks item as pending", () => {
			const list = createOptimisticList<Item>([]);
			list.add({ name: "New Item" });

			const isPending = get(list.isPending);
			expect(isPending(get(list)[0].id)).toBe(true);
		});

		it("confirm replaces temp item with real data", () => {
			const list = createOptimisticList<Item>([]);
			const { confirm } = list.add({ name: "New Item" });

			confirm({ id: "real-123", name: "Real Item" });

			expect(get(list)).toHaveLength(1);
			expect(get(list)[0].id).toBe("real-123");
			expect(get(list)[0].name).toBe("Real Item");
		});

		it("confirm clears pending state", () => {
			const list = createOptimisticList<Item>([]);
			const { confirm } = list.add({ name: "New Item" });
			const tempId = get(list)[0].id;

			confirm({ id: "real-123", name: "Real Item" });

			const isPending = get(list.isPending);
			expect(isPending(tempId)).toBe(false);
			expect(isPending("real-123")).toBe(false);
		});

		it("rollback removes the optimistic item", () => {
			const list = createOptimisticList<Item>([]);
			const { rollback } = list.add({ name: "New Item" });

			expect(get(list)).toHaveLength(1);

			rollback();

			expect(get(list)).toHaveLength(0);
		});

		it("rollback clears pending state", () => {
			const list = createOptimisticList<Item>([]);
			const { rollback } = list.add({ name: "New Item" });
			const tempId = get(list)[0].id;

			rollback();

			const isPending = get(list.isPending);
			expect(isPending(tempId)).toBe(false);
		});

		it("supports custom tempId generator", () => {
			let counter = 0;
			const list = createOptimisticList<Item>([], {
				tempId: () => `custom-${++counter}`
			});

			list.add({ name: "First" });
			list.add({ name: "Second" });

			expect(get(list)[0].id).toBe("custom-1");
			expect(get(list)[1].id).toBe("custom-2");
		});
	});

	describe("remove", () => {
		it("removes item optimistically", () => {
			const list = createOptimisticList<Item>([
				{ id: "1", name: "First" },
				{ id: "2", name: "Second" }
			]);

			list.remove("1");

			expect(get(list)).toHaveLength(1);
			expect(get(list)[0].id).toBe("2");
		});

		it("marks removed item as pending", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "First" }]);

			list.remove("1");

			const isPending = get(list.isPending);
			expect(isPending("1")).toBe(true);
		});

		it("confirm clears pending state", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "First" }]);

			const { confirm } = list.remove("1");
			confirm();

			const isPending = get(list.isPending);
			expect(isPending("1")).toBe(false);
		});

		it("rollback restores the removed item", () => {
			const list = createOptimisticList<Item>([
				{ id: "1", name: "First" },
				{ id: "2", name: "Second" }
			]);

			const { rollback } = list.remove("1");

			expect(get(list)).toHaveLength(1);

			rollback();

			expect(get(list)).toHaveLength(2);
			expect(get(list).find((i) => i.id === "1")).toBeDefined();
		});

		it("rollback restores item at original position", () => {
			const list = createOptimisticList<Item>([
				{ id: "1", name: "First" },
				{ id: "2", name: "Second" },
				{ id: "3", name: "Third" }
			]);

			const { rollback } = list.remove("2");

			expect(get(list)).toHaveLength(2);

			rollback();

			expect(get(list)).toHaveLength(3);
			expect(get(list)[1].id).toBe("2");
		});

		it("handles removing non-existent item", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "First" }]);

			const { rollback } = list.remove("non-existent");

			expect(get(list)).toHaveLength(1);

			rollback();

			expect(get(list)).toHaveLength(1);
		});
	});

	describe("update", () => {
		it("updates item optimistically", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			list.update("1", { name: "Updated" });

			expect(get(list)[0].name).toBe("Updated");
		});

		it("marks updated item as pending", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			list.update("1", { name: "Updated" });

			const isPending = get(list.isPending);
			expect(isPending("1")).toBe(true);
		});

		it("confirm with real data replaces item", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			const { confirm } = list.update("1", { name: "Optimistic" });
			confirm({ id: "1", name: "From Server" });

			expect(get(list)[0].name).toBe("From Server");
		});

		it("confirm without data keeps optimistic changes", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			const { confirm } = list.update("1", { name: "Updated" });
			confirm();

			expect(get(list)[0].name).toBe("Updated");
			const isPending = get(list.isPending);
			expect(isPending("1")).toBe(false);
		});

		it("rollback restores original values", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			const { rollback } = list.update("1", { name: "Updated" });

			expect(get(list)[0].name).toBe("Updated");

			rollback();

			expect(get(list)[0].name).toBe("Original");
		});
	});

	describe("set", () => {
		it("replaces entire list", () => {
			const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

			list.set([
				{ id: "a", name: "New A" },
				{ id: "b", name: "New B" }
			]);

			expect(get(list)).toHaveLength(2);
			expect(get(list)[0].id).toBe("a");
		});

		it("clears pending state", () => {
			const list = createOptimisticList<Item>([]);
			list.add({ name: "Pending Item" });
			const tempId = get(list)[0].id;

			list.set([{ id: "fresh", name: "Fresh" }]);

			const isPending = get(list.isPending);
			expect(isPending(tempId)).toBe(false);
			expect(get(list.pendingIds).size).toBe(0);
		});
	});

	describe("pendingIds", () => {
		it("tracks all pending IDs", () => {
			const list = createOptimisticList<Item>([
				{ id: "1", name: "First" },
				{ id: "2", name: "Second" }
			]);

			list.add({ name: "New" });
			list.update("1", { name: "Updated" });

			const pending = get(list.pendingIds);
			expect(pending.size).toBe(2);
			expect(pending.has("1")).toBe(true);
		});
	});
});

// ============================================================================
// createOptimisticToggle
// ============================================================================

describe("createOptimisticToggle", () => {
	describe("initialization", () => {
		it("starts with false by default", () => {
			const toggle = createOptimisticToggle();
			expect(get(toggle)).toBe(false);
		});

		it("starts with initial value", () => {
			const toggle = createOptimisticToggle(true);
			expect(get(toggle)).toBe(true);
		});

		it("starts with no pending operation", () => {
			const toggle = createOptimisticToggle();
			expect(get(toggle.pending)).toBe(false);
		});
	});

	describe("toggle", () => {
		it("toggles value optimistically", () => {
			const toggle = createOptimisticToggle(false);
			toggle.toggle();

			expect(get(toggle)).toBe(true);
		});

		it("sets pending state", () => {
			const toggle = createOptimisticToggle(false);
			toggle.toggle();

			expect(get(toggle.pending)).toBe(true);
		});

		it("confirm clears pending state", () => {
			const toggle = createOptimisticToggle(false);
			const { confirm } = toggle.toggle();

			confirm();

			expect(get(toggle)).toBe(true);
			expect(get(toggle.pending)).toBe(false);
		});

		it("rollback restores previous value", () => {
			const toggle = createOptimisticToggle(false);
			const { rollback } = toggle.toggle();

			expect(get(toggle)).toBe(true);

			rollback();

			expect(get(toggle)).toBe(false);
			expect(get(toggle.pending)).toBe(false);
		});
	});

	describe("set", () => {
		it("sets value optimistically", () => {
			const toggle = createOptimisticToggle(false);
			toggle.set(true);

			expect(get(toggle)).toBe(true);
		});

		it("no-op when setting same value", () => {
			const toggle = createOptimisticToggle(true);
			const { rollback } = toggle.set(true);

			expect(get(toggle.pending)).toBe(false);

			rollback(); // Should be no-op
			expect(get(toggle)).toBe(true);
		});

		it("rollback restores previous value", () => {
			const toggle = createOptimisticToggle(false);
			const { rollback } = toggle.set(true);

			rollback();

			expect(get(toggle)).toBe(false);
		});

		it("confirm clears pending state after set", () => {
			const toggle = createOptimisticToggle(false);
			const { confirm } = toggle.set(true);
			expect(get(toggle.pending)).toBe(true);

			confirm();

			expect(get(toggle)).toBe(true);
			expect(get(toggle.pending)).toBe(false);
		});
	});
});

// ============================================================================
// createOptimisticValue
// ============================================================================

describe("createOptimisticValue", () => {
	describe("initialization", () => {
		it("starts with initial value", () => {
			const value = createOptimisticValue("initial");
			expect(get(value)).toBe("initial");
		});

		it("starts with no pending operation", () => {
			const value = createOptimisticValue("initial");
			expect(get(value.pending)).toBe(false);
		});
	});

	describe("set", () => {
		it("sets value optimistically", () => {
			const value = createOptimisticValue("initial");
			value.set("updated");

			expect(get(value)).toBe("updated");
			expect(get(value.pending)).toBe(true);
		});

		it("confirm clears pending state", () => {
			const value = createOptimisticValue("initial");
			const { confirm } = value.set("updated");

			confirm();

			expect(get(value)).toBe("updated");
			expect(get(value.pending)).toBe(false);
		});

		it("confirm with real value updates to server value", () => {
			const value = createOptimisticValue("initial");
			const { confirm } = value.set("optimistic");

			confirm("from-server");

			expect(get(value)).toBe("from-server");
		});

		it("rollback restores previous value", () => {
			const value = createOptimisticValue("initial");
			const { rollback } = value.set("updated");

			rollback();

			expect(get(value)).toBe("initial");
			expect(get(value.pending)).toBe(false);
		});

		it("no-op when setting same value", () => {
			const value = createOptimisticValue("same");
			const { rollback } = value.set("same");

			expect(get(value.pending)).toBe(false);
			rollback(); // Should be no-op
		});

		it("supports custom equals function", () => {
			const value = createOptimisticValue(
				{ x: 1, y: 2 },
				{
					equals: (a, b) => a.x === b.x && a.y === b.y
				}
			);

			// Same value by equals
			value.set({ x: 1, y: 2 });
			expect(get(value.pending)).toBe(false);

			// Different value
			value.set({ x: 1, y: 3 });
			expect(get(value.pending)).toBe(true);
		});
	});
});

// ============================================================================
// createOptimisticCounter
// ============================================================================

describe("createOptimisticCounter", () => {
	describe("initialization", () => {
		it("starts with 0 by default", () => {
			const counter = createOptimisticCounter();
			expect(get(counter)).toBe(0);
		});

		it("starts with initial value", () => {
			const counter = createOptimisticCounter(42);
			expect(get(counter)).toBe(42);
		});
	});

	describe("increment", () => {
		it("increments by 1 by default", () => {
			const counter = createOptimisticCounter(5);
			counter.increment();

			expect(get(counter)).toBe(6);
		});

		it("increments by custom amount", () => {
			const counter = createOptimisticCounter(5);
			counter.increment(10);

			expect(get(counter)).toBe(15);
		});

		it("sets pending state", () => {
			const counter = createOptimisticCounter(5);
			counter.increment();

			expect(get(counter.pending)).toBe(true);
		});

		it("rollback restores previous value", () => {
			const counter = createOptimisticCounter(5);
			const { rollback } = counter.increment(10);

			expect(get(counter)).toBe(15);

			rollback();

			expect(get(counter)).toBe(5);
		});

		it("confirm clears pending state", () => {
			const counter = createOptimisticCounter(5);
			const { confirm } = counter.increment(2);
			expect(get(counter.pending)).toBe(true);

			confirm();

			expect(get(counter)).toBe(7);
			expect(get(counter.pending)).toBe(false);
		});
	});

	describe("decrement", () => {
		it("decrements by 1 by default", () => {
			const counter = createOptimisticCounter(5);
			counter.decrement();

			expect(get(counter)).toBe(4);
		});

		it("decrements by custom amount", () => {
			const counter = createOptimisticCounter(15);
			counter.decrement(10);

			expect(get(counter)).toBe(5);
		});

		it("rollback restores previous value", () => {
			const counter = createOptimisticCounter(5);
			const { rollback } = counter.decrement(3);

			expect(get(counter)).toBe(2);

			rollback();

			expect(get(counter)).toBe(5);
		});
	});

	describe("set", () => {
		it("sets value optimistically", () => {
			const counter = createOptimisticCounter(5);
			counter.set(100);

			expect(get(counter)).toBe(100);
		});

		it("confirm with server value updates count", () => {
			const counter = createOptimisticCounter(5);
			const { confirm } = counter.set(10);

			confirm(12); // Server says actual count is 12

			expect(get(counter)).toBe(12);
		});

		it("confirm without server value keeps optimistic count", () => {
			const counter = createOptimisticCounter(5);
			const { confirm } = counter.set(10);

			confirm();

			expect(get(counter)).toBe(10);
			expect(get(counter.pending)).toBe(false);
		});

		it("no-op when setting same value", () => {
			const counter = createOptimisticCounter(5);
			counter.set(5);

			expect(get(counter.pending)).toBe(false);
		});
	});
});
