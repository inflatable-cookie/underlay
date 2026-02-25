// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FormError from "../../src/components/FormError.svelte";

describe("components/FormError.svelte", () => {
	it("renders alert message when provided", () => {
		const view = render(FormError, {
			message: "Something went wrong",
		});

		const alert = screen.getByRole("alert");
		expect(alert.textContent).toContain("Something went wrong");
		expect(alert.getAttribute("aria-live")).toBe("polite");
		expect(view.container.querySelector(".underlay-form-error")).toBeTruthy();
	});

	it("renders nothing when message is null/empty", () => {
		const nullCase = render(FormError, { message: null });
		expect(nullCase.container.textContent?.trim()).toBe("");
		nullCase.unmount();

		const undefinedCase = render(FormError, { message: undefined });
		expect(undefinedCase.container.textContent?.trim()).toBe("");
	});
});
