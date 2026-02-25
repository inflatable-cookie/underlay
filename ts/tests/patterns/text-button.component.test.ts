// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import TextButtonHarness from "../fixtures/TextButtonHarness.svelte";

describe("components/TextButton.svelte", () => {
	it("renders button mode with type/variant/class and click handling", async () => {
		const onclick = vi.fn();
		render(TextButtonHarness, {
			label: "Delete",
			type: "submit",
			variant: "danger",
			className: "extra",
			onclick,
		});

		const button = screen.getByRole("button", { name: "Delete" });
		expect(button.getAttribute("type")).toBe("submit");
		expect(button.classList.contains("underlay-text-button")).toBe(true);
		expect(button.classList.contains("underlay-text-button--danger")).toBe(true);
		expect(button.classList.contains("extra")).toBe(true);

		await fireEvent.click(button);
		expect(onclick).toHaveBeenCalledTimes(1);
	});

	it("renders anchor mode with href and aria-disabled", () => {
		render(TextButtonHarness, {
			label: "Open",
			href: "/docs",
			variant: "success",
			disabled: true,
		});

		const link = screen.getByRole("link", { name: "Open" });
		expect(link.getAttribute("href")).toBe("/docs");
		expect(link.classList.contains("underlay-text-button--success")).toBe(true);
		expect(link.getAttribute("aria-disabled")).toBe("true");
	});

	it("does not fire click handler when disabled button is clicked via native click", () => {
		const onclick = vi.fn();
		render(TextButtonHarness, {
			label: "Blocked",
			disabled: true,
			onclick,
		});

		const button = screen.getByRole("button", { name: "Blocked" }) as HTMLButtonElement;
		expect(button.disabled).toBe(true);
		button.click();
		expect(onclick).toHaveBeenCalledTimes(0);
	});
});
