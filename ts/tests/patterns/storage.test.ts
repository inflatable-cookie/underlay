import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { get } from "svelte/store";

class MemoryStorage implements Storage {
	private data = new Map<string, string>();
	private failWrites = false;

	enableWriteFailures(): void {
		this.failWrites = true;
	}

	disableWriteFailures(): void {
		this.failWrites = false;
	}

	get length(): number {
		return this.data.size;
	}

	clear(): void {
		this.data.clear();
	}

	getItem(key: string): string | null {
		return this.data.has(key) ? this.data.get(key)! : null;
	}

	key(index: number): string | null {
		return Array.from(this.data.keys())[index] ?? null;
	}

	removeItem(key: string): void {
		this.data.delete(key);
	}

	setItem(key: string, value: string): void {
		if (this.failWrites && key !== "__underlay_storage_test__") {
			throw new Error("write disabled");
		}
		this.data.set(key, value);
	}
}

type StorageListener = (event: {
	key: string | null;
	newValue: string | null;
	storageArea: Storage | null;
}) => void;

function createWindowMock() {
	const localStorage = new MemoryStorage();
	const sessionStorage = new MemoryStorage();
	const listeners: Record<string, StorageListener[]> = {};

	const windowMock = {
		localStorage,
		sessionStorage,
		addEventListener(type: string, listener: StorageListener) {
			listeners[type] ??= [];
			listeners[type].push(listener);
		},
		removeEventListener(type: string, listener: StorageListener) {
			listeners[type] = (listeners[type] ?? []).filter((entry) => entry !== listener);
		},
		dispatchStorageEvent(event: {
			key: string | null;
			newValue: string | null;
			storageArea: Storage | null;
		}) {
			for (const listener of listeners.storage ?? []) {
				listener(event);
			}
		}
	};

	return windowMock;
}

describe("patterns/storage (SSR mode)", () => {
	beforeEach(() => {
		vi.resetModules();
	});

	it("returns defaults and no-ops safely when BROWSER is false", async () => {
		const { storage } = await import("../../src/patterns/storage");
		expect(storage.local.get("theme", "light")).toBe("light");
		expect(storage.local.has("theme")).toBe(false);
		expect(() => storage.local.set("theme", "dark")).not.toThrow();
		expect(() => storage.local.remove("theme")).not.toThrow();
		expect(() => storage.local.clear()).not.toThrow();
	});
});

describe("patterns/storage (browser mode)", () => {
	const originalWindow = (globalThis as { window?: unknown }).window;

	beforeEach(() => {
		vi.resetModules();
		vi.doMock("esm-env", () => ({ BROWSER: true }));
	});

	afterEach(() => {
		vi.restoreAllMocks();
		(globalThis as { window?: unknown }).window = originalWindow;
	});

	it("supports get/set/has/remove for local and session storage", async () => {
		const windowMock = createWindowMock();
		(globalThis as { window?: unknown }).window = windowMock;
		const { storage } = await import("../../src/patterns/storage");

		storage.local.set("theme", { dark: true });
		expect(storage.local.has("theme")).toBe(true);
		expect(storage.local.get("theme", { dark: false })).toEqual({ dark: true });
		storage.local.remove("theme");
		expect(storage.local.has("theme")).toBe(false);

		storage.session.set("draft", "abc");
		expect(storage.session.get("draft", "none")).toBe("abc");
		storage.session.clear();
		expect(storage.session.has("draft")).toBe(false);
	});

	it("supports custom serializer and deserializer", async () => {
		const windowMock = createWindowMock();
		(globalThis as { window?: unknown }).window = windowMock;
		const { storage } = await import("../../src/patterns/storage");

		storage.local.set("counter", 42, {
			serialize: (value) => `v:${String(value)}`,
			deserialize: (value) => Number(value.replace("v:", ""))
		});

		expect(storage.local.get("counter", 0, {
			deserialize: (value) => Number(value.replace("v:", ""))
		})).toBe(42);
	});

	it("logs and does not throw when storage write fails", async () => {
		const windowMock = createWindowMock();
		windowMock.localStorage.enableWriteFailures();
		(globalThis as { window?: unknown }).window = windowMock;
		const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
		const { storage } = await import("../../src/patterns/storage");

		expect(() => storage.local.set("x", "y")).not.toThrow();
		expect(warn).toHaveBeenCalledOnce();
	});

	it("persisted stores round-trip values and react to storage events", async () => {
		const windowMock = createWindowMock();
		(globalThis as { window?: unknown }).window = windowMock;
		const { createPersistedStore } = await import("../../src/patterns/storage");

		const store = createPersistedStore("prefs", { count: 0 });
		expect(get(store)).toEqual({ count: 0 });

		store.set({ count: 2 });
		expect(windowMock.localStorage.getItem("prefs")).toBe(JSON.stringify({ count: 2 }));
		expect(get(store)).toEqual({ count: 2 });

		store.update((current) => ({ count: current.count + 1 }));
		expect(get(store)).toEqual({ count: 3 });

		windowMock.dispatchStorageEvent({
			key: "prefs",
			newValue: JSON.stringify({ count: 9 }),
			storageArea: windowMock.localStorage
		});
		expect(get(store)).toEqual({ count: 9 });
	});
});
