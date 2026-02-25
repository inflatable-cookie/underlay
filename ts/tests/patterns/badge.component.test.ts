// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import BadgeHarness from "../fixtures/BadgeHarness.svelte";

describe("components/Badge.svelte", () => {
	it("renders default variant/size with content", () => {
		const view = render(BadgeHarness, {
			label: "Default",
		});

		const badge = screen.getByTestId("badge-node");
		expect(badge.classList.contains("underlay-badge")).toBe(true);
		expect(badge.classList.contains("underlay-badge--default")).toBe(true);
		expect(badge.classList.contains("underlay-badge--md")).toBe(true);
		expect(badge.classList.contains("underlay-badge--pill")).toBe(false);
		expect(view.container.querySelector(".underlay-badge-icon")).toBeNull();
	});

	it("supports variant/size/pill/icon/className props", () => {
		const view = render(BadgeHarness, {
			label: "Warn",
			variant: "warning",
			size: "lg",
			pill: true,
			icon: "!",
			className: "extra-badge",
		});

		const badge = screen.getByTestId("badge-node");
		expect(badge.classList.contains("underlay-badge--warning")).toBe(true);
		expect(badge.classList.contains("underlay-badge--lg")).toBe(true);
		expect(badge.classList.contains("underlay-badge--pill")).toBe(true);
		expect(badge.classList.contains("extra-badge")).toBe(true);
		expect(view.container.querySelector(".underlay-badge-icon")?.textContent).toContain("!");
	});
});
