#!/usr/bin/env bun

/**
 * Consumer env/secret-authority checker.
 *
 * Validates the static `config/env-manifest.txt` and
 * `config/required-secrets.txt` inventory defined by contracts 024 and 031.
 * Separate from the live value check in `scripts/check-env-manifest.sh`.
 *
 * @example
 * ```bash
 * bun underlay/ts/src/tools/env-authority.ts /path/to/consumer
 * bun underlay/ts/src/tools/env-authority.ts .
 * ```
 */

import { readdir, readFile, stat } from 'node:fs/promises';
import path from 'node:path';

export const ENV_AUTHORITY_RULE_IDS = {
	ENV_MANIFEST_MISSING: 'env-manifest-missing',
	REQUIRED_SECRETS_MISSING: 'required-secrets-missing',
	ENV_MANIFEST_INVALID: 'env-manifest-invalid',
	REQUIRED_SECRETS_INVALID: 'required-secrets-invalid',
	REQUIRED_SECRET_UNDECLARED: 'required-secret-undeclared',
} as const;

export type EnvAuthorityRuleId = (typeof ENV_AUTHORITY_RULE_IDS)[keyof typeof ENV_AUTHORITY_RULE_IDS];

export interface EnvAuthorityViolation {
	ruleId: EnvAuthorityRuleId;
	path: string;
	detail: string;
}

const ENV_MANIFEST_RELATIVE = 'config/env-manifest.txt';
const REQUIRED_SECRETS_RELATIVE = 'config/required-secrets.txt';
const KEY_PATTERN = /^[A-Za-z_][A-Za-z0-9_]*$/;
const SOURCE_EXTENSIONS = new Set(['.rs', '.ts', '.js', '.mjs', '.cjs', '.jsx', '.tsx', '.svelte']);

const SKIP_DIR_NAMES = new Set([
	'node_modules',
	'target',
	'dist',
	'build',
	'.turbo',
	'coverage',
	'.svelte-kit',
	'.effigy',
	'.cache',
]);

const ENV_READER_PATTERNS = [
	/\bstd::env::/,
	/\benv::var(?:_os)?\s*\(/,
	/\bprocess\.env\b/,
	/\bimport\.meta\.env\b/,
];

function pushViolation(
	violations: EnvAuthorityViolation[],
	ruleId: EnvAuthorityRuleId,
	filePath: string,
	detail: string,
): void {
	violations.push({ ruleId, path: filePath, detail });
}

function sortViolations(violations: EnvAuthorityViolation[]): EnvAuthorityViolation[] {
	return [...violations].sort((a, b) => {
		if (a.ruleId !== b.ruleId) return a.ruleId.localeCompare(b.ruleId);
		if (a.path !== b.path) return a.path.localeCompare(b.path);
		return a.detail.localeCompare(b.detail);
	});
}

async function pathExists(target: string): Promise<boolean> {
	try {
		await stat(target);
		return true;
	} catch {
		return false;
	}
}

async function readTextFile(target: string): Promise<string | undefined> {
	try {
		return await readFile(target, 'utf8');
	} catch {
		return undefined;
	}
}

function isSkippedDir(name: string): boolean {
	return name === '.git' || SKIP_DIR_NAMES.has(name);
}

function isSourceFile(fileName: string): boolean {
	if (fileName.endsWith('.d.ts')) return false;
	return SOURCE_EXTENSIONS.has(path.extname(fileName));
}

function looksLikeEnvReader(content: string): boolean {
	return ENV_READER_PATTERNS.some((pattern) => pattern.test(content));
}

async function findEnvReaderPaths(root: string): Promise<string[]> {
	const found: string[] = [];

	async function walk(relative: string): Promise<void> {
		const dirPath = relative ? path.join(root, relative) : root;
		let entries;
		try {
			entries = await readdir(dirPath, { withFileTypes: true });
		} catch {
			return;
		}

		for (const entry of entries) {
			if (isSkippedDir(entry.name)) continue;

			const entryRelative = relative ? `${relative}/${entry.name}` : entry.name;
			if (entry.isDirectory()) {
				await walk(entryRelative);
				continue;
			}

			if (!entry.isFile() || !isSourceFile(entry.name)) continue;

			const content = await readTextFile(path.join(root, entryRelative));
			if (content && looksLikeEnvReader(content)) {
				found.push(entryRelative.replace(/\\/g, '/'));
			}
		}
	}

	await walk('');
	return found.sort((a, b) => a.localeCompare(b));
}

interface ParsedKeyFile {
	keys: string[];
	duplicates: string[];
	invalidLines: string[];
}

function parseKeyFile(content: string): ParsedKeyFile {
	const keys: string[] = [];
	const seen = new Set<string>();
	const duplicates: string[] = [];
	const invalidLines: string[] = [];

	for (const rawLine of content.split(/\r?\n/)) {
		const withoutComment = rawLine.replace(/#.*$/, '');
		const line = withoutComment.trim();
		if (line.length === 0) continue;

		if (line.includes('=')) {
			invalidLines.push(line);
			continue;
		}

		if (!KEY_PATTERN.test(line)) {
			invalidLines.push(line);
			continue;
		}

		if (seen.has(line)) {
			duplicates.push(line);
			continue;
		}

		seen.add(line);
		keys.push(line);
	}

	return { keys, duplicates, invalidLines };
}

function recordParseViolations(
	violations: EnvAuthorityViolation[],
	relativePath: string,
	parsed: ParsedKeyFile,
	invalidRule: EnvAuthorityRuleId,
): void {
	for (const line of parsed.invalidLines.sort((a, b) => a.localeCompare(b))) {
		pushViolation(
			violations,
			invalidRule,
			relativePath,
			`line must be a KEY token without a value (found ${JSON.stringify(line)})`,
		);
	}

	for (const key of parsed.duplicates.sort((a, b) => a.localeCompare(b))) {
		pushViolation(violations, invalidRule, relativePath, `duplicate key ${key}`);
	}
}

export async function checkEnvAuthority(rootPath: string): Promise<EnvAuthorityViolation[]> {
	const root = path.resolve(rootPath);
	const violations: EnvAuthorityViolation[] = [];
	const readerPaths = await findEnvReaderPaths(root);
	const firstReader = readerPaths[0];

	const manifestAbsolute = path.join(root, ENV_MANIFEST_RELATIVE);
	const requiredAbsolute = path.join(root, REQUIRED_SECRETS_RELATIVE);
	const manifestExists = await pathExists(manifestAbsolute);
	const requiredExists = await pathExists(requiredAbsolute);
	const authorityRequired = readerPaths.length > 0 || manifestExists || requiredExists;

	if (authorityRequired && !manifestExists) {
		const detail = firstReader
			? `runtime env reader at ${firstReader} has no ${ENV_MANIFEST_RELATIVE} authority`
			: `${ENV_MANIFEST_RELATIVE} is missing`;
		pushViolation(
			violations,
			ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_MISSING,
			ENV_MANIFEST_RELATIVE,
			detail,
		);
	}

	if (authorityRequired && !requiredExists) {
		const detail = firstReader
			? `runtime env reader at ${firstReader} has no ${REQUIRED_SECRETS_RELATIVE} authority`
			: `${REQUIRED_SECRETS_RELATIVE} is missing`;
		pushViolation(
			violations,
			ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_MISSING,
			REQUIRED_SECRETS_RELATIVE,
			detail,
		);
	}

	let manifestKeys: string[] = [];
	if (manifestExists) {
		const content = await readTextFile(manifestAbsolute);
		if (content === undefined) {
			pushViolation(
				violations,
				ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_INVALID,
				ENV_MANIFEST_RELATIVE,
				'env manifest is unreadable',
			);
		} else {
			const parsed = parseKeyFile(content);
			recordParseViolations(
				violations,
				ENV_MANIFEST_RELATIVE,
				parsed,
				ENV_AUTHORITY_RULE_IDS.ENV_MANIFEST_INVALID,
			);
			manifestKeys = parsed.keys;
		}
	}

	if (requiredExists) {
		const content = await readTextFile(requiredAbsolute);
		if (content === undefined) {
			pushViolation(
				violations,
				ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_INVALID,
				REQUIRED_SECRETS_RELATIVE,
				'required-secrets file is unreadable',
			);
		} else {
			const parsed = parseKeyFile(content);
			recordParseViolations(
				violations,
				REQUIRED_SECRETS_RELATIVE,
				parsed,
				ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRETS_INVALID,
			);

			if (manifestExists) {
				const manifestSet = new Set(manifestKeys);
				for (const key of parsed.keys) {
					if (!manifestSet.has(key)) {
						pushViolation(
							violations,
							ENV_AUTHORITY_RULE_IDS.REQUIRED_SECRET_UNDECLARED,
							REQUIRED_SECRETS_RELATIVE,
							`${key} is required but not declared in ${ENV_MANIFEST_RELATIVE}`,
						);
					}
				}
			}
		}
	}

	return sortViolations(violations);
}

export function formatEnvAuthorityReport(
	rootPath: string,
	violations: EnvAuthorityViolation[],
): string {
	const lines: string[] = [`Env authority report for: ${rootPath}`, ''];

	if (violations.length === 0) {
		lines.push('All env authority checks passed.');
		return lines.join('\n');
	}

	for (const violation of violations) {
		lines.push(`  FAIL  ${violation.ruleId}: ${violation.path} — ${violation.detail}`);
	}

	lines.push('', `${violations.length} env authority violation(s) found.`);
	return lines.join('\n');
}

export function runEnvAuthorityCli(argv: string[] = process.argv): void {
	(async () => {
		const args = argv.slice(2);
		let rootArg: string | undefined;

		for (let i = 0; i < args.length; i++) {
			if (args[i] === '--help' || args[i] === '-h') {
				console.log(`
Env authority - Consumer env/secret inventory conformance

Usage:
  underlay-env-authority [path]
  bun underlay/ts/bin/underlay-env-authority.ts [path]

Options:
  --help, -h        Show this help message

The path defaults to the current working directory.

This check is static. It does not read .env files or secret values.
Live value presence remains scripts/check-env-manifest.sh.
`);
				process.exit(0);
			}

			if (!args[i].startsWith('-')) {
				rootArg = args[i];
			}
		}

		const root = path.resolve(rootArg ?? process.cwd());
		const violations = await checkEnvAuthority(root);
		console.log(formatEnvAuthorityReport(root, violations));

		if (violations.length > 0) {
			process.exit(1);
		}
	})();
}

if (import.meta.url === `file://${process.argv[1]}`) {
	runEnvAuthorityCli();
}
