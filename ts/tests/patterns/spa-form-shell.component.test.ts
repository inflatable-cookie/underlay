// @vitest-environment jsdom
import { fireEvent, render } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import SpaFormShell from "../../src/patterns/SpaFormShell.svelte";
import type { SpaFormResult } from "../../src/patterns/spa-form-types";

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((res) => {
		resolve = res;
	});
	return { promise, resolve };
}

describe("patterns/SpaFormShell.svelte", () => {
	it("skips onResult and navigate when unmounted while onSubmit is awaiting", async () => {
		const submitGate = deferred<SpaFormResult>();
		const onSubmit = vi.fn(() => submitGate.promise);
		const onResult = vi.fn();
		const navigate = vi.fn();

		const { container, unmount } = render(SpaFormShell, {
			title: "Profile",
			onSubmit,
			onResult,
			navigate,
		});

		const form = container.querySelector("form");
		expect(form).not.toBeNull();

		await fireEvent.submit(form!);
		expect(onSubmit).toHaveBeenCalledTimes(1);

		// Unmount before the submission resolves
		unmount();

		submitGate.resolve({ success: true, redirectTo: "/done" });
		await new Promise((r) => setTimeout(r, 0));

		expect(onResult).not.toHaveBeenCalled();
		expect(navigate).not.toHaveBeenCalled();
	});
});
