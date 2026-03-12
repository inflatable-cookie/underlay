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
  schema: "underlay.migration.decision_reuse_summary_lint.v1";
  schema_version: 1;
  generated_at: string;
  summary_file: string;
  schema_file: string;
  status: LintStatus;
  error_count: number;
  errors: string[];
};

type ReuseSummary = {
  run_scope?: string;
  reuse_stats?: {
    reused_count?: number;
    new_count?: number;
    invalidated_count?: number;
    unresolved_count?: number;
  };
  suppression_kpi?: {
    candidate_decisions_total?: number;
    reused_decisions_total?: number;
    new_ai_calls_total?: number;
    new_human_required_total?: number;
    ai_call_suppression_ratio?: number;
    reuse_ratio?: number;
    human_queue_ratio?: number;
  };
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

function defaultSummaryPath(config: Record<string, string>): string {
  const configured = readOptional(config, "DECISION_REUSE_SUMMARY_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "refresh");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.decision-reuse-summary.json`);
}

function defaultSchemaPath(config: Record<string, string>): string {
  const configured = readOptional(config, "DECISION_REUSE_SUMMARY_SCHEMA_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  return frameworkPath("decision-reuse-summary.schema.json");
}

function defaultLintOutputPath(config: Record<string, string>): string {
  const configured = readOptional(config, "DECISION_REUSE_SUMMARY_LINT_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "refresh");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.decision-reuse-summary-lint.json`);
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

function asCount(value: unknown): number {
  if (typeof value === "number" && Number.isFinite(value) && value >= 0) {
    return Math.floor(value);
  }
  return 0;
}

function nearlyEqual(a: number, b: number): boolean {
  return Math.abs(a - b) <= 0.0001;
}

function semanticChecks(summary: ReuseSummary, errors: string[]): void {
  if (summary.run_scope !== "refresh") {
    errors.push(`$.run_scope must be refresh`);
  }

  const reuse = summary.reuse_stats;
  const kpi = summary.suppression_kpi;
  if (!reuse || !kpi) {
    return;
  }

  const candidate = asCount(kpi.candidate_decisions_total);
  const reused = asCount(reuse.reused_count);
  const expectedNew = Math.max(0, candidate - reused);
  const providedNew = asCount(reuse.new_count);
  if (providedNew !== expectedNew) {
    errors.push(`$.reuse_stats.new_count expected ${expectedNew}, got ${providedNew}`);
  }

  const expectedReuseRatio = candidate > 0 ? reused / candidate : 0;
  if (!nearlyEqual((kpi.reuse_ratio as number) ?? 0, expectedReuseRatio)) {
    errors.push(`$.suppression_kpi.reuse_ratio does not match counts`);
  }

  const humanRequired = asCount(kpi.new_human_required_total);
  const expectedHumanQueueRatio = candidate > 0 ? humanRequired / candidate : 0;
  if (!nearlyEqual((kpi.human_queue_ratio as number) ?? 0, expectedHumanQueueRatio)) {
    errors.push(`$.suppression_kpi.human_queue_ratio does not match counts`);
  }

  const aiCalls = asCount(kpi.new_ai_calls_total);
  const expectedSuppression = candidate > 0 ? (candidate - aiCalls) / candidate : 0;
  if (!nearlyEqual((kpi.ai_call_suppression_ratio as number) ?? 0, expectedSuppression)) {
    errors.push(`$.suppression_kpi.ai_call_suppression_ratio does not match counts`);
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

  const summaryPath = resolve(args.input || defaultSummaryPath(values as Record<string, string>));
  const schemaPath = resolve(args.schema || defaultSchemaPath(values as Record<string, string>));
  const lintOutputPath = resolve(args.output || defaultLintOutputPath(values as Record<string, string>));

  const errors: string[] = [];
  let summary: unknown = null;

  if (!existsSync(summaryPath)) {
    errors.push(`summary file not found: ${summaryPath}`);
  }
  if (!existsSync(schemaPath)) {
    errors.push(`schema file not found: ${schemaPath}`);
  }

  if (errors.length === 0) {
    summary = readJson(summaryPath);
    const schemaRoot = readJson(schemaPath) as JsonSchema & { properties?: Record<string, JsonSchema> };
    const rootSchema: JsonSchema = {
      type: schemaRoot.type,
      required: schemaRoot.required,
      additionalProperties: schemaRoot.additionalProperties,
      properties: schemaRoot.properties,
    };
    validateAgainstSchema(summary, rootSchema, "$", errors);
    semanticChecks(summary as ReuseSummary, errors);
  }

  const status: LintStatus = errors.length === 0 ? "passed" : "failed";
  const lintResult: LintResult = {
    schema: "underlay.migration.decision_reuse_summary_lint.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    summary_file: summaryPath,
    schema_file: schemaPath,
    status,
    error_count: errors.length,
    errors,
  };

  writeLintResult(lintOutputPath, lintResult);
  if (args.json) {
    console.log(JSON.stringify(lintResult, null, 2));
  } else {
    console.log(`decision reuse summary lint ${status}: ${summaryPath}`);
    console.log(`schema: ${schemaPath}`);
    console.log(`lint report: ${lintOutputPath}`);
  }

  if (status === "failed") {
    const message = [
      `decision reuse summary lint failed (${errors.length} issue${errors.length === 1 ? "" : "s"})`,
      ...errors.map((error) => `- ${error}`),
    ].join("\n");
    fail("MIG_REFRESH_001", message);
  }
}

main();
