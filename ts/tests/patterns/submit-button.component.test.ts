// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import SubmitButton from "../../src/patterns/SubmitButton.svelte";

describe("patterns/SubmitButton.svelte", () => {
	it("renders default submit label and base attributes", () => {
		const view = render(SubmitButton);
		const button = view.container.querySelector("button");

		expect(button).toBeTruthy();
		expect(button?.getAttribute("type")).toBe("submit");
		expect(button?.textContent).toContain("Submit");
		expect(button?.hasAttribute("disabled")).toBe(false);
		expect(button?.getAttribute("aria-busy")).toBe("false");
	});

	it("shows submitting state with spinner text and disables interactions", () => {
		const view = render(SubmitButton, {
			submitting: true,
			submittingText: "Saving changes...",
			variant: "secondary",
			class: "extra-class",
		});
		const button = view.container.querySelector("button");

		expect(button?.hasAttribute("disabled")).toBe(true);
		expect(button?.getAttribute("aria-busy")).toBe("true");
		expect(button?.className).toContain("extra-class");
		expect(button?.className).toContain("underlay-button--secondary");
		expect(view.container.querySelector(".underlay-submit-button__spinner")).toBeTruthy();
		expect(button?.textContent).toContain("Saving changes...");
	});

	it("honors disabled flag even when not submitting", () => {
		const view = render(SubmitButton, { disabled: true, submitting: false });
		const button = view.container.querySelector("button");
		expect(button?.hasAttribute("disabled")).toBe(true);
		expect(button?.getAttribute("aria-busy")).toBe("false");
	});
});
