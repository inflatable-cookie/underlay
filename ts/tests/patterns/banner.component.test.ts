// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import Banner from "../../src/patterns/Banner.svelte";

describe("patterns/Banner.svelte", () => {
	it("renders message, default warning variant, and icon markup", () => {
		const view = render(Banner, {
			message: "Heads up",
		});

		const root = view.container.querySelector(".underlay-banner");
		expect(root).toBeTruthy();
		expect(root?.classList.contains("underlay-banner--warning")).toBe(true);
		expect(root?.getAttribute("role")).toBe("status");
		expect(view.container.querySelector(".underlay-banner__message")?.textContent).toContain("Heads up");
		expect(view.container.querySelector("svg")).toBeTruthy();
		expect(view.container.querySelector("svg circle")).toBeTruthy();
	});

	it("supports explicit variants and renders variant-specific icon paths", () => {
		const errorView = render(Banner, {
			variant: "error",
			message: "Error",
		});
		expect(errorView.container.querySelector(".underlay-banner")?.classList.contains("underlay-banner--error")).toBe(true);
		expect(errorView.container.querySelectorAll("svg line").length).toBe(2);
		errorView.unmount();

		const infoView = render(Banner, {
			variant: "info",
			message: "Info",
		});
		expect(infoView.container.querySelector(".underlay-banner")?.classList.contains("underlay-banner--info")).toBe(true);
		expect(infoView.container.querySelectorAll("svg line").length).toBe(2);
		infoView.unmount();

		const successView = render(Banner, {
			variant: "success",
			message: "Success",
		});
		expect(successView.container.querySelector(".underlay-banner")?.classList.contains("underlay-banner--success")).toBe(true);
		expect(successView.container.querySelector("svg polyline")).toBeTruthy();
	});
});
