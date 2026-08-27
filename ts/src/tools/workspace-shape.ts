#!/usr/bin/env bun

/**
 * Consumer workspace-shape conformance checker.
 *
 * Validates the single-repository Bun workspace topology defined in contract 024.
 * Separate from security conformance in `scripts/check-consumer-conformance.sh`.
 *
 * @example
 * ```bash
 * bun underlay/ts/src/tools/workspace-shape.ts /path/to/consumer
 * bun underlay/ts/src/tools/workspace-shape.ts .
 * ```
 */

import { readdir, readFile, realpath, stat } from 'node:fs/promises';
import path from 'node:path';

// =============================================================================
// Types
// =============================================================================

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
} as const;

export type WorkspaceShapeRuleId = (typeof WORKSPACE_SHAPE_RULE_IDS)[keyof typeof WORKSPACE_SHAPE_RULE_IDS];

export interface WorkspaceShapeViolation {
	ruleId: WorkspaceShapeRuleId;
	path: string;
	detail: string;
}

interface PackageJson {
	name?: string;
	private?: boolean;
	packageManager?: string;
	workspaces?: unknown;
	dependencies?: Record<string, string>;
	devDependencies?: Record<string, string>;
	peerDependencies?: Record<string, string>;
	optionalDependencies?: Record<string, string>;
}

// =============================================================================
// Constants
// =============================================================================

const PACKAGE_MANAGER_PATTERN = /^bun@\d+\.\d+\.\d+$/;

const SKIP_DIR_NAMES = new Set([
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

const DEPENDENCY_FIELDS = [
	'dependencies',
	'devDependencies',
	'peerDependencies',
	'optionalDependencies',
] as const;

const WORKSPACE_DISCOVERY_PREFIXES = ['apps', 'packages'];

const RELEASED_SHARED_PACKAGE_NAMES = new Set([
	'@inflatable-cookie/underlay',
	'@inflatable-cookie/poodle',
	'@inflatable-cookie/poodle-core',
	'@inflatable-cookie/poodle-svelte',
]);

// =============================================================================
// Helpers
// =============================================================================

function pushViolation(
	violations: WorkspaceShapeViolation[],
	ruleId: WorkspaceShapeRuleId,
	filePath: string,
	detail: string,
): void {
	violations.push({ ruleId, path: filePath, detail });
}

async function pathExists(target: string): Promise<boolean> {
	try {
		await stat(target);
		return true;
	} catch {
		return false;
	}
}

async function readPackageJson(manifestPath: string): Promise<PackageJson | undefined> {
	try {
		const raw = await readFile(manifestPath, 'utf8');
		const parsed = JSON.parse(raw) as PackageJson;
		return parsed && typeof parsed === 'object' ? parsed : undefined;
	} catch {
		return undefined;
	}
}

function hasGlobPattern(workspacePath: string): boolean {
	return /[*?{}]/.test(workspacePath);
}

function isPathInsideRoot(root: string, targetPath: string): boolean {
	const normalizedRoot = path.resolve(root);
	const resolved = path.resolve(normalizedRoot, targetPath);
	return resolved === normalizedRoot || resolved.startsWith(`${normalizedRoot}${path.sep}`);
}

function normalizeWorkspacePath(workspacePath: string): string {
	return workspacePath.replace(/\\/g, '/').replace(/^\.\//, '').replace(/\/+$/, '');
}

function hasSupportedWorkspacePrefix(workspacePath: string): boolean {
	const normalized = normalizeWorkspacePath(workspacePath);
	return normalized.startsWith('apps/') || normalized.startsWith('packages/');
}

function isReleasedSharedPackageName(name: string): boolean {
	return RELEASED_SHARED_PACKAGE_NAMES.has(name) || name.startsWith('@inflatable-cookie/poodle-');
}

async function isRealPathInsideRoot(root: string, relativePath: string): Promise<boolean> {
	const normalizedPath = normalizeWorkspacePath(relativePath);
	const absoluteTarget = path.join(root, normalizedPath);

	try {
		const rootReal = await realpath(root);
		const targetReal = await realpath(absoluteTarget);
		return targetReal === rootReal || targetReal.startsWith(`${rootReal}${path.sep}`);
	} catch {
		return false;
	}
}

async function workspacePathContainedByRoot(
	root: string,
	workspacePath: string,
): Promise<'inside' | 'missing' | 'outside'> {
	const normalizedPath = normalizeWorkspacePath(workspacePath);

	if (!isPathInsideRoot(root, normalizedPath)) {
		return 'outside';
	}

	const absoluteTarget = path.join(root, normalizedPath);
	if (!(await pathExists(absoluteTarget))) {
		return 'missing';
	}

	if (!(await isRealPathInsideRoot(root, normalizedPath))) {
		return 'outside';
	}

	return 'inside';
}

function sortViolations(violations: WorkspaceShapeViolation[]): WorkspaceShapeViolation[] {
	return [...violations].sort((a, b) => {
		if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
		if (a.path !== b.path) return a.path.localeCompare(b.path);
		return a.detail.localeCompare(b.detail);
	});
}

async function* walkDirectories(root: string, relative = ''): AsyncGenerator<string> {
	const dirPath = relative ? path.join(root, relative) : root;
	const entries = await readdir(dirPath, { withFileTypes: true });

	for (const entry of entries) {
		if (entry.name === '.git' || SKIP_DIR_NAMES.has(entry.name)) continue;

		const entryRelative = relative ? `${relative}/${entry.name}` : entry.name;
		if (entry.isDirectory()) {
			yield entryRelative;
			yield* walkDirectories(root, entryRelative);
		}
	}
}

async function findBunLockPaths(root: string): Promise<string[]> {
	const locks: string[] = [];

	async function walk(relative: string): Promise<void> {
		const dirPath = relative ? path.join(root, relative) : root;
		const entries = await readdir(dirPath, { withFileTypes: true });

		for (const entry of entries) {
			if (entry.name === '.git' || SKIP_DIR_NAMES.has(entry.name)) continue;

			const entryRelative = relative ? `${relative}/${entry.name}` : entry.name;
			if (entry.isDirectory()) {
				await walk(entryRelative);
			} else if (entry.name === 'bun.lock') {
				locks.push(entryRelative);
			}
		}
	}

	await walk('');
	return locks;
}

function dependencyEntries(pkg: PackageJson): Array<{ field: string; name: string; value: string }> {
	const entries: Array<{ field: string; name: string; value: string }> = [];

	for (const field of DEPENDENCY_FIELDS) {
		const section = pkg[field];
		if (!section || typeof section !== 'object') continue;

		for (const [name, value] of Object.entries(section)) {
			if (typeof value === 'string') {
				entries.push({ field, name, value });
			}
		}
	}

	return entries;
}

function isInternalFileDependency(
	root: string,
	manifestRelativeDir: string,
	depName: string,
	depValue: string,
	internalNames: Set<string>,
	workspacePaths: string[],
): boolean {
	if (!depValue.startsWith('file:')) return false;
	if (internalNames.has(depName)) return true;

	const targetPath = depValue.slice('file:'.length);
	const resolved = path.resolve(path.join(root, manifestRelativeDir), targetPath);

	for (const workspacePath of workspacePaths) {
		const workspaceRoot = path.resolve(root, workspacePath);
		if (resolved === workspaceRoot || resolved.startsWith(`${workspaceRoot}${path.sep}`)) {
			return true;
		}
	}

	return false;
}

async function sharedFileTargetPackageName(
	root: string,
	manifestRelativeDir: string,
	depValue: string,
): Promise<string | undefined> {
	if (!depValue.startsWith('file:')) return undefined;

	const targetPath = depValue.slice('file:'.length);
	const resolved = path.resolve(path.join(root, manifestRelativeDir), targetPath);
	const pkg = await readPackageJson(path.join(resolved, 'package.json'));
	return typeof pkg?.name === 'string' ? pkg.name : undefined;
}

async function discoverWorkspacePackagePaths(root: string): Promise<string[]> {
	const discovered: string[] = [];

	for (const prefix of WORKSPACE_DISCOVERY_PREFIXES) {
		const prefixPath = path.join(root, prefix);
		if (!(await pathExists(prefixPath))) continue;
		await collectWorkspacePackagePaths(root, prefix, discovered);
	}

	return [...new Set(discovered)].sort();
}

async function collectWorkspacePackagePaths(
	root: string,
	relativeDir: string,
	discovered: string[],
): Promise<void> {
	const dirPath = path.join(root, relativeDir);
	if (await pathExists(path.join(dirPath, 'package.json'))) {
		discovered.push(normalizeWorkspacePath(relativeDir));
	}

	const entries = await readdir(dirPath, { withFileTypes: true });
	for (const entry of entries) {
		if (entry.name === '.git' || SKIP_DIR_NAMES.has(entry.name) || !entry.isDirectory()) continue;
		await collectWorkspacePackagePaths(root, path.join(relativeDir, entry.name), discovered);
	}
}

// =============================================================================
// Checks
// =============================================================================

async function checkGitRoot(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
	if (!(await pathExists(path.join(root, '.git')))) {
		pushViolation(
			violations,
			WORKSPACE_SHAPE_RULE_IDS.GIT_ROOT_MISSING,
			'.git',
			'root Git metadata is missing (.git directory or worktree file)',
		);
	}
}

async function checkNestedGitRepos(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
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

function checkRootManifestFields(
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

async function checkWorkspacePaths(
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

async function checkWorkspaceMembership(
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

async function checkLockfiles(root: string, violations: WorkspaceShapeViolation[]): Promise<void> {
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

async function collectInternalPackageNames(
	root: string,
	workspacePaths: string[],
): Promise<Set<string>> {
	const names = new Set<string>();

	for (const workspacePath of workspacePaths) {
		const manifestPath = path.join(root, workspacePath, 'package.json');
		const pkg = await readPackageJson(manifestPath);
		if (pkg?.name && typeof pkg.name === 'string') {
			names.add(pkg.name);
		}
	}

	return names;
}

async function checkInternalEdges(
	root: string,
	workspacePaths: string[],
	internalNames: Set<string>,
	violations: WorkspaceShapeViolation[],
): Promise<void> {
	for (const workspacePath of workspacePaths) {
		const manifestRelative = path.join(workspacePath, 'package.json');
		const pkg = await readPackageJson(path.join(root, manifestRelative));
		if (!pkg) continue;

		for (const { field, name, value } of dependencyEntries(pkg)) {
			if (
				isInternalFileDependency(root, workspacePath, name, value, internalNames, workspacePaths)
			) {
				pushViolation(
					violations,
					WORKSPACE_SHAPE_RULE_IDS.INTERNAL_FILE_DEPENDENCY,
					manifestRelative,
					`${field}.${name} uses file:${value.slice('file:'.length)} for an internal workspace edge`,
				);
				continue;
			}

			if (internalNames.has(name) && value !== 'workspace:*') {
				pushViolation(
					violations,
					WORKSPACE_SHAPE_RULE_IDS.INTERNAL_EDGE_NOT_WORKSPACE,
					manifestRelative,
					`${field}.${name} must use workspace:* (found ${value})`,
				);
			}
		}
	}
}

async function checkSharedFileDependencies(
	root: string,
	workspacePaths: string[],
	violations: WorkspaceShapeViolation[],
): Promise<void> {
	const manifests = [
		'package.json',
		...workspacePaths.map((workspacePath) => path.join(workspacePath, 'package.json')),
	];

	for (const manifestRelative of manifests) {
		const pkg = await readPackageJson(path.join(root, manifestRelative));
		if (!pkg) continue;

		const manifestDir =
			path.dirname(manifestRelative) === '.' ? '' : path.dirname(manifestRelative);

		for (const { field, name, value } of dependencyEntries(pkg)) {
			if (!value.startsWith('file:')) continue;

			const targetName = await sharedFileTargetPackageName(root, manifestDir, value);
			if (!isReleasedSharedPackageName(name) && !isReleasedSharedPackageName(targetName ?? '')) {
				continue;
			}

			pushViolation(
				violations,
				WORKSPACE_SHAPE_RULE_IDS.SHARED_FILE_DEPENDENCY,
				manifestRelative,
				`${field}.${name} uses file:${value.slice('file:'.length)} for a released Underlay/Poodle dependency`,
			);
		}
	}
}

// =============================================================================
// Public API
// =============================================================================

export async function checkWorkspaceShape(rootPath: string): Promise<WorkspaceShapeViolation[]> {
	const root = path.resolve(rootPath);
	const violations: WorkspaceShapeViolation[] = [];

	await checkGitRoot(root, violations);
	await checkNestedGitRepos(root, violations);

	const rootManifest = await readPackageJson(path.join(root, 'package.json'));
	const workspacePaths = checkRootManifestFields(rootManifest, violations);
	const hasWorkspacesArray = Array.isArray(rootManifest?.workspaces);

	if (hasWorkspacesArray) {
		await checkWorkspaceMembership(root, workspacePaths, violations);
	}

	if (workspacePaths.length > 0) {
		await checkWorkspacePaths(root, workspacePaths, violations);
	}

	await checkLockfiles(root, violations);

	if (workspacePaths.length > 0) {
		const internalNames = await collectInternalPackageNames(root, workspacePaths);
		await checkInternalEdges(root, workspacePaths, internalNames, violations);
	}

	await checkSharedFileDependencies(root, workspacePaths, violations);

	return sortViolations(violations);
}

export function formatWorkspaceShapeReport(
	rootPath: string,
	violations: WorkspaceShapeViolation[],
): string {
	const lines: string[] = [`Workspace shape report for: ${rootPath}`, ''];

	if (violations.length === 0) {
		lines.push('All workspace shape checks passed.');
		return lines.join('\n');
	}

	for (const violation of violations) {
		lines.push(`  FAIL  ${violation.ruleId}: ${violation.path} — ${violation.detail}`);
	}

	lines.push('', `${violations.length} workspace shape violation(s) found.`);
	return lines.join('\n');
}

// =============================================================================
// CLI
// =============================================================================

export function runWorkspaceShapeCli(argv: string[] = process.argv): void {
	(async () => {
		const args = argv.slice(2);
		let rootArg: string | undefined;

		for (let i = 0; i < args.length; i++) {
			if (args[i] === '--help' || args[i] === '-h') {
				console.log(`
Workspace shape - Consumer Bun workspace topology conformance

Usage:
  underlay-workspace-shape [path]
  bun underlay/ts/bin/underlay-workspace-shape.ts [path]

Options:
  --help, -h        Show this help message

The path defaults to the current working directory.
`);
				process.exit(0);
			}

			if (!args[i].startsWith('-')) {
				rootArg = args[i];
			}
		}

		const root = path.resolve(rootArg ?? process.cwd());
		const violations = await checkWorkspaceShape(root);
		console.log(formatWorkspaceShapeReport(root, violations));

		if (violations.length > 0) {
			process.exit(1);
		}
	})();
}

if (import.meta.url === `file://${process.argv[1]}`) {
	runWorkspaceShapeCli();
}
