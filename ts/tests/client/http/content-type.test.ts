import { afterEach, describe, expect, it, vi } from "vitest";
import { UnderlayHttpError } from "../../../src/client/errors";
import { createHttpClient } from "../../../src/client/http";

function jsonResponse(body: unknown, status = 200, contentType = "application/json") {
	return {
		ok: status >= 200 && status < 300,
		status,
		headers: new Headers({ "content-type": contentType }),
		json: async () => body,
	};
}

function untypedResponse(body: string, status = 200) {
	return {
		ok: status >= 200 && status < 300,
		status,
		text: async () => body,
	};
}

describe("createHttpClient content-type handling", () => {
	afterEach(() => {
		vi.restoreAllMocks();
	});

	it("parses JSON media types case-insensitively and with parameters", async () => {
		const fetchMock = vi
			.fn()
			.mockResolvedValue(
				jsonResponse({ data: { id: "1" } }, 200, "Application/JSON; charset=utf-8"),
			);
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(client.get("/thing")).resolves.toEqual({ data: { id: "1" } });
	});

	it("parses structured +json error envelopes", async () => {
		const fetchMock = vi
			.fn()
			.mockResolvedValue(
				jsonResponse(
					{ error: { code: "problem.validation", message: "Bad input" } },
					422,
					"application/problem+json",
				),
			);
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(client.get("/thing")).rejects.toMatchObject({
			status: 422,
			message: "Bad input",
			envelope: { error: { code: "problem.validation" } },
		});
	});

	it("treats a headerless empty response as a null body instead of a JSON error", async () => {
		const fetchMock = vi.fn().mockResolvedValue(untypedResponse(""));
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		const response = await client.getWithMeta("/empty");

		expect(response.status).toBe(200);
		expect(response.body).toBeNull();
	});

	it("returns headerless non-JSON bodies as text", async () => {
		const fetchMock = vi.fn().mockResolvedValue(untypedResponse("plain text response"));
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(client.get("/text")).resolves.toBe("plain text response");
	});

	it("still parses headerless JSON bodies", async () => {
		const fetchMock = vi.fn().mockResolvedValue(untypedResponse('{"id":"1"}'));
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(client.get("/json")).resolves.toEqual({ id: "1" });
	});

	it("does not invent an error envelope for headerless non-JSON error bodies", async () => {
		const fetchMock = vi.fn().mockResolvedValue(untypedResponse("upstream exploded", 400));
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		const error = await client.get("/broken").catch((err: unknown) => err);

		expect(error).toBeInstanceOf(UnderlayHttpError);
		expect(error).toMatchObject({ status: 400, message: "HTTP 400" });
		expect((error as UnderlayHttpError).envelope).toBeUndefined();
	});

	it("still parses JSON error envelopes when the header is missing", async () => {
		const fetchMock = vi
			.fn()
			.mockResolvedValue(untypedResponse('{"error":{"code":"bad","message":"Nope"}}', 400));
		const client = createHttpClient({ baseUrl: "https://api.example.com", fetch: fetchMock });

		await expect(client.get("/broken")).rejects.toMatchObject({
			status: 400,
			message: "Nope",
			envelope: { error: { code: "bad", message: "Nope" } },
		});
	});
});
