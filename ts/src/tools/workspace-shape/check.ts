import path from 'node:path';
import {
	checkInternalEdges,
	checkSharedFileDependencies,
	collectInternalPackageNames,
} from './dependencies.js';
import { readPackageJson } from './fs.js';
import { sortViolations, type WorkspaceShapeViolation } from './model.js';
import {
	checkGitRoot,
	checkLockfiles,
	checkNestedGitRepos,
	checkRootManifestFields,
	checkWorkspaceMembership,
	checkWorkspacePaths,
} from './topology.js';

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
