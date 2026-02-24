import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const { session } = vi.hoisted(() => ({
	session: {
		get: vi.fn(),
		set: vi.fn(),
		remove: vi.fn()
	}
}));

vi.mock("../../src/patterns/storage", () => ({
	storage: {
		session
	}
}));

import {
	storePageState,
	retrievePageState,
	consumePageState,
	clearPageStates
} from "../../src/patterns/navigation-state";

const originalWindow = (globalThis as { window?: unknown }).window;

describe("navigation-state", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	afterEach(() => {
		(globalThis as { window?: unknown }).window = originalWindow;
	});

	it("stores and retrieves page state by pathname", () => {
		session.get.mockReturnValueOnce({});
		storePageState("/users", { page: 2, filter: "active" });
		expect(session.set).toHaveBeenCalledWith("underlay:nav-state", {
			"/users": { page: 2, filter: "active" }
		});

		session.get.mockReturnValueOnce({
			"/users": { page: 2, filter: "active" }
		});
		expect(retrievePageState<{ page: number; filter: string }>("/users")).toEqual({
			page: 2,
			filter: "active"
		});
	});

	it("returns null when page state is missing", () => {
		session.get.mockReturnValueOnce({});
		expect(retrievePageState("/missing")).toBeNull();
	});

	it("consumes and removes state for explicit pathname", () => {
		session.get.mockReturnValueOnce({
			"/users": { page: 3 },
			"/other": { page: 1 }
		});

		expect(consumePageState<{ page: number }>("/users")).toEqual({ page: 3 });
		expect(session.set).toHaveBeenCalledWith("underlay:nav-state", {
			"/other": { page: 1 }
		});
	});

	it("uses window.location.pathname when pathname is omitted", () => {
		(globalThis as { window?: unknown }).window = {
			location: { pathname: "/auto" }
		};
		session.get.mockReturnValueOnce({
			"/auto": { q: "x" }
		});

		expect(consumePageState<{ q: string }>()).toEqual({ q: "x" });
	});

	it("returns null when no target path is available", () => {
		(globalThis as { window?: unknown }).window = undefined;
		expect(consumePageState()).toBeNull();
	});

	it("clears all stored states", () => {
		clearPageStates();
		expect(session.remove).toHaveBeenCalledWith("underlay:nav-state");
	});
});
