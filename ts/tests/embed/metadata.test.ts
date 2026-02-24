import { describe, expect, it, vi } from "vitest";
import {
	createMetaCache,
	formatDuration,
	lookupMeta,
	lookupMetaBatch,
	lookupMetaById,
	lookupMetaWithResult,
	parseDuration,
} from "../../src/embed/metadata";
import { defaultRegistry } from "../../src/embed/providers";
import type { EmbedProvider, ParsedEmbed } from "../../src/embed/types";

describe("embed/metadata", () => {
	it("returns null when provider is unknown or has no lookupMeta", async () => {
		const unknownParsed: ParsedEmbed = { provider: "unknown-provider", id: "abc" };
		expect(await lookupMeta(unknownParsed)).toBeNull();
		expect(await lookupMetaById("unknown-provider", "abc")).toBeNull();
	});

	it("performs lookups for registered providers and catches errors", async () => {
		const providerName = "metadata-test-provider";
		let calls = 0;
		const provider: EmbedProvider = {
			name: providerName,
			parse: () => null,
			getEmbedUrl: () => "",
			lookupMeta: async (id) => {
				calls += 1;
				if (id === "throw") throw new Error("boom");
				return { title: `title:${id}` };
			},
		};
		defaultRegistry.register(provider);

		const parsed: ParsedEmbed = { provider: providerName, id: "ok-id" };
		expect(await lookupMeta(parsed)).toEqual({ title: "title:ok-id" });
		expect(await lookupMetaById(providerName, "ok-2")).toEqual({
			title: "title:ok-2",
		});
		expect(await lookupMeta({ provider: providerName, id: "throw" })).toBeNull();
		expect(calls).toBe(3);
	});

	it("returns detailed lookup results", async () => {
		expect(
			await lookupMetaWithResult({ provider: "definitely-missing", id: "x" })
		).toEqual({
			success: false,
			error: "Unknown provider: definitely-missing",
		});

		const noMetaProvider = "metadata-no-lookup-provider";
		defaultRegistry.register({
			name: noMetaProvider,
			parse: () => null,
			getEmbedUrl: () => "",
		});
		expect(
			await lookupMetaWithResult({ provider: noMetaProvider, id: "x" })
		).toEqual({
			success: false,
			error: `Provider "${noMetaProvider}" does not support metadata lookup`,
		});

		const nullMetaProvider = "metadata-null-provider";
		defaultRegistry.register({
			name: nullMetaProvider,
			parse: () => null,
			getEmbedUrl: () => "",
			lookupMeta: async () => null,
		});
		expect(
			await lookupMetaWithResult({ provider: nullMetaProvider, id: "x" })
		).toEqual({
			success: false,
			error: "Metadata lookup returned no results",
		});

		const thrownStringProvider = "metadata-string-error-provider";
		defaultRegistry.register({
			name: thrownStringProvider,
			parse: () => null,
			getEmbedUrl: () => "",
			lookupMeta: async () => {
				throw "nope";
			},
		});
		expect(
			await lookupMetaWithResult({ provider: thrownStringProvider, id: "x" })
		).toEqual({
			success: false,
			error: "Metadata lookup failed",
		});
	});

	it("supports batch lookup and cache behavior", async () => {
		const providerName = "metadata-cache-provider";
		let calls = 0;
		defaultRegistry.register({
			name: providerName,
			parse: () => null,
			getEmbedUrl: () => "",
			lookupMeta: async (id) => {
				calls += 1;
				return { title: `cached:${id}` };
			},
		});

		const embeds: ParsedEmbed[] = [
			{ provider: providerName, id: "a" },
			{ provider: providerName, id: "b" },
		];
		const batch = await lookupMetaBatch(embeds);
		expect(batch.get(`${providerName}:a`)).toEqual({ title: "cached:a" });
		expect(batch.get(`${providerName}:b`)).toEqual({ title: "cached:b" });

		const cache = createMetaCache();
		expect(cache.get(providerName, "a")).toBeNull();
		const fetched = await cache.getOrFetch({ provider: providerName, id: "a" });
		expect(fetched).toEqual({ title: "cached:a" });
		expect(cache.has(providerName, "a")).toBe(true);
		expect(await cache.getOrFetch({ provider: providerName, id: "a" })).toEqual({
			title: "cached:a",
		});
		expect(calls).toBe(3);

		cache.clear();
		expect(cache.has(providerName, "a")).toBe(false);
	});

	it("expires cache entries after TTL and supports direct set/get", () => {
		vi.useFakeTimers();
		const now = vi.spyOn(Date, "now");
		now.mockReturnValue(1_000);

		const cache = createMetaCache();
		cache.set("youtube", "id1", { title: "hello" });
		expect(cache.get("youtube", "id1")).toEqual({ title: "hello" });

		now.mockReturnValue(1_000 + 5 * 60 * 1000 + 1);
		expect(cache.get("youtube", "id1")).toBeNull();
		expect(cache.has("youtube", "id1")).toBe(false);

		vi.useRealTimers();
	});

	it("formats and parses duration values", () => {
		expect(formatDuration(undefined)).toBeNull();
		expect(formatDuration(0)).toBe("0:00");
		expect(formatDuration(65)).toBe("1:05");
		expect(formatDuration(3661)).toBe("1:01:01");

		expect(parseDuration("01:01")).toBe(61);
		expect(parseDuration("01:01:01")).toBe(3661);
		expect(parseDuration("abc")).toBeNull();
		expect(parseDuration("10")).toBeNull();
	});
});
