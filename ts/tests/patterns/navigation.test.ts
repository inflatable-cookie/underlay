import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

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

function createWindowMock(pathname = "/") {
	return {
		localStorage: new MemoryStorage(),
		sessionStorage: new MemoryStorage(),
		location: {
			origin: "https://example.com",
			pathname,
			href: `https://example.com${pathname}`,
		},
		history: {
			length: 2,
			back: vi.fn(),
		},
	};
}

describe("patterns/navigation", () => {
	const originalWindow = (globalThis as { window?: unknown }).window;

	beforeEach(() => {
		vi.resetModules();
		vi.doMock("esm-env", () => ({ BROWSER: true }));
	});

	afterEach(() => {
		vi.restoreAllMocks();
		(globalThis as { window?: unknown }).window = originalWindow;
	});

	it("pushes, peeks, pops, and clears context stack", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/projects/1/edit");
		const nav = await import("../../src/patterns/navigation");

		nav.configureNavigationContext({ storageKey: "test:nav", maxDepth: 2 });
		nav.pushNavigationContext({ label: "Projects", href: "/projects", type: "list" });
		nav.pushNavigationContext({ label: "Project A", href: "/projects/1", type: "detail" });
		nav.pushNavigationContext({ label: "Edit", href: "/projects/1/edit", type: "edit" });

		expect(nav.getNavigationContextStack()).toEqual([
			{ label: "Project A", href: "/projects/1", type: "detail" },
			{ label: "Edit", href: "/projects/1/edit", type: "edit" },
		]);
		expect(nav.peekNavigationContext()).toEqual({
			label: "Edit",
			href: "/projects/1/edit",
			type: "edit",
		});
		expect(nav.popNavigationContext()).toEqual({
			label: "Edit",
			href: "/projects/1/edit",
			type: "edit",
		});

		nav.clearNavigationContext();
		expect(nav.getNavigationContextStack()).toEqual([]);
	});

	it("resolves return URLs and back info with target validation", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/projects/1/edit");
		const nav = await import("../../src/patterns/navigation");

		nav.configureNavigationContext({ storageKey: "test:return", maxDepth: 3 });
		nav.clearNavigationContext();

		nav.pushNavigationContext({
			label: "Projects",
			href: "/projects",
			type: "list",
			targetHref: "/projects/1/edit",
		});
		expect(nav.getReturnUrl("/fallback")).toBe("/projects");
		expect(nav.getBackButtonInfo("Back", "/fallback")).toEqual({
			label: "Back to Projects",
			href: "/projects",
			isContextual: true,
		});

		nav.clearNavigationContext();
		nav.pushNavigationContext({
			label: "Stale",
			href: "/stale",
			type: "list",
			targetHref: "/wrong/path",
		});
		expect(nav.getReturnUrl("/fallback")).toBe("/fallback");
		expect(nav.getBackButtonInfo("Back", "/fallback")).toEqual({
			label: "Back",
			href: "/fallback",
			isContextual: false,
		});

		nav.clearNavigationContext();
		expect(nav.getReturnUrl("/fallback-only")).toBe("/fallback-only");
	});

	it("consumes context once and computes fallback-aware back info", async () => {
		(globalThis as { window?: unknown }).window = createWindowMock("/articles/1/edit");
		const nav = await import("../../src/patterns/navigation");

		nav.configureNavigationContext({ storageKey: "test:consume", maxDepth: 3 });
		nav.clearNavigationContext();
		nav.pushNavigationContext({
			label: "Articles",
			href: "/articles",
			type: "list",
			targetHref: "/articles/1/edit",
		});

		const consumed = nav.consumeNavigationContext("Back", "/fallback");
		expect(consumed).toEqual({
			backInfo: {
				label: "Back to Articles",
				href: "/articles",
				isContextual: true,
			},
			returnTo: "/articles",
		});
		expect(nav.getNavigationContextStack()).toEqual([]);

		expect(nav.computeBackInfo(consumed.backInfo, { href: "/forced", label: "Forced" })).toEqual(
			consumed.backInfo
		);
		expect(
			nav.computeBackInfo({ label: "Back", href: "/fallback", isContextual: false }, {
				href: "/forced",
				label: "Forced",
			})
		).toEqual({
			label: "Forced",
			href: "/forced",
			isContextual: false,
		});
	});

	it("derives parent paths safely", async () => {
		const { deriveParentPath } = await import("../../src/patterns/navigation");

		expect(deriveParentPath("/content/videos/new")).toBe("/content/videos");
		expect(deriveParentPath("/learning/pathways/123")).toBe("/learning/pathways");
		expect(deriveParentPath("/")).toBe("/");
		expect(deriveParentPath("")).toBe("/");
	});
});
