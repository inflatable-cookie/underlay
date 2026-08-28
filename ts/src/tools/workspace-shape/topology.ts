import { readdir, stat } from 'node:fs/promises';
import path from 'node:path';
import {
	discoverWorkspacePackagePaths,
	findBunLockPaths,
	hasGlobPattern,
	hasSupportedWorkspacePrefix,
	isRealPathInsideRoot,
	normalizeWorkspacePath,
	pathExists,
	walkDirectories,
	workspacePathContainedByRoot,
} from './fs.js';
import {
	DISPOSABLE_RETIRED_TOP_LEVEL_NAMES,
	PACKAGE_MANAGER_PATTERN,
	WORKSPACE_SHAPE_RULE_IDS,
	formatRetiredDisposableCleanupCommand,
	pushViolation,
	type PackageJson,
	type WorkspaceShapeViolation,
} from './model.js';

export async function checkGitRoot(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
	if (!(await pathExists(path.join(root, '.git')))) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.GIT_ROOT_MISSING,
			'.git',
			'root Git metadata is missing (.git directory or worktree file)',
		);
	}
}

export async function checkNestedGitRepos(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
	for await (const relativeDir of walkDirectories(root)) {
		const gitPath = path.join(root, relativeDir, '.git');
		if (await pathExists(gitPath)) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.NESTED_GIT_REPO,
				path.join(relativeDir, '.git'),
				'nested Git metadata inside the workspace',
			);
		}
	}
}

export function checkRootManifestFields(
	rootManifest: PackageJson | undefined,
	violations: WorkspaceShapeViolation[],
): string[] {
	const manifestPath = 'package.json';

	if (!rootManifest) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.ROOT_MANIFEST_MISSING,
			manifestPath,
			'root package.json is missing or unreadable',
		);
		return [];
	}

	if (rootManifest.private !== true) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.ROOT_NOT_PRIVATE,
			manifestPath,
			`private must be true (found ${JSON.stringify(rootManifest.private)})`,
		);
	}

	if (typeof rootManifest.packageManager !== 'string' || rootManifest.packageManager.length === 0) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.PACKAGE_MANAGER_MISSING,
			manifestPath,
			'packageManager must be a pinned bun@X.Y.Z value at the root',
		);
	} else if (!PACKAGE_MANAGER_PATTERN.test(rootManifest.packageManager)) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.PACKAGE_MANAGER_INVALID,
			manifestPath,
			`packageManager must match bun@X.Y.Z (found ${rootManifest.packageManager})`,
		);
	}

	if (!Array.isArray(rootManifest.workspaces)) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.WORKSPACES_MISSING,
			manifestPath,
			'workspaces must be an explicit path array at the root',
		);
		return [];
	}

	const workspacePaths: string[] = [];
	for (const entry of rootManifest.workspaces) {
		if (typeof entry !== 'string' || entry.length === 0) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACES_INVALID,
				manifestPath,
				`workspaces entries must be non-empty strings (found ${JSON.stringify(entry)})`,
			);
			continue;
		}

		if (hasGlobPattern(entry)) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACES_INVALID,
				manifestPath,
				`workspace path must be explicit, not a glob pattern (${entry})`,
			);
			continue;
		}

		workspacePaths.push(entry);
	}

	return workspacePaths;
}

export async function checkWorkspacePaths(
	root: string,
	workspacePaths: string[],
	violations: WorkspaceShapeViolation[],
): Promise<void> {
	for (const workspacePath of workspacePaths) {
		const normalizedPath = normalizeWorkspacePath(workspacePath);

		if (!hasSupportedWorkspacePrefix(workspacePath)) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PREFIX_UNSUPPORTED,
				'package.json',
				`workspace path ${workspacePath} must live under apps/* or packages/*`,
			);
		}

		const containment = await workspacePathContainedByRoot(root, workspacePath);

		if (containment === 'outside') {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_OUTSIDE_ROOT,
				'package.json',
				`workspace path ${workspacePath} resolves outside the Git root`,
			);
			continue;
		}

		const manifestPath = path.join(normalizedPath, 'package.json');
		if (!(await pathExists(path.join(root, manifestPath)))) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_MISSING,
				manifestPath,
				`workspace path ${workspacePath} does not resolve to a package manifest`,
			);
		}
	}
}

export async function checkWorkspaceMembership(
	root: string,
	workspacePaths: string[],
	violations: WorkspaceShapeViolation[],
): Promise<void> {
	const discoveredPaths = await discoverWorkspacePackagePaths(root);
	const declaredPaths = new Set(workspacePaths.map((entry) => normalizeWorkspacePath(entry)));

	for (const discoveredPath of discoveredPaths) {
		if (!(await isRealPathInsideRoot(root, discoveredPath))) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_PATH_OUTSIDE_ROOT,
				path.join(discoveredPath, 'package.json'),
				`discovered workspace manifest resolves outside the Git root (${discoveredPath})`,
			);
			continue;
		}

		if (!declaredPaths.has(discoveredPath)) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.WORKSPACE_MEMBER_UNDECLARED,
				path.join(discoveredPath, 'package.json'),
				`JavaScript package manifest is not declared in root workspaces (${discoveredPath})`,
			);
		}
	}
}

export async function checkLockfiles(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
	if (!(await pathExists(path.join(root, 'bun.lock')))) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.ROOT_LOCK_MISSING,
			'bun.lock',
			'root bun.lock is missing',
		);
	}

	for (const lockPath of await findBunLockPaths(root)) {
		if (lockPath !== 'bun.lock') {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.CHILD_LOCKFILE,
				lockPath,
				'child bun.lock files are not allowed',
			);
		}
	}
}

/**
 * After apps/* / packages/* migrations, ignored build/cache trees often remain
 * at the old top-level package names. Inventory only — never delete.
 *
 * Recursive deletion is suggested only when every immediate child is a known
 * disposable leftover name. Anything else is reported for explicit inspection
 * without a deletion command.
 */
export async function checkRetiredTopLevelPackages(
	root: string,
	workspacePaths: string[],
	violations: WorkspaceShapeViolation[],
): Promise<void> {
	const seen = new Set<string>();

	for (const workspacePath of workspacePaths) {
		const normalized = normalizeWorkspacePath(workspacePath);
		if (!hasSupportedWorkspacePrefix(normalized)) continue;

		const basename = path.posix.basename(normalized);
		if (!basename || basename === '.' || basename === '..') continue;
		if (seen.has(basename)) continue;
		seen.add(basename);

		const retiredAbs = path.join(root, basename);
		if (!(await pathExists(retiredAbs))) continue;

		let isDirectory = false;
		try {
			isDirectory = (await stat(retiredAbs)).isDirectory();
		} catch {
			continue;
		}
		if (!isDirectory) continue;

		const disposableOnly = await isDisposableRetiredTopLevel(retiredAbs);
		if (disposableOnly) {
			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.RETIRED_TOP_LEVEL_PACKAGE,
				basename,
				`disposable leftover at top-level path while live package is ${normalized}; inventory only — safe cleanup: ${formatRetiredDisposableCleanupCommand(basename)}`,
			);
			continue;
		}

		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.RETIRED_TOP_LEVEL_PACKAGE,
			basename,
			`top-level path shares a name with live package ${normalized}; inspect or relocate explicitly — do not delete from basename evidence alone`,
		);
	}
}

async function isDisposableRetiredTopLevel(absolutePath: string): Promise<boolean> {
	let entries: string[];
	try {
		entries = await readdir(absolutePath);
	} catch {
		return false;
	}

	return entries.every((entry) => DISPOSABLE_RETIRED_TOP_LEVEL_NAMES.has(entry));
}
