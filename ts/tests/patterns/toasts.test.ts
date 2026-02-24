import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { get } from "svelte/store";
import {
	createToastStore,
	pushSuccessToast,
	pushErrorToast,
	type Toast
} from "../../src/patterns/toasts";

function getToasts(store: ReturnType<typeof createToastStore>): Toast[] {
	return get(store.toasts);
}

describe("toasts", () => {
	beforeEach(() => {
		vi.spyOn(Date, "now").mockReturnValue(1_700_000_000_000);
		vi.spyOn(Math, "random").mockReturnValue(0.123456789);
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("pushes and dismisses toasts", () => {
		const store = createToastStore();
		const id = store.push({ message: "Hello" });

		expect(id).toBe("1700000000000-4fzzzxjy");
		expect(getToasts(store)).toEqual([
			{
				id: "1700000000000-4fzzzxjy",
				variant: "info",
				title: undefined,
				message: "Hello",
				createdAtMs: 1_700_000_000_000
			}
		]);

		store.dismiss(id);
		expect(getToasts(store)).toEqual([]);
	});

	it("supports explicit id/variant/title and clear()", () => {
		const store = createToastStore();
		store.push({
			id: "custom-id",
			variant: "success",
			title: "Saved",
			message: "Done"
		});
		store.push({
			id: "second",
			variant: "error",
			message: "Oops"
		});
		expect(getToasts(store)).toHaveLength(2);

		store.clear();
		expect(getToasts(store)).toEqual([]);
	});

	it("pushSuccessToast creates success variant", () => {
		const store = createToastStore();
		const id = pushSuccessToast(store, "Updated", "Success");
		const toast = getToasts(store)[0];

		expect(id).toBe(toast.id);
		expect(toast.variant).toBe("success");
		expect(toast.title).toBe("Success");
		expect(toast.message).toBe("Updated");
	});

	it("pushErrorToast normalizes common error shapes", () => {
		const store = createToastStore();

		pushErrorToast(store, "plain");
		pushErrorToast(store, new Error("boom"));
		pushErrorToast(store, { message: "from-object" });
		pushErrorToast(store, {});

		const messages = getToasts(store).map((toast) => toast.message);
		expect(messages).toEqual(["plain", "boom", "from-object", "Something went wrong"]);
	});

	it("pushErrorToast falls back for unserializable values", () => {
		const store = createToastStore();
		const circular: { self?: unknown } = {};
		circular.self = circular;

		pushErrorToast(store, circular, "Fallback");
		expect(getToasts(store)[0].message).toBe("Unknown error");
	});
});
