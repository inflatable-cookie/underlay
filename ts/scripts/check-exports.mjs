import fs from "node:fs";
import path from "node:path";
import process from "node:process";

// ts/scripts -> repo root
const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");

const pkgPath = path.join(repoRoot, "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));

const exportsField = pkg.exports;
if (!exportsField || typeof exportsField !== "object") {
  console.error("package.json exports is missing or invalid");
  process.exit(1);
}

/** @type {string[]} */
const missing = [];

function checkTarget(target, exportKey, condition) {
  if (typeof target !== "string") return;

  // Ignore package self references (none expected here, but safe)
  if (!target.startsWith("./")) return;

  const fullPath = path.join(repoRoot, target);
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
