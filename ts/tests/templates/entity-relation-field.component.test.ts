// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";

import EntityRelationFieldHarness from "./EntityRelationFieldHarness.svelte";

describe("templates/EntityRelationField", () => {
	it("does not mark the hidden input required and wires the field id/describedBy to the control group", async () => {
		render(EntityRelationFieldHarness, {
			id: "relation-field",
			label: "Related item",
			required: true,
			error: "Related item is required",
		});

		const hidden = document.querySelector<HTMLInputElement>(
			'input[type="hidden"][name="relation-field"]',
		);
		expect(hidden).not.toBeNull();
		expect(hidden?.hasAttribute("required")).toBe(false);

		const control = document.getElementById("relation-field");
		expect(control).not.toBeNull();
		expect(control?.getAttribute("role")).toBe("group");

		const describedBy = control?.getAttribute("aria-describedby") ?? "";
		expect(describedBy).toContain("relation-field-error");

		const errorNode = document.getElementById("relation-field-error");
		expect(errorNode?.textContent?.trim()).toBe("Related item is required");
		expect(await screen.findByText("Select related item")).toBeTruthy();
	});

	it("leaves describedBy unset when there is no error or description", () => {
		render(EntityRelationFieldHarness, { id: "clean-field", label: "Related item" });

		const control = document.getElementById("clean-field");
		expect(control).not.toBeNull();
		expect(control?.hasAttribute("aria-describedby")).toBe(false);
	});

	it("keeps the hidden input value in sync with the selected relation", () => {
		render(EntityRelationFieldHarness, {
			id: "value-field",
			label: "Related item",
			value: "item-2",
		});

		const hidden = document.querySelector<HTMLInputElement>(
			'input[type="hidden"][name="value-field"]',
		);
		expect(hidden?.value).toBe("item-2");
	});
});
