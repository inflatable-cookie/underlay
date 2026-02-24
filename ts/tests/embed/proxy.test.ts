import { describe, expect, it, vi } from "vitest";
import {
	CORS_BLOCKED_PROVIDERS,
	createProxyAwareLookup,
	proxyResponseToMeta,
	requiresServerProxy,
} from "../../src/embed/proxy";

describe("embed/proxy", () => {
	it("identifies providers requiring server proxy", () => {
		expect(CORS_BLOCKED_PROVIDERS).toContain("audioboom");
		expect(requiresServerProxy("audioboom")).toBe(true);
		expect(requiresServerProxy("youtube")).toBe(false);
	});

	it("maps proxy response to embed meta", () => {
		expect(proxyResponseToMeta({ success: false, error: "nope" })).toBeNull();
		expect(
			proxyResponseToMeta({
				success: true,
				title: "Title",
				description: null,
				duration: 120,
				thumbnailUrl: "https://img",
				authorName: null,
			})
		).toEqual({
			title: "Title",
			description: undefined,
			duration: 120,
			thumbnailUrl: "https://img",
			authorName: undefined,
		});
	});

	it("routes through proxy providers and falls back to base lookup", async () => {
		const proxyFn = vi.fn().mockResolvedValue({ success: true, title: "Proxy Title" });
		const baseLookup = vi.fn().mockResolvedValue({ title: "Base Title" });
		const lookup = createProxyAwareLookup({ proxyFn }, baseLookup as any);

		await expect(
			lookup({ provider: "audioboom", id: "123", embedType: "single" } as any)
		).resolves.toEqual({
			title: "Proxy Title",
			description: undefined,
			duration: undefined,
			thumbnailUrl: undefined,
			authorName: undefined,
		});
		expect(proxyFn).toHaveBeenCalledWith({ provider: "audioboom", id: "123", embedType: "single" });
		expect(baseLookup).not.toHaveBeenCalled();

		await expect(lookup({ provider: "youtube", id: "abc" } as any)).resolves.toEqual({ title: "Base Title" });
		expect(baseLookup).toHaveBeenCalledWith({ provider: "youtube", id: "abc" }, undefined);
	});

	it("returns null when proxy request throws and supports custom proxy provider list", async () => {
		const proxyFn = vi.fn().mockRejectedValue(new Error("network"));
		const baseLookup = vi.fn().mockResolvedValue({ title: "Base" });
		const lookup = createProxyAwareLookup({ proxyFn, proxyProviders: ["youtube"] }, baseLookup as any);

		await expect(lookup({ provider: "youtube", id: "abc" } as any)).resolves.toBeNull();
		await expect(lookup({ provider: "vimeo", id: "v1" } as any)).resolves.toEqual({ title: "Base" });
	});
});
