/**
 * SvelteKit SSR Safety Rules
 *
 * Prevents module-scope browser API usage that would break SSR.
 *
 * These rules enforce:
 * - No `window.*`, `document.*`, `navigator.*` at module scope
 * - No storage APIs (`localStorage`, `sessionStorage`) at module scope
 * - No location/history APIs at module scope
 *
 * Safe alternatives:
 * - Use `onMount(() => { ... })` for client-only code
 * - Use `if (typeof window !== "undefined")` guards
 * - Use dynamic imports for client-only modules
 *
 * @example
 * ```typescript
 * // ❌ BAD: Module-scope browser API
 * const width = window.innerWidth;
 *
 * // ✅ GOOD: Inside onMount
 * let width = 0;
 * onMount(() => {
 *   width = window.innerWidth;
 * });
 *
 * // ✅ GOOD: Type guard
 * const width = typeof window !== "undefined" ? window.innerWidth : 0;
 * ```
 */

import type { ModuleScopeCheck } from '../guardrails.js';

export const moduleScopeChecks: ModuleScopeCheck[] = [
	{
		name: 'window.*',
		kind: 'prefix',
		value: 'window.',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'document.*',
		kind: 'prefix',
		value: 'document.',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'navigator.*',
		kind: 'prefix',
		value: 'navigator.',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'localStorage',
		kind: 'identifier',
		value: 'localStorage',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'sessionStorage',
		kind: 'identifier',
		value: 'sessionStorage',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'matchMedia(...)',
		kind: 'call',
		value: 'matchMedia',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'location.*',
		kind: 'prefix',
		value: 'location.',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'history.*',
		kind: 'prefix',
		value: 'history.',
		message: 'No browser globals at module scope. Use onMount(), a typeof guard, or a client-only dynamic import.'
	},
	{
		name: 'configureAuth(...)',
		kind: 'call',
		value: 'configureAuth',
		message: 'No configureAuth() at module scope. It sets process-global auth config shared across concurrent SSR requests and can leak tokens between users. Call it in onMount() or behind a typeof window guard.'
	},
	{
		name: 'configureNightfireStrategies(...)',
		kind: 'call',
		value: 'configureNightfireStrategies',
		message: 'No configureNightfireStrategies() at module scope. Its fetchStrategies closure usually captures per-user auth; module-global state is shared across concurrent SSR requests. Call it in onMount() or behind a typeof window guard.'
	}
];

export default moduleScopeChecks;
