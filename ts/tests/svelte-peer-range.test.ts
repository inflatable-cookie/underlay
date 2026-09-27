import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const repoRoot = path.resolve(fileURLToPath(new URL(".", import.meta.url)), "../..");
const sveltePeerRange = ">=5.56.8 <6";
const floorVersion = "5.56.8";
const newestLockedVersion = "5.57.1";

type Manifest = {
	peerDependencies?: Record<string, string>;
	devDependencies?: Record<string, string>;
};

function parseTriple(version: string): [number, number, number] {
	const match = /^(\d+)\.(\d+)\.(\d+)(?:-|$)/.exec(version);
	if (!match) {
		throw new Error(`unparseable version: ${version}`);
	}
	return [Number(match[1]), Number(match[2]), Number(match[3])];
}

function cmp(left: string, right: string): number {
	const [lMajor, lMinor, lPatch] = parseTriple(left);
	const [rMajor, rMinor, rPatch] = parseTriple(right);
	return lMajor - rMajor || lMinor - rMinor || lPatch - rPatch;
}

function satisfiesSveltePeer(version: string): boolean {
	return cmp(version, floorVersion) >= 0 && parseTriple(version)[0] < 6;
}

function lockedSvelteVersion(lockSource: string): string {
	const match = /"svelte": \["svelte@([^"]+)"/.exec(lockSource);
	if (!match) {
		throw new Error("bun.lock does not pin svelte");
	}
	return match[1];
}

describe("svelte peer range", () => {
	const manifest = JSON.parse(
		readFileSync(path.join(repoRoot, "package.json"), "utf8"),
	) as Manifest;
	const contract = readFileSync(
		path.join(repoRoot, "docs/knowledge/contracts/023-release-and-compatibility-rollout.md"),
		"utf8",
	);
	const changelog = readFileSync(path.join(repoRoot, "CHANGELOG.md"), "utf8");
	const lockSource = readFileSync(path.join(repoRoot, "bun.lock"), "utf8");
	const locked = lockedSvelteVersion(lockSource);

	it("declares >=5.56.8 <6 in package.json, contract 023, and the unreleased changelog", () => {
		expect(manifest.peerDependencies?.svelte).toBe(sveltePeerRange);
		expect(contract).toContain(`\`${sveltePeerRange}\``);
		expect(changelog).toMatch(
			/## \[Unreleased\][\s\S]*?`svelte` peer from `\^5\.0\.0` to `>=5\.56\.8 <6`/,
		);
	});

	it("includes the floor and the newest locked Svelte 5, and excludes the versions beside the range", () => {
		expect(satisfiesSveltePeer(floorVersion)).toBe(true);
		expect(satisfiesSveltePeer(newestLockedVersion)).toBe(true);
		expect(satisfiesSveltePeer("5.56.7")).toBe(false);
		expect(satisfiesSveltePeer("6.0.0")).toBe(false);
	});

	it("pins the newest Svelte 5 in bun.lock for typecheck and tests", () => {
		expect(locked).toBe(newestLockedVersion);
		expect(satisfiesSveltePeer(locked)).toBe(true);
		expect(manifest.devDependencies?.svelte).toBe(`^${newestLockedVersion}`);
	});
});
