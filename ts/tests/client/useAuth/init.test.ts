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

	it("does not write tokens to the store even if session GET echoes them", async () => {
		const { commands, tokenStore } = makeDeps();
		const session = makeSession("leaky");
		commands.session.mockResolvedValue(session);
		const auth = createAuthStore({ commands: commands as any, tokenStore: tokenStore as any });

		await auth.init();

		expect(tokenStore.setAccessToken).not.toHaveBeenCalled();
		expect(tokenStore.setRefreshToken).not.toHaveBeenCalled();
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
});
