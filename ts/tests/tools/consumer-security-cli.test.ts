import { execFile } from "node:child_process";
import { mkdtemp, mkdir, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { afterEach, describe, expect, it } from "vitest";

const execFileAsync = promisify(execFile);
const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const securityCli = path.join(repoRoot, "ts/bin/underlay-consumer-security.sh");
const workspaceShapeTool = path.join(repoRoot, "ts/src/tools/workspace-shape.ts");
const envAuthorityTool = path.join(repoRoot, "ts/src/tools/env-authority.ts");

const tempDirs: string[] = [];

async function makeTempDir(): Promise<string> {
	const dir = await mkdtemp(path.join(tmpdir(), "underlay-cli-tools-"));
	tempDirs.push(dir);
	return dir;
}

afterEach(async () => {
	await Promise.all(tempDirs.splice(0).map((dir) => rm(dir, { recursive: true, force: true })));
});

describe("tools CLI entrypoints and security script", () => {
	it("fails explicitly when rg is missing from PATH in underlay-consumer-security.sh", async () => {
		const dir = await makeTempDir();
		let exitCode: number | undefined;
		let stderr = "";

		try {
			await execFileAsync("/bin/bash", [securityCli, dir], {
				env: { PATH: "" },
			});
		} catch (err) {
			const error = err as { code?: number; stderr?: string };
			exitCode = error.code;
			stderr = error.stderr ?? "";
		}

		expect(exitCode).toBe(2);
		expect(stderr).toMatch(/rg/i);
	});

	it("runs workspace-shape.ts and env-authority.ts via symlinks, relative paths, and paths with spaces", async () => {
		const dir = await makeTempDir();
		const targetDir = path.join(dir, "workspace with spaces");
		await mkdir(targetDir, { recursive: true });
		await writeFile(
			path.join(targetDir, "package.json"),
			JSON.stringify({
				name: "@fixture/spaces",
				private: true,
				packageManager: "bun@1.3.14",
				workspaces: [],
			}),
		);
		await writeFile(path.join(targetDir, "bun.lock"), "# fixture\n");
		await writeFile(path.join(targetDir, ".git"), "gitdir: fixture\n");

		const linkDir = path.join(dir, "bin with spaces");
		await mkdir(linkDir, { recursive: true });
		const wsLink = path.join(linkDir, "ws-check.ts");
		const envLink = path.join(linkDir, "env-check.ts");
		await symlink(workspaceShapeTool, wsLink);
		await symlink(envAuthorityTool, envLink);

		const wsRelative = path.relative(repoRoot, workspaceShapeTool);
		const envRelative = path.relative(repoRoot, envAuthorityTool);

		const [wsRelOut, wsLinkOut, envRelOut, envLinkOut] = await Promise.all([
			execFileAsync("bun", [wsRelative, targetDir], { cwd: repoRoot }),
			execFileAsync("bun", [wsLink, targetDir], { cwd: repoRoot }),
			execFileAsync("bun", [envRelative, targetDir], { cwd: repoRoot }),
			execFileAsync("bun", [envLink, targetDir], { cwd: repoRoot }),
		]);

		expect(wsRelOut.stdout).toContain("All workspace shape checks passed.");
		expect(wsLinkOut.stdout).toContain("All workspace shape checks passed.");
		expect(envRelOut.stdout).toContain("All env authority checks passed.");
		expect(envLinkOut.stdout).toContain("All env authority checks passed.");
	});
});
