import { cp, mkdtemp, rm, writeFile } from "node:fs/promises";
import { execFile } from "node:child_process";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { afterEach, describe, expect, it } from "vitest";
import {
	checkEnvAuthority,
	ENV_AUTHORITY_RULE_IDS,
	formatEnvAuthorityReport,
} from "@inflatable-cookie/underlay/tools/env-authority";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const fixturesRoot = path.join(
	path.dirname(fileURLToPath(import.meta.url)),
	"../fixtures/env-authority",
);
const publishedBin = path.join(repoRoot, "ts/bin/underlay-env-authority.ts");
const execFileAsync = promisify(execFile);

const tempDirs: string[] = [];

async function makeTempDir(): Promise<string> {
	const dir = await mkdtemp(path.join(tmpdir(), "underlay-env-authority-"));
	tempDirs.push(dir);
	return dir;
}

async function loadFixture(name: string): Promise<string> {
	const dir = await makeTempDir();
	await cp(path.join(fixturesRoot, name), dir, { recursive: true });
	return dir;
}

afterEach(async () => {
	await Promise.all(tempDirs.splice(0).map((dir) => rm(dir, { recursive: true, force: true })));
});

describe("tools/env-authority", () => {
	it("accepts a declared env/secret inventory", async () => {
		const violations = await checkEnvAuthority(await loadFixture("compliant"));
		expect(violations).toEqual([]);
	});

	it("accepts an empty required-secrets file without inventing keys", async () => {
		const violations = await checkEnvAuthority(await loadFixture("empty-required-ok"));
		expect(violations).toEqual([]);
	});

	it("accepts a tree with no runtime env reader and no authority files", async () => {
		const violations = await checkEnvAuthority(await loadFixture("no-env-reader"));
		expect(violations).toEqual([]);
	});

	it("ignores env-reader mentions that live only in comments", async () => {
		const violations = await checkEnvAuthority(await loadFixture("comment-only-reader"));
		expect(violations).toEqual([]);
	});

	it("ignores env reads that live only in tests", async () => {
		const violations = await checkEnvAuthority(await loadFixture("test-only-reader"));
		expect(violations).toEqual([]);
	});

	it("ignores env reads that live only in fixtures", async () => {
		const violations = await checkEnvAuthority(await loadFixture("fixture-only-reader"));
		expect(violations).toEqual([]);
	});

	it("flags a runtime env reader with no env-manifest", async () => {
		const violations = await checkEnvAuthority(await loadFixture("missing-manifest"));
		expect(violations.map((v) => v.ruleId)).toContain(ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_MISSING);
		expect(violations.find((v) => v.ruleId === ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_MISSING)?.detail)
			.toContain("apps/api/src/main.rs");
	});

	it("flags a runtime env reader with no required-secrets file", async () => {
		const violations = await checkEnvAuthority(await loadFixture("missing-required-secrets"));
		expect(violations.map((v) => v.ruleId)).toContain(
			ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_MISSING,
		);
	});

	it("flags a TypeScript process.env reader without authority files", async () => {
		const violations = await checkEnvAuthority(await loadFixture("process-env-reader"));
		expect(violations.map((v) => v.ruleId)).toEqual([
			ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_MISSING,
			ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_MISSING,
		]);
	});

	it("flags value assignments in both authority files without echoing secrets", async () => {
		const dir = await loadFixture("invalid-manifest");
		const violations = await checkEnvAuthority(dir);
		const ruleIds = violations.map((v) => v.ruleId);
		expect(ruleIds).toContain(ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_INVALID);
		expect(ruleIds).toContain(ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_INVALID);

		const report = formatEnvAuthorityReport(dir, violations);
		const details = violations.map((v) => v.detail).join("\n");
		expect(details).toContain("line 1: DATABASE_URL has a value");
		expect(details).toContain("line 1: AUTH_JWT_PRIVATE_KEY has a value");
		expect(details).not.toContain("super-secret-value");
		expect(report).not.toContain("super-secret-value");
	});

	it("flags required secrets that are not declared in the manifest", async () => {
		const violations = await checkEnvAuthority(await loadFixture("undeclared-required-secret"));
		expect(violations.map((v) => v.ruleId)).toContain(
			ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRET_UNDECLARED,
		);
	});

	it("does not read secret values from .env files", async () => {
		const dir = await loadFixture("secret-file-ignored");
		await writeFile(
			path.join(dir, ".env"),
			"DATABASE_URL=super-secret-value\nAUTH_JWT_PRIVATE_KEY=do-not-print-me\n",
		);
		const violations = await checkEnvAuthority(dir);
		expect(violations).toEqual([]);

		const report = formatEnvAuthorityReport(dir, violations);
		expect(report).not.toContain("super-secret-value");
		expect(report).not.toContain("do-not-print-me");
	});

	it("returns deterministic sorted diagnostics", async () => {
		const violations = await checkEnvAuthority(await loadFixture("process-env-reader"));
		const sorted = [...violations].sort((a, b) => {
			if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
			if (a.path !== b.path) return a.path.localeCompare(b.path);
			return a.detail.localeCompare(b.detail);
		});
		expect(violations).toEqual(sorted);
	});

	it("runs through the published underlay-env-authority bin entry", async () => {
		const fixtureDir = await loadFixture("compliant");
		const { stdout } = await execFileAsync("bun", [publishedBin, fixtureDir], {
			cwd: repoRoot,
		});

		expect(stdout).toContain("All env authority checks passed.");
	});
});
