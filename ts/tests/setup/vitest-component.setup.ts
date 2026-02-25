import { afterEach } from "vitest";
import { cleanup } from "@testing-library/svelte";

// Allow deferred DOM/timer cleanup from UI primitives (e.g. body scroll lock)
// to settle before jsdom tears down the environment.
afterEach(async () => {
	cleanup();
	await new Promise((resolve) => setTimeout(resolve, 25));
});
