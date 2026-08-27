import path from 'node:path';
import { readPackageJson } from './fs.js';
import {
	DEPENDENCY_FIELDS,
	RELEASED_SHARED_PACKAGE_NAMES,
	WORKSPACE_SHAPE_RULE_IDS,
	pushViolation,
	type PackageJson,
	type WorkspaceShapeViolation,
} from './model.js';

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

function isReleasedSharedPackageName(name: string): boolean {
	return RELEASED_SHARED_PACKAGE_NAMES.has(name) || name.startsWith('@inflatable-cookie/poodle-');
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

export async function collectInternalPackageNames(
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

export async function checkInternalEdges(
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

export async function checkSharedFileDependencies(
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
