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
	bin?: string | Record<string, string>;
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

function checkBinTarget(target: unknown, binKey: string): void {
	if (typeof target !== "string" || target.trim().length === 0) {
		missing.push(`${binKey} -> ${String(target)}`);
		return;
	}

	const fullPath = path.resolve(repoRoot, target);
	if (!fs.existsSync(fullPath) || !fs.statSync(fullPath).isFile()) {
		missing.push(`${binKey} -> ${target}`);
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

if (pkg.bin !== undefined) {
	if (typeof pkg.bin === "string") {
		checkBinTarget(pkg.bin, "bin");
	} else if (pkg.bin && typeof pkg.bin === "object" && !Array.isArray(pkg.bin)) {
		for (const [binName, target] of Object.entries(pkg.bin)) {
			checkBinTarget(target, `bin.${binName}`);
		}
	} else {
		console.error("package.json bin is invalid");
		process.exit(1);
	}
}

if (missing.length > 0) {
	console.error("Invalid package.json exports or bin targets. Missing targets:");
	for (const item of missing) {
		console.error(`- ${item}`);
	}
	process.exit(1);
}

console.log("package.json exports and bin targets look valid.");
