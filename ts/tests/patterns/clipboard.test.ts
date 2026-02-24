import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { copyTextToClipboard, copyToClipboard } from "../../src/patterns/clipboard";
import { createToastStore } from "../../src/patterns/toasts";

describe("copyTextToClipboard", () => {
	const originalNavigatorDescriptor = Object.getOwnPropertyDescriptor(globalThis, "navigator");

	function setNavigator(value: unknown): void {
		Object.defineProperty(globalThis, "navigator", {
			configurable: true,
			value
		});
	}

	afterEach(() => {
		if (originalNavigatorDescriptor) {
			Object.defineProperty(globalThis, "navigator", originalNavigatorDescriptor);
		}
	});

	it("writes text when Clipboard API is available", async () => {
		const writeText = vi.fn(async () => {});
		setNavigator({
			clipboard: { writeText }
		});

		await copyTextToClipboard("hello");
		expect(writeText).toHaveBeenCalledWith("hello");
	});

	it("throws when Clipboard API is unavailable", async () => {
		setNavigator({});
		await expect(copyTextToClipboard("x")).rejects.toThrow("Clipboard API unavailable");
	});
});

describe("copyToClipboard", () => {
	const originalNavigatorDescriptor = Object.getOwnPropertyDescriptor(globalThis, "navigator");

	function setNavigator(value: unknown): void {
		Object.defineProperty(globalThis, "navigator", {
			configurable: true,
			value
		});
	}

	beforeEach(() => {
		vi.restoreAllMocks();
	});

	afterEach(() => {
		if (originalNavigatorDescriptor) {
			Object.defineProperty(globalThis, "navigator", originalNavigatorDescriptor);
		}
	});

	it("pushes success toast when copy succeeds", async () => {
		const writeText = vi.fn(async () => {});
		setNavigator({
			clipboard: { writeText }
		});
		const toasts = createToastStore();
		const pushSpy = vi.spyOn(toasts, "push");

		await copyToClipboard(toasts, "abc", "Copied");

		expect(writeText).toHaveBeenCalledWith("abc");
		expect(pushSpy).toHaveBeenCalledWith({ variant: "success", message: "Copied" });
	});

	it("pushes error toast and rethrows when copy fails", async () => {
		const writeText = vi.fn(async () => {
			throw new Error("permission denied");
		});
		setNavigator({
			clipboard: { writeText }
		});
		const toasts = createToastStore();
		const pushSpy = vi.spyOn(toasts, "push");
		const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});

		await expect(copyToClipboard(toasts, "abc", "Copied", "Copy failed")).rejects.toThrow(
			"permission denied"
		);

		expect(pushSpy).toHaveBeenCalledWith({ variant: "error", message: "Copy failed" });
		expect(consoleSpy).toHaveBeenCalled();
	});
});
