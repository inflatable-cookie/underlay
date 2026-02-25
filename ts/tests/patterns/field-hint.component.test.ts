// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import FieldHint from "../../src/components/FieldHint.svelte";

describe("components/FieldHint.svelte", () => {
	it("renders hint trigger with content as aria label", () => {
		const view = render(FieldHint, {
			content: "Helpful guidance",
		});

		const trigger = screen.getByRole("button", { name: "Helpful guidance" });
		expect(trigger.textContent).toContain("?");
		expect(trigger.className).toContain("underlay-field-hint__trigger");
		expect(view.container.querySelector(".underlay-popover-trigger")).toBeTruthy();
	});

	it("opens popover tooltip content on click", async () => {
		render(FieldHint, {
			content: "Tooltip text",
			side: "right",
			sideOffset: 10,
			align: "start",
			alignOffset: 2,
			avoidCollisions: false,
			collisionPadding: 4,
		});

		await fireEvent.click(screen.getByRole("button", { name: "Tooltip text" }));
		expect(screen.getByRole("tooltip").textContent).toContain("Tooltip text");
	});
});
