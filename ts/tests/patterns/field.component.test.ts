// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import FieldHarness from "../fixtures/FieldHarness.svelte";

describe("components/Field.svelte", () => {
	it("renders label/content and required marker with generated control id", () => {
		const view = render(FieldHarness, {
			label: "Email",
			required: true,
		});

		const label = screen.getByText("Email");
		expect(label.tagName.toLowerCase()).toBe("label");
		expect(view.container.querySelector(".underlay-field__required")?.textContent).toBe("*");
		expect(screen.getByTestId("field-input")).toBeTruthy();
		expect((label as HTMLLabelElement).htmlFor).toMatch(/^underlay-field-control-\d+$/);
	});

	it("uses explicit forId and renders error with linked id", () => {
		render(FieldHarness, {
			label: "Username",
			forId: "username-input",
			error: "Required",
		});

		const label = screen.getByText("Username") as HTMLLabelElement;
		expect(label.htmlFor).toBe("username-input");
		const error = screen.getByText("Required");
		expect(error.id).toBe("username-input-error");
	});

	it("renders hint trigger and span/wide styles", async () => {
		const view = render(FieldHarness, {
			label: "Bio",
			hint: "Provide additional details",
			span: "full",
			wide: true,
		});

		const root = view.container.querySelector(".underlay-field") as HTMLElement;
		expect(root.classList.contains("underlay-field--wide")).toBe(true);
		expect(root.getAttribute("style")).toContain("grid-column: 1 / -1;");

		const hintTrigger = screen.getByRole("button", { name: "Provide additional details" });
		expect(hintTrigger.textContent).toContain("?");
		await fireEvent.click(hintTrigger);
		expect(screen.getByRole("tooltip")).toBeTruthy();
	});
});
