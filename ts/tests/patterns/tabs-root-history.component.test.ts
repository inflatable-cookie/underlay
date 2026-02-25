// @vitest-environment jsdom
import { beforeEach, describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import TabsRootHistoryHarness from "../fixtures/TabsRootHistoryHarness.svelte";

describe("components/TabsRoot.svelte history sync", () => {
	beforeEach(() => {
		window.history.replaceState({}, "", "/");
	});

	it("initializes active tab from URL query when present", async () => {
		window.history.replaceState({}, "", "/?tab=two");
		render(TabsRootHistoryHarness, {
			initialValue: "one",
			historyKey: "tab",
		});

		await waitFor(() => {
			expect(screen.getByTestId("history-active-tab").textContent).toBe("two");
		});
	});

	it("writes initial value to URL when query key is absent", async () => {
		render(TabsRootHistoryHarness, {
			initialValue: "one",
			historyKey: "tab",
		});

		await waitFor(() => {
			expect(new URL(window.location.href).searchParams.get("tab")).toBe("one");
		});
	});

	it("updates URL and state on tab clicks and popstate navigation", async () => {
		render(TabsRootHistoryHarness, {
			initialValue: "one",
			historyKey: "tab",
		});

		await fireEvent.click(screen.getByRole("tab", { name: "Details" }));
		await waitFor(() => {
			expect(screen.getByTestId("history-active-tab").textContent).toBe("two");
		});
		expect(new URL(window.location.href).searchParams.get("tab")).toBe("two");

		window.history.replaceState({}, "", "/?tab=one");
		window.dispatchEvent(new PopStateEvent("popstate"));
		await waitFor(() => {
			expect(screen.getByTestId("history-active-tab").textContent).toBe("one");
		});
	});
});
