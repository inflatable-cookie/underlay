import { describe, expect, it } from 'vitest';
import {
	planMergeLocalCleanup,
	type MergeLocalCleanupPlan,
} from '../../../scripts/lib/merge-pr-closeout-cleanup.ts';

const OID_A = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
const OID_B = 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';

function kinds(plan: MergeLocalCleanupPlan): string {
	return plan.kind;
}

describe('planMergeLocalCleanup', () => {
	it('reports absent when the local branch tip is missing', () => {
		const plan = planMergeLocalCleanup({
			headRef: 'worker/example',
			providerHeadOid: OID_A,
			localBranchTip: null,
			holdingWorktrees: [],
		});
		expect(kinds(plan)).toBe('absent');
		expect(plan.lines.join('\n')).toContain('local branch already absent');
		expect(plan.lines.join('\n')).not.toContain('git branch -D');
	});

	it('suggests branch delete only when tips match and no worktree holds the branch', () => {
		const plan = planMergeLocalCleanup({
			headRef: 'worker/example',
			providerHeadOid: OID_A,
			localBranchTip: OID_A,
			holdingWorktrees: [],
		});
		expect(kinds(plan)).toBe('safe-cleanup');
		expect(plan.lines.join('\n')).toContain('git branch -D');
		expect(plan.lines.join('\n')).toContain(OID_A);
	});

	it('suggests worktree then branch cleanup when tips match', () => {
		const plan = planMergeLocalCleanup({
			headRef: 'worker/example',
			providerHeadOid: OID_A,
			localBranchTip: OID_A,
			holdingWorktrees: ['/tmp/worker-a'],
		});
		expect(kinds(plan)).toBe('safe-cleanup');
		const text = plan.lines.join('\n');
		expect(text).toContain('git worktree remove');
		expect(text).toContain('git branch -D');
		expect(text).toContain('/tmp/worker-a');
	});

	it('refuses destructive cleanup when local tip diverges from provider head', () => {
		const plan = planMergeLocalCleanup({
			headRef: 'worker/example',
			providerHeadOid: OID_A,
			localBranchTip: OID_B,
			holdingWorktrees: ['/tmp/worker-a'],
		});
		expect(kinds(plan)).toBe('inspect-divergence');
		const text = plan.lines.join('\n');
		expect(text).toContain('inspect only');
		expect(text).not.toContain('git branch -D');
		expect(text).not.toContain('git worktree remove');
		expect(text).toContain('/tmp/worker-a');
	});
});
