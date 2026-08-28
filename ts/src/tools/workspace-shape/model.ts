export const WORKSPACE_SHAPE_RULE_IDS = {
	GIT_ROOT_MISSING: 'git-root-missing',
	NESTED_GIT_REPO: 'nested-git-repo',
	ROOT_MANIFEST_MISSING: 'root-manifest-missing',
	ROOT_NOT_PRIVATE: 'root-not-private',
	PACKAGE_MANAGER_MISSING: 'package-manager-missing',
	PACKAGE_MANAGER_INVALID: 'package-manager-invalid',
	WORKSPACES_MISSING: 'workspaces-missing',
	WORKSPACES_INVALID: 'workspaces-invalid',
	WORKSPACE_PATH_MISSING: 'workspace-path-missing',
	WORKSPACE_PATH_OUTSIDE_ROOT: 'workspace-path-outside-root',
	WORKSPACE_MEMBER_UNDECLARED: 'workspace-member-undeclared',
	ROOT_LOCK_MISSING: 'root-lock-missing',
	CHILD_LOCKFILE: 'child-lockfile',
	INTERNAL_FILE_DEPENDENCY: 'internal-file-dependency',
	INTERNAL_EDGE_NOT_WORKSPACE: 'internal-edge-not-workspace',
	WORKSPACE_PREFIX_UNSUPPORTED: 'workspace-prefix-unsupported',
	SHARED_FILE_DEPENDENCY: 'shared-file-dependency',
	RETIRED_TOP_LEVEL_PACKAGE: 'retired-top-level-package',
} as const;

export type WorkspaceShapeRuleId = (typeof WORKSPACE_SHAPE_RULE_IDS)[keyof typeof WORKSPACE_SHAPE_RULE_IDS];

export interface WorkspaceShapeViolation {
	ruleId: WorkspaceShapeRuleId;
	path: string;
	detail: string;
}

export interface PackageJson {
	name?: string;
	private?: boolean;
	packageManager?: string;
	workspaces?: unknown;
	dependencies?: Record<string, string>;
	devDependencies?: Record<string, string>;
	peerDependencies?: Record<string, string>;
	optionalDependencies?: Record<string, string>;
}

export const PACKAGE_MANAGER_PATTERN = /^bun@\d+\.\d+\.\d+$/;

export const SKIP_DIR_NAMES = new Set([
	'node_modules',
	'target',
	'dist',
	'build',
	'.turbo',
	'coverage',
	'.svelte-kit',
	'.effigy',
	'.cache',
]);

/** Immediate children that may remain after a package move into apps/packages. */
export const DISPOSABLE_RETIRED_TOP_LEVEL_NAMES = new Set([
	...SKIP_DIR_NAMES,
	'.DS_Store',
]);

/** POSIX single-quote a path for safe paste into a shell command. */
export function posixShellSingleQuote(value: string): string {
	return `'${value.replace(/'/g, `'\\''`)}'`;
}

/** Inventory-only disposable leftover cleanup command with option terminator. */
export function formatRetiredDisposableCleanupCommand(basename: string): string {
	return `rm -rf -- ${posixShellSingleQuote(basename)}`;
}

export const DEPENDENCY_FIELDS = [
	'dependencies',
	'devDependencies',
	'peerDependencies',
	'optionalDependencies',
] as const;

export const WORKSPACE_DISCOVERY_PREFIXES = ['apps', 'packages'];

export const RELEASED_SHARED_PACKAGE_NAMES = new Set([
	'@inflatable-cookie/underlay',
	'@inflatable-cookie/poodle',
	'@inflatable-cookie/poodle-core',
	'@inflatable-cookie/poodle-svelte',
]);

export function pushViolation(
	violations: WorkspaceShapeViolation[],
	ruleId: WorkspaceShapeRuleId,
	filePath: string,
	detail: string,
): void {
	violations.push({ ruleId, path: filePath, detail });
}

export function sortViolations(violations: WorkspaceShapeViolation[]): WorkspaceShapeViolation[] {
	return [...violations].sort((a, b) => {
		if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
		if (a.path !== b.path) return a.path.localeCompare(b.path);
		return a.detail.localeCompare(b.detail);
	});
}
