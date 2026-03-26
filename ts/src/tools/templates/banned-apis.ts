/**
 * Banned Browser APIs Template
 *
 * Common patterns to ban in modern web applications.
 *
 * These rules enforce better UX patterns:
 * - No blocking dialogs (alert, confirm, prompt)
 * - No direct clipboard access (use helpers for better error handling)
 *
 * @example
 * ```typescript
 * // ❌ BAD: Blocking alert
 * window.alert("Hello");
 *
 * // ✅ GOOD: Use toast or dialog component
 * showToast({ message: "Hello", type: "info" });
 *
 * // ❌ BAD: Blocking confirm
 * if (window.confirm("Delete?")) { ... }
 *
 * // ✅ GOOD: Use AlertDialog
 * <AlertDialog
 *   title="Confirm deletion"
 *   onConfirm={() => { ... }}
 * />
 * ```
 */

import type { BannedPattern } from '../guardrails.js';

export const bannedPatterns: BannedPattern[] = [
	{
		name: 'window.alert',
		regex: /\bwindow\.alert\s*\(/g,
		message: 'Use a toast or dialog component instead of window.alert().'
	},
	{
		name: 'window.confirm',
		regex: /\bwindow\.confirm\s*\(/g,
		message: 'Use AlertDialog instead of window.confirm().'
	},
	{
		name: 'window.prompt',
		regex: /\bwindow\.prompt\s*\(/g,
		message: 'Use a form dialog instead of window.prompt().'
	},
	{
		name: 'navigator.clipboard',
		regex: /\bnavigator\.clipboard\b/g,
		message: 'Use a clipboard helper function for better error handling and user feedback.'
	}
];

export default bannedPatterns;
