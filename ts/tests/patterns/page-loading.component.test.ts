// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import PageLoadingHarness from "../fixtures/PageLoadingHarness.svelte";

describe("components/PageLoading.svelte", () => {
  it("renders status role, message text, and default spinner sizing", () => {
    const view = render(PageLoadingHarness, {
      message: "Loading page...",
      size: "md"
    });

    const root = view.container.querySelector(".underlay-page-loading");
    const spinner = view.container.querySelector(".underlay-page-loading__spinner") as HTMLElement;
    expect(root?.getAttribute("role")).toBe("status");
    expect(root?.getAttribute("aria-live")).toBe("polite");
    expect(screen.getByText("Loading page...")).toBeTruthy();
    expect(spinner.getAttribute("style")).toContain("width: 2rem;");
    expect(spinner.getAttribute("style")).toContain("height: 2rem;");
  });

  it("supports alternate sizes and hides text when message is empty", () => {
    const view = render(PageLoadingHarness, {
      message: "",
      size: "lg"
    });

    const spinner = view.container.querySelector(".underlay-page-loading__spinner") as HTMLElement;
    expect(spinner.getAttribute("style")).toContain("width: 3rem;");
    expect(spinner.getAttribute("style")).toContain("height: 3rem;");
    expect(view.container.querySelector(".underlay-page-loading__text")).toBeNull();
  });
});
