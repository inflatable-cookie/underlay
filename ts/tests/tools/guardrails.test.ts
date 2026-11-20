import { spawnSync } from "node:child_process";
import { mkdir, mkdtemp, rm, symlink, writeFile } from "node:fs/promises";
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
				'const safe = typeof window !== "undefined" ? window.innerWidth : 0;',
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
				'if (typeof window !== "undefined") { configureAuth({}); }',
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
		expect(config.bannedPatterns.map((rule) => rule.name)).toContain(["window", "alert"].join("."));
		expect(config.moduleScopeChecks.map((rule) => rule.name)).toContain("window.*");
	});

	it("applies banned API template rules", async () => {
		const dir = await makeTempDir();
		await writeFile(path.join(dir, "sample.ts"), `${["window", "alert"].join(".")}('stop');`);
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

	it("passes custom suppressionPrefix through to suppression checks", async () => {
		const dir = await makeTempDir();
		const bannedRule = ["window", "alert"].join(".");
		await writeFile(
			path.join(dir, "sample.ts"),
			[
				"// custom-ignore-next-line module-scope-browser-api",
				"const okWidth = window.innerWidth;",
				"function notify() {",
				`  // custom-ignore-next-line ${bannedRule}`,
				`  ${bannedRule}('suppressed');`,
				`  // guardrails-disable-next-line ${bannedRule}`,
				`  ${bannedRule}('unsuppressed');`,
				"}",
				"// guardrails-disable-next-line module-scope-browser-api",
				"const badWidth = window.innerWidth;",
			].join("\n"),
		);
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: dir,
			extensions: [".ts"],
			bannedPatterns,
			moduleScopeChecks,
			suppressionPrefix: "custom-ignore",
		});

		expect(failures).toBe(2);
	});

	it("fails closed when explicit config file is missing or template import fails", async () => {
		const dir = await makeTempDir();
		const missingConfig = path.join(dir, "missing.json");
		await expect(loadConfig(missingConfig)).rejects.toThrow(/Could not load config/);

		const badTemplateConfig = path.join(dir, "bad-template.json");
		await writeFile(
			badTemplateConfig,
			JSON.stringify({
				srcDir: dir,
				bannedPatterns: "@inflatable-cookie/underlay/tools/templates/does-not-exist",
			}),
		);
		await expect(loadConfig(badTemplateConfig)).rejects.toThrow(
			/Could not load bannedPatterns template/,
		);

		const badSsrTemplateConfig = path.join(dir, "bad-ssr-template.json");
		await writeFile(
			badSsrTemplateConfig,
			JSON.stringify({
				srcDir: dir,
				moduleScopeChecks: "@inflatable-cookie/underlay/tools/templates/does-not-exist",
			}),
		);
		await expect(loadConfig(badSsrTemplateConfig)).rejects.toThrow(
			/Could not load moduleScopeChecks template/,
		);

		const cliMissing = spawnSync(
			"bun",
			[path.resolve("ts/src/tools/guardrails.ts"), "--config", missingConfig],
			{ encoding: "utf8" },
		);
		expect(cliMissing.status).not.toBe(0);

		const cliBadTemplate = spawnSync(
			"bun",
			[path.resolve("ts/src/tools/guardrails.ts"), "--config", badTemplateConfig],
			{ encoding: "utf8" },
		);
		expect(cliBadTemplate.status).not.toBe(0);
	});

	it("scans directories with spaces/special chars and runs CLI via symlink, relative path, and spaces", async () => {
		const dir = await makeTempDir();
		const srcWithSpaces = path.join(dir, "src with spaces", "sub#dir");
		await mkdir(srcWithSpaces, { recursive: true });
		await writeFile(path.join(srcWithSpaces, "clean#file.ts"), "export const ok = 1;\n");
		await writeFile(path.join(srcWithSpaces, "bad file.ts"), "const w = window.innerWidth;\n");
		vi.spyOn(console, "error").mockImplementation(() => undefined);

		const failures = await scanFiles({
			srcDir: path.join(dir, "src with spaces"),
			extensions: [".ts"],
			bannedPatterns: [],
			moduleScopeChecks,
			suppressionPrefix: "guardrails-disable",
		});
		expect(failures).toBe(1);

		const cleanDir = path.join(dir, "clean src");
		await mkdir(cleanDir, { recursive: true });
		await writeFile(path.join(cleanDir, "ok.ts"), "export const ok = 1;\n");

		const linkDir = path.join(dir, "bin dir");
		await mkdir(linkDir, { recursive: true });
		const linkPath = path.join(linkDir, "guardrails-link.ts");
		await symlink(path.resolve("ts/src/tools/guardrails.ts"), linkPath);

		const viaRelative = spawnSync("bun", ["ts/src/tools/guardrails.ts", "--src", cleanDir], {
			encoding: "utf8",
		});
		expect(viaRelative.status).toBe(0);
		expect(viaRelative.stdout).toContain("Guardrails passed");

		const viaSymlinkWithSpaces = spawnSync("bun", [linkPath, "--src", cleanDir], {
			encoding: "utf8",
		});
		expect(viaSymlinkWithSpaces.status).toBe(0);
		expect(viaSymlinkWithSpaces.stdout).toContain("Guardrails passed");
	});
});
