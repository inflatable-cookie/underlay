// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import ButtonHarness from "../fixtures/ButtonHarness.svelte";

describe("components/Button.svelte", () => {
	it("renders default classes/content and forwards type", () => {
		const view = render(ButtonHarness, {
			label: "Save",
			type: "submit",
		});

		const button = screen.getByRole("button", { name: "Save" });
		expect(button.getAttribute("type")).toBe("submit");
		expect(button.classList.contains("underlay-button")).toBe(true);
		expect(button.classList.contains("underlay-button--primary")).toBe(true);
		expect(button.classList.contains("underlay-button--md")).toBe(true);
		expect(button.classList.contains("underlay-button--pill")).toBe(true);
		expect(view.container.querySelector(".underlay-button--square")).toBeNull();
	});

	it("supports variant/size/pill overrides and custom class", () => {
		render(ButtonHarness, {
			label: "Delete",
			variant: "danger-subtle",
			size: "icon-sm",
			pill: false,
			className: "extra",
		});

		const button = screen.getByRole("button", { name: "Delete" });
		expect(button.classList.contains("underlay-button--danger-subtle")).toBe(true);
		expect(button.classList.contains("underlay-button--icon-sm")).toBe(true);
		expect(button.classList.contains("underlay-button--square")).toBe(true);
		expect(button.classList.contains("extra")).toBe(true);
	});

	it("invokes click handler and respects disabled state", async () => {
		const onclick = vi.fn();
		const first = render(ButtonHarness, {
			label: "Run",
			onclick,
			disabled: false,
		});
		const activeButton = screen.getByRole("button", { name: "Run" });
		await fireEvent.click(activeButton);
		expect(onclick).toHaveBeenCalledTimes(1);
		first.unmount();

		const disabledClick = vi.fn();
		render(ButtonHarness, {
			label: "Blocked",
			onclick: disabledClick,
			disabled: true,
		});
		const disabledButton = screen.getByRole("button", { name: "Blocked" });
		expect(disabledButton.hasAttribute("disabled")).toBe(true);
		(disabledButton as HTMLButtonElement).click();
		expect(disabledClick).toHaveBeenCalledTimes(0);
	});
});
