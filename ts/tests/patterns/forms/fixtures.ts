import { vi } from "vitest";
import type { StorageWrapper } from "../../../src/patterns/storage";

export function createMockForm(action = "https://example.com/form", method = "post") {
	let submitHandler: ((event: SubmitEvent) => Promise<void>) | undefined;
	return {
		action,
		method,
		addEventListener: vi.fn((event: string, cb: (event: SubmitEvent) => Promise<void>) => {
			if (event === "submit") submitHandler = cb;
		}),
		removeEventListener: vi.fn(),
		submit: async () => {
			const event = { preventDefault: vi.fn() } as unknown as SubmitEvent;
			await submitHandler?.(event);
			return event;
		}
	};
}

export function createMemoryDraftStorage(initial = new Map<string, unknown>()) {
	const data = initial;
	const wrapper: StorageWrapper = {
		get: vi.fn((key, defaultValue) => (data.has(key) ? (data.get(key) as any) : defaultValue)),
		set: vi.fn((key, value) => {
			data.set(key, value);
		}),
		remove: vi.fn((key) => {
			data.delete(key);
		}),
		has: vi.fn((key) => data.has(key)),
		isExpired: vi.fn(() => false),
		store: vi.fn(),
		clear: vi.fn(() => data.clear())
	};

	return { data, wrapper };
}
