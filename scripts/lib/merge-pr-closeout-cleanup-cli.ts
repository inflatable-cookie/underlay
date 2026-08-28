#!/usr/bin/env bun

import {
	assertPreMergeReviewedHead,
	formatMergeLocalCleanupPlan,
	formatPreMergeReviewedHeadFailure,
	formatReviewedMergeFailure,
	planMergeLocalCleanup,
	verifyReviewedMergeHead,
} from './merge-pr-closeout-cleanup.ts';

function usage(): never {
	console.error(`Usage:
  bun scripts/lib/merge-pr-closeout-cleanup-cli.ts cleanup \\
    --head-ref <branch> \\
    --provider-oid <sha> \\
    --local-tip <sha|""> \\
    [--worktree <path>]...

  bun scripts/lib/merge-pr-closeout-cleanup-cli.ts assert-pre-merge \\
    --reviewed-oid <sha> \\
    --provider-oid <sha>

  bun scripts/lib/merge-pr-closeout-cleanup-cli.ts verify-reviewed-head \\
    --reviewed-oid <sha> \\
    --observed-oid <sha> \\
    --state <OPEN|MERGED|...>
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
if (argv.length === 0 || argv.includes('-h') || argv.includes('--help')) usage();

const mode = argv[0];
const rest = argv.slice(1);

if (mode === 'cleanup') {
	const headRef = readArg(rest, '--head-ref');
	const providerOid = readArg(rest, '--provider-oid');
	const localTipRaw = readArg(rest, '--local-tip');
	if (!headRef || !providerOid || localTipRaw === undefined) usage();

	const plan = planMergeLocalCleanup({
		headRef,
		providerHeadOid: providerOid,
		localBranchTip: localTipRaw.length > 0 ? localTipRaw : null,
		holdingWorktrees: readAllArgs(rest, '--worktree'),
	});
	console.log(formatMergeLocalCleanupPlan(plan));
	process.exit(0);
}

if (mode === 'assert-pre-merge') {
	const reviewedOid = readArg(rest, '--reviewed-oid');
	const providerOid = readArg(rest, '--provider-oid');
	if (!reviewedOid || !providerOid) usage();

	const result = assertPreMergeReviewedHead({
		reviewedHeadOid: reviewedOid,
		providerHeadOid: providerOid,
	});
	if (!result.ok) {
		console.error(formatPreMergeReviewedHeadFailure(result));
		process.exit(1);
	}
	process.exit(0);
}

if (mode === 'verify-reviewed-head') {
	const reviewedOid = readArg(rest, '--reviewed-oid');
	const observedOid = readArg(rest, '--observed-oid');
	const state = readArg(rest, '--state');
	if (!reviewedOid || !observedOid || !state) usage();

	const result = verifyReviewedMergeHead({
		reviewedHeadOid: reviewedOid,
		observedHeadOid: observedOid,
		state,
	});
	if (!result.ok) {
		console.error(formatReviewedMergeFailure(result));
		process.exit(1);
	}
	process.exit(0);
}

usage();
