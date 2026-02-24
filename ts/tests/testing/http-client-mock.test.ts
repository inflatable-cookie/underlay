import { describe, expect, it } from "vitest";
import { createMockHttpClient } from "../../src/testing/http-client-mock";

describe("testing/http-client-mock", () => {
	it("uses nextResponse for request and tracks calls", async () => {
		const client = createMockHttpClient({ nextResponse: { ok: true } });

		await expect(client.request<{ ok: boolean }>({ method: "GET", path: "/a" })).resolves.toEqual({
			ok: true,
		});
		expect(client.calls).toEqual([
			{
				method: "GET",
				path: "/a",
				body: undefined,
				headers: undefined,
			},
		]);
	});

	it("supports keyed responders, function responders, and meta response coercion", async () => {
		const client = createMockHttpClient({
			responses: {
				"GET /meta": {
					status: 201,
					headers: { "x-test": "1" },
					body: { id: "m1" },
				},
				"POST /fn": (call) => ({ echoed: call.body }),
			},
		});
		client.setNextResponse({ fallback: true });

		await expect(client.get<{ fallback: boolean }>("/none")).resolves.toEqual({ fallback: true });
		await expect(client.post<{ echoed: unknown }>("/fn", { a: 1 })).resolves.toEqual({
			echoed: { a: 1 },
		});
		await expect(client.requestWithMeta<{ id: string }>({ method: "GET", path: "/meta" })).resolves.toEqual(
			{
				status: 201,
				headers: { "x-test": "1" },
				body: { id: "m1" },
			}
		);
		await expect(
			client.requestWithMeta<{ fallback: boolean }>(
				{ method: "GET", path: "/coerce" },
				{ acceptedStatuses: [200] }
			)
		).resolves.toEqual({
			status: 200,
			headers: {},
			body: { fallback: true },
		});
	});

	it("exposes convenience methods and reset semantics", async () => {
		const client = createMockHttpClient();
		client.nextResponse = { ok: "setter" };
		expect(client.nextResponse).toEqual({ ok: "setter" });

		client.setResponse("PUT", "/u", { done: "put" });
		client.setResponse("PATCH", "/p", { done: "patch" });
		client.setResponse("DELETE", "/d", { done: "delete" });
		client.setResponse("GET", "/gm", {
			status: 204,
			headers: {},
			body: null,
		});

		await expect(client.put("/u", { x: 1 }, { h: "1" })).resolves.toEqual({ done: "put" });
		await expect(client.patch("/p", { y: 2 })).resolves.toEqual({ done: "patch" });
		await expect(client.delete("/d")).resolves.toEqual({ done: "delete" });
		await expect(client.getWithMeta("/gm")).resolves.toEqual({ status: 204, headers: {}, body: null });

		expect(client.calls).toHaveLength(4);
		client.reset();
		expect(client.calls).toHaveLength(0);
		expect(client.nextResponse).toBeUndefined();
		expect(client.responses.size).toBe(0);
	});
});
