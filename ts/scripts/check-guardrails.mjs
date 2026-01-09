import fs from "node:fs";
import path from "node:path";
import process from "node:process";

// ts/scripts -> repo root
const repoRoot = path.resolve(path.dirname(new URL(import.meta.url).pathname), "..", "..");
const sourceRoot = path.join(repoRoot, "ts", "src");

/** @type {Array<{file: string, message: string, line: number, snippet: string}>} */
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
    } else if (entry.isFile()) {
      if (entry.name.endsWith(".ts") || entry.name.endsWith(".svelte")) {
        out.push(full);
      }
    }
  }
  return out;
}

/**
 * @param {string} content
 * @param {RegExp} re
 * @param {string} file
 * @param {string} message
 */
function addMatches(content, re, file, message) {
  const lines = content.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    if (re.test(lines[i])) {
      violations.push({
        file: path.relative(repoRoot, file),
        message,
        line: i + 1,
        snippet: lines[i].trim()
      });
    }
  }
}

const files = listFiles(sourceRoot);

for (const file of files) {
  const content = fs.readFileSync(file, "utf8");

  // Hard bans: disruptive UI.
  addMatches(
    content,
    /\bwindow\.(alert|confirm|prompt)\b/,
    file,
    "Do not use window.alert/confirm/prompt"
  );

  // Clipboard must go through helper.
  if (!file.endsWith(path.join("patterns", "clipboard.ts"))) {
    addMatches(
      content,
      /\bnavigator\s*\?\.|\bnavigator\.|\bwindow\s*\?\.|\bwindow\.|\bnavigator\.clipboard\b/,
      file,
      "Use globalThis + shared helpers (no bare window/navigator)"
    );

    addMatches(
      content,
      /\bnavigator\s*\?\.\s*clipboard\b|\bnavigator\.clipboard\b/,
      file,
      "Use copyToClipboard/copyTextToClipboard helper (no direct navigator.clipboard)"
    );
  }

  // Avoid bare `document` access (can throw ReferenceError in SSR if executed).
  addMatches(
    content,
    /\bdocument\s*\?\.|\bdocument\./,
    file,
    "Use globalThis?.document instead of bare document"
  );

  // Avoid storage at module scope; require explicit helper in apps.
  addMatches(
    content,
    /\blocalStorage\b|\bsessionStorage\b/,
    file,
    "Avoid direct localStorage/sessionStorage usage in Underlay"
  );

  // Avoid hydration mismatches from runtime randomness in components.
  if (file.endsWith(".svelte")) {
    addMatches(
      content,
      /\bMath\.random\(/,
      file,
      "Avoid Math.random() in Svelte components (hydration mismatch risk)"
    );

    addMatches(
      content,
      /\bcrypto\.randomUUID\(/,
      file,
      "Avoid crypto.randomUUID() in Svelte components (hydration mismatch risk)"
    );
  }
}

if (violations.length > 0) {
  console.error("Underlay guardrails failed:");
  for (const v of violations) {
    console.error(`- ${v.file}:${v.line} ${v.message}: ${v.snippet}`);
  }
  process.exit(1);
}

console.log("Underlay guardrails passed.");
