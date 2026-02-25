// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import OpsSection from "../../src/patterns/OpsSection.svelte";
import TestIcon from "../fixtures/TestIcon.svelte";

describe("patterns/OpsSection.svelte", () => {
	it("renders title and class without icon by default", () => {
		const view = render(OpsSection, {
			title: "Operations",
			class: "custom-section",
		});

		const section = view.container.querySelector("section");
		expect(section).toBeTruthy();
		expect(section?.className).toContain("underlay-ops-section");
		expect(section?.className).toContain("custom-section");
		expect(view.container.querySelector(".underlay-ops-section__title")?.textContent).toContain("Operations");
		expect(view.container.querySelector('[data-testid="test-icon"]')).toBeNull();
		expect(view.container.querySelector(".underlay-ops-section__controls")).toBeNull();
	});

	it("renders icon component when provided", () => {
		const view = render(OpsSection, {
			title: "Metrics",
			icon: TestIcon,
		});

		expect(view.container.querySelector(".underlay-ops-section__title")?.textContent).toContain("Metrics");
		expect(view.container.querySelector('[data-testid="test-icon"]')).toBeTruthy();
		expect(view.container.querySelector('[data-testid="test-icon"]')?.getAttribute("width")).toBe("16");
	});
});
