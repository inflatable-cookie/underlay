import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import { UnderlayHttpError } from "../../../src/client/errors";
import { createAuthStore } from "../../../src/client/useAuth";
import { makeDeps, makeSession, makeSessionInfo } from "./fixtures";

describe("client/useAuth init", () => {
	it("initializes as authenticated when session request succeeds", async () => {
		const { commands, tokenStore } = makeDeps();
		const session = makeSessionInfo("init");
		commands.session.mockResolvedValue(session);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.setAccessToken).not.toHaveBeenCalled();
		expect(tokenStore.setRefreshToken).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual({
			status: "authenticated",
			session,
			loading: false,
			error: null,
		});
	});

	it("does not write tokens to the store or state.session even if session GET echoes them", async () => {
		const { commands, tokenStore } = makeDeps();
		const session = makeSession("leaky");
		commands.session.mockResolvedValue(session);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.setAccessToken).not.toHaveBeenCalled();
		expect(tokenStore.setRefreshToken).not.toHaveBeenCalled();
		const storedSession = get(auth.state).session as unknown as Record<string, unknown>;
		expect(storedSession).toEqual(makeSessionInfo("leaky"));
		expect("accessToken" in storedSession).toBe(false);
		expect("refreshToken" in storedSession).toBe(false);
	});

	it("refreshes on init when session is unauthorized without storing tokens onto state.session", async () => {
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
		const storedSession = get(auth.state).session as unknown as Record<string, unknown>;
		expect(storedSession).toEqual(makeSessionInfo("refresh"));
		expect("accessToken" in storedSession).toBe(false);
		expect("refreshToken" in storedSession).toBe(false);
	});

	it.each(["during refresh", "after refresh"])(
		"shares one refresh when the init session request returns 401 %s",
		async (timing) => {
			const { commands, tokenStore } = makeDeps();
			const unauthorized = new UnderlayHttpError(401, "Unauthorized");
			const refreshed = makeSession("shared");
			let rejectSession: ((error: UnderlayHttpError) => void) | undefined;
			commands.session.mockReturnValue(
				new Promise((_, reject) => {
					rejectSession = reject;
				}),
			);
			commands.refresh.mockResolvedValue(refreshed);
			const auth = createAuthStore({ commands, tokenStore });

			const initializing = auth.init();
			const refreshing = auth.refresh();
			if (timing === "after refresh") await refreshing;
			rejectSession?.(unauthorized);
			await initializing;
			await expect(refreshing).resolves.toEqual(refreshed);

			expect(commands.refresh).toHaveBeenCalledOnce();
			expect(tokenStore.setAccessToken).toHaveBeenCalledExactlyOnceWith(refreshed.accessToken);
			expect(tokenStore.setRefreshToken).toHaveBeenCalledExactlyOnceWith(refreshed.refreshToken);
			expect(get(auth.state)).toEqual({
				status: "authenticated",
				session: makeSessionInfo("shared"),
				loading: false,
				error: null,
			});
		},
	);

	it("clears tokens once when a shared init and refresh fails with 401", async () => {
		const { commands, tokenStore } = makeDeps();
		const unauthorized = new UnderlayHttpError(401, "Unauthorized");
		const refreshErr = new UnderlayHttpError(401, "Refresh failed");
		commands.session.mockRejectedValue(unauthorized);
		commands.refresh.mockRejectedValue(refreshErr);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await Promise.all([auth.init(), expect(auth.refresh()).rejects.toBe(refreshErr)]);

		expect(commands.refresh).toHaveBeenCalledOnce();
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

	it("handles non-auth errors as anonymous with error payload", async () => {
		const { commands, tokenStore } = makeDeps();
		const err = new UnderlayHttpError(500, "Server Error");
		commands.session.mockRejectedValue(err);
		const auth = createAuthStore({
			commands: commands as any,
			tokenStore: tokenStore as any,
			refreshOnUnauthorized: false,
		});

		await auth.init();
		expect(tokenStore.clear).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: err,
		});
	});

	it("preserves stored tokens when a refresh on init fails with a transient 5xx", async () => {
		const { commands, tokenStore } = makeDeps();
		const unavailable = new UnderlayHttpError(503, "Service unavailable");
		commands.session.mockRejectedValue(new UnderlayHttpError(401, "Unauthorized"));
		commands.refresh.mockRejectedValue(unavailable);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.clear).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: unavailable,
		});
	});

	it("preserves stored tokens when a refresh on init fails with a network error", async () => {
		const { commands, tokenStore } = makeDeps();
		commands.session.mockRejectedValue(new UnderlayHttpError(401, "Unauthorized"));
		commands.refresh.mockRejectedValue(new TypeError("fetch failed"));
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.clear).not.toHaveBeenCalled();
		expect(get(auth.state)).toEqual({
			status: "anonymous",
			session: null,
			loading: false,
			error: null,
		});
	});

	it("deduplicates concurrent init calls into one session request", async () => {
		const { commands, tokenStore } = makeDeps();
		commands.session.mockImplementation(
			() =>
				new Promise((resolve) => {
					setTimeout(() => resolve(makeSessionInfo("shared")), 0);
				}),
		);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await Promise.all([auth.init(), auth.init()]);

		expect(commands.session).toHaveBeenCalledOnce();
		expect(get(auth.state)).toEqual({
			status: "authenticated",
			session: makeSessionInfo("shared"),
			loading: false,
			error: null,
		});
	});
});
