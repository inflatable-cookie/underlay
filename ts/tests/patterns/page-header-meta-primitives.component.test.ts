// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import PageHeaderMetaPrimitivesHarness from "../fixtures/PageHeaderMetaPrimitivesHarness.svelte";

describe("patterns/PageHeaderMeta*.svelte", () => {
	it("renders meta group and row wrappers", () => {
		const view = render(PageHeaderMetaPrimitivesHarness);
		expect(view.container.querySelector(".underlay-page-header__meta-group")).toBeTruthy();
		expect(view.container.querySelector(".underlay-page-header__meta-row")).toBeTruthy();
	});

	it("renders labelled and unlabelled meta items plus separator", () => {
		const view = render(PageHeaderMetaPrimitivesHarness, {
			label: "State",
			value: "Published",
		});

		expect(view.container.querySelectorAll(".underlay-page-header__meta-item").length).toBe(2);
		expect(screen.getByText("State")).toBeTruthy();
		expect(screen.getByTestId("meta-item-value").textContent).toContain("Published");
		expect(screen.getByTestId("meta-item-unlabelled").textContent).toContain("Unlabelled");
		expect(view.container.querySelector(".underlay-page-header__meta-separator")?.textContent).toContain("·");
	});
});
