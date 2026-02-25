// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render } from "@testing-library/svelte";
import RelationPickerSearch from "../../src/patterns/relation-picker/RelationPickerSearch.svelte";

describe("patterns/relation-picker/RelationPickerSearch.svelte", () => {
	it("renders input props and wires input/keydown/ref callbacks", async () => {
		const onInput = vi.fn();
		const onKeyDown = vi.fn();
		const onInputRef = vi.fn();

		const view = render(RelationPickerSearch, {
			placeholder: "Search relations",
			value: "alpha",
			searching: false,
			onInput,
			onKeyDown,
			onInputRef,
		});

		const input = view.container.querySelector("input") as HTMLInputElement;
		expect(input).toBeTruthy();
		expect(input.placeholder).toBe("Search relations");
		expect(input.value).toBe("alpha");
		expect(input.getAttribute("aria-controls")).toBe("relation-picker-list");
		expect(input.getAttribute("aria-autocomplete")).toBe("list");
		expect(onInputRef).toHaveBeenCalledWith(input);

		await fireEvent.input(input, { target: { value: "beta" } });
		expect(onInput).toHaveBeenCalledTimes(1);

		await fireEvent.keyDown(input, { key: "ArrowDown" });
		expect(onKeyDown).toHaveBeenCalledTimes(1);
	});

	it("shows loader icon only when searching", () => {
		const idle = render(RelationPickerSearch, {
			placeholder: "Search",
			value: "",
			searching: false,
			onInput: vi.fn(),
			onKeyDown: vi.fn(),
			onInputRef: vi.fn(),
		});
		expect(idle.container.querySelector(".relation-picker-dialog__search-loader")).toBeNull();
		idle.unmount();

		const busy = render(RelationPickerSearch, {
			placeholder: "Search",
			value: "",
			searching: true,
			onInput: vi.fn(),
			onKeyDown: vi.fn(),
			onInputRef: vi.fn(),
		});
		expect(busy.container.querySelector(".relation-picker-dialog__search-loader")).toBeTruthy();
	});
});
