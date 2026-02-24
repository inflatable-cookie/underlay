import { describe, it, expect, vi, beforeEach } from "vitest";

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
	readNavigationContextStack,
	writeNavigationContextStack,
	popNavigationContextStack,
	peekNavigationContextStack,
	clearNavigationContextStack
} from "../../src/patterns/navigation-context-store";

describe("navigation-context-store", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("reads and writes stack entries", () => {
		session.get.mockReturnValueOnce([{ href: "/x" }]);
		expect(readNavigationContextStack<{ href: string }>("k")).toEqual([{ href: "/x" }]);
		expect(session.get).toHaveBeenCalledWith("k", []);

		writeNavigationContextStack("k", [{ href: "/y" }]);
		expect(session.set).toHaveBeenCalledWith("k", [{ href: "/y" }]);
	});

	it("pops top item and persists updated stack", () => {
		session.get.mockReturnValueOnce([{ href: "/a" }, { href: "/b" }]);
		const popped = popNavigationContextStack<{ href: string }>("k");

		expect(popped).toEqual({ href: "/b" });
		expect(session.set).toHaveBeenCalledWith("k", [{ href: "/a" }]);
	});

	it("returns null on pop/peek for empty stack", () => {
		session.get.mockReturnValueOnce([]);
		expect(popNavigationContextStack("k")).toBeNull();
		expect(session.set).not.toHaveBeenCalled();

		session.get.mockReturnValueOnce([]);
		expect(peekNavigationContextStack("k")).toBeNull();
	});

	it("peeks top item and clears stack key", () => {
		session.get.mockReturnValueOnce([{ href: "/a" }, { href: "/b" }]);
		expect(peekNavigationContextStack<{ href: string }>("k")).toEqual({ href: "/b" });

		clearNavigationContextStack("k");
		expect(session.remove).toHaveBeenCalledWith("k");
	});
});
