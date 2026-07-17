import { afterEach, describe, expect, it, vi } from "vitest";
import { writable } from "svelte/store";
import {
	configureAuth,
	getAuthConfig,
	getAuthState,
	isAuthenticated,
	requireAuth,
	requireRole,
} from "../../src/patterns/auth";

describe("patterns/auth", () => {
	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it("stores and exposes global auth config in the browser", () => {
		vi.stubGlobal("window", {});
		const config = {
			getToken: () => "token-1",
			onRefresh: async () => "token-2",
			getAuthLoading: () => false,
			getCurrentUser: () => ({ id: "u1" }),
		};

		configureAuth(config);
		expect(getAuthConfig()).toBe(config);
		expect(getAuthConfig()?.getToken()).toBe("token-1");
	});

	it("throws when configureAuth runs during SSR (no window)", () => {
		// Default vitest env is node: `window` is undefined here, simulating SSR.
		expect(typeof window).toBe("undefined");
		expect(() =>
			configureAuth({
				getToken: () => "leaky",
				onRefresh: async () => null,
			}),
		).toThrow(/SSR/);
	});

	it("validates auth state and enforces required auth", () => {
		const authenticatedState = {
			status: "authenticated",
			session: {
				user: {
					id: "u1",
					email: "user@example.com",
					displayName: "User",
					status: "active",
					createdAt: "2026-01-01T00:00:00Z",
					updatedAt: "2026-01-01T00:00:00Z",
				},
				session: {
					id: "s1",
					userId: "u1",
					accessTokenFingerprint: "a",
					refreshTokenFingerprint: "r",
					accessTokenExpiresAt: "2026-01-01T01:00:00Z",
					refreshTokenExpiresAt: "2026-01-02T01:00:00Z",
					createdAt: "2026-01-01T00:00:00Z",
					lastUsedAt: "2026-01-01T00:00:00Z",
					ipAddress: null,
					userAgent: null,
					status: "active",
					revocationReason: null,
					revokedAt: null,
				},
				accessToken: "access",
				refreshToken: "refresh",
			},
			loading: false,
			error: null,
		} as const;

		const anonymousState = {
			status: "anonymous",
			session: null,
			loading: false,
			error: null,
		} as const;

		expect(isAuthenticated(authenticatedState as any)).toBe(true);
		expect(isAuthenticated(anonymousState as any)).toBe(false);
		expect(requireAuth(authenticatedState as any)).toBe(authenticatedState.session);
		expect(() => requireAuth(anonymousState as any)).toThrow("Authentication required");
	});

	it("enforces roles and exposes store state readable", () => {
		expect(() => requireRole(() => true, "admin")).not.toThrow();
		expect(() => requireRole(() => false, "admin")).toThrow("Forbidden");

		const stateReadable = writable({
			status: "unknown",
			session: null,
			loading: false,
			error: null,
		});

		const store = {
			state: stateReadable,
			init: async () => {},
			register: async () => {
				throw new Error("not used");
			},
			loginWithPassword: async () => {
				throw new Error("not used");
			},
			loginWithPasskey: async () => {
				throw new Error("not used");
			},
			refresh: async () => {
				throw new Error("not used");
			},
			logout: async () => {},
		};

		expect(getAuthState(store as any)).toBe(stateReadable);
	});
});
