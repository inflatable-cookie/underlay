// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import TabsComponentsHarness from "../fixtures/TabsComponentsHarness.svelte";

describe("components/Tabs*.svelte", () => {
	it("renders list/trigger/content/separator classes for variant+size", () => {
		const view = render(TabsComponentsHarness, {
			variant: "boxed",
			size: "sm",
			withSeparator: true,
		});

		expect(view.container.querySelector(".underlay-tabs-list--boxed")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-list--sm")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-trigger--boxed")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-trigger--sm")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-content--boxed")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-separator--boxed")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-separator--sm")).toBeTruthy();
		expect(view.container.querySelector(".underlay-tabs-trigger__count")?.textContent).toContain("2");
	});

	it("switches active tab via trigger clicks and updates visible tabpanel state", async () => {
		const view = render(TabsComponentsHarness, {
			variant: "underline",
			size: "default",
			initialValue: "one",
		});

		expect(screen.getByTestId("active-tab").textContent).toBe("one");
		const tabPanels = view.container.querySelectorAll('[role="tabpanel"]');
		expect(tabPanels.length).toBe(2);
		expect(tabPanels[0]?.hasAttribute("hidden")).toBe(false);
		expect(tabPanels[1]?.hasAttribute("hidden")).toBe(true);

		await fireEvent.click(screen.getByRole("tab", { name: "Details" }));
		await waitFor(() => {
			expect(screen.getByTestId("active-tab").textContent).toBe("two");
		});

		expect(tabPanels[0]?.hasAttribute("hidden")).toBe(true);
		expect(tabPanels[1]?.hasAttribute("hidden")).toBe(false);
		expect(screen.getByTestId("content-two")).toBeTruthy();
	});

	it("renders form-registry validation dots for invalid/incomplete sections", () => {
		render(TabsComponentsHarness, {
			variant: "form",
			stateOne: "invalid",
			stateTwo: "incomplete",
		});

		expect(screen.getByLabelText("Has validation errors")).toBeTruthy();
		expect(screen.getByLabelText("Has required fields")).toBeTruthy();
	});

	it("renders collapsible container path when collapsible mode is enabled", () => {
		const originalResizeObserver = globalThis.ResizeObserver;
		class ResizeObserverMock {
			observe() {
				return undefined;
			}
			disconnect() {
				return undefined;
			}
		}
		// @ts-expect-error test shim
		globalThis.ResizeObserver = ResizeObserverMock;

		try {
			const view = render(TabsComponentsHarness, {
				collapsible: true,
				withSeparator: false,
			});

			expect(view.container.querySelector(".underlay-tabs-list-container")).toBeTruthy();
		} finally {
			globalThis.ResizeObserver = originalResizeObserver;
		}
	});
});
