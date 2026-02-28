import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
// ts/scripts -> repo root
const repoRoot = path.resolve(scriptDir, "..", "..");

const pkgPath = path.join(repoRoot, "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8")) as {
	exports?: Record<string, string | Record<string, string>>;
};

const exportsField = pkg.exports;
if (!exportsField || typeof exportsField !== "object") {
	console.error("package.json exports is missing or invalid");
	process.exit(1);
}

const missing: string[] = [];

function checkTarget(target: unknown, exportKey: string, condition?: string): void {
	if (typeof target !== "string") return;

	// Ignore package self references (none expected here, but safe)
	if (!target.startsWith("./")) return;

	// For wildcard export targets, validate the base directory exists.
	const normalizedTarget = target.includes("*")
		? target.slice(0, target.indexOf("*")).replace(/\/$/, "")
		: target;
	const fullPath = path.join(repoRoot, normalizedTarget);
	if (!fs.existsSync(fullPath)) {
		const label = condition ? `${exportKey} (${condition})` : exportKey;
		missing.push(`${label} -> ${target}`);
	}
}

for (const [exportKey, exportValue] of Object.entries(exportsField)) {
	if (typeof exportValue === "string") {
		checkTarget(exportValue, exportKey);
		continue;
	}

	if (!exportValue || typeof exportValue !== "object") {
		continue;
	}

	for (const [condition, target] of Object.entries(exportValue)) {
		checkTarget(target, exportKey, condition);
	}
}

if (missing.length > 0) {
	console.error("Invalid package.json exports. Missing targets:");
	for (const item of missing) {
		console.error(`- ${item}`);
	}
	process.exit(1);
}

console.log("package.json exports look valid.");
