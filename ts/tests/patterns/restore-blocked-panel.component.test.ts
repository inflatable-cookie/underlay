// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import RestoreBlockedPanel from "../../src/patterns/RestoreBlockedPanel.svelte";

describe("patterns/RestoreBlockedPanel.svelte", () => {
  it("renders blocker summaries, conflicts, and hints", () => {
    render(RestoreBlockedPanel, {
      result: {
        status: "blocked",
        scope: "delete_batch",
        deleteBatchId: "batch-1",
        blockerKinds: ["conflict", "parent_state"],
        blockers: [
          {
            kind: "conflict",
            entity: {
              kind: "section",
              id: "section-1",
              displayName: "Section A",
            },
            message: null,
            fieldConflicts: [
              {
                fieldName: "label",
                candidateValue: "A",
                activeOccupant: {
                  kind: "section",
                  id: "section-2",
                  displayName: "Section A",
                },
                resolutionHints: ["Rename the active section or keep it deleted."],
              },
            ],
          },
        ],
      },
    });

    expect(screen.getByText("Restore blocked")).toBeTruthy();
    expect(screen.getByText("Restore blocked by 1 blocker.")).toBeTruthy();
    expect(screen.getAllByText('Label "A" is already used by Section: Section A.')).toHaveLength(2);
    expect(screen.getByText("Rename the active section or keep it deleted.")).toBeTruthy();
  });

  it("renders optional resolution action links from the resolver", () => {
    render(RestoreBlockedPanel, {
      result: {
        status: "blocked",
        scope: "delete_batch",
        deleteBatchId: "batch-1",
        blockerKinds: ["conflict"],
        blockers: [
          {
            kind: "conflict",
            entity: {
              kind: "section",
              id: "section-1",
              displayName: "Section A",
            },
            message: null,
            fieldConflicts: [
              {
                fieldName: "label",
                candidateValue: "A",
                activeOccupant: {
                  kind: "section",
                  id: "section-2",
                  displayName: "Section A",
                },
                resolutionHints: [],
              },
            ],
          },
        ],
      },
      getActions: ({ role, reference }) =>
        role === "active_occupant"
          ? [{ label: "Open active Section", href: `/learning/sections/${reference.id}` }]
          : [],
    });

    const link = screen.getByRole("link", { name: "Open active Section" });
    expect(link.getAttribute("href")).toBe("/learning/sections/section-2");
  });

  it("supports custom reference formatting for embedded domain labels", () => {
    render(RestoreBlockedPanel, {
      result: {
        status: "blocked",
        scope: "delete_batch",
        deleteBatchId: "batch-1",
        blockerKinds: ["conflict"],
        blockers: [
          {
            kind: "conflict",
            entity: {
              kind: "area",
              id: "area-1",
              displayName: "Area 1",
            },
            parent: {
              kind: "section",
              id: "section-b",
              displayName: "Section B",
            },
            message: null,
            fieldConflicts: [
              {
                fieldName: "number",
                candidateValue: "2",
                activeOccupant: {
                  kind: "area",
                  id: "area-2",
                  displayName: "Existing area",
                },
                resolutionHints: [],
              },
            ],
          },
        ],
      },
      formatReference: ({ reference, role, conflict }) => {
        if (reference.kind === "area" && (role === "entity" || role === "active_occupant")) {
          return `Area B${conflict?.candidateValue ?? "2"}: ${reference.displayName}`;
        }
        return null;
      },
    });

    expect(screen.getByText("Area B2: Area 1")).toBeTruthy();
    expect(screen.getAllByText('Number "2" is already used by Area B2: Existing area.')).toHaveLength(2);
  });

  it("supports embedded dialog rendering without its own inner surface", () => {
    const { container } = render(RestoreBlockedPanel, {
      embedded: true,
      result: {
        status: "blocked",
        scope: "delete_batch",
        deleteBatchId: "batch-1",
        blockerKinds: ["conflict"],
        blockers: [
          {
            kind: "conflict",
            entity: {
              kind: "section",
              id: "section-1",
              displayName: "Section A",
            },
            message: null,
            fieldConflicts: [],
          },
        ],
      },
    });

    expect(
      container.querySelector(".underlay-restore-blocked-panel__body--embedded")
    ).toBeTruthy();
  });
});
