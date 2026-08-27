import path from 'node:path';
import { checkWorkspaceShape } from './check.js';
import { formatWorkspaceShapeReport } from './report.js';

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
