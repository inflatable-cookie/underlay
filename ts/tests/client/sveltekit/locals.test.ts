import { beforeEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../src/client/errors";
import { authRoutes, createCookiesMock } from "./fixtures";

const mocks = vi.hoisted(() => ({
	createHttpClient: vi.fn(),
	createAuthCommands: vi.fn(),
}));

vi.mock("../../../src/client/http", () => ({
	createHttpClient: mocks.createHttpClient,
}));

vi.mock("../../../src/client/auth", () => ({
	createAuthCommands: mocks.createAuthCommands,
}));

describe("client/sveltekit auth locals", () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it("locals auth getSession rethrows non-401 and clearTokens clears cookies", async () => {
		const { createAuthHandle } = await import("../../../src/client/sveltekit");

		const commands = { session: vi.fn().mockRejectedValue(new UnderlayHttpError(500, "boom")) };
		mocks.createHttpClient.mockReturnValue({});
		mocks.createAuthCommands.mockReturnValue(commands);

		const cookies = createCookiesMock();
		const event = {
			cookies,
			fetch: vi.fn(),
			locals: {},
			url: new URL("https://example.com/app"),
		} as any;
		const resolve = vi.fn(async () => new Response("ok", { status: 200 }));

		const handle = createAuthHandle({
			baseUrl: "https://api.example.com",
			routes: authRoutes,
			cookies: { accessTokenCookie: "access", refreshTokenCookie: "refresh" },
			shouldProtect: () => false,
		});

		await handle({ event, resolve } as any);
		const authLocals = (event.locals as any).auth;

		await expect(authLocals.getSession()).rejects.toMatchObject({ status: 500 });
		await authLocals.clearTokens();
		expect(cookies.delete).toHaveBeenCalledWith("access", { path: "/" });
		expect(cookies.delete).toHaveBeenCalledWith("refresh", { path: "/" });
	});
});
