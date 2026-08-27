import { readdir, readFile, realpath, stat } from 'node:fs/promises';
import path from 'node:path';
import {
	SKIP_DIR_NAMES,
	WORKSPACE_DISCOVERY_PREFIXES,
	type PackageJson,
} from './model.js';

export async function pathExists(target: string): Promise<boolean> {
	try {
		await stat(target);
		return true;
	} catch {
		return false;
	}
}

export async function readPackageJson(manifestPath: string): Promise<PackageJson | undefined> {
	try {
		const raw = await readFile(manifestPath, 'utf8');
		const parsed = JSON.parse(raw) as PackageJson;
		return parsed && typeof parsed === 'object' ? parsed : undefined;
	} catch {
		return undefined;
	}
}

export function hasGlobPattern(workspacePath: string): boolean {
	return /[*?{}]/.test(workspacePath);
}

export function isPathInsideRoot(root: string, targetPath: string): boolean {
	const normalizedRoot = path.resolve(root);
	const resolved = path.resolve(normalizedRoot, targetPath);
	return resolved === normalizedRoot || resolved.startsWith(`${normalizedRoot}${path.sep}`);
}

export function normalizeWorkspacePath(workspacePath: string): string {
	return workspacePath.replace(/\\/g, '/').replace(/^\.\//, '').replace(/\/+$/, '');
}

export function hasSupportedWorkspacePrefix(workspacePath: string): boolean {
	const normalized = normalizeWorkspacePath(workspacePath);
	return normalized.startsWith('apps/') || normalized.startsWith('packages/');
}

export async function isRealPathInsideRoot(root: string, relativePath: string): Promise<boolean> {
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

export async function workspacePathContainedByRoot(
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

export async function* walkDirectories(root: string, relative = ''): AsyncGenerator<string> {
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

export async function findBunLockPaths(root: string): Promise<string[]> {
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

export async function discoverWorkspacePackagePaths(root: string): Promise<string[]> {
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
