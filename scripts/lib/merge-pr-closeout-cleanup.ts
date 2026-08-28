export type MergeLocalCleanupInput = {
	headRef: string;
	providerHeadOid: string;
	localBranchTip: string | null;
	holdingWorktrees: string[];
};

export type MergeLocalCleanupPlan = {
	kind: 'absent' | 'safe-cleanup' | 'inspect-divergence';
	lines: string[];
};

/**
 * Decide local branch/worktree cleanup messaging after a provider MERGED state.
 * Destructive cleanup is suggested only when the local tip matches the reviewed
 * provider head OID. Divergence requires manual inspection.
 */
export function planMergeLocalCleanup(input: MergeLocalCleanupInput): MergeLocalCleanupPlan {
	const { headRef, providerHeadOid, localBranchTip, holdingWorktrees } = input;
	const providerOid = providerHeadOid.trim().toLowerCase();
	const worktrees = holdingWorktrees.filter((entry) => entry.trim().length > 0);

	if (!localBranchTip) {
		return {
			kind: 'absent',
			lines: [
				`local cleanup: no registered worktree holds ${headRef}`,
				'local cleanup: local branch already absent',
			],
		};
	}

	const localOid = localBranchTip.trim().toLowerCase();
	if (!providerOid || localOid !== providerOid) {
		const lines = [
			`local cleanup: inspect only — local tip diverges from provider head OID`,
			`  headRef: ${headRef}`,
			`  providerHeadOid: ${providerHeadOid}`,
			`  localBranchTip: ${localBranchTip}`,
			'  do not remove the holding worktree or force-delete the branch until tips match or an operator confirms the local commits are disposable',
		];
		if (worktrees.length > 0) {
			lines.push('  holding worktree(s):');
			for (const worktree of worktrees) {
				lines.push(`    ${worktree}`);
			}
		}
		return { kind: 'inspect-divergence', lines };
	}

	if (worktrees.length === 0) {
		return {
			kind: 'safe-cleanup',
			lines: [
				`local cleanup: no registered worktree holds ${headRef}`,
				`local cleanup: local tip matches provider head OID ${providerHeadOid}`,
				'local cleanup: from the primary checkout run:',
				`  git branch -D ${shellQuote(headRef)}`,
			],
		};
	}

	const lines = [
		`local cleanup: head branch still belongs to registered worktree(s)`,
		`local cleanup: local tip matches provider head OID ${providerHeadOid}`,
	];
	for (const worktree of worktrees) {
		lines.push(`  worktree: ${worktree}`);
		lines.push('  safe cleanup:');
		lines.push(`    git worktree remove ${shellQuote(worktree)}`);
		lines.push(`    git branch -D ${shellQuote(headRef)}`);
	}
	return { kind: 'safe-cleanup', lines };
}

function shellQuote(value: string): string {
	return `'${value.replace(/'/g, `'\\''`)}'`;
}

export function formatMergeLocalCleanupPlan(plan: MergeLocalCleanupPlan): string {
	return plan.lines.join('\n');
}
