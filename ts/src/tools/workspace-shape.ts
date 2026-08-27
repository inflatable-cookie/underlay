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

import { runWorkspaceShapeCli } from './workspace-shape/cli.js';

export { WORKSPACE_SHAPE_RULE_IDS } from './workspace-shape/model.js';
export type { WorkspaceShapeRuleId, WorkspaceShapeViolation } from './workspace-shape/model.js';
export { checkWorkspaceShape } from './workspace-shape/check.js';
export { formatWorkspaceShapeReport } from './workspace-shape/report.js';
export { runWorkspaceShapeCli };

if (import.meta.url === `file://${process.argv[1]}`) {
	runWorkspaceShapeCli();
}
