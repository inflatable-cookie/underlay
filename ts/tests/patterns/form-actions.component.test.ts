// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FormActionsHarness from "../fixtures/FormActionsHarness.svelte";

describe("components/FormActions.svelte", () => {
	it("renders primary children and alignment class", () => {
		const view = render(FormActionsHarness);

		expect(screen.getByTestId("primary-action")).toBeTruthy();
		expect(view.container.querySelector(".underlay-action-area--end")).toBeTruthy();
	});

	it("renders full danger slot when provided", () => {
		const view = render(FormActionsHarness);

		expect(screen.getByTestId("danger-action")).toBeTruthy();
		expect(view.container.querySelector(".underlay-form-actions__danger--full")).toBeTruthy();
	});

	it("renders collapsed danger dropdown trigger when dangerItems are provided", () => {
		const view = render(FormActionsHarness);

		expect(view.container.querySelector(".underlay-form-actions__danger--collapsed")).toBeTruthy();
		expect(screen.getByRole("button", { name: "More actions" })).toBeTruthy();
	});
});
