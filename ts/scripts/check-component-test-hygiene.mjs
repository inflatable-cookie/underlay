import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");
const testsRoot = path.join(repoRoot, "ts", "tests");
const vitestComponentConfigPath = path.join(repoRoot, "vitest.component.config.ts");
const sharedSetupPath = path.join(repoRoot, "ts", "tests", "setup", "vitest-component.setup.ts");

/** @type {string[]} */
const violations = [];

/**
 * @param {string} dir
 * @returns {string[]}
 */
function listFiles(dir) {
	/** @type {string[]} */
	const out = [];
	for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
		const full = path.join(dir, entry.name);
		if (entry.isDirectory()) {
			out.push(...listFiles(full));
			continue;
		}
		if (entry.isFile()) {
			out.push(full);
		}
	}
	return out;
}

/**
 * @param {string} content
 * @param {RegExp} regex
 * @returns {boolean}
 */
function has(content, regex) {
	return regex.test(content);
}

/**
 * @param {string} content
 * @returns {boolean}
 */
function hasWindowPromptApis(content) {
	return has(content, /\bwindow\.(alert|confirm|prompt)\b/);
}

/**
 * @param {string} file
 * @param {string} content
 */
function checkGenericTestFile(file, content) {
	const rel = path.relative(repoRoot, file);

	if (hasWindowPromptApis(content)) {
		violations.push(`${rel}: window.alert/confirm/prompt is disallowed in tests`);
	}

	const usesFakeTimers = has(content, /\bvi\.useFakeTimers\(/);
	const usesRealTimers = has(content, /\bvi\.useRealTimers\(/);
	if (usesFakeTimers && !usesRealTimers) {
		violations.push(`${rel}: uses vi.useFakeTimers() without vi.useRealTimers()`);
	}
}

function checkSharedSetupWiring() {
	if (!fs.existsSync(vitestComponentConfigPath)) {
		violations.push("Missing vitest.component.config.ts");
		return;
	}
	const vitestConfig = fs.readFileSync(vitestComponentConfigPath, "utf8");
	if (!vitestConfig.includes('setupFiles: ["./ts/tests/setup/vitest-component.setup.ts"]')) {
		violations.push(
			"vitest.component.config.ts must declare setupFiles with ./ts/tests/setup/vitest-component.setup.ts"
		);
	}

	if (!fs.existsSync(sharedSetupPath)) {
		violations.push("Missing shared component setup file: ts/tests/setup/vitest-component.setup.ts");
		return;
	}
	const setupContent = fs.readFileSync(sharedSetupPath, "utf8");
	if (!has(setupContent, /\bcleanup\s*\(\s*\)/)) {
		violations.push(
			"Shared component setup must call cleanup() in afterEach (ts/tests/setup/vitest-component.setup.ts)"
		);
	}
	if (!has(setupContent, /\bsetTimeout\s*\(/)) {
		violations.push(
			"Shared component setup must include deferred timer flush in afterEach (setTimeout)"
		);
	}
}

/**
 * @param {string} file
 * @param {string} content
 */
function checkComponentTestFile(file, content) {
	const rel = path.relative(repoRoot, file);

	// Enforce shared setup ownership of cleanup lifecycle.
	if (has(content, /\bcleanup\s*\(/)) {
		violations.push(`${rel}: inline cleanup() is disallowed; rely on shared setup`);
	}

	// Avoid dragging cleanup imports back into test files.
	if (has(content, /from\s+"@testing-library\/svelte"/) && has(content, /\{[^}]*\bcleanup\b[^}]*\}/s)) {
		violations.push(`${rel}: cleanup import is disallowed; rely on shared setup`);
	}
}

checkSharedSetupWiring();

const allTestFiles = listFiles(testsRoot).filter((file) => file.endsWith(".test.ts"));
for (const file of allTestFiles) {
	const content = fs.readFileSync(file, "utf8");
	checkGenericTestFile(file, content);
	if (file.endsWith(".component.test.ts")) {
		checkComponentTestFile(file, content);
	}
}

if (violations.length > 0) {
	console.error("Component test hygiene check failed:");
	for (const item of violations) {
		console.error(`- ${item}`);
	}
	process.exit(1);
}

console.log("Component test hygiene check passed.");
