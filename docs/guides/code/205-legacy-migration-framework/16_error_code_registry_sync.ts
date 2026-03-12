import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { basename, resolve } from "node:path";

import { fail } from "./error_codes.ts";
import {
  collectCodesFromScript,
  DEFAULT_ERROR_CODE_SCRIPTS,
  inferCategory,
  type Registry,
  type RegistryEntry,
} from "./error_registry_shared.ts";
import { frameworkDir, frameworkPath } from "./script_paths.ts";

type ParsedArgs = {
  registry?: string;
  check?: boolean;
  json?: boolean;
};

function parseArgs(argv: string[]): ParsedArgs {
  const parsed: ParsedArgs = {};
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--registry" && i + 1 < argv.length) {
      parsed.registry = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--registry=")) {
      parsed.registry = token.slice("--registry=".length).trim();
      continue;
    }
    if (token === "--check") {
      parsed.check = true;
      continue;
    }
    if (token === "--json") {
      parsed.json = true;
      continue;
    }
    fail("MIG_CLI_001", `unknown argument: ${token}`);
  }
  return parsed;
}

function readJson(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function sortedUnique(values: Iterable<string>): string[] {
  return [...new Set(values)].sort((a, b) => a.localeCompare(b));
}

function buildRegistry(
  scriptPaths: string[],
  previous: Registry | null,
): Registry {
  const scriptsByCode = new Map<string, Set<string>>();
  for (const scriptPath of scriptPaths) {
    const scriptName = basename(scriptPath);
    const codes = collectCodesFromScript(scriptPath);
    for (const code of codes) {
      const existing = scriptsByCode.get(code) ?? new Set<string>();
      existing.add(scriptName);
      scriptsByCode.set(code, existing);
    }
  }

  const previousByCode = new Map<string, RegistryEntry>();
  for (const entry of previous?.codes ?? []) {
    if (entry.code && entry.code.startsWith("MIG_")) {
      previousByCode.set(entry.code, entry);
    }
  }

  const codes = sortedUnique(scriptsByCode.keys()).map((code) => {
    const prev = previousByCode.get(code);
    const scripts = sortedUnique(scriptsByCode.get(code) ?? []);
    return {
      code,
      category: prev?.category ?? inferCategory(code),
      scripts,
      meaning: prev?.meaning ?? "TODO: add concise error meaning.",
      remediation: prev?.remediation ?? "TODO: add concrete remediation command or step.",
    } satisfies RegistryEntry;
  });

  return {
    schema: "underlay.migration.error_registry.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    codes,
  };
}

function comparableRegistry(value: Registry): Pick<Registry, "schema" | "schema_version" | "codes"> {
  return {
    schema: value.schema,
    schema_version: value.schema_version,
    codes: value.codes,
  };
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const registryPath = resolve(
    args.registry || frameworkPath("migration-error-registry.json"),
  );
  const scriptNames = [...DEFAULT_ERROR_CODE_SCRIPTS].sort((a, b) => a.localeCompare(b));
  const scriptPaths = scriptNames.map((name) =>
    resolve(frameworkDir(), name),
  );

  const missingScripts = scriptPaths.filter((path) => !existsSync(path));
  if (missingScripts.length > 0) {
    fail("MIG_CFG_002", `registry sync missing script files: ${missingScripts.join(", ")}`);
  }

  let previous: Registry | null = null;
  if (existsSync(registryPath)) {
    const parsed = readJson(registryPath);
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
      fail("MIG_CFG_003", `registry must be a JSON object: ${registryPath}`);
    }
    previous = parsed as Registry;
  }

  const nextRegistry = buildRegistry(scriptPaths, previous);
  const nextRaw = `${JSON.stringify(nextRegistry, null, 2)}\n`;
  const changed = previous
    ? JSON.stringify(comparableRegistry(previous)) !== JSON.stringify(comparableRegistry(nextRegistry))
    : true;

  if (!args.check) {
    writeFileSync(registryPath, nextRaw, "utf-8");
  } else if (changed) {
    fail("MIG_CFG_005", `registry is out of sync: ${registryPath}`);
  }

  const report = {
    schema: "underlay.migration.error_registry_sync.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    registry_file: registryPath,
    mode: args.check ? "check" : "write",
    checked_scripts: scriptPaths.map((path) => basename(path)).sort((a, b) => a.localeCompare(b)),
    code_count: nextRegistry.codes.length,
    changed,
  };

  if (args.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    console.log(`error registry sync mode=${report.mode} changed=${String(report.changed)} file=${registryPath}`);
    console.log(`checked_scripts=${report.checked_scripts.length}`);
    console.log(`code_count=${report.code_count}`);
  }
}

main();
