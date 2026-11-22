/**
 * Configuration loader for guardrails
 *
 * Loads config from:
 * 1. CLI arguments (highest priority)
 * 2. .guardrailsrc.json
 * 3. package.json "guardrails" field
 * 4. Built-in defaults (lowest priority)
 */

import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import type { BannedPattern, ModuleScopeCheck, GuardrailsConfig } from "./guardrails.js";

interface GuardrailsConfigFile {
	srcDir?: string;
	extensions?: string[];
	bannedPatterns?:
		| Array<{
				name: string;
				regex: string;
				message: string;
		  }>
		| string;
	moduleScopeChecks?: ModuleScopeCheck[] | string; // Can be array or template reference
	suppressionPrefix?: string;
}

function parsePatternConfigs(
	patterns: Array<{
		name: string;
		regex: string;
		message: string;
	}>,
): BannedPattern[] {
	return patterns.map((p) => ({
		name: p.name,
		regex: new RegExp(p.regex, "g"),
		message: p.message,
	}));
}

const TEMPLATE_SPECIFIER_PREFIX = "@inflatable-cookie/underlay/tools/templates/";

/** Templated rule packs shipped alongside this module. */
const BUNDLED_TEMPLATE_DIR = resolve(dirname(fileURLToPath(import.meta.url)), "templates");

async function importTemplateModule(templateRef: string): Promise<unknown> {
	try {
		return await import(templateRef);
	} catch {
		// Fallback for repo checkouts without the package alias. Resolve relative to
		// this module, not the caller's cwd, so the tool works from any directory.
		const normalizedRef = templateRef.startsWith(TEMPLATE_SPECIFIER_PREFIX)
			? templateRef.slice(TEMPLATE_SPECIFIER_PREFIX.length)
			: templateRef;
		const localTemplatePath = pathToFileURL(resolve(BUNDLED_TEMPLATE_DIR, normalizedRef)).href;
		return await import(localTemplatePath);
	}
}

async function readConfigObject(file: string, optional: boolean): Promise<object | undefined> {
	let content: string;
	try {
		content = await readFile(file, "utf8");
	} catch (error) {
		if (optional && error instanceof Error && "code" in error && error.code === "ENOENT") {
			return undefined;
		}
		throw new Error(`Could not load config from ${file}`, { cause: error });
	}

	let parsed: unknown;
	try {
		parsed = JSON.parse(content);
	} catch (error) {
		throw new Error(`Could not load config from ${file}: invalid JSON`, { cause: error });
	}
	if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
		throw new Error(`Config file ${file} must contain a JSON object`);
	}
	return parsed;
}

/**
 * Load configuration from file system
 */
export async function loadConfig(
	configPath?: string,
	srcDirOverride?: string,
): Promise<GuardrailsConfig> {
	const fileConfig = await readConfigObject(configPath || ".guardrailsrc.json", !configPath);
	const pkg = fileConfig ? undefined : await readConfigObject("package.json", true);
	const config = (fileConfig ??
		(pkg && "guardrails" in pkg ? pkg.guardrails : undefined) ??
		{}) as GuardrailsConfigFile;

	// Load banned patterns (can be template reference or inline array)
	let bannedPatterns: BannedPattern[] = [];
	if (typeof config.bannedPatterns === "string") {
		let template: unknown;
		try {
			template = await importTemplateModule(config.bannedPatterns);
		} catch (error) {
			throw new Error(`Could not load bannedPatterns template ${config.bannedPatterns}`, {
				cause: error,
			});
		}
		const resolved = template as { bannedPatterns?: BannedPattern[]; default?: BannedPattern[] };
		const rules = resolved.bannedPatterns ?? resolved.default;
		if (!Array.isArray(rules)) {
			throw new Error(`Template ${config.bannedPatterns} did not export a bannedPatterns array`);
		}
		bannedPatterns = rules;
	} else {
		bannedPatterns = parsePatternConfigs(config.bannedPatterns ?? []);
	}

	// Load module scope checks (can be template reference or inline array)
	let moduleScopeChecks: ModuleScopeCheck[] = [];
	if (typeof config.moduleScopeChecks === "string") {
		let template: unknown;
		try {
			template = await importTemplateModule(config.moduleScopeChecks);
		} catch (error) {
			throw new Error(`Could not load moduleScopeChecks template ${config.moduleScopeChecks}`, {
				cause: error,
			});
		}
		const resolved = template as {
			moduleScopeChecks?: ModuleScopeCheck[];
			default?: ModuleScopeCheck[];
		};
		const checks = resolved.moduleScopeChecks ?? resolved.default;
		if (!Array.isArray(checks)) {
			throw new Error(
				`Template ${config.moduleScopeChecks} did not export a moduleScopeChecks array`,
			);
		}
		moduleScopeChecks = checks;
	} else if (Array.isArray(config.moduleScopeChecks)) {
		moduleScopeChecks = config.moduleScopeChecks;
	}

	return {
		srcDir: srcDirOverride ?? config.srcDir ?? "./src",
		extensions: config.extensions ?? [".ts", ".svelte"],
		bannedPatterns,
		moduleScopeChecks,
		suppressionPrefix: config.suppressionPrefix ?? "guardrails-disable",
	};
}

/**
 * Get default configuration (used when no config file found)
 */
export function getDefaultConfig(): GuardrailsConfig {
	return {
		srcDir: "./src",
		extensions: [".ts", ".svelte"],
		bannedPatterns: [],
		moduleScopeChecks: [],
		suppressionPrefix: "guardrails-disable",
	};
}
