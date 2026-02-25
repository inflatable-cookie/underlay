// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import IconButtonHarness from "../fixtures/IconButtonHarness.svelte";

describe("components/IconButton.svelte", () => {
	it("renders label/variant/size/style/class and icon snippet", () => {
		render(IconButtonHarness, {
			label: "Remove item",
			variant: "danger",
			sizeRem: 2,
			className: "extra",
			withIcon: true,
		});

		const button = screen.getByRole("button", { name: "Remove item" });
		expect(button.classList.contains("underlay-icon-button")).toBe(true);
		expect(button.classList.contains("underlay-icon-button--danger")).toBe(true);
		expect(button.classList.contains("extra")).toBe(true);
		expect(button.getAttribute("style")).toContain("--underlay-icon-button-size: 2rem;");
		expect(screen.getByTestId("icon-content")).toBeTruthy();
	});

	it("forwards click handler in enabled mode", async () => {
		const onclick = vi.fn();
		render(IconButtonHarness, {
			label: "Run",
			onclick,
			disabled: false,
		});

		await fireEvent.click(screen.getByRole("button", { name: "Run" }));
		expect(onclick).toHaveBeenCalledTimes(1);
	});

	it("respects disabled state and renders no child when withIcon=false", () => {
		const onclick = vi.fn();
		render(IconButtonHarness, {
			label: "Disabled",
			onclick,
			disabled: true,
			withIcon: false,
		});

		const button = screen.getByRole("button", { name: "Disabled" }) as HTMLButtonElement;
		expect(button.disabled).toBe(true);
		button.click();
		expect(onclick).toHaveBeenCalledTimes(0);
		expect(screen.queryByTestId("icon-content")).toBeNull();
	});
});
