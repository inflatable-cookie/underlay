// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render } from "@testing-library/svelte";
import RelationSelectorPopoverSearch from "../../src/patterns/RelationSelector/RelationSelectorPopoverSearch.svelte";

describe("patterns/RelationSelector/RelationSelectorPopoverSearch.svelte", () => {
	it("renders input props and wires callbacks and input ref", async () => {
		const onInput = vi.fn();
		const onKeyDown = vi.fn();
		const onInputRef = vi.fn();

		const view = render(RelationSelectorPopoverSearch, {
			placeholder: "Search relations",
			value: "alpha",
			showLoading: false,
			onInput,
			onKeyDown,
			onInputRef,
		});

		const input = view.container.querySelector("input") as HTMLInputElement;
		expect(input).toBeTruthy();
		expect(input.placeholder).toBe("Search relations");
		expect(input.value).toBe("alpha");
		expect(input.getAttribute("aria-controls")).toBe("relation-selector-popover-list");
		expect(input.getAttribute("aria-autocomplete")).toBe("list");
		expect(onInputRef).toHaveBeenCalledWith(input);

		await fireEvent.input(input, { target: { value: "beta" } });
		expect(onInput).toHaveBeenCalledTimes(1);

		await fireEvent.keyDown(input, { key: "ArrowDown" });
		expect(onKeyDown).toHaveBeenCalledTimes(1);
	});

	it("shows loader icon only when showLoading is true", () => {
		const idle = render(RelationSelectorPopoverSearch, {
			placeholder: "Search",
			value: "",
			showLoading: false,
			onInput: vi.fn(),
			onKeyDown: vi.fn(),
			onInputRef: vi.fn(),
		});
		expect(idle.container.querySelector(".relation-selector-popover__search-loader")).toBeNull();
		idle.unmount();

		const busy = render(RelationSelectorPopoverSearch, {
			placeholder: "Search",
			value: "",
			showLoading: true,
			onInput: vi.fn(),
			onKeyDown: vi.fn(),
			onInputRef: vi.fn(),
		});
		expect(busy.container.querySelector(".relation-selector-popover__search-loader")).toBeTruthy();
	});
});
