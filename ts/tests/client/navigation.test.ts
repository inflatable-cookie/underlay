import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	goto: vi.fn(async () => {}),
}));

vi.mock("$app/navigation", () => ({
	goto: mocks.goto,
}));

vi.mock("$app/environment", () => ({
	browser: true,
}));

class MemoryStorage implements Storage {
	private data = new Map<string, string>();

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
		this.data.set(key, value);
	}
}

function createWindowMock(pathname = "/", historyLength = 2) {
	const location = {
		origin: "https://example.com",
		pathname,
		href: `https://example.com${pathname}`,
	};

	return {
		localStorage: new MemoryStorage(),
		sessionStorage: new MemoryStorage(),
		location,
		history: {
			length: historyLength,
			back: vi.fn(),
		},
	};
}

describe("client/navigation", () => {
	const originalWindow = (globalThis as { window?: unknown }).window;
	const originalLocation = (globalThis as { location?: unknown }).location;

	beforeEach(() => {
		vi.resetModules();
		vi.clearAllMocks();
		vi.doMock("esm-env", () => ({ BROWSER: true }));
	});

	afterEach(() => {
		(globalThis as { window?: unknown }).window = originalWindow;
		(globalThis as { location?: unknown }).location = originalLocation;
	});

	it("gotoWithContext stores state, pushes context with targetHref, and navigates", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/from");
		const clientNav = await import("../../src/client/navigation");
		const patternsNav = await import("../../src/patterns/navigation");

		patternsNav.configureNavigationContext({ storageKey: "client-nav:test", maxDepth: 3 });
		patternsNav.clearNavigationContext();
		patternsNav.clearPageStates();

		await clientNav.gotoWithContext(
			"/items/1/edit",
			{
				label: "Items",
				href: "/items?tab=active",
				type: "list",
				state: { tab: "active", page: 2 },
			},
			{ replaceState: true }
		);

		expect(mocks.goto).toHaveBeenCalledWith("/items/1/edit", { replaceState: true });
		expect(patternsNav.peekNavigationContext()).toEqual({
			label: "Items",
			href: "/items?tab=active",
			type: "list",
			state: { tab: "active", page: 2 },
			targetHref: "/items/1/edit",
		});
		expect(patternsNav.retrievePageState("/items")).toEqual({ tab: "active", page: 2 });
	});

	it("gotoWithContext stores state for absolute hrefs using global location origin", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/from");
		(globalThis as { location?: unknown }).location = { origin: "https://example.com" };
		const clientNav = await import("../../src/client/navigation");
		const patternsNav = await import("../../src/patterns/navigation");

		patternsNav.configureNavigationContext({ storageKey: "client-nav:absolute", maxDepth: 3 });
		patternsNav.clearNavigationContext();
		patternsNav.clearPageStates();

		await clientNav.gotoWithContext("/target", {
			label: "Items",
			href: "https://example.com/items?tab=active",
			type: "list",
			state: { tab: "active" },
		});

		expect(patternsNav.retrievePageState("/items")).toEqual({ tab: "active" });
	});

	it("navigateBack prefers stack context and falls back to parent path", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/projects/1/edit");
		const clientNav = await import("../../src/client/navigation");
		const patternsNav = await import("../../src/patterns/navigation");

		patternsNav.configureNavigationContext({ storageKey: "client-nav:back", maxDepth: 3 });
		patternsNav.clearNavigationContext();
		patternsNav.pushNavigationContext({ label: "Projects", href: "/projects", type: "list" });

		expect(clientNav.navigateBack("/fallback")).toBe("/projects");
		expect(mocks.goto).toHaveBeenCalledWith("/projects");

		mocks.goto.mockClear();
		expect(clientNav.navigateBack()).toBe("/projects/1");
		expect(mocks.goto).toHaveBeenCalledWith("/projects/1");
	});

	it("navigateOnCancel uses explicit href, parent derivation, then history/root fallback", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/articles/1/edit", 2);
		const clientNav = await import("../../src/client/navigation");

		clientNav.navigateOnCancel("/explicit");
		expect((globalThis as any).window.location.href).toBe("/explicit");

		(globalThis as any).window.location.pathname = "/articles/1/edit";
		(globalThis as any).window.location.href = "https://example.com/articles/1/edit";
		clientNav.navigateOnCancel(undefined);
		expect((globalThis as any).window.location.href).toBe("https://example.com/articles/1");

		(globalThis as any).window.location.pathname = "/";
		(globalThis as any).window.history.length = 2;
		clientNav.navigateOnCancel(undefined);
		expect((globalThis as any).window.history.back).toHaveBeenCalledOnce();

		(globalThis as any).window.history.length = 1;
		clientNav.navigateOnCancel(undefined);
		expect((globalThis as any).window.location.href).toBe("/");
	});

	it("initPageState merges consumed values and capturePageState passes through", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/reports");
		const clientNav = await import("../../src/client/navigation");
		const patternsNav = await import("../../src/patterns/navigation");

		patternsNav.storePageState("/reports", {
			activeTab: "stats",
			currentPage: 4,
			ignored: true,
		});

		expect(clientNav.initPageState({ activeTab: "overview", currentPage: 1 })).toEqual({
			activeTab: "stats",
			currentPage: 4,
		});
		expect(patternsNav.consumePageState("/reports")).toBeNull();

		expect(clientNav.capturePageState({ q: "hello", page: 3 })).toEqual({ q: "hello", page: 3 });
	});

	it("uses safe fallbacks when browser mode is disabled", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/reports");
		vi.resetModules();
		vi.doMock("$app/environment", () => ({ browser: false }));
		const clientNav = await import("../../src/client/navigation");

		expect(clientNav.navigateBack()).toBe("/");
		expect(mocks.goto).toHaveBeenCalledWith("/");

		const hrefBefore = (globalThis as any).window.location.href;
		clientNav.navigateOnCancel("/ignored-when-server");
		expect((globalThis as any).window.location.href).toBe(hrefBefore);

		expect(clientNav.initPageState({ activeTab: "overview", page: 1 })).toEqual({
			activeTab: "overview",
			page: 1,
		});
	});
});
