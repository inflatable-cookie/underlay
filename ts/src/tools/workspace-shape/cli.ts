import path from "node:path";
import { checkWorkspaceShape } from "./check.js";
import { formatWorkspaceShapeReport } from "./report.js";

export function runWorkspaceShapeCli(argv: string[] = process.argv): void {
	(async () => {
		const args = argv.slice(2);
		let rootArg: string | undefined;

		for (const arg of args) {
			if (arg === "--help" || arg === "-h") {
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

			if (arg.startsWith("-")) {
				throw new Error(`Unknown option: ${arg}. Run with --help for usage.`);
			}

			rootArg = arg;
		}

		const root = path.resolve(rootArg ?? process.cwd());
		const violations = await checkWorkspaceShape(root);
		console.log(formatWorkspaceShapeReport(root, violations));

		if (violations.length > 0) {
			process.exit(1);
		}
	})().catch((error: unknown) => {
		console.error(error instanceof Error ? error.message : String(error));
		process.exit(1);
	});
}
