import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { UnderlayHttpError, toUserMessage } from "../../src/client/errors";

// Repo root: this file is ts/tests/client/, so up three levels.
const repoRoot = path.resolve(fileURLToPath(new URL(".", import.meta.url)), "../../..");
const yamlPath = path.join(repoRoot, "contracts/openapi/underlay.openapi.yaml");
const tsPath = path.join(repoRoot, "ts/src/client/envelopes.ts");
const dtoPath = path.join(repoRoot, "rust/crates/underlay-core/src/dto.rs");
const openapiRustPath = path.join(repoRoot, "rust/crates/underlay-http/src/openapi.rs");
const pageListRustPath = path.join(repoRoot, "rust/crates/underlay-http/src/page_list.rs");
const authErrorsRustPath = path.join(repoRoot, "rust/crates/underlay-auth/src/errors.rs");
const tsErrorsPath = path.join(repoRoot, "ts/src/client/errors.ts");

/**
 * Contract-sync drift guard. Response envelopes are declared across Rust, TS,
 * and OpenAPI without a shared code generator. The schema names stay aligned,
 * while the page-list field guard preserves the intentional raw `has_more` to
 * public-client `hasMore` normalization.
 */

type YamlValue = string | number | boolean | null | YamlValue[] | YamlObject;
type YamlObject = { [key: string]: YamlValue };

function parseYamlScalar(raw: string): YamlValue {
	const value = raw.trim();
	if (value === "{}") return {};
	if (value === "[]") return [];
	if (value === "true") return true;
	if (value === "false") return false;
	if (value === "null" || value === "~") return null;
	if (value.startsWith("[") && value.endsWith("]")) {
		const inner = value.slice(1, -1).trim();
		if (!inner) return [];
		return inner.split(",").map((item) => parseYamlScalar(item));
	}
	if (
		(value.startsWith('"') && value.endsWith('"')) ||
		(value.startsWith("'") && value.endsWith("'"))
	) {
		return value.slice(1, -1);
	}
	if (/^-?\d+(?:\.\d+)?$/.test(value)) {
		return Number(value);
	}
	return value;
}

function stripYamlComment(line: string): string {
	let quote: '"' | "'" | null = null;
	for (let i = 0; i < line.length; i += 1) {
		const ch = line[i];
		if (quote) {
			if (ch === quote && line[i - 1] !== "\\") {
				quote = null;
			}
			continue;
		}
		if (ch === '"' || ch === "'") {
			quote = ch;
		} else if (ch === "#" && (i === 0 || /\s/.test(line[i - 1]))) {
			return line.slice(0, i);
		}
	}
	return line;
}

function parseOpenApiYaml(source: string): YamlObject {
	const root: YamlObject = {};
	const stack: Array<{ indent: number; obj: YamlObject }> = [{ indent: -1, obj: root }];

	for (const rawLine of source.split(/\r?\n/)) {
		const cleaned = stripYamlComment(rawLine).replace(/\s+$/, "");
		if (!cleaned.trim()) continue;

		const indent = cleaned.length - cleaned.trimStart().length;
		const trimmed = cleaned.trim();
		const colonIdx = trimmed.indexOf(":");
		if (colonIdx <= 0) {
			throw new Error(`Unsupported OpenAPI YAML line: ${rawLine}`);
		}

		const key = trimmed.slice(0, colonIdx).trim();
		const rest = trimmed.slice(colonIdx + 1).trim();

		while (stack.length > 1 && indent <= stack[stack.length - 1].indent) {
			stack.pop();
		}
		const parent = stack[stack.length - 1].obj;

		if (rest === "") {
			const child: YamlObject = {};
			parent[key] = child;
			stack.push({ indent, obj: child });
		} else {
			parent[key] = parseYamlScalar(rest);
		}
	}

	return root;
}

// Envelope interfaces are the request/response wrappers — not the `Uuid` alias
// or the leaf `ErrorBody` payload's own scalar fields.
function tsEnvelopeNames(source: string): Set<string> {
	const names = new Set<string>();
	const re = /export interface (\w+)/g;
	let match: RegExpExecArray | null;
	while ((match = re.exec(source))) {
		names.add(match[1]);
	}
	return names;
}

function extractRustStructBlock(source: string, structName: string): string {
	const escaped = structName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
	const re = new RegExp(
		`(?:(?:#\\[[^\\]]+\\]\\s*)*)pub struct ${escaped}(?:<[^>]+>)?\\s*\\{[\\s\\S]*?\\n\\}`,
	);
	const match = source.match(re);
	if (!match) {
		throw new Error(`Missing Rust struct ${structName}`);
	}
	return match[0];
}

describe("envelope contract drift", () => {
	const tsSource = readFileSync(tsPath, "utf8");
	const yamlSource = readFileSync(yamlPath, "utf8");
	const dtoSource = readFileSync(dtoPath, "utf8");
	const openapiRustSource = readFileSync(openapiRustPath, "utf8");
	const pageListSource = readFileSync(pageListRustPath, "utf8");

	const parsedYaml = parseOpenApiYaml(yamlSource);
	const components = parsedYaml.components as YamlObject;
	const yamlSchemas = components.schemas as Record<string, YamlObject>;

	it("declares the same envelope set in TS and the OpenAPI YAML", () => {
		const ts = tsEnvelopeNames(tsSource);
		const yaml = new Set(Object.keys(yamlSchemas));
		expect([...ts].sort()).toEqual([...yaml].sort());
	});

	it("includes PagedListResponse on both surfaces", () => {
		expect(tsSource).toContain("interface PagedListResponse");
		expect(yamlSchemas.PagedListResponse).toBeDefined();
	});

	it("keeps raw page-list casing separate from the TS client boundary", () => {
		const rustPageList = extractRustStructBlock(pageListSource, "PageList");
		const yamlPageList = yamlSchemas.PagedListResponse;
		const yamlPageListProps = yamlPageList.properties as Record<string, YamlObject>;
		const tsPageList = tsSource.match(/export interface PagedListResponse<T> \{[\s\S]*?\n\}/)?.[0];

		expect(rustPageList).toContain("pub data: Vec<T>");
		expect(rustPageList).toContain("pub total: u64");
		expect(rustPageList).toContain("pub has_more: bool");
		expect(rustPageList).not.toContain("hasMore");
		expect(yamlPageList.required).toEqual(["data", "total", "has_more"]);
		expect(Object.keys(yamlPageListProps).sort()).toEqual(["data", "has_more", "total"]);
		expect(yamlPageListProps.has_more).toEqual({ type: "boolean" });
		expect(yamlPageListProps.hasMore).toBeUndefined();
		expect(tsPageList).toContain("hasMore: boolean");
		expect(tsPageList).not.toContain("has_more");
	});

	it("keeps ErrorBody and ErrorEnvelope aligned across TS, OpenAPI YAML, underlay-core, and underlay-http", () => {
		const yamlErrorBody = yamlSchemas.ErrorBody;
		const yamlErrorBodyProps = yamlErrorBody.properties as Record<string, YamlObject>;
		expect(yamlErrorBody.required).toEqual(["code", "message"]);
		expect(Object.keys(yamlErrorBodyProps).sort()).toEqual(["code", "fieldErrors", "message"]);
		expect(yamlErrorBodyProps.code).toEqual({ type: "string" });
		expect(yamlErrorBodyProps.message).toEqual({ type: "string" });
		expect(yamlErrorBodyProps.fieldErrors).toEqual({
			type: "object",
			additionalProperties: { type: "string" },
		});

		const yamlErrorEnvelope = yamlSchemas.ErrorEnvelope;
		const yamlErrorEnvelopeProps = yamlErrorEnvelope.properties as Record<string, YamlObject>;
		expect(yamlErrorEnvelope.required).toEqual(["error"]);
		expect(yamlErrorEnvelopeProps.error).toEqual({ $ref: "#/components/schemas/ErrorBody" });

		const tsErrorBody = tsSource.match(/export interface ErrorBody \{[\s\S]*?\n\}/)?.[0];
		expect(tsErrorBody).toMatch(/code:\s*string/);
		expect(tsErrorBody).toMatch(/message:\s*string/);
		expect(tsErrorBody).toMatch(/fieldErrors\?:\s*Record<string,\s*string>/);

		const dtoErrorEnvelope = extractRustStructBlock(dtoSource, "ErrorEnvelope");
		expect(dtoErrorEnvelope).toContain('#[serde(rename_all = "snake_case")]');
		expect(dtoErrorEnvelope).toContain("pub error: ErrorBody");

		const dtoErrorBody = extractRustStructBlock(dtoSource, "ErrorBody");
		expect(dtoErrorBody).toContain('#[serde(rename_all = "snake_case")]');
		expect(dtoErrorBody).toContain("pub code: String");
		expect(dtoErrorBody).toContain("pub message: String");
		expect(dtoErrorBody).toContain('#[serde(rename = "fieldErrors")]');
		expect(dtoErrorBody).toContain('#[serde(skip_serializing_if = "Option::is_none")]');
		expect(dtoErrorBody).toContain("pub field_errors: Option<HashMap<String, String>>");

		const apiErrorEnvelope = extractRustStructBlock(openapiRustSource, "ApiErrorEnvelope");
		expect(apiErrorEnvelope).toContain("ToSchema");
		expect(apiErrorEnvelope).toContain('#[serde(rename_all = "snake_case")]');
		expect(apiErrorEnvelope).toContain("pub error: ApiErrorBody");

		const apiErrorBody = extractRustStructBlock(openapiRustSource, "ApiErrorBody");
		expect(apiErrorBody).toContain("ToSchema");
		expect(apiErrorBody).toContain('#[serde(rename_all = "snake_case")]');
		expect(apiErrorBody).toContain("pub code: String");
		expect(apiErrorBody).toContain("pub message: String");
		expect(apiErrorBody).toContain('#[serde(rename = "fieldErrors")]');
		expect(apiErrorBody).toContain('#[serde(skip_serializing_if = "Option::is_none")]');
		expect(apiErrorBody).toContain("pub field_errors: Option<HashMap<String, String>>");
	});

	it("keeps SingleResponse and ListResponse aligned across TS, OpenAPI YAML, underlay-core, and underlay-http", () => {
		expect(yamlSchemas.SingleResponse.required).toEqual(["data"]);
		expect((yamlSchemas.SingleResponse.properties as YamlObject).data).toEqual({});
		expect(yamlSchemas.ListResponse.required).toEqual(["data"]);
		expect((yamlSchemas.ListResponse.properties as YamlObject).data).toEqual({
			type: "array",
			items: {},
		});

		const dtoSingle = extractRustStructBlock(dtoSource, "SingleResponse");
		expect(dtoSingle).toContain('#[serde(rename_all = "snake_case")]');
		expect(dtoSingle).toContain("pub data: T");

		const dtoList = extractRustStructBlock(dtoSource, "ListResponse");
		expect(dtoList).toContain('#[serde(rename_all = "snake_case")]');
		expect(dtoList).toContain("pub data: Vec<T>");

		const apiSingle = extractRustStructBlock(openapiRustSource, "ApiSingleResponse");
		expect(apiSingle).toContain("ToSchema");
		expect(apiSingle).toContain('#[serde(rename_all = "snake_case")]');
		expect(apiSingle).toContain("pub data: T");

		const apiList = extractRustStructBlock(openapiRustSource, "ApiListResponse");
		expect(apiList).toContain("ToSchema");
		expect(apiList).toContain('#[serde(rename_all = "snake_case")]');
		expect(apiList).toContain("pub data: Vec<T>");
	});

	it("keeps TS client auth error codes aligned with underlay-auth AuthError::code()", () => {
		const authErrorsRust = readFileSync(authErrorsRustPath, "utf8");
		const tsErrorsSource = readFileSync(tsErrorsPath, "utf8");

		const codeFnMatch = authErrorsRust.match(
			/pub fn code\(&self\) -> &'static str \{[\s\S]*?\n    \}/,
		);
		expect(codeFnMatch).not.toBeNull();

		const rustCodes: string[] = [];
		const rustRe = /AuthError::\w+(?:\([^)]*\)|\s*\{[^}]*\})?\s*=>\s*"(auth\.[a-z0-9_]+)"/g;
		let match: RegExpExecArray | null;
		while ((match = rustRe.exec(codeFnMatch![0]))) {
			rustCodes.push(match[1]);
		}

		expect(rustCodes.length).toBeGreaterThan(0);
		const rustCodeSet = new Set(rustCodes);
		expect(rustCodeSet.size).toBe(rustCodes.length);

		// Strip line and block comments before scanning for auth.* literals so
		// commented-out comparisons cannot satisfy the drift check.
		const tsCodeWithoutComments = tsErrorsSource
			.replace(/\/\*[\s\S]*?\*\//g, "")
			.replace(/\/\/[^\n]*/g, "");

		const tsCodes = new Set<string>();
		const tsRe = /"(auth\.[a-z0-9_]+)"/g;
		while ((match = tsRe.exec(tsCodeWithoutComments))) {
			tsCodes.add(match[1]);
		}

		expect(tsCodes.size).toBeGreaterThan(0);
		for (const code of tsCodes) {
			expect(
				rustCodeSet.has(code),
				`TS auth error code "${code}" missing from AuthError::code()`,
			).toBe(true);
		}

		const expectedSessionExpiryCodes = [
			"auth.session_expired",
			"auth.session_revoked",
			"auth.token_invalid",
			"auth.token_fingerprint_mismatch",
		];

		expect(tsCodes).toEqual(new Set(expectedSessionExpiryCodes));

		// Exercise the real client behavior for every session-expiry code to
		// confirm each maps to the session-expired message rather than the
		// generic 401 fallback.
		for (const code of expectedSessionExpiryCodes) {
			const error = new UnderlayHttpError(401, "", {
				error: { code, message: "" },
			});
			expect(toUserMessage(error), `toUserMessage failed to handle ${code}`).toBe(
				"Your session has expired. Please sign in again.",
			);
		}

		// Control: an unrelated 401 auth code falls back to the generic prompt.
		const genericUnauthorized = new UnderlayHttpError(401, "", {
			error: { code: "auth.unauthorized", message: "" },
		});
		expect(toUserMessage(genericUnauthorized)).toBe("Please sign in to continue.");
	});
});
