// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import RelationSelectorPopoverCreateAction from "../../src/patterns/RelationSelector/RelationSelectorPopoverCreateAction.svelte";

describe("patterns/RelationSelector/RelationSelectorPopoverCreateAction.svelte", () => {
	it("renders action button when create is allowed and create form exists", async () => {
		const onCreate = vi.fn();
		const view = render(RelationSelectorPopoverCreateAction, {
			allowCreate: true,
			hasCreateForm: true,
			createLabel: "Add relation",
			onCreate,
		});

		expect(view.container.querySelector(".relation-selector-popover__create")).toBeTruthy();
		const button = screen.getByRole("button", { name: "Add relation" });
		await fireEvent.click(button);
		expect(onCreate).toHaveBeenCalledTimes(1);
	});

	it("does not render action when disallowed or create form is unavailable", () => {
		const disallowed = render(RelationSelectorPopoverCreateAction, {
			allowCreate: false,
			hasCreateForm: true,
			createLabel: "Add relation",
			onCreate: vi.fn(),
		});
		expect(disallowed.container.querySelector(".relation-selector-popover__create")).toBeNull();
		disallowed.unmount();

		const noForm = render(RelationSelectorPopoverCreateAction, {
			allowCreate: true,
			hasCreateForm: false,
			createLabel: "Add relation",
			onCreate: vi.fn(),
		});
		expect(noForm.container.querySelector(".relation-selector-popover__create")).toBeNull();
	});
});
