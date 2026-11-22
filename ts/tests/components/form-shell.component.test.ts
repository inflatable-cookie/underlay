// @vitest-environment jsdom
import { fireEvent, render } from "@testing-library/svelte";
import { createRawSnippet, tick } from "svelte";
import { describe, expect, it, vi } from "vitest";
import FormShell from "../../src/patterns/FormShell.svelte";
import SpaFormShell from "../../src/patterns/SpaFormShell.svelte";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

describe("patterns/FormShell.svelte useEnhanced lifecycle", () => {
	it("does not tear down or re-attach enhance when error, success, or title props update", async () => {
		const destroySpy = vi.fn();
		const enhanceSpy = vi.fn(() => ({ destroy: destroySpy }));
		const prepare = vi.fn();
		const children = createRawSnippet(() => ({
			render: () => '<input name="title" type="text" />',
		}));

		const { rerender, unmount } = render(FormShell, {
			title: "Initial",
			enhance: enhanceSpy,
			prepare,
			error: null,
			success: null,
			successMessage: null,
			children,
		});

		expect(enhanceSpy).toHaveBeenCalledTimes(1);
		expect(destroySpy).not.toHaveBeenCalled();

		// Updating status/banner/title props must not churn the enhance action.
		await rerender({
			title: "Updated",
			enhance: enhanceSpy,
			prepare,
			error: "Validation failed",
			success: false,
			successMessage: null,
			children,
		});
		await tick();

		expect(enhanceSpy).toHaveBeenCalledTimes(1);
		expect(destroySpy).not.toHaveBeenCalled();

		await rerender({
			title: "Updated",
			enhance: enhanceSpy,
			prepare,
			error: null,
			success: true,
			successMessage: "Saved",
			children,
		});
		await tick();

		expect(enhanceSpy).toHaveBeenCalledTimes(1);
		expect(destroySpy).not.toHaveBeenCalled();

		// Changing the enhance or prepare callback reference DOES re-attach cleanly.
		const nextPrepare = vi.fn();
		await rerender({
			title: "Updated",
			enhance: enhanceSpy,
			prepare: nextPrepare,
			error: null,
			success: true,
			successMessage: "Saved",
			children,
		});
		await tick();

		expect(destroySpy).toHaveBeenCalledTimes(1);
		expect(enhanceSpy).toHaveBeenCalledTimes(2);

		unmount();
		expect(destroySpy).toHaveBeenCalledTimes(2);
	});

	it("keeps a single form submit listener attached across SpaFormShell loading transitions", async () => {
		const gate = deferred<{ success: boolean }>();
		const onSubmit = vi.fn(() => gate.promise);
		const prepare = vi.fn();

		const { container } = render(SpaFormShell, {
			title: "Profile",
			prepare,
			onSubmit,
		});

		const form = container.querySelector("form");
		expect(form).not.toBeNull();

		const addSpy = vi.spyOn(form!, "addEventListener");
		const removeSpy = vi.spyOn(form!, "removeEventListener");

		await fireEvent.submit(form!);
		expect(container.textContent).toContain("Saving");

		gate.resolve({ success: true });
		await vi.waitFor(() => {
			expect(container.textContent).not.toContain("Saving");
		});

		expect(addSpy).not.toHaveBeenCalledWith("submit", expect.any(Function));
		expect(removeSpy).not.toHaveBeenCalledWith("submit", expect.any(Function));
	});
});
