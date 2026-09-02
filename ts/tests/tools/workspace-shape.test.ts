import { cp, mkdtemp, mkdir, rm, symlink, writeFile } from "node:fs/promises";
import { execFile } from "node:child_process";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { afterEach, describe, expect, it } from "vitest";
import {
	checkWorkspaceShape,
	formatWorkspaceShapeReport,
	WORKSPACE_SHAPE_RULE_IDS,
} from "@inflatable-cookie/underlay/tools/workspace-shape";
import {
	formatRetiredDisposableCleanupCommand,
	posixShellSingleQuote,
} from "../../src/tools/workspace-shape/model.js";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const fixturesRoot = path.join(
	path.dirname(fileURLToPath(import.meta.url)),
	"../fixtures/workspace-shape",
);
const publishedBin = path.join(repoRoot, "ts/bin/underlay-workspace-shape.ts");
const execFileAsync = promisify(execFile);

const tempDirs: string[] = [];

async function makeTempDir(): Promise<string> {
	const dir = await mkdtemp(path.join(tmpdir(), "underlay-workspace-shape-"));
	tempDirs.push(dir);
	return dir;
}

async function loadFixture(name: string): Promise<string> {
	const dir = await makeTempDir();
	await cp(path.join(fixturesRoot, name), dir, { recursive: true });
	await writeFile(path.join(dir, ".git"), "gitdir: fixture\n");

	if (name === "nested-git") {
		await writeFile(path.join(dir, "apps/app/.git"), "gitdir: fixture\n");
	}

	if (name === "retired-top-level-package") {
		await mkdir(path.join(dir, "app", "node_modules"), { recursive: true });
	}

	return dir;
}

async function loadOutsideRootFixture(): Promise<string> {
	const parent = await makeTempDir();
	const repoDir = path.join(parent, "repo");
	await cp(path.join(fixturesRoot, "workspace-outside-root"), repoDir, { recursive: true });
	await cp(path.join(fixturesRoot, "_outside-repo"), path.join(parent, "outside"), {
		recursive: true,
	});
	await writeFile(path.join(repoDir, ".git"), "gitdir: fixture\n");
	return repoDir;
}

async function loadSymlinkOutsideFixture(): Promise<string> {
	const parent = await makeTempDir();
	const repoDir = path.join(parent, "repo");
	const outsideDir = path.join(parent, "outside");
	await cp(path.join(fixturesRoot, "workspace-symlink-outside-root"), repoDir, {
		recursive: true,
	});
	await cp(path.join(fixturesRoot, "_outside-repo"), outsideDir, { recursive: true });
	await mkdir(path.join(repoDir, "packages"), { recursive: true });
	await symlink(outsideDir, path.join(repoDir, "packages/lib"));
	await writeFile(path.join(repoDir, ".git"), "gitdir: fixture\n");
	return repoDir;
}

afterEach(async () => {
	await Promise.all(tempDirs.splice(0).map((dir) => rm(dir, { recursive: true, force: true })));
});

describe("tools/workspace-shape", () => {
	it("accepts the compliant fixture workspace", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("compliant"));
		expect(violations).toEqual([]);
	});

	it("accepts empty workspaces when no JavaScript package manifests exist", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("empty-workspaces-valid"));
		expect(violations).toEqual([]);
	});

	it("flags nested Git metadata", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("nested-git"));
		expect(violations.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.NESTED_GIT_REPO);
	});

	it("flags undeclared workspace members under apps/* and packages/*", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("undeclared-workspace-member"));
		expect(violations.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_MEMBER_UNDECLARED,
		);
	});

	it("flags workspace paths that resolve outside the Git root", async () => {
		const violations = await checkWorkspaceShape(await loadOutsideRootFixture());
		expect(violations.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_OUTSIDE_ROOT,
		);
	});

	it("flags symlinked workspace paths that escape the Git root", async () => {
		const violations = await checkWorkspaceShape(await loadSymlinkOutsideFixture());
		expect(violations.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_OUTSIDE_ROOT,
		);
	});

	it("flags root manifest drift cases", async () => {
		const notPrivate = await checkWorkspaceShape(await loadFixture("root-not-private"));
		expect(notPrivate.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.ROOT_NOT_PRIVATE);

		const invalidPm = await checkWorkspaceShape(await loadFixture("package-manager-invalid"));
		expect(invalidPm.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.PACKAGE_MANAGER_INVALID,
		);

		const wildcard = await checkWorkspaceShape(await loadFixture("workspaces-wildcard"));
		expect(wildcard.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.WORKSPACES_INVALID);

		const missingPath = await checkWorkspaceShape(await loadFixture("workspace-path-missing"));
		expect(missingPath.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_MISSING,
		);
	});

	it("flags lockfile drift cases", async () => {
		const missingRootLock = await checkWorkspaceShape(await loadFixture("root-lock-missing"));
		expect(missingRootLock.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.ROOT_LOCK_MISSING,
		);

		const childLock = await checkWorkspaceShape(await loadFixture("child-lockfile"));
		expect(childLock.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.CHILD_LOCKFILE);
	});

	it("flags internal dependency drift cases", async () => {
		const fileDep = await checkWorkspaceShape(await loadFixture("internal-file-dependency"));
		expect(fileDep.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.INTERNAL_FILE_DEPENDENCY,
		);

		const edge = await checkWorkspaceShape(await loadFixture("internal-edge-not-workspace"));
		expect(edge.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.INTERNAL_EDGE_NOT_WORKSPACE,
		);
	});

	it("flags declared JavaScript workspaces outside apps/* and packages/*", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("workspace-prefix-unsupported"));
		expect(violations.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PREFIX_UNSUPPORTED,
		);
	});

	it("flags committed file: Underlay and Poodle dependencies", async () => {
		const named = await checkWorkspaceShape(await loadFixture("shared-file-dependency"));
		expect(named.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.SHARED_FILE_DEPENDENCY);

		const byTarget = await checkWorkspaceShape(
			await loadFixture("shared-file-dependency-by-target"),
		);
		expect(byTarget.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.SHARED_FILE_DEPENDENCY);
	});

	it("allows unrelated external file: development dependencies", async () => {
		const violations = await checkWorkspaceShape(
			await loadFixture("external-file-dependency-allowed"),
		);
		expect(violations).toEqual([]);
	});

	it("flags disposable leftover top-level package trees after apps/packages migration", async () => {
		const violations = await checkWorkspaceShape(
			await loadFixture("retired-top-level-package"),
		);
		const retired = violations.filter(
			(v) => v.ruleId === WORKSPACE_SHAPE_RULE_IDS.RETIRED_TOP_LEVEL_PACKAGE,
		);
		expect(retired).toHaveLength(1);
		expect(retired[0]?.path).toBe("app");
		expect(retired[0]?.detail).toContain("apps/app");
		expect(retired[0]?.detail).toContain("disposable leftover");
		expect(retired[0]?.detail).toContain(formatRetiredDisposableCleanupCommand("app"));
		expect(retired[0]?.detail).toContain("rm -rf -- ");
	});

	it("flags ambiguous same-basename top-level paths for inspection without deletion", async () => {
		const violations = await checkWorkspaceShape(
			await loadFixture("retired-top-level-ambiguous"),
		);
		const retired = violations.filter(
			(v) => v.ruleId === WORKSPACE_SHAPE_RULE_IDS.RETIRED_TOP_LEVEL_PACKAGE,
		);
		expect(retired).toHaveLength(1);
		expect(retired[0]?.path).toBe("config");
		expect(retired[0]?.detail).toContain("packages/config");
		expect(retired[0]?.detail).toContain("inspect or relocate");
		expect(retired[0]?.detail).not.toContain("rm -rf");
	});

	it("POSIX-quotes disposable cleanup paths for quote, expansion, and leading-dash names", () => {
		expect(posixShellSingleQuote("o'pkg")).toBe(`'o'\\''pkg'`);
		expect(posixShellSingleQuote("$(reboot)")).toBe(`'$(reboot)'`);
		expect(posixShellSingleQuote("-dash")).toBe(`'-dash'`);

		expect(formatRetiredDisposableCleanupCommand("o'pkg")).toBe(`rm -rf -- 'o'\\''pkg'`);
		expect(formatRetiredDisposableCleanupCommand("$(reboot)")).toBe(`rm -rf -- '$(reboot)'`);
		expect(formatRetiredDisposableCleanupCommand("-dash")).toBe(`rm -rf -- '-dash'`);
	});

	it("emits POSIX-quoted cleanup for disposable leftovers with hostile basenames", async () => {
		for (const name of ["o'pkg", "$(reboot)", "-dash"] as const) {
			const root = await makeTempDir();
			await writeFile(
				path.join(root, "package.json"),
				JSON.stringify({
					name: "@fixture/hostile",
					private: true,
					packageManager: "bun@1.3.14",
					workspaces: [`apps/${name}`],
				}),
			);
			await writeFile(path.join(root, "bun.lock"), "# fixture\n");
			await writeFile(path.join(root, ".git"), "gitdir: fixture\n");
			await mkdir(path.join(root, "apps", name), { recursive: true });
			await writeFile(
				path.join(root, "apps", name, "package.json"),
				JSON.stringify({ name: `@fixture/${name}` }),
			);
			await mkdir(path.join(root, name, "node_modules"), { recursive: true });

			const violations = await checkWorkspaceShape(root);
			const retired = violations.filter(
				(v) => v.ruleId === WORKSPACE_SHAPE_RULE_IDS.RETIRED_TOP_LEVEL_PACKAGE,
			);
			expect(retired).toHaveLength(1);
			expect(retired[0]?.path).toBe(name);
			expect(retired[0]?.detail).toContain(formatRetiredDisposableCleanupCommand(name));
			expect(retired[0]?.detail).toContain("rm -rf -- ");
			expect(retired[0]?.detail).not.toContain(JSON.stringify(name));
		}
	});

	it("formats empty and failing reports with stable copy", () => {
		expect(formatWorkspaceShapeReport("/repo", [])).toBe(
			"Workspace shape report for: /repo\n\nAll workspace shape checks passed.",
		);

		expect(
			formatWorkspaceShapeReport("/repo", [
				{
					ruleId: WORKSPACE_SHAPE_RULE_IDS.ROOT_LOCK_MISSING,
					path: "bun.lock",
					detail: "root bun.lock is missing",
				},
			]),
		).toBe(
			"Workspace shape report for: /repo\n\n  FAIL  root-lock-missing: bun.lock — root bun.lock is missing\n\n1 workspace shape violation(s) found.",
		);
	});

	it("returns deterministic sorted diagnostics", async () => {
		const violations = await checkWorkspaceShape(await loadFixture("nested-git"));
		const sorted = [...violations].sort((a, b) => {
			if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
			if (a.path !== b.path) return a.path.localeCompare(b.path);
			return a.detail.localeCompare(b.detail);
		});
		expect(violations).toEqual(sorted);
	});

	it("runs through the published underlay-workspace-shape bin entry", async () => {
		const fixtureDir = await loadFixture("compliant");
		const { stdout } = await execFileAsync("bun", [publishedBin, fixtureDir], {
			cwd: repoRoot,
		});

		expect(stdout).toContain("All workspace shape checks passed.");
	});
});
