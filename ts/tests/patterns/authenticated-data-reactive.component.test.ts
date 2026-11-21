// @vitest-environment jsdom
import { render, screen, waitFor } from "@testing-library/svelte";
import { describe, expect, it, vi } from "vitest";
import { configureAuth } from "../../src/patterns/auth";
import AuthenticatedDataReactiveHarness from "./AuthenticatedDataReactiveHarness.svelte";

describe("patterns/authenticated-data reactive configureAuth", () => {
	it("triggers auto-fetch when configureAuth() is called after child component mount", async () => {
		const fetcher = vi.fn(async (_fetch: typeof fetch, token: string) => `loaded:${token}`);

		render(AuthenticatedDataReactiveHarness, { fetcher });

		// Before configureAuth is called with auto-fetch getters, fetcher should not have run.
		await new Promise((resolve) => setTimeout(resolve, 20));
		expect(fetcher).not.toHaveBeenCalled();

		// Simulate parent layout's onMount calling configureAuth().
		configureAuth({
			getToken: () => "late-token",
			onRefresh: async () => null,
			getAuthLoading: () => false,
			getCurrentUser: () => ({ id: "u1" }),
		});

		await waitFor(() => expect(fetcher).toHaveBeenCalledTimes(1));
		expect(await screen.findByText("loaded:late-token")).toBeTruthy();
	});
});
