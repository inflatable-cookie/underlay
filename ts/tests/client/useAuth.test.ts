import { describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";
import { UnderlayHttpError } from "../../src/client/errors";
import { createAuthStore } from "../../src/client/useAuth";

function makeSession(id = "s1") {
	return {
		user: {
			id: "u1",
			email: "user@example.com",
			displayName: "User",
			status: "active" as const,
			createdAt: "2026-01-01T00:00:00Z",
			updatedAt: "2026-01-01T00:00:00Z",
		},
		session: {
			id,
			userId: "u1",
			accessTokenFingerprint: "a",
			refreshTokenFingerprint: "r",
			accessTokenExpiresAt: "2026-01-01T01:00:00Z",
			refreshTokenExpiresAt: "2026-01-02T01:00:00Z",
			createdAt: "2026-01-01T00:00:00Z",
			lastUsedAt: "2026-01-01T00:00:00Z",
			ipAddress: null,
			userAgent: null,
			status: "active" as const,
			revocationReason: null,
			revokedAt: null,
		},
		accessToken: `access-${id}`,
		refreshToken: `refresh-${id}`,
	};
}

function makeDeps() {
	const commands = {
		session: vi.fn(),
		refresh: vi.fn(),
		register: vi.fn(),
		loginWithPassword: vi.fn(),
		loginWithPasskey: vi.fn(),
		logout: vi.fn(),
	};

	const tokenStore = {
		setAccessToken: vi.fn(async () => {}),
		setRefreshToken: vi.fn(async () => {}),
		clear: vi.fn(async () => {}),
		getAccessToken: vi.fn(() => null),
		getRefreshToken: vi.fn(() => null),
	};

	return { commands, tokenStore };
}

describe("client/useAuth", () => {
	it("initializes as authenticated when session request succeeds", async () => {
		const { commands, tokenStore } = makeDeps();
		const session = makeSession("init");
		commands.session.mockResolvedValue(session);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.setAccessToken).toHaveBeenCalledWith("access-init");
		expect(tokenStore.setRefreshToken).toHaveBeenCalledWith("refresh-init");
		expect(get(auth.state)).toEqual({
			status: "authenticated",
			session,
			loading: false,
			error: null,
		});
	});

	it("refreshes on init when session is unauthorized", async () => {
		const { commands, tokenStore } = makeDeps();
		const unauthorized = new UnderlayHttpError(401, "Unauthorized");
		const refreshed = makeSession("refresh");
		commands.session.mockRejectedValue(unauthorized);
		commands.refresh.mockResolvedValue(refreshed);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(commands.refresh).toHaveBeenCalledOnce();
		expect(get(auth.state).status).toBe("authenticated");
		expect(tokenStore.clear).not.toHaveBeenCalled();
	});

	it("becomes anonymous when refresh on init fails", async () => {
		const { commands, tokenStore } = makeDeps();
		const unauthorized = new UnderlayHttpError(401, "Unauthorized");
		const refreshErr = new UnderlayHttpError(401, "Refresh failed");
		commands.session.mockRejectedValue(unauthorized);
		commands.refresh.mockRejectedValue(refreshErr);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.clear).toHaveBeenCalledOnce();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: refreshErr,
		});
	});

	it("clears tokens and keeps 401 error when refreshOnUnauthorized is disabled", async () => {
		const { commands, tokenStore } = makeDeps();
		const unauthorized = new UnderlayHttpError(401, "Unauthorized");
		commands.session.mockRejectedValue(unauthorized);
		const auth = createAuthStore({
			commands: commands as any,
			tokenStore: tokenStore as any,
			refreshOnUnauthorized: false,
		});

		await auth.init();

		expect(commands.refresh).not.toHaveBeenCalled();
		expect(tokenStore.clear).toHaveBeenCalledOnce();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: unauthorized,
		});
	});

	it("sets anonymous with null error when refresh throws non-http error during init", async () => {
		const { commands, tokenStore } = makeDeps();
		commands.session.mockRejectedValue(new UnderlayHttpError(401, "Unauthorized"));
		commands.refresh.mockRejectedValue(new Error("refresh exploded"));
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.clear).toHaveBeenCalledOnce();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: null,
		});
	});

	it("register/login flows set authenticated state and preserve non-http errors", async () => {
		const { commands, tokenStore } = makeDeps();
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		const registered = makeSession("register");
		commands.register.mockResolvedValue(registered);
		await expect(
			auth.register({ email: "u@example.com", password: "pw", displayName: "User" })
		).resolves.toEqual(registered);
		expect(get(auth.state).status).toBe("authenticated");

		const login = makeSession("login");
		commands.loginWithPassword.mockResolvedValue(login);
		await auth.loginWithPassword({ email: "u@example.com", password: "pw" });
		expect(commands.loginWithPassword).toHaveBeenLastCalledWith({ email: "u@example.com", password: "pw" });

		commands.loginWithPassword.mockResolvedValue(makeSession("login-code"));
		await auth.loginWithPassword({ email: "u@example.com", password: "pw", code: "123456" });
		expect(commands.loginWithPassword).toHaveBeenLastCalledWith({
			email: "u@example.com",
			password: "pw",
			code: "123456",
		});

		const passkey = makeSession("passkey");
		commands.loginWithPasskey.mockResolvedValue(passkey);
		await auth.loginWithPasskey({ credential: { id: "cred-1" } });
		expect(get(auth.state).status).toBe("authenticated");

		const plainErr = new Error("boom");
		commands.register.mockRejectedValue(plainErr);
		await expect(
			auth.register({ email: "u@example.com", password: "pw", displayName: "User" })
		).rejects.toThrow("boom");
		expect(get(auth.state).error).toBeNull();
	});

	it("refresh handles unauthorized by clearing tokens and setting anonymous", async () => {
		const { commands, tokenStore } = makeDeps();
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		commands.refresh.mockResolvedValueOnce(makeSession("ok"));
		await expect(auth.refresh()).resolves.toEqual(makeSession("ok"));

		const unauthorized = new UnderlayHttpError(401, "Unauthorized");
		commands.refresh.mockRejectedValueOnce(unauthorized);
		await expect(auth.refresh()).rejects.toBe(unauthorized);
		expect(tokenStore.clear).toHaveBeenCalledOnce();
		expect(get(auth.state).status).toBe("anonymous");
		expect(get(auth.state).error).toBe(unauthorized);
	});

	it("sets state error for http failures in register/login/passkey/refresh non-401 branch", async () => {
		const { commands, tokenStore } = makeDeps();
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		const registerErr = new UnderlayHttpError(409, "Conflict");
		commands.register.mockRejectedValueOnce(registerErr);
		await expect(
			auth.register({ email: "u@example.com", password: "pw", displayName: "User" })
		).rejects.toBe(registerErr);
		expect(get(auth.state).error).toBe(registerErr);

		const passwordErr = new UnderlayHttpError(422, "Validation");
		commands.loginWithPassword.mockRejectedValueOnce(passwordErr);
		await expect(
			auth.loginWithPassword({ email: "u@example.com", password: "pw" })
		).rejects.toBe(passwordErr);
		expect(get(auth.state).error).toBe(passwordErr);

		const passkeyErr = new UnderlayHttpError(401, "Passkey rejected");
		commands.loginWithPasskey.mockRejectedValueOnce(passkeyErr);
		await expect(
			auth.loginWithPasskey({ credential: { id: "cred-1" } })
		).rejects.toBe(passkeyErr);
		expect(get(auth.state).error).toBe(passkeyErr);

		const refreshErr = new UnderlayHttpError(500, "Server exploded");
		commands.refresh.mockRejectedValueOnce(refreshErr);
		await expect(auth.refresh()).rejects.toBe(refreshErr);
		expect(tokenStore.clear).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual(
			expect.objectContaining({
				loading: false,
				error: refreshErr,
			})
		);
	});

	it("clears state error for non-http failures in password/passkey login", async () => {
		const { commands, tokenStore } = makeDeps();
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		const httpErr = new UnderlayHttpError(401, "bad credentials");
		commands.loginWithPassword.mockRejectedValueOnce(httpErr);
		await expect(
			auth.loginWithPassword({ email: "u@example.com", password: "pw" })
		).rejects.toBe(httpErr);
		expect(get(auth.state).error).toBe(httpErr);

		const nonHttpPasswordErr = new Error("offline");
		commands.loginWithPassword.mockRejectedValueOnce(nonHttpPasswordErr);
		await expect(
			auth.loginWithPassword({ email: "u@example.com", password: "pw" })
		).rejects.toBe(nonHttpPasswordErr);
		expect(get(auth.state).error).toBeNull();

		const nonHttpPasskeyErr = new Error("aborted");
		commands.loginWithPasskey.mockRejectedValueOnce(nonHttpPasskeyErr);
		await expect(
			auth.loginWithPasskey({ credential: { id: "cred-1" } })
		).rejects.toBe(nonHttpPasskeyErr);
		expect(get(auth.state).error).toBeNull();
	});

	it("logout always clears tokens and returns anonymous, even when command fails", async () => {
		const { commands, tokenStore } = makeDeps();
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		commands.logout.mockRejectedValueOnce(new Error("network"));
		await expect(auth.logout()).rejects.toThrow("network");
		expect(tokenStore.clear).toHaveBeenCalledOnce();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: null,
		});
	});

	it("init handles non-auth errors as anonymous with error payload", async () => {
		const { commands, tokenStore } = makeDeps();
		const err = new UnderlayHttpError(500, "Server Error");
		commands.session.mockRejectedValue(err);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any, refreshOnUnauthorized: false });

		await auth.init();
		expect(tokenStore.clear).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: err,
		});
	});
});
