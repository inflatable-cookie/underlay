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

	it("stays usable and settles loading when prepare prop changes during a pending submit", async () => {
		const firstGate = deferred<SpaFormResult>();
		const secondGate = deferred<SpaFormResult>();
		let submitCount = 0;
		const onSubmit = vi.fn(() => {
			submitCount += 1;
			return submitCount === 1 ? firstGate.promise : secondGate.promise;
		});
		const onResult = vi.fn();
		const navigate = vi.fn();
		const firstPrepare = vi.fn();
		const secondPrepare = vi.fn();

		const { container, rerender } = render(SpaFormShell, {
			title: "Profile",
			prepare: firstPrepare,
			onSubmit,
			onResult,
			navigate,
		});

		const form = container.querySelector("form");
		expect(form).not.toBeNull();

		await fireEvent.submit(form!);
		expect(onSubmit).toHaveBeenCalledTimes(1);
		expect(firstPrepare).toHaveBeenCalledTimes(1);

		// Change prepare while the first submission is still in flight; FormShell reapplies enhance.
		await rerender({
			title: "Profile",
			prepare: secondPrepare,
			onSubmit,
			onResult,
			navigate,
		});

		firstGate.resolve({ success: true, redirectTo: "/done" });
		await new Promise((r) => setTimeout(r, 0));

		expect(onResult).toHaveBeenCalledWith({ success: true, redirectTo: "/done" });
		expect(navigate).toHaveBeenCalledWith("/done");
		expect(container.textContent).not.toContain("Saving");

		// A subsequent submit on the same mounted form must still work.
		const nextForm = container.querySelector("form");
		expect(nextForm).not.toBeNull();
		await fireEvent.submit(nextForm!);
		expect(onSubmit).toHaveBeenCalledTimes(2);
		expect(secondPrepare).toHaveBeenCalledTimes(1);

		secondGate.resolve({ success: true });
		await new Promise((r) => setTimeout(r, 0));
		expect(onResult).toHaveBeenCalledTimes(2);
	});
});
