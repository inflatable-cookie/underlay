// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RestoreResolutionShellHarness from "../fixtures/RestoreResolutionShellHarness.svelte";

describe("patterns/RestoreResolutionShell.svelte", () => {
  it("renders shared restore shell regions and close action", async () => {
    const onClose = vi.fn();

    render(RestoreResolutionShellHarness, { onClose });

    expect(screen.getByRole("heading", { name: "Resolve restore" })).toBeTruthy();
    expect(screen.getByText("Choose a new home for this item.")).toBeTruthy();
    expect(screen.getByText("Original parent: Section A")).toBeTruthy();
    expect(screen.getByText("Preview ready")).toBeTruthy();
    expect(screen.getByText("Planner body")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Apply" })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
