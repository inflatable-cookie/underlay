import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
	checkWorkspaceShape,
	WORKSPACE_SHAPE_RULE_IDS,
} from "@inflatable-cookie/underlay/tools/workspace-shape";

const fixturesRoot = path.join(
	path.dirname(fileURLToPath(import.meta.url)),
	"../fixtures/workspace-shape",
);

const acowtancyRoot = "/Users/tom/Dev/projects/acowtancy";

function fixturePath(name: string): string {
	return path.join(fixturesRoot, name);
}

describe("tools/workspace-shape", () => {
	it("accepts the compliant fixture workspace", async () => {
		const violations = await checkWorkspaceShape(fixturePath("compliant"));
		expect(violations).toEqual([]);
	});

	it("accepts Acowtancy's live workspace shape", async () => {
		const violations = await checkWorkspaceShape(acowtancyRoot);
		expect(violations).toEqual([]);
	});

	it("flags nested Git metadata", async () => {
		const violations = await checkWorkspaceShape(fixturePath("nested-git"));
		expect(violations.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.NESTED_GIT_REPO);
	});

	it("flags root manifest drift cases", async () => {
		const notPrivate = await checkWorkspaceShape(fixturePath("root-not-private"));
		expect(notPrivate.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.ROOT_NOT_PRIVATE);

		const invalidPm = await checkWorkspaceShape(fixturePath("package-manager-invalid"));
		expect(invalidPm.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.PACKAGE_MANAGER_INVALID,
		);

		const wildcard = await checkWorkspaceShape(fixturePath("workspaces-wildcard"));
		expect(wildcard.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.WORKSPACES_INVALID);

		const missingPath = await checkWorkspaceShape(fixturePath("workspace-path-missing"));
		expect(missingPath.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_MISSING,
		);
	});

	it("flags lockfile drift cases", async () => {
		const missingRootLock = await checkWorkspaceShape(fixturePath("root-lock-missing"));
		expect(missingRootLock.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.ROOT_LOCK_MISSING,
		);

		const childLock = await checkWorkspaceShape(fixturePath("child-lockfile"));
		expect(childLock.map((v) => v.ruleId)).toContain(WORKSPACE_SHAPE_RULE_IDS.CHILD_LOCKFILE);
	});

	it("flags internal dependency drift cases", async () => {
		const fileDep = await checkWorkspaceShape(fixturePath("internal-file-dependency"));
		expect(fileDep.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.INTERNAL_FILE_DEPENDENCY,
		);

		const edge = await checkWorkspaceShape(fixturePath("internal-edge-not-workspace"));
		expect(edge.map((v) => v.ruleId)).toContain(
			WORKSPACE_SHAPE_RULE_IDS.INTERNAL_EDGE_NOT_WORKSPACE,
		);
	});

	it("returns deterministic sorted diagnostics", async () => {
		const violations = await checkWorkspaceShape(fixturePath("nested-git"));
		const sorted = [...violations].sort((a, b) => {
			if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
			if (a.path !== b.path) return a.path.localeCompare(b.path);
			return a.detail.localeCompare(b.detail);
		});
		expect(violations).toEqual(sorted);
	});
});
