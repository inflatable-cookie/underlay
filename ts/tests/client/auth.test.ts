import { describe, expect, it, vi } from "vitest";
import { createAuthCommands, type AuthRoutes } from "../../src/client/auth";

const routes: AuthRoutes = {
	register: "/auth/register",
	loginPassword: "/auth/login/password",
	loginPasskey: "/auth/login/passkey",
	logout: "/auth/logout",
	refresh: "/auth/refresh",
	session: "/auth/session",
};

describe("client/auth", () => {
	it("calls register route and returns payload", async () => {
		const post = vi.fn().mockResolvedValue({ data: { id: "session-1" } });
		const auth = createAuthCommands({ post, get: vi.fn() } as any, routes);

		await expect(
			auth.register({ email: "user@example.com", password: "pw", displayName: "User" })
		).resolves.toEqual({ id: "session-1" });
		expect(post).toHaveBeenCalledWith(routes.register, {
			email: "user@example.com",
			password: "pw",
			displayName: "User",
		});
	});

	it("calls password and passkey login routes", async () => {
		const post = vi
			.fn()
			.mockResolvedValueOnce({ data: { id: "session-password" } })
			.mockResolvedValueOnce({ data: { id: "session-passkey" } });
		const auth = createAuthCommands({ post, get: vi.fn() } as any, routes);

		await expect(
			auth.loginWithPassword({ email: "user@example.com", password: "pw", code: "123456" })
		).resolves.toEqual({ id: "session-password" });
		expect(post).toHaveBeenNthCalledWith(1, routes.loginPassword, {
			email: "user@example.com",
			password: "pw",
			code: "123456",
		});

		await expect(auth.loginWithPasskey({ credential: { id: "cred-1" } })).resolves.toEqual({
			id: "session-passkey",
		});
		expect(post).toHaveBeenNthCalledWith(2, routes.loginPasskey, {
			credential: { id: "cred-1" },
		});
	});

	it("calls logout, refresh, and session routes", async () => {
		const post = vi
			.fn()
			.mockResolvedValueOnce(undefined)
			.mockResolvedValueOnce({ data: { id: "session-refresh" } });
		const get = vi
			.fn()
			.mockResolvedValueOnce({ data: { user: { id: "u1" }, session: { id: "session-current" } } });
		const auth = createAuthCommands({ post, get } as any, routes);

		await expect(auth.logout()).resolves.toBeUndefined();
		expect(post).toHaveBeenNthCalledWith(1, routes.logout);

		await expect(auth.refresh()).resolves.toEqual({ id: "session-refresh" });
		expect(post).toHaveBeenNthCalledWith(2, routes.refresh);

		await expect(auth.session()).resolves.toEqual({
			user: { id: "u1" },
			session: { id: "session-current" },
		});
		expect(get).toHaveBeenCalledWith(routes.session);
	});

	it("strips token fields from session GET responses", async () => {
		const get = vi.fn().mockResolvedValueOnce({
			data: {
				user: { id: "u1" },
				session: { id: "s1" },
				accessToken: "leaked-access",
				refreshToken: "leaked-refresh",
			},
		});
		const auth = createAuthCommands({ post: vi.fn(), get } as any, routes);

		const result = await auth.session();

		expect(result).toEqual({ user: { id: "u1" }, session: { id: "s1" } });
		expect(result).not.toHaveProperty("refreshToken");
		expect(result).not.toHaveProperty("accessToken");
	});
});
