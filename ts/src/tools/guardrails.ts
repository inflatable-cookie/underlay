#!/usr/bin/env bun

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
 * bun underlay/ts/src/tools/guardrails.ts
 *
 * # Custom config
 * bun underlay/ts/src/tools/guardrails.ts --config .guardrailsrc.json
 *
 * # Custom source directory
 * bun underlay/ts/src/tools/guardrails.ts --src ./app
 * ```
 */

import { readdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { getLineNumberFromIndex, getLineStarts } from './guardrails/line-utils.js';
import { scanModuleScopeBrowserApis } from './guardrails/scanner.js';
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
  bun underlay/ts/src/tools/guardrails.ts [options]

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
