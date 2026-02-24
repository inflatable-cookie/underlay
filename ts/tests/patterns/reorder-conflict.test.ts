import { describe, expect, it } from "vitest";

import {
  applyReorderConflict,
  extractReorderConflict,
  type ReorderConflictDetails
} from "../../src/patterns/reorder-conflict";
import type { ReorderController } from "../../src/patterns/reorder-controller.svelte";

interface Item {
  id: string;
  label: string;
}

function createController(initial: Item[]) {
  let pending = [...initial];
  return {
    get pending() {
      return pending;
    },
    set pending(value: Item[]) {
      pending = value;
    },
    mergeNewItems(items: Item[]) {
      const existing = new Set(pending.map((item) => item.id));
      pending = [...pending, ...items.filter((item) => !existing.has(item.id))];
    },
    removeItems(idsToRemove: string[]) {
      const removeSet = new Set(idsToRemove);
      pending = pending.filter((item) => !removeSet.has(item.id));
    }
  };
}

describe("extractReorderConflict", () => {
  it("extracts conflict details from raw.context", () => {
    const conflict = extractReorderConflict({
      status: 409,
      message: "Conflict",
      raw: {
        error: { message: "Items changed" },
        context: {
          added_ids: ["new-a"],
          removed_ids: ["old-z"]
        }
      }
    });

    expect(conflict).toEqual({
      addedIds: ["new-a"],
      removedIds: ["old-z"],
      message: "Items changed"
    });
  });

  it("returns null for non-conflict errors", () => {
    const conflict = extractReorderConflict({
      status: 400,
      message: "Bad request",
      raw: {}
    });
    expect(conflict).toBeNull();
  });

  it("returns null when 409 has no usable context or empty context arrays", () => {
    expect(
      extractReorderConflict({
        status: 409,
        message: "Conflict",
        raw: { error: { context: { added_ids: [], removed_ids: [] } } }
      })
    ).toBeNull();

    expect(
      extractReorderConflict({
        status: 409,
        message: "Conflict",
        raw: { error: { details: "not-an-object" } }
      })
    ).toBeNull();
  });
});

describe("applyReorderConflict", () => {
  it("removes deleted items and appends added items", () => {
    const controller = createController([
      { id: "a", label: "A" },
      { id: "b", label: "B" },
      { id: "c", label: "C" }
    ]);

    const conflict: ReorderConflictDetails = {
      addedIds: ["x"],
      removedIds: ["b"],
      message: "Items changed"
    };

    const resolution = applyReorderConflict(
      controller as unknown as ReorderController<Item>,
      conflict,
      [
        { id: "a", label: "A" },
        { id: "c", label: "C" },
        { id: "x", label: "X" }
      ]
    );

    expect(controller.pending.map((item) => item.id)).toEqual(["a", "c", "x"]);
    expect(resolution).toEqual({
      addedCount: 1,
      removedCount: 1,
      unresolvedAddedIds: []
    });
  });

  it("reports unresolved IDs missing from latest snapshot", () => {
    const controller = createController([{ id: "a", label: "A" }]);

    const resolution = applyReorderConflict(
      controller as unknown as ReorderController<Item>,
      {
        addedIds: ["missing"],
        removedIds: [],
        message: "Items changed"
      },
      [{ id: "a", label: "A" }]
    );

    expect(controller.pending.map((item) => item.id)).toEqual(["a"]);
    expect(resolution).toEqual({
      addedCount: 0,
      removedCount: 0,
      unresolvedAddedIds: ["missing"]
    });
  });
});
