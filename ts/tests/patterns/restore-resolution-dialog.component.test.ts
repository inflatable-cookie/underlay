// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import RestoreResolutionDialogHarness from "../fixtures/RestoreResolutionDialogHarness.svelte";

describe("patterns/RestoreResolutionDialog.svelte", () => {
  it("renders restore content inside the shared Underlay dialog surface", () => {
    render(RestoreResolutionDialogHarness);

    expect(screen.getByRole("dialog")).toBeTruthy();
    expect(screen.getByTestId("restore-resolution-dialog-body").textContent).toContain("Restore planner content");
    expect(document.querySelector(".underlay-dialog-content")?.classList.contains("underlay-restore-resolution-dialog")).toBe(true);
  });
});
