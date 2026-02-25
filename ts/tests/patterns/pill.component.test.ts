// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import PillHarness from "../fixtures/PillHarness.svelte";

describe("components/Pill.svelte", () => {
	it("renders default pill class and content", () => {
		const view = render(PillHarness, {
			label: "Draft",
		});

		const pill = view.container.querySelector(".underlay-pill");
		expect(pill).toBeTruthy();
		expect(screen.getByText("Draft")).toBeTruthy();
		expect(pill?.classList.contains("underlay-pill--accent")).toBe(false);
	});

	it("applies accent class/style and custom className when accent provided", () => {
		const view = render(PillHarness, {
			label: "Active",
			accent: "#22c55e",
			className: "extra-pill",
		});

		const pill = view.container.querySelector(".underlay-pill") as HTMLElement;
		expect(pill.classList.contains("underlay-pill--accent")).toBe(true);
		expect(pill.classList.contains("extra-pill")).toBe(true);
		expect(pill.getAttribute("style")).toContain("--pill-accent");
	});
});
