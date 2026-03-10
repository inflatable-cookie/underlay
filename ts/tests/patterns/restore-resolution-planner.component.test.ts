/// <reference types="vitest" />
// @vitest-environment jsdom

import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import RestoreResolutionPlanner from "../../src/patterns/RestoreResolutionPlanner.svelte";
import { normalizeRestoreResolutionOrder } from "../../src/patterns/restore-resolution";
import {
  SHADOW_ITEM_MARKER_PROPERTY_NAME,
  SHADOW_PLACEHOLDER_ITEM_ID,
} from "svelte-dnd-action";

afterEach(() => {
  cleanup();
});

function normalize(text: string | null | undefined): string {
  return (text ?? "").replace(/\s+/g, " ").trim();
}

describe("patterns/RestoreResolutionPlanner.svelte", () => {
  it("renders muted previous labels and updates order after keyboard reorder", async () => {
    const items = [
      {
        id: "section-a",
        label: "Section A: Context",
        preview: {
          prefixText: "Section ",
          previousOrderText: null,
          currentOrderText: "A",
          title: "Context",
        },
      },
      {
        id: "restore:section-b",
        label: "Section B: Qualitative characteristics",
        preview: {
          prefixText: "Section ",
          previousOrderText: "B",
          currentOrderText: "C",
          title: "Qualitative characteristics",
        },
      },
      {
        id: "section-c",
        label: "Section C: Management information",
        preview: {
          prefixText: "Section ",
          previousOrderText: "C",
          currentOrderText: "B",
          title: "Management information",
        },
      },
    ];

    let value = ["section-a", "restore:section-b", "section-c"];

    const view = render(RestoreResolutionPlanner, {
      items,
      value,
      onChange: (next: string[]) => {
        value = next;
      },
    });

    let rows = screen.getAllByRole("listitem");
    expect(rows).toHaveLength(3);
    expect(rows[1].querySelector(".underlay-restore-resolution-planner__previous")?.textContent).toBe("B");
    expect(normalize(rows[1].textContent)).toContain("Section B C : Qualitative characteristics");

    const reorderButton = rows[1].querySelector("button");
    expect(reorderButton).not.toBeNull();
    reorderButton?.focus();
    await fireEvent.keyDown(reorderButton!, { key: " ", code: "Space", charCode: 32 });
    await fireEvent.keyDown(reorderButton!, { key: "ArrowDown", code: "ArrowDown" });
    await fireEvent.keyDown(reorderButton!, { key: " ", code: "Space", charCode: 32 });

    expect(value).toEqual(["section-a", "section-c", "restore:section-b"]);

    await view.rerender({
      items: [
        items[0],
        {
          ...items[2],
          preview: {
            prefixText: "Section ",
            previousOrderText: null,
            currentOrderText: "B",
            title: "Management information",
          },
        },
        items[1],
      ],
      value,
      onChange: (next: string[]) => {
        value = next;
      },
    });

    rows = screen.getAllByRole("listitem");
    expect(normalize(rows[2].textContent)).toContain("Section B C : Qualitative characteristics");
  });

  it("ignores dnd shadow placeholder items when finalizing pointer reorders", async () => {
    const items = [
      {
        id: "section-a",
        label: "Section A: Context",
      },
      {
        id: "restore:section-b",
        label: "Section B: Qualitative characteristics",
      },
      {
        id: "section-c",
        label: "Section C: Management information",
      },
    ];

    let value = ["section-a", "restore:section-b", "section-c"];

    render(RestoreResolutionPlanner, {
      items,
      value,
      onChange: (next: string[]) => {
        value = next;
      },
    });

    const list = screen.getByRole("list");
    await fireEvent(
      list,
      new CustomEvent("finalize", {
        detail: {
          items: [
            items[0],
            {
              id: SHADOW_PLACEHOLDER_ITEM_ID,
              label: "shadow",
              [SHADOW_ITEM_MARKER_PROPERTY_NAME]: true,
            },
            items[2],
            items[1],
          ],
        },
      })
    );

    expect(value).toEqual(["section-a", "section-c", "restore:section-b"]);
  });

  it("normalizes duplicate and unknown ids against the canonical order", () => {
    expect(
      normalizeRestoreResolutionOrder(
        ["section-c", "section-c", "id:dnd-shadow-placeholder-0000"],
        ["section-a", "restore:section-b", "section-c"],
      )
    ).toEqual(["section-c", "section-a", "restore:section-b"]);
  });
});
