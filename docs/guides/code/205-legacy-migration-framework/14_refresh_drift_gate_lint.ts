import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { fail } from "./error_codes.ts";
import { frameworkPath } from "./script_paths.ts";

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
  properties?: Record<string, JsonSchema>;
  items?: JsonSchema;
};

type LintStatus = "passed" | "failed";

type LintResult = {
  schema: "underlay.migration.refresh_drift_gate_lint.v1";
  schema_version: 1;
  generated_at: string;
  gate_file: string;
  schema_file: string;
  status: LintStatus;
  error_count: number;
  errors: string[];
};

type DriftGate = {
  status?: string;
  exit_code?: number;
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): { input?: string; schema?: string; output?: string; json?: boolean } {
  const parsed: { input?: string; schema?: string; output?: string; json?: boolean } = {};
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--input" && i + 1 < argv.length) {
      parsed.input = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--input=")) {
      parsed.input = token.slice("--input=".length).trim();
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
    if (token === "--output" && i + 1 < argv.length) {
      parsed.output = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--output=")) {
      parsed.output = token.slice("--output=".length).trim();
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

function defaultGatePath(config: Record<string, string>): string {
  const configured = readOptional(config, "REFRESH_DRIFT_GATE_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.refresh.${runDate}.refresh-drift-gate.json`);
}

function defaultSchemaPath(config: Record<string, string>): string {
  const configured = readOptional(config, "REFRESH_DRIFT_GATE_SCHEMA_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  return frameworkPath("refresh-drift-gate.schema.json");
}

function defaultLintOutputPath(config: Record<string, string>): string {
  const configured = readOptional(config, "REFRESH_DRIFT_GATE_LINT_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.refresh.${runDate}.refresh-drift-gate-lint.json`);
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

function semanticChecks(gate: DriftGate, errors: string[]): void {
  if (gate.status === "passed" && (gate.exit_code ?? -1) !== 0) {
    errors.push(`$.exit_code must be 0 when status=passed`);
  }
  if (gate.status === "failed" && (gate.exit_code ?? 0) === 0) {
    errors.push(`$.exit_code must be non-zero when status=failed`);
  }
}

function writeLintResult(path: string, result: LintResult): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(result, null, 2) + "\n", "utf-8");
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const gatePath = resolve(args.input || defaultGatePath(values as Record<string, string>));
  const schemaPath = resolve(args.schema || defaultSchemaPath(values as Record<string, string>));
  const lintOutputPath = resolve(args.output || defaultLintOutputPath(values as Record<string, string>));

  const errors: string[] = [];
  let gate: unknown = null;

  if (!existsSync(gatePath)) {
    errors.push(`gate file not found: ${gatePath}`);
  }
  if (!existsSync(schemaPath)) {
    errors.push(`schema file not found: ${schemaPath}`);
  }

  if (errors.length === 0) {
    gate = readJson(gatePath);
    const schemaRoot = readJson(schemaPath) as JsonSchema & { properties?: Record<string, JsonSchema> };
    const rootSchema: JsonSchema = {
      type: schemaRoot.type,
      required: schemaRoot.required,
      additionalProperties: schemaRoot.additionalProperties,
      properties: schemaRoot.properties,
    };
    validateAgainstSchema(gate, rootSchema, "$", errors);
    semanticChecks(gate as DriftGate, errors);
  }

  const status: LintStatus = errors.length === 0 ? "passed" : "failed";
  const lintResult: LintResult = {
    schema: "underlay.migration.refresh_drift_gate_lint.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    gate_file: gatePath,
    schema_file: schemaPath,
    status,
    error_count: errors.length,
    errors,
  };

  writeLintResult(lintOutputPath, lintResult);
  if (args.json) {
    console.log(JSON.stringify(lintResult, null, 2));
  } else {
    console.log(`refresh drift gate lint ${status}: ${gatePath}`);
    console.log(`schema: ${schemaPath}`);
    console.log(`lint report: ${lintOutputPath}`);
  }

  if (status === "failed") {
    const message = [
      `refresh drift gate lint failed (${errors.length} issue${errors.length === 1 ? "" : "s"})`,
      ...errors.map((error) => `- ${error}`),
    ].join("\n");
    fail("MIG_REFRESH_002", message);
  }
}

main();
