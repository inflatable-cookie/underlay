/**
 * Configuration loader for guardrails
 *
 * Loads config from:
 * 1. CLI arguments (highest priority)
 * 2. .guardrailsrc.json
 * 3. package.json "guardrails" field
 * 4. Built-in defaults (lowest priority)
 */

import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import type { BannedPattern, ModuleScopeCheck, GuardrailsConfig } from './guardrails.js';

interface GuardrailsConfigFile {
	srcDir?: string;
	extensions?: string[];
	bannedPatterns?: Array<{
		name: string;
		regex: string;
		message: string;
	}>;
	moduleScopeChecks?: ModuleScopeCheck[] | string; // Can be array or template reference
	suppressionPrefix?: string;
}

/**
 * Load configuration from file system
 */
export async function loadConfig(configPath?: string, srcDirOverride?: string): Promise<GuardrailsConfig> {
	let config: GuardrailsConfigFile = {};

	// Try loading from config file
	if (configPath) {
		try {
			const content = await readFile(configPath, 'utf8');
			config = JSON.parse(content);
		} catch (error) {
			console.error(`Warning: Could not load config from ${configPath}`);
		}
	} else {
		// Try .guardrailsrc.json
		try {
			const content = await readFile('.guardrailsrc.json', 'utf8');
			config = JSON.parse(content);
		} catch {
			// Try package.json
			try {
				const pkgContent = await readFile('package.json', 'utf8');
				const pkg = JSON.parse(pkgContent);
				if (pkg.guardrails) {
					config = pkg.guardrails;
				}
			} catch {
				// Use defaults
			}
		}
	}

	// Parse regex patterns from strings
	const bannedPatterns: BannedPattern[] = (config.bannedPatterns ?? []).map((p) => ({
		name: p.name,
		regex: new RegExp(p.regex, 'g'),
		message: p.message
	}));

	// Load module scope checks (can be template reference or inline array)
	let moduleScopeChecks: ModuleScopeCheck[] = [];
	if (typeof config.moduleScopeChecks === 'string') {
		// Template reference (e.g., "@decodelabs/underlay/tools/templates/sveltekit-ssr")
		try {
			const templatePath = config.moduleScopeChecks.replace(
				'@decodelabs/underlay/tools/templates/',
				resolve(process.cwd(), 'ts/src/tools/templates/') + '/'
			);
			const template = await import(templatePath);
			moduleScopeChecks = template.moduleScopeChecks ?? template.default ?? [];
		} catch (error) {
			console.error(`Warning: Could not load template ${config.moduleScopeChecks}`);
		}
	} else if (Array.isArray(config.moduleScopeChecks)) {
		moduleScopeChecks = config.moduleScopeChecks;
	}

	return {
		srcDir: srcDirOverride ?? config.srcDir ?? './src',
		extensions: config.extensions ?? ['.ts', '.svelte'],
		bannedPatterns,
		moduleScopeChecks,
		suppressionPrefix: config.suppressionPrefix ?? 'guardrails-disable'
	};
}

/**
 * Get default configuration (used when no config file found)
 */
export function getDefaultConfig(): GuardrailsConfig {
	return {
		srcDir: './src',
		extensions: ['.ts', '.svelte'],
		bannedPatterns: [],
		moduleScopeChecks: [],
		suppressionPrefix: 'guardrails-disable'
	};
}
