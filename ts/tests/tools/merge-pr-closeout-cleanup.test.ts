import { describe, expect, it } from 'vitest';
import {
	assertPreMergeReviewedHead,
	formatPreMergeReviewedHeadFailure,
	formatReviewedMergeFailure,
	planMergeLocalCleanup,
	verifyReviewedMergeHead,
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

describe('assertPreMergeReviewedHead', () => {
	it('allows merge only when the live provider head equals the caller-reviewed OID', () => {
		const result = assertPreMergeReviewedHead({
			reviewedHeadOid: OID_A,
			providerHeadOid: OID_A,
		});
		expect(result).toEqual({ ok: true, reviewedHeadOid: OID_A });
	});

	it('refuses when the head changed after review but before wrapper invocation', () => {
		const result = assertPreMergeReviewedHead({
			reviewedHeadOid: OID_A,
			providerHeadOid: OID_B,
		});
		expect(result.ok).toBe(false);
		if (result.ok) return;
		expect(result.kind).toBe('head-changed-before-merge');
		expect(formatPreMergeReviewedHeadFailure(result)).toContain(
			'provider head differs from the caller-supplied reviewed OID',
		);
	});
});

describe('verifyReviewedMergeHead', () => {
	it('accepts a MERGED PR that still records the reviewed head OID', () => {
		const result = verifyReviewedMergeHead({
			reviewedHeadOid: OID_A,
			observedHeadOid: OID_A,
			state: 'MERGED',
		});
		expect(result).toEqual({ ok: true, reviewedHeadOid: OID_A });
	});

	it('refuses when the provider state is not MERGED', () => {
		const result = verifyReviewedMergeHead({
			reviewedHeadOid: OID_A,
			observedHeadOid: OID_A,
			state: 'OPEN',
		});
		expect(result.ok).toBe(false);
		if (result.ok) return;
		expect(result.kind).toBe('not-merged');
		expect(formatReviewedMergeFailure(result)).toContain('did not complete');
	});

	it('refuses the changed-head path when observed OID differs after merge', () => {
		const result = verifyReviewedMergeHead({
			reviewedHeadOid: OID_A,
			observedHeadOid: OID_B,
			state: 'MERGED',
		});
		expect(result.ok).toBe(false);
		if (result.ok) return;
		expect(result.kind).toBe('head-changed');
		expect(formatReviewedMergeFailure(result)).toContain('differs from the reviewed OID');
	});
});
