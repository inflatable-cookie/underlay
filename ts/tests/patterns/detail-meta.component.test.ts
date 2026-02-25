// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import DetailMetaId from "../../src/patterns/DetailPageShell/DetailMetaId.svelte";
import DetailMetaStatus from "../../src/patterns/DetailPageShell/DetailMetaStatus.svelte";
import DetailMetaHarness from "../fixtures/DetailMetaHarness.svelte";

describe("patterns/DetailPageShell/DetailMeta*.svelte", () => {
	it("renders grouped meta row with separators and composed child components", () => {
		const view = render(DetailMetaHarness, {
			idValue: "user-42",
			idLabel: "Record",
			statusValue: false,
			trueLabel: "On",
			falseLabel: "Off",
			variant: "danger",
		});

		expect(view.container.querySelector(".underlay-page-header__meta-group")).toBeTruthy();
		expect(view.container.querySelector(".underlay-page-header__meta-row")).toBeTruthy();
		expect(screen.getByTestId("meta-first").textContent).toContain("First item");
		expect(view.container.querySelectorAll(".underlay-page-header__meta-separator").length).toBe(2);
		expect(screen.getByText("Record")).toBeTruthy();
		expect(screen.getByText("user-42")).toBeTruthy();
		expect(screen.getByText("Off")).toBeTruthy();
	});

	it("renders DetailMetaId defaults and custom label with copy affordance", () => {
		const defaultId = render(DetailMetaId, { value: "id-default" });
		expect(screen.getByText("ID")).toBeTruthy();
		expect(screen.getByText("id-default")).toBeTruthy();
		expect(screen.getByRole("button", { name: "Copy code" })).toBeTruthy();
		defaultId.unmount();

		render(DetailMetaId, { value: "id-custom", label: "Identifier" });
		expect(screen.getByText("Identifier")).toBeTruthy();
		expect(screen.getByText("id-custom")).toBeTruthy();
	});

	it("renders DetailMetaStatus labels and variant classes for true/false states", () => {
		const active = render(DetailMetaStatus, {
			value: true,
			trueLabel: "Active",
			falseLabel: "Inactive",
			variant: "success",
		});
		const activeBadge = active.container.querySelector(".underlay-status-badge");
		expect(screen.getByText("Active")).toBeTruthy();
		expect(activeBadge?.classList.contains("underlay-status-badge--true")).toBe(true);
		expect(activeBadge?.classList.contains("underlay-status-badge--danger")).toBe(false);
		active.unmount();

		const inactive = render(DetailMetaStatus, {
			value: false,
			trueLabel: "Active",
			falseLabel: "Inactive",
			variant: "danger",
		});
		const inactiveBadge = inactive.container.querySelector(".underlay-status-badge");
		expect(screen.getByText("Inactive")).toBeTruthy();
		expect(inactiveBadge?.classList.contains("underlay-status-badge--false")).toBe(true);
		expect(inactiveBadge?.classList.contains("underlay-status-badge--danger")).toBe(true);
	});
});
