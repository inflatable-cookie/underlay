#!/usr/bin/env node

/**
 * Guardrails - Architectural rule enforcement for TypeScript and Svelte projects
 *
 * Scans source files for:
 * - Banned patterns (e.g., window.alert, navigator.clipboard)
 * - Module-scope browser API usage (SSR safety)
 *
 * Supports suppression via comments:
 * - `guardrails-disable-line [rule-id]`
 * - `guardrails-disable-next-line [rule-id]`
 *
 * @example
 * ```bash
 * # Use default config
 * node --import tsx underlay/ts/src/tools/guardrails.ts
 *
 * # Custom config
 * node --import tsx underlay/ts/src/tools/guardrails.ts --config .guardrailsrc.json
 *
 * # Custom source directory
 * node --import tsx underlay/ts/src/tools/guardrails.ts --src ./app
 * ```
 */

import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { getLineNumberFromIndex, getLineStarts } from './guardrails/line-utils.js';
import { isSuppressed } from './guardrails/suppressions.js';

// =============================================================================
// Types
// =============================================================================

export interface BannedPattern {
	name: string;
	regex: RegExp;
	message: string;
}

export interface ModuleScopeCheck {
	name: string;
	kind: 'prefix' | 'identifier' | 'call';
	value: string;
	message: string;
}

export interface GuardrailsConfig {
	srcDir: string;
	extensions: string[];
	bannedPatterns: BannedPattern[];
	moduleScopeChecks: ModuleScopeCheck[];
	suppressionPrefix: string;
}

interface Issue {
	index: number;
	name: string;
	message: string;
}

interface SvelteScriptBlock {
	attrs: string;
	content: string;
	contentStart: number;
}

// =============================================================================
// File Walker
// =============================================================================

async function* walk(dirUrl: URL, extensions: string[]): AsyncGenerator<URL> {
	const dirPath = fileURLToPath(dirUrl);
	const entries = await readdir(dirPath, { withFileTypes: true });

	for (const entry of entries) {
		if (entry.name.startsWith('.')) continue;

		const entryUrl = new URL(entry.name + (entry.isDirectory() ? '/' : ''), dirUrl);

		if (entry.isDirectory()) {
			yield* walk(entryUrl, extensions);
		} else {
			const ext = path.extname(entry.name);
			if (extensions.includes(ext)) {
				yield entryUrl;
			}
		}
	}
}

function toRelative(fileUrl: URL, rootUrl: URL): string {
	const filePath = fileURLToPath(fileUrl);
	const rootPath = fileURLToPath(rootUrl);
	return path.relative(rootPath, filePath);
}

// =============================================================================
// Module-Scope Browser API Scanner
// =============================================================================

function isIdentChar(char: string): boolean {
	return /[a-zA-Z0-9_$]/.test(char);
}

function hasIdentifierAt(text: string, index: number, identifier: string): boolean {
	if (!text.startsWith(identifier, index)) return false;

	const before = index === 0 ? '' : text[index - 1];
	const after = text[index + identifier.length] ?? '';

	if (before && isIdentChar(before)) return false;
	if (after && isIdentChar(after)) return false;

	return true;
}

function hasCallAt(text: string, index: number, identifier: string): boolean {
	if (!hasIdentifierAt(text, index, identifier)) return false;

	let i = index + identifier.length;
	while (i < text.length && /\s/.test(text[i])) i++;

	return text[i] === '(';
}

function isGuardedContext(text: string, boundaryIndex: number, matchIndex: number): boolean {
	const windowStart = Math.max(boundaryIndex, matchIndex - 140);
	const context = text.slice(windowStart, matchIndex);

	// Guardrails are intentionally strict: `if (browser)` is not considered a safe
	// module-scope guard; prefer explicit `typeof` checks or `onMount()`.
	return /typeof\s+(window|document|navigator|location|history)\b/.test(context);
}

function isFunctionBodyStart(text: string, braceIndex: number): boolean {
	let j = braceIndex - 1;
	while (j >= 0 && /\s/.test(text[j])) j--;

	// Arrow function body: `(...) => {`
	if (text[j] === '>') {
		let k = j - 1;
		while (k >= 0 && /\s/.test(text[k])) k--;
		if (text[k] === '=') return true;
	}

	// Function declaration/expression: `function ... {`
	const prefixStart = Math.max(0, braceIndex - 120);
	const prefix = text.slice(prefixStart, braceIndex);
	if (/\bfunction\b/.test(prefix)) return true;

	// Method body: `method(...) {` (but not `if (...) {`, `for (...) {`, etc.)
	if (text[j] === ')') {
		let depth = 0;
		for (let p = j; p >= 0; p--) {
			const c = text[p];
			if (c === ')') depth++;
			if (c === '(') {
				depth--;
				if (depth === 0) {
					let q = p - 1;
					while (q >= 0 && /\s/.test(text[q])) q--;

					let start = q;
					while (start >= 0 && isIdentChar(text[start])) start--;
					const word = text.slice(start + 1, q + 1);

					if (['if', 'for', 'while', 'switch', 'catch', 'with'].includes(word)) {
						return false;
					}

					return word.length > 0;
				}
			}
		}
	}

	return false;
}

function scanModuleScopeBrowserApis(
	text: string,
	baseIndex: number,
	checks: ModuleScopeCheck[]
): Issue[] {
	const issues: Issue[] = [];

	/**
	 * Modes:
	 * - code
	 * - line_comment
	 * - block_comment
	 * - single_quote
	 * - double_quote
	 * - template
	 * - template_expr
	 */
	let mode: 'code' | 'line_comment' | 'block_comment' | 'single_quote' | 'double_quote' | 'template' | 'template_expr' = 'code';

	const braceStack: boolean[] = [];
	let functionDepth = 0;

	let boundaryIndex = 0;
	let templateExprBraceBalance = 0;

	for (let i = 0; i < text.length; i++) {
		const char = text[i];
		const next = text[i + 1] ?? '';

		if (char === '\n') {
			if (mode === 'line_comment') mode = 'code';
			if (functionDepth === 0) boundaryIndex = i + 1;
			continue;
		}

		if (mode === 'line_comment') continue;

		if (mode === 'block_comment') {
			if (char === '*' && next === '/') {
				mode = 'code';
				i++;
			}
			continue;
		}

		if (mode === 'single_quote') {
			if (char === '\\') {
				i++;
				continue;
			}

			if (char === "'") mode = 'code';
			continue;
		}

		if (mode === 'double_quote') {
			if (char === '\\') {
				i++;
				continue;
			}

			if (char === '"') mode = 'code';
			continue;
		}

		if (mode === 'template') {
			if (char === '\\') {
				i++;
				continue;
			}

			if (char === '`') {
				mode = 'code';
				continue;
			}

			if (char === '$' && next === '{') {
				mode = 'template_expr';
				templateExprBraceBalance = 0;
				i++;
			}

			continue;
		}

		// mode: code | template_expr
		if (char === '/' && next === '/') {
			mode = 'line_comment';
			i++;
			continue;
		}

		if (char === '/' && next === '*') {
			mode = 'block_comment';
			i++;
			continue;
		}

		if (char === "'") {
			mode = 'single_quote';
			continue;
		}

		if (char === '"') {
			mode = 'double_quote';
			continue;
		}

		if (char === '`') {
			mode = 'template';
			continue;
		}

		if (char === ';' && functionDepth === 0) {
			boundaryIndex = i + 1;
		}

		if (functionDepth === 0) {
			for (const check of checks) {
				let matched = false;

				if (check.kind === 'prefix') {
					matched = text.startsWith(check.value, i);
				} else if (check.kind === 'identifier') {
					matched = hasIdentifierAt(text, i, check.value);
				} else if (check.kind === 'call') {
					matched = hasCallAt(text, i, check.value);
				}

				if (matched && !isGuardedContext(text, boundaryIndex, i)) {
					issues.push({
						index: baseIndex + i,
						name: check.name,
						message: check.message
					});
					break;
				}
			}
		}

		if (mode === 'template_expr') {
			if (char === '{') {
				templateExprBraceBalance++;
				const isFunction = isFunctionBodyStart(text, i);
				braceStack.push(isFunction);
				if (isFunction) functionDepth++;
			} else if (char === '}') {
				if (templateExprBraceBalance === 0) {
					mode = 'template';
				} else {
					templateExprBraceBalance--;
					const wasFunction = braceStack.pop();
					if (wasFunction) functionDepth = Math.max(0, functionDepth - 1);
				}
			}

			continue;
		}

		if (char === '{') {
			const isFunction = isFunctionBodyStart(text, i);
			braceStack.push(isFunction);
			if (isFunction) functionDepth++;
		} else if (char === '}') {
			const wasFunction = braceStack.pop();
			if (wasFunction) functionDepth = Math.max(0, functionDepth - 1);
		}
	}

	return issues;
}

// =============================================================================
// Svelte Script Parser
// =============================================================================

function getSvelteScriptBlocks(text: string): SvelteScriptBlock[] {
	const blocks: SvelteScriptBlock[] = [];

	const scriptTag = /<script\b([^>]*)>([\s\S]*?)<\/script>/gi;
	let match: RegExpExecArray | null;
	while ((match = scriptTag.exec(text))) {
		const attrs = match[1] ?? '';
		const content = match[2] ?? '';
		const contentStart = match.index + match[0].indexOf(content);
		blocks.push({ attrs, content, contentStart });
	}

	return blocks;
}

// =============================================================================
// Main Scanner
// =============================================================================

export async function scanFiles(config: GuardrailsConfig): Promise<number> {
	const rootUrl = new URL(config.srcDir + '/', 'file://' + process.cwd() + '/');
	let failures = 0;

	for await (const fileUrl of walk(rootUrl, config.extensions)) {
		const filePath = fileURLToPath(fileUrl);
		const text = await readFile(filePath, 'utf8');
		const lineStarts = getLineStarts(text);

		// Scan for banned patterns
		for (const pattern of config.bannedPatterns) {
			pattern.regex.lastIndex = 0;

			let match: RegExpExecArray | null;
			while ((match = pattern.regex.exec(text))) {
				if (isSuppressed(text, lineStarts, match.index, ['banned', pattern.name])) {
					continue;
				}

				failures++;
				const line = getLineNumberFromIndex(lineStarts, match.index);
				console.error(`${toRelative(fileUrl, rootUrl)}:${line}: banned ${pattern.name}. ${pattern.message}`);
			}
		}

		const ext = path.extname(filePath);

		// Scan TypeScript files for module-scope browser APIs
		if (ext === '.ts') {
			for (const issue of scanModuleScopeBrowserApis(text, 0, config.moduleScopeChecks)) {
				if (
					isSuppressed(text, lineStarts, issue.index, [
						'module-scope',
						'module-scope-browser-api',
						issue.name
					])
				) {
					continue;
				}

				failures++;
				const line = getLineNumberFromIndex(lineStarts, issue.index);
				console.error(`${toRelative(fileUrl, rootUrl)}:${line}: module-scope ${issue.name}. ${issue.message}`);
			}
		}

		// Scan Svelte script blocks for module-scope browser APIs
		if (ext === '.svelte') {
			const blocks = getSvelteScriptBlocks(text);

			for (const block of blocks) {
				for (const issue of scanModuleScopeBrowserApis(block.content, block.contentStart, config.moduleScopeChecks)) {
					if (
						isSuppressed(text, lineStarts, issue.index, [
							'module-scope',
							'module-scope-browser-api',
							issue.name
						])
					) {
						continue;
					}

					failures++;
					const line = getLineNumberFromIndex(lineStarts, issue.index);
					console.error(`${toRelative(fileUrl, rootUrl)}:${line}: module-scope ${issue.name}. ${issue.message}`);
				}
			}
		}
	}

	return failures;
}

// =============================================================================
// CLI Entry Point
// =============================================================================

if (import.meta.url === `file://${process.argv[1]}`) {
	(async () => {
		// Parse CLI args
		const args = process.argv.slice(2);
		let srcDir: string | undefined;
		let configPath: string | undefined;

		for (let i = 0; i < args.length; i++) {
			if (args[i] === '--src' && args[i + 1]) {
				srcDir = args[i + 1];
				i++;
			} else if (args[i] === '--config' && args[i + 1]) {
				configPath = args[i + 1];
				i++;
			} else if (args[i] === '--help' || args[i] === '-h') {
				console.log(`
Guardrails - Architectural rule enforcement for TypeScript and Svelte

Usage:
  node --import tsx underlay/ts/src/tools/guardrails.ts [options]

Options:
  --config <path>   Path to config file (.guardrailsrc.json)
  --src <dir>       Source directory to scan (default: ./src)
  --help, -h        Show this help message

Configuration:
  Create a .guardrailsrc.json file in your project root:

  {
    "srcDir": "./src",
    "extensions": [".ts", ".svelte"],
    "bannedPatterns": [
      {
        "name": "window.alert",
        "regex": "\\\\bwindow\\\\.alert\\\\s*\\\\(",
        "message": "Use a toast instead"
      }
    ],
    "moduleScopeChecks": "@decodelabs/underlay/tools/templates/sveltekit-ssr"
  }

Suppression:
  Use comments to suppress specific rules:
  
  // guardrails-disable-next-line window.alert
  window.alert("OK");

  const width = window.innerWidth; // guardrails-disable-line module-scope-browser-api
				`);
				process.exit(0);
			}
		}

		// Load config (with CLI overrides)
		const { loadConfig } = await import('./guardrails-config.js');
		const config = await loadConfig(configPath, srcDir);

		console.log(`Scanning ${config.srcDir} for guardrails violations...`);

		const failures = await scanFiles(config);

		if (failures > 0) {
			console.error(`\nGuardrails failed: ${failures} issue(s) found.`);
			process.exit(1);
		}

		console.log('✓ Guardrails passed: no issues found.');
	})();
}
