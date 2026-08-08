import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { scanFiles } from "@inflatable-cookie/underlay/tools/guardrails";
import { loadConfig } from "@inflatable-cookie/underlay/tools/guardrails-config";
import { moduleScopeChecks } from "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr";
import { bannedPatterns } from "@inflatable-cookie/underlay/tools/templates/banned-apis";

const tempDirs: string[] = [];

async function makeTempDir(): Promise<string> {
	const dir = await mkdtemp(path.join(tmpdir(), "underlay-guardrails-"));
	tempDirs.push(dir);
	return dir;
}

afterEach(async () => {
	vi.restoreAllMocks();
	await Promise.all(tempDirs.splice(0).map((dir) => rm(dir, { recursive: true, force: true })));
});

describe("tools/guardrails", () => {
	it("detects module-scope browser APIs while allowing guarded and function-local access", async () => {
		const dir = await makeTempDir();
		await writeFile(
			path.join(dir, "sample.ts"),
			[
				"const width = window.innerWidth;",
				"const safe = typeof window !== \"undefined\" ? window.innerWidth : 0;",
				"function handler() { return window.innerWidth; }",
			].join("\n"),
		);
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: dir,
			extensions: [".ts"],
			bannedPatterns: [],
			moduleScopeChecks,
			suppressionPrefix: "guardrails-disable",
		});

		expect(failures).toBe(1);
	});

	it("flags module-scope configureAuth/configureNightfireStrategies calls but not declarations or guarded calls", async () => {
		const dir = await makeTempDir();
		await writeFile(
			path.join(dir, "sample.ts"),
			[
				// declaration: must NOT be flagged
				"export function configureAuth(cfg: unknown) { return cfg; }",
				// guarded call: must NOT be flagged
				"if (typeof window !== \"undefined\") { configureAuth({}); }",
				// function-local call: must NOT be flagged",
				"function setup() { configureNightfireStrategies({}); }",
				// unguarded module-scope calls: 2 flagged
				"configureAuth({});",
				"configureNightfireStrategies({});",
			].join("\n"),
		);
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: dir,
			extensions: [".ts"],
			bannedPatterns: [],
			moduleScopeChecks,
			suppressionPrefix: "guardrails-disable",
		});

		expect(failures).toBe(2);
	});

	it("honors suppressions for TypeScript and scans Svelte script blocks", async () => {
		const dir = await makeTempDir();
		await writeFile(
			path.join(dir, "suppressed.ts"),
			[
				"// guardrails-disable-next-line module-scope-browser-api",
				"const width = window.innerWidth;",
			].join("\n"),
		);
		await writeFile(
			path.join(dir, "Component.svelte"),
			"<script>const saved = localStorage.getItem('x');</script>",
		);
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: dir,
			extensions: [".ts", ".svelte"],
			bannedPatterns: [],
			moduleScopeChecks,
			suppressionPrefix: "guardrails-disable",
		});

		expect(failures).toBe(1);
	});

	it("loads package-style templates from config", async () => {
		const dir = await makeTempDir();
		const configPath = path.join(dir, "guardrails.json");
		await writeFile(
			configPath,
			JSON.stringify({
				srcDir: dir,
				extensions: [".ts", ".svelte"],
				bannedPatterns: "@inflatable-cookie/underlay/tools/templates/banned-apis",
				moduleScopeChecks: "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr",
			}),
		);

		const config = await loadConfig(configPath);

		expect(config.srcDir).toBe(dir);
		expect(config.bannedPatterns.map((rule) => rule.name)).toContain(
			["window", "alert"].join("."),
		);
		expect(config.moduleScopeChecks.map((rule) => rule.name)).toContain("window.*");
	});

	it("applies banned API template rules", async () => {
		const dir = await makeTempDir();
		await writeFile(
			path.join(dir, "sample.ts"),
			`${["window", "alert"].join(".")}('stop');`,
		);
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: dir,
			extensions: [".ts"],
			bannedPatterns,
			moduleScopeChecks: [],
			suppressionPrefix: "guardrails-disable",
		});

		expect(failures).toBe(1);
	});
});
