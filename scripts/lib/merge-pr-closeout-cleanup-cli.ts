#!/usr/bin/env bun

import {
	formatMergeLocalCleanupPlan,
	planMergeLocalCleanup,
} from './merge-pr-closeout-cleanup.ts';

function usage(): never {
	console.error(`Usage:
  bun scripts/lib/merge-pr-closeout-cleanup-cli.ts \\
    --head-ref <branch> \\
    --provider-oid <sha> \\
    --local-tip <sha|""> \\
    [--worktree <path>]...
`);
	process.exit(2);
}

function readArg(argv: string[], flag: string): string | undefined {
	const index = argv.indexOf(flag);
	if (index === -1) return undefined;
	return argv[index + 1];
}

function readAllArgs(argv: string[], flag: string): string[] {
	const values: string[] = [];
	for (let i = 0; i < argv.length; i++) {
		if (argv[i] === flag && argv[i + 1]) {
			values.push(argv[i + 1]!);
			i += 1;
		}
	}
	return values;
}

const argv = process.argv.slice(2);
if (argv.includes('-h') || argv.includes('--help')) usage();

const headRef = readArg(argv, '--head-ref');
const providerOid = readArg(argv, '--provider-oid');
const localTipRaw = readArg(argv, '--local-tip');

if (!headRef || !providerOid || localTipRaw === undefined) usage();

const plan = planMergeLocalCleanup({
	headRef,
	providerHeadOid: providerOid,
	localBranchTip: localTipRaw.length > 0 ? localTipRaw : null,
	holdingWorktrees: readAllArgs(argv, '--worktree'),
});

console.log(formatMergeLocalCleanupPlan(plan));
process.exit(0);
