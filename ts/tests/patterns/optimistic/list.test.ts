import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import { createOptimisticList } from "../../../src/patterns/optimistic";

interface Item {
  id: string;
  name: string;
}

describe("createOptimisticList", () => {
  describe("initialization", () => {
    it("starts with empty array by default", () => {
      const list = createOptimisticList<Item>();
      expect(get(list)).toEqual([]);
    });

    it("starts with initial items", () => {
      const list = createOptimisticList<Item>([
        { id: "1", name: "First" },
        { id: "2", name: "Second" },
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
        tempId: () => `custom-${++counter}`,
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
        { id: "2", name: "Second" },
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
        { id: "2", name: "Second" },
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
        { id: "3", name: "Third" },
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

    it("rollback appends removed item when original index is out of bounds", () => {
      const list = createOptimisticList<Item>([
        { id: "1", name: "First" },
        { id: "2", name: "Second" },
      ]);

      const { rollback } = list.remove("2");
      list.set([]);
      rollback();

      expect(get(list)).toEqual([{ id: "2", name: "Second" }]);
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

    it("handles update operations for non-existent IDs", () => {
      const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);
      const { confirm, rollback } = list.update("missing", { name: "Updated" });

      expect(get(list)).toEqual([{ id: "1", name: "Original" }]);

      confirm({ id: "missing", name: "From Server" } as any);
      expect(get(list)).toEqual([{ id: "1", name: "Original" }]);

      rollback();
      expect(get(list)).toEqual([{ id: "1", name: "Original" }]);
    });

    it("rollback preserves unrelated items while restoring updated item", () => {
      const list = createOptimisticList<Item>([
        { id: "1", name: "Original" },
        { id: "2", name: "Other" },
      ]);

      const { rollback } = list.update("1", { name: "Updated" });
      rollback();

      expect(get(list)).toEqual([
        { id: "1", name: "Original" },
        { id: "2", name: "Other" },
      ]);
    });
  });

  describe("set", () => {
    it("replaces entire list", () => {
      const list = createOptimisticList<Item>([{ id: "1", name: "Original" }]);

      list.set([
        { id: "a", name: "New A" },
        { id: "b", name: "New B" },
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
        { id: "2", name: "Second" },
      ]);

      list.add({ name: "New" });
      list.update("1", { name: "Updated" });

      const pending = get(list.pendingIds);
      expect(pending.size).toBe(2);
      expect(pending.has("1")).toBe(true);
    });
  });
});
