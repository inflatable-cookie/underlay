// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import EmptyStateHarness from "../fixtures/EmptyStateHarness.svelte";

describe("components/EmptyState.svelte", () => {
  it("renders default copy and compact icon sizing", () => {
    render(EmptyStateHarness, {
      title: "No data",
      description: "Try a broader filter.",
      variant: "compact",
      withIcon: true
    });

    expect(screen.getByText("No data")).toBeTruthy();
    expect(screen.getByText("Try a broader filter.")).toBeTruthy();
    const icon = screen.getByTestId("test-icon");
    expect(icon.getAttribute("width")).toBe("24");
    expect(icon.getAttribute("height")).toBe("24");
  });

  it("renders link action when href is provided", () => {
    const view = render(EmptyStateHarness, {
      actionLabel: "Create",
      actionHref: "/create"
    });

    const link = view.container.querySelector(".underlay-empty-state__action a") as HTMLAnchorElement;
    expect(link).toBeTruthy();
    expect(link.getAttribute("href")).toBe("/create");
    expect(link.textContent).toContain("Create");
  });

  it("renders button action with callback", async () => {
    const onaction = vi.fn();
    const actionView = render(EmptyStateHarness, {
      actionLabel: "Retry",
      onaction
    });

    await fireEvent.click(screen.getByText("Retry"));
    expect(onaction).toHaveBeenCalledTimes(1);
    expect(actionView.container.querySelector(".underlay-empty-state__action button")).toBeTruthy();
  });

  it("supports children override", () => {
    const childrenView = render(EmptyStateHarness, {
      withChildren: true,
      title: "Ignored title"
    });

    expect(screen.getByTestId("empty-state-custom")).toBeTruthy();
    expect(childrenView.container.querySelector(".underlay-empty-state__title")).toBeNull();
  });
});
