import { describe, expect, it } from "vitest";
import {
  formatRestoreBlockedHeadline,
  formatRestoreBlockerSummary,
  formatRestoreFieldConflict,
  formatRestoreReference,
} from "../../src/patterns/restore-blocked";

describe("patterns/restore-blocked", () => {
  it("formats references with display names", () => {
    expect(
      formatRestoreReference({
        kind: "section",
        id: "section-1",
        displayName: "Section A",
      }),
    ).toBe("Section: Section A");
  });

  it("formats field conflicts with active occupants", () => {
    expect(
      formatRestoreFieldConflict({
        fieldName: "label",
        candidateValue: "A",
        activeOccupant: {
          kind: "section",
          id: "section-2",
          displayName: "Section A",
        },
        resolutionHints: [],
      }),
    ).toBe('Label "A" is already used by Section: Section A.');
  });

  it("formats blocker summaries and headlines", () => {
    expect(
      formatRestoreBlockerSummary({
        kind: "parent_state",
        entity: { kind: "outcome", id: "outcome-1", displayName: "Outcome B" },
        parent: { kind: "area", id: "area-1", displayName: "Area 1" },
        parentState: "deleted",
        message: null,
        fieldConflicts: [],
      }),
    ).toContain("depends on Area: Area 1");

    expect(
      formatRestoreBlockedHeadline({
        status: "blocked",
        scope: "delete_batch",
        deleteBatchId: "batch-1",
        blockerKinds: ["conflict"],
        blockers: [
          {
            kind: "conflict",
            entity: { kind: "section", id: "section-1", displayName: "Section A" },
            message: null,
            fieldConflicts: [],
          },
        ],
      }),
    ).toBe("Restore blocked by 1 blocker.");
  });

  it("uses a formatter override when provided", () => {
    expect(
      formatRestoreBlockerSummary(
        {
          kind: "conflict",
          entity: { kind: "area", id: "area-1", displayName: "Area 1" },
          parent: { kind: "section", id: "section-b", displayName: "Section B" },
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
        ({ reference, role, conflict }) => {
          if (reference.kind === "area" && (role === "entity" || role === "active_occupant")) {
            return `Area B${conflict?.candidateValue ?? "2"}: ${reference.displayName}`;
          }
          return null;
        },
      ),
    ).toBe('Number "2" is already used by Area B2: Existing area.');
  });
});
