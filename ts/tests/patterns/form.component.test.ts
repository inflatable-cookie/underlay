// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import FormHarness from "../fixtures/FormHarness.svelte";

describe("components/Form.svelte", () => {
	it("renders form attributes and children", () => {
		const view = render(FormHarness, {
			method: "get",
			className: "custom-form",
			autocomplete: "off",
		});

		const form = screen.getByTestId("form-root") as HTMLFormElement;
		expect(form.getAttribute("method")).toBe("get");
		expect(form.getAttribute("autocomplete")).toBe("off");
		expect(form.classList.contains("custom-form")).toBe(true);
		expect(screen.getByTestId("form-input")).toBeTruthy();
		expect(view.container.querySelector("form")).toBeTruthy();
	});

	it("calls prepare on formdata when enhance is not provided", () => {
		const prepare = vi.fn();
		render(FormHarness, {
			prepare,
			enhance: null,
		});

		const form = screen.getByTestId("form-root") as HTMLFormElement;
		const formData = new FormData(form);
		const event = new Event("formdata") as FormDataEvent;
		Object.defineProperty(event, "formData", { value: formData });
		form.dispatchEvent(event);

		expect(prepare).toHaveBeenCalledTimes(1);
		expect(prepare).toHaveBeenCalledWith(formData);
	});

	it("uses enhance hook submit callback and cleans up on update", async () => {
		const prepare = vi.fn();
		const destroy = vi.fn();
		let submitFromEnhance: ((options: { formData: FormData }) => void) | undefined;
		const enhance = vi.fn((_: HTMLFormElement, submit?: (options: { formData: FormData }) => void) => {
			submitFromEnhance = submit;
			return { destroy };
		});

		const view = render(FormHarness, {
			prepare,
			enhance,
		});

		expect(enhance).toHaveBeenCalledTimes(1);
		const payload = new FormData();
		submitFromEnhance?.({ formData: payload });
		expect(prepare).toHaveBeenCalledWith(payload);

		await view.rerender({
			prepare,
			enhance: null,
		});
		expect(destroy).toHaveBeenCalledTimes(1);
	});
});
