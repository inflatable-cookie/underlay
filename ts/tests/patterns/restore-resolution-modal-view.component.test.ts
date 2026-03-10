// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import RestoreResolutionModalViewHarness from "../fixtures/RestoreResolutionModalViewHarness.svelte";

describe("patterns/RestoreResolutionModalView.svelte", () => {
  it("renders ready content inside the shared restore dialog", () => {
    render(RestoreResolutionModalViewHarness, { mode: "ready" });

    expect(screen.getByRole("dialog")).toBeTruthy();
    expect(screen.getByTestId("restore-ready-content")).toBeTruthy();
  });

  it("renders blocked content inside the shared restore dialog", () => {
    render(RestoreResolutionModalViewHarness, { mode: "blocked" });

    expect(screen.getByRole("dialog")).toBeTruthy();
    expect(screen.getByTestId("restore-blocked-content")).toBeTruthy();
  });
});
