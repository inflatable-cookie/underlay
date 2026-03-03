import { existsSync, readFileSync } from "node:fs";
import { basename, resolve } from "node:path";

import { fail } from "./error_codes.ts";
import {
  collectCodesFromScript,
  DEFAULT_ERROR_CODE_SCRIPTS,
  type Registry,
  type RegistryEntry,
} from "./error_registry_shared.ts";

type JsonSchema = {
  type?: string;
  const?: unknown;
  enum?: unknown[];
  minLength?: number;
  pattern?: string;
  required?: string[];
  additionalProperties?: boolean;
  minimum?: number;
  maximum?: number;
  minItems?: number;
  uniqueItems?: boolean;
  properties?: Record<string, JsonSchema>;
  items?: JsonSchema;
};

function parseArgs(argv: string[]): { registry?: string; schema?: string; json?: boolean } {
  const parsed: { registry?: string; schema?: string; json?: boolean } = {};
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
    if (token === "--schema" && i + 1 < argv.length) {
      parsed.schema = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--schema=")) {
      parsed.schema = token.slice("--schema=".length).trim();
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

function typeOfValue(value: unknown): string {
  if (Array.isArray(value)) return "array";
  if (value === null) return "null";
  return typeof value;
}

function validateAgainstSchema(
  value: unknown,
  schema: JsonSchema,
  path: string,
  errors: string[],
): void {
  if (schema.type) {
    const actual = typeOfValue(value);
    if (actual !== schema.type) {
      errors.push(`${path}: expected type ${schema.type}, got ${actual}`);
      return;
    }
  }

  if (schema.const !== undefined && value !== schema.const) {
    errors.push(`${path}: expected const ${JSON.stringify(schema.const)}, got ${JSON.stringify(value)}`);
  }

  if (Array.isArray(schema.enum) && !schema.enum.some((item) => item === value)) {
    errors.push(`${path}: expected one of ${JSON.stringify(schema.enum)}, got ${JSON.stringify(value)}`);
  }

  if (typeof value === "number") {
    if (typeof schema.minimum === "number" && value < schema.minimum) {
      errors.push(`${path}: number must be >= ${schema.minimum}`);
    }
    if (typeof schema.maximum === "number" && value > schema.maximum) {
      errors.push(`${path}: number must be <= ${schema.maximum}`);
    }
  }

  if (typeof value === "string") {
    if (typeof schema.minLength === "number" && value.length < schema.minLength) {
      errors.push(`${path}: string length must be >= ${schema.minLength}`);
    }
    if (schema.pattern) {
      const re = new RegExp(schema.pattern);
      if (!re.test(value)) {
        errors.push(`${path}: string does not match pattern ${schema.pattern}`);
      }
    }
  }

  if (Array.isArray(value)) {
    if (typeof schema.minItems === "number" && value.length < schema.minItems) {
      errors.push(`${path}: array length must be >= ${schema.minItems}`);
    }
    if (schema.uniqueItems === true) {
      const seen = new Set<string>();
      for (let i = 0; i < value.length; i += 1) {
        const key = JSON.stringify(value[i]);
        if (seen.has(key)) {
          errors.push(`${path}[${i}]: duplicate array item not allowed`);
          break;
        }
        seen.add(key);
      }
    }
    if (schema.items) {
      for (let i = 0; i < value.length; i += 1) {
        validateAgainstSchema(value[i], schema.items, `${path}[${i}]`, errors);
      }
    }
    return;
  }

  if (value && typeof value === "object" && !Array.isArray(value)) {
    const obj = value as Record<string, unknown>;
    const properties = schema.properties ?? {};
    const required = schema.required ?? [];

    for (const key of required) {
      if (!(key in obj)) {
        errors.push(`${path}.${key}: missing required field`);
      }
    }

    if (schema.additionalProperties === false) {
      for (const key of Object.keys(obj)) {
        if (!(key in properties)) {
          errors.push(`${path}.${key}: additional property not allowed`);
        }
      }
    }

    for (const [key, childSchema] of Object.entries(properties)) {
      if (key in obj) {
        validateAgainstSchema(obj[key], childSchema, `${path}.${key}`, errors);
      }
    }
  }
}

function validateRegistrySemantics(registry: Registry, errors: string[]): void {
  const codes = registry.codes.map((entry) => entry.code);
  const sortedCodes = [...codes].sort((a, b) => a.localeCompare(b));
  if (codes.join("|") !== sortedCodes.join("|")) {
    errors.push("$.codes: entries must be sorted by code ascending");
  }

  for (let i = 0; i < registry.codes.length; i += 1) {
    const entry = registry.codes[i];
    const scripts = entry.scripts ?? [];
    const sortedScripts = [...scripts].sort((a, b) => a.localeCompare(b));
    if (scripts.join("|") !== sortedScripts.join("|")) {
      errors.push(`$.codes[${i}].scripts: scripts must be sorted ascending`);
    }
    if (entry.meaning.trim().length === 0) {
      errors.push(`$.codes[${i}].meaning: must be non-empty`);
    }
    if (entry.remediation.trim().length === 0) {
      errors.push(`$.codes[${i}].remediation: must be non-empty`);
    }
    if (/\bTODO\b/i.test(entry.meaning)) {
      errors.push(`$.codes[${i}].meaning: TODO placeholders are not allowed`);
    }
    if (/\bTODO\b/i.test(entry.remediation)) {
      errors.push(`$.codes[${i}].remediation: TODO placeholders are not allowed`);
    }
  }
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const registryPath = resolve(
    args.registry || "./docs/guides/code/205-legacy-migration-framework/migration-error-registry.json",
  );
  const schemaPath = resolve(
    args.schema || "./docs/guides/code/205-legacy-migration-framework/migration-error-registry.schema.json",
  );
  if (!existsSync(registryPath)) {
    fail("MIG_CFG_001", `registry file not found: ${registryPath}`);
  }
  if (!existsSync(schemaPath)) {
    fail("MIG_CFG_002", `registry schema file not found: ${schemaPath}`);
  }

  const raw = readJson(registryPath);
  if (!raw || typeof raw !== "object" || Array.isArray(raw)) {
    fail("MIG_CFG_003", `registry must be a JSON object: ${registryPath}`);
  }

  const schemaRaw = readJson(schemaPath) as JsonSchema & { properties?: Record<string, JsonSchema> };
  const rootSchema: JsonSchema = {
    type: schemaRaw.type,
    required: schemaRaw.required,
    additionalProperties: schemaRaw.additionalProperties,
    properties: schemaRaw.properties,
  };

  const registry = raw as Registry;
  const registryEntries = Array.isArray(registry.codes) ? registry.codes : [];
  const errors: string[] = [];
  validateAgainstSchema(registry, rootSchema, "$", errors);
  if (errors.length === 0) {
    validateRegistrySemantics({ ...registry, codes: registryEntries }, errors);
  }

  const scriptNames = [...DEFAULT_ERROR_CODE_SCRIPTS].sort((a, b) => a.localeCompare(b));
  const scriptPaths = scriptNames.map((name) =>
    resolve("./docs/guides/code/205-legacy-migration-framework", name),
  );
  const missingScripts = scriptPaths.filter((path) => !existsSync(path));
  if (missingScripts.length > 0) {
    fail("MIG_CFG_002", `registry lint missing script files: ${missingScripts.join(", ")}`);
  }

  const registryCodes = new Set<string>();
  const registryByCode = new Map<string, RegistryEntry>();
  for (const entry of registryEntries) {
    if (!entry.code || typeof entry.code !== "string" || !entry.code.startsWith("MIG_")) {
      errors.push(`invalid code entry: ${JSON.stringify(entry)}`);
      continue;
    }
    if (registryCodes.has(entry.code)) {
      errors.push(`duplicate code in registry: ${entry.code}`);
      continue;
    }
    registryCodes.add(entry.code);
    registryByCode.set(entry.code, entry);
  }

  const actualByScript = new Map<string, Set<string>>();
  const actualCodes = new Set<string>();
  for (const path of scriptPaths) {
    const scriptName = basename(path);
    const codes = collectCodesFromScript(path);
    actualByScript.set(scriptName, codes);
    for (const code of codes) {
      actualCodes.add(code);
    }
  }

  for (const code of registryCodes) {
    if (!actualCodes.has(code)) {
      errors.push(`registry code not found in scripts: ${code}`);
    }
  }

  for (const code of actualCodes) {
    if (!registryCodes.has(code)) {
      errors.push(`script code missing from registry: ${code}`);
    }
  }

  for (const [code, entry] of registryByCode.entries()) {
    for (const scriptName of entry.scripts) {
      const scriptCodes = actualByScript.get(scriptName);
      if (!scriptCodes) {
        errors.push(`registry code ${code} references unknown script: ${scriptName}`);
        continue;
      }
      if (!scriptCodes.has(code)) {
        errors.push(`registry code ${code} not found in listed script: ${scriptName}`);
      }
    }
  }

  const status = errors.length === 0 ? "passed" : "failed";
  const report = {
    schema: "underlay.migration.error_registry_lint.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    registry_file: registryPath,
    schema_file: schemaPath,
    checked_scripts: scriptPaths.map((path) => basename(path)).sort((a, b) => a.localeCompare(b)),
    status,
    error_count: errors.length,
    errors,
  };

  if (args.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    console.log(`error registry lint ${status}: ${registryPath}`);
    console.log(`checked_scripts=${report.checked_scripts.length}`);
    console.log(`error_count=${report.error_count}`);
  }

  if (status === "failed") {
    const message = [
      `error registry lint failed (${errors.length} issue${errors.length === 1 ? "" : "s"})`,
      ...errors.map((item) => `- ${item}`),
    ].join("\n");
    fail("MIG_CFG_005", message);
  }
}

main();
