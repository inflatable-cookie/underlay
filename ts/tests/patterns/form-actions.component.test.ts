// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FormActionsHarness from "../fixtures/FormActionsHarness.svelte";

describe("components/FormActions.svelte", () => {
	it("renders primary children and alignment class", () => {
		const view = render(FormActionsHarness, {
			align: "end",
			withDangerSlot: false,
			dangerItems: [],
		});

		expect(screen.getByTestId("primary-action")).toBeTruthy();
		expect(view.container.querySelector(".underlay-form-actions--end")).toBeTruthy();
	});

	it("renders full danger slot when provided", () => {
		const view = render(FormActionsHarness, {
			align: "start",
			withDangerSlot: true,
			dangerItems: [],
		});

		expect(screen.getByTestId("danger-action")).toBeTruthy();
		expect(view.container.querySelector(".underlay-form-actions__danger--full")).toBeTruthy();
		expect(view.container.querySelector(".underlay-form-actions__danger--collapsed")).toBeNull();
	});

	it("renders collapsed danger dropdown trigger when dangerItems are provided", () => {
		const onSelect = vi.fn();
		const view = render(FormActionsHarness, {
			withDangerSlot: false,
			dangerItems: [{ label: "Delete", onSelect }],
		});

		expect(view.container.querySelector(".underlay-form-actions__danger--collapsed")).toBeTruthy();
		expect(screen.getByRole("button", { name: "More actions" })).toBeTruthy();
	});
});
