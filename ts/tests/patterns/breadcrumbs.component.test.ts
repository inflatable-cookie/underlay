// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import BreadcrumbsHarness from "../fixtures/BreadcrumbsHarness.svelte";

describe("components/Breadcrumbs.svelte", () => {
  it("renders links/current item and passthrough attributes when not collapsed", () => {
    const view = render(BreadcrumbsHarness, {
      items: [
        { label: "Home", href: "/" },
        { label: "Library", href: "/library" },
        { label: "Detail" }
      ],
      collapseOnMobile: false,
      separator: "/",
      className: "custom-breadcrumbs",
      dataTestId: "breadcrumbs-nav"
    });

    const nav = screen.getByTestId("breadcrumbs-nav");
    expect(nav.classList.contains("custom-breadcrumbs")).toBe(true);

    const links = view.container.querySelectorAll(".underlay-breadcrumb-link");
    const current = view.container.querySelector(".underlay-breadcrumb-current");
    const separators = view.container.querySelectorAll(".underlay-breadcrumb-separator");
    expect(links.length).toBe(2);
    expect(current?.textContent).toContain("Detail");
    expect(current?.getAttribute("aria-current")).toBe("page");
    expect(separators.length).toBe(2);
    expect(view.container.querySelectorAll(".underlay-breadcrumb-separator")[0]?.textContent).toContain("/");
  });

  it("collapses middle items when threshold is exceeded", () => {
    const view = render(BreadcrumbsHarness, {
      items: [
        { label: "Home", href: "/" },
        { label: "Workspace", href: "/workspace" },
        { label: "Library", href: "/library" },
        { label: "Books", href: "/books" },
        { label: "Detail" }
      ],
      collapseOnMobile: true,
      maxItems: 4
    });

    const list = view.container.querySelector(".underlay-breadcrumb-list") as HTMLElement;
    const ellipsis = view.container.querySelector(".underlay-breadcrumb-ellipsis");
    const collapsedItem = view.container.querySelector(".underlay-breadcrumb-item.underlay-collapsed");

    expect(list.classList.contains("underlay-collapsible")).toBe(true);
    expect(ellipsis?.textContent).toContain("…");
    expect(collapsedItem).toBeTruthy();
    expect(screen.getByText("Home")).toBeTruthy();
    expect(screen.getByText("Books")).toBeTruthy();
    expect(screen.getByText("Detail")).toBeTruthy();
    expect(screen.queryByText("Workspace")).toBeNull();
  });
});
