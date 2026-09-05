import { afterEach } from "vitest";
import { cleanup } from "@testing-library/svelte";

if (typeof window !== "undefined" && typeof window.matchMedia !== "function") {
	Object.defineProperty(window, "matchMedia", {
		writable: true,
		value: (query: string) => ({
			matches: false,
			media: query,
			onchange: null,
			addListener: () => {},
			removeListener: () => {},
			addEventListener: () => {},
			removeEventListener: () => {},
			dispatchEvent: () => false,
		}),
	});
}

if (typeof window !== "undefined" && typeof window.ResizeObserver !== "function") {
	class MockResizeObserver {
		disconnect() {}
		observe() {}
		unobserve() {}
	}

	Object.defineProperty(window, "ResizeObserver", {
		writable: true,
		value: MockResizeObserver,
	});
}

// Allow deferred DOM/timer cleanup from UI primitives (e.g. body scroll lock)
// to settle before jsdom tears down the environment.
afterEach(async () => {
	cleanup();
	await new Promise((resolve) => setTimeout(resolve, 25));
});
