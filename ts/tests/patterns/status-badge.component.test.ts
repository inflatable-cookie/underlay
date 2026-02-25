// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import StatusBadge from "../../src/components/StatusBadge.svelte";
import StatusBadgeHarness from "../fixtures/StatusBadgeHarness.svelte";

describe("components/StatusBadge.svelte", () => {
	it("renders success variant classes and label for true/false values", () => {
		const trueState = render(StatusBadge, {
			value: true,
			trueLabel: "Live",
			falseLabel: "Draft",
			variant: "success",
		});
		const trueBadge = trueState.container.querySelector(".underlay-status-badge");
		expect(screen.getByText("Live")).toBeTruthy();
		expect(trueBadge?.classList.contains("underlay-status-badge--true")).toBe(true);
		expect(trueBadge?.classList.contains("underlay-status-badge--danger")).toBe(false);
		trueState.unmount();

		const falseState = render(StatusBadge, {
			value: false,
			trueLabel: "Live",
			falseLabel: "Draft",
			variant: "success",
		});
		const falseBadge = falseState.container.querySelector(".underlay-status-badge");
		expect(screen.getByText("Draft")).toBeTruthy();
		expect(falseBadge?.classList.contains("underlay-status-badge--false")).toBe(true);
	});

	it("applies danger variant class for false state", () => {
		const view = render(StatusBadge, {
			value: false,
			trueLabel: "Allowed",
			falseLabel: "Blocked",
			variant: "danger",
		});
		const badge = view.container.querySelector(".underlay-status-badge");
		expect(screen.getByText("Blocked")).toBeTruthy();
		expect(badge?.classList.contains("underlay-status-badge--danger")).toBe(true);
	});

	it("renders true/false icon snippets according to current value", () => {
		const trueIcon = render(StatusBadgeHarness, { value: true, variant: "success" });
		expect(screen.getByTestId("status-icon-true")).toBeTruthy();
		expect(screen.queryByTestId("status-icon-false")).toBeNull();
		trueIcon.unmount();

		render(StatusBadgeHarness, { value: false, variant: "danger" });
		expect(screen.getByTestId("status-icon-false")).toBeTruthy();
		expect(screen.queryByTestId("status-icon-true")).toBeNull();
	});
});
