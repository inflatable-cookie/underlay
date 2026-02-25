// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import DetailPageShellHarness from "../fixtures/DetailPageShellHarness.svelte";

describe("patterns/DetailPageShell/DetailPageShell.svelte", () => {
	it("renders non-tab child content with meta/actions snippets and custom class", () => {
		const view = render(DetailPageShellHarness, {
			className: "custom-shell",
			useMeta: true,
			useActions: true,
			useChildren: true,
			useTabs: false,
		});

		expect(view.container.querySelector(".underlay-detail-page")?.classList.contains("custom-shell")).toBe(true);
		expect(screen.getByTestId("detail-shell-meta")).toBeTruthy();
		expect(screen.getByTestId("detail-shell-action")).toBeTruthy();
		expect(screen.getByTestId("detail-shell-children").textContent).toContain("Children content");
	});

	it("renders tabs, defaults active tab, and lazy-mounts tab content after tab switch", async () => {
		render(DetailPageShellHarness, {
			useTabs: true,
			useChildren: false,
		});

		expect(screen.getByTestId("detail-shell-active-tab").textContent).toBe("overview");
		expect(screen.getByTestId("tab-content-overview")).toBeTruthy();
		expect(screen.queryByTestId("tab-content-audit")).toBeNull();
		expect(screen.getAllByRole("separator").length).toBeGreaterThan(0);

		await fireEvent.click(screen.getByText("Audit"));
		await waitFor(() => {
			expect(screen.getByTestId("detail-shell-active-tab").textContent).toBe("audit");
		});

		expect(screen.getByTestId("tab-content-audit")).toBeTruthy();
		expect(screen.getByTestId("tab-content-overview")).toBeTruthy();
	});
});
