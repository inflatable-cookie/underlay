import type { WorkspaceShapeViolation } from './model.js';

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
