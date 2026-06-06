import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import { createOptimisticValue } from "../../../src/patterns/optimistic";

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
      rollback();
    });

    it("supports custom equals function", () => {
      const value = createOptimisticValue(
        { x: 1, y: 2 },
        {
          equals: (a, b) => a.x === b.x && a.y === b.y,
        },
      );

      value.set({ x: 1, y: 2 });
      expect(get(value.pending)).toBe(false);

      value.set({ x: 1, y: 3 });
      expect(get(value.pending)).toBe(true);
    });
  });
});
