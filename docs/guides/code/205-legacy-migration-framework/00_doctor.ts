import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

import { loadConfig, readOptional } from "./config.ts";
import { fail, withCode } from "./error_codes.ts";
import { frameworkPath, frameworkScriptPath } from "./script_paths.ts";

type CheckStatus = "passed" | "failed" | "skipped";

type CheckResult = {
  name: string;
  status: CheckStatus;
  command?: string;
  exit_code?: number;
  output_excerpt?: string;
  reason?: string;
};

type SettingView = {
  value: string;
  source: "config" | "env" | "default" | "unset";
};

type DoctorReport = {
  schema: "underlay.migration.doctor.v1";
  schema_version: 1;
  generated_at: string;
  config_file: string;
  output_file: string;
  checks: CheckResult[];
  effective_settings: Record<string, SettingView>;
  summary: {
    passed_count: number;
    failed_count: number;
    skipped_count: number;
    status: "passed" | "failed";
  };
};

const DEFAULTS: Record<string, string> = {
  SOURCE_SYSTEM: "legacy_site",
  TARGET_SCHEMA_VERSION: "schema-v1",
  MEDIA_DIR: "./legacy-export/media",
  BUNDLE_FILE: "./dist/migration-bundle.oci",
  OUTPUT_DIR: "./runtime/demo-pass",
  RUN_REPORT: "./runtime/demo-pass/run-report.json",
  GOVERNANCE_POLICY_FILE: "./runtime/governance-policy.json",
  PROJECT_NAME: "migration",
  RUN_SCOPE: "demo",
  DRIFT_MAX_UNRESOLVED: "0",
  DRIFT_MAX_GOVERNANCE: "0",
  DRIFT_MAX_LINEAGE: "0",
};

const KNOWN_KEYS = [
  "SOURCE_SYSTEM",
  "TARGET_SCHEMA_VERSION",
  "MEDIA_DIR",
  "BUNDLE_FILE",
  "OCI_REF_TAG",
  "BUNDLE_REF",
  "BUNDLE_REF_FILE",
  "OUTPUT_DIR",
  "RUN_REPORT",
  "GOVERNANCE_POLICY_FILE",
  "REUSE_FROM_DIGEST_REF",
  "REUSE_FROM_DIGEST_REF_FILE",
  "UNDERLAY_DEVTOOLS_CMD",
  "APP_MIGRATION_RUNNER_CMD",
  "DECISION_INDEX_FILE",
  "DECISION_JOURNAL_FILE",
  "PROJECT_NAME",
  "RUN_SCOPE",
  "RUN_DATE_UTC",
  "DOCTOR_REPORT",
  "VERIFY_ARTIFACT_FILE",
  "AUDIT_ARTIFACT_FILE",
  "ARTIFACT_MANIFEST_FILE",
  "PROMOTION_DECISION_FILE",
  "PROMOTION_DECISION_SCHEMA_FILE",
  "PROMOTION_DECISION_LINT_FILE",
  "PROMOTION_RELEASE_NOTE_JSON_FILE",
  "PROMOTION_RELEASE_NOTE_MD_FILE",
  "PROMOTION_CI_GUARD_FILE",
  "DRIFT_MAX_UNRESOLVED",
  "DRIFT_MAX_GOVERNANCE",
  "DRIFT_MAX_LINEAGE",
  "MIGRATION_CONFIG_FILE",
  "MIGRATION_CONFIG_SCHEMA_FILE",
] as const;

function parseOutputArg(argv: string[]): string | null {
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--output" && i + 1 < argv.length) {
      const value = argv[i + 1].trim();
      if (value.length === 0) {
        fail("MIG_CLI_002", "--output requires a non-empty path");
      }
      return value;
    }
    if (token.startsWith("--output=")) {
      const value = token.slice("--output=".length).trim();
      if (value.length === 0) {
        fail("MIG_CLI_002", "--output requires a non-empty path");
      }
      return value;
    }
  }
  return null;
}

function runCommandCapture(command: string, args: string[], env: NodeJS.ProcessEnv): CheckResult {
  const cmd = `${command} ${args.join(" ")}`;
  const result = spawnSync(command, args, {
    encoding: "utf-8",
    stdio: ["ignore", "pipe", "pipe"],
    env,
  });

  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`.trim();
  return {
    name: cmd,
    status: result.status === 0 ? "passed" : "failed",
    command: cmd,
    exit_code: result.status ?? -1,
    output_excerpt: output.slice(0, 1600),
  };
}

function redact(key: string, value: string): string {
  const upper = key.toUpperCase();
  if (
    upper.includes("SECRET") ||
    upper.includes("TOKEN") ||
    upper.includes("PASSWORD") ||
    upper.includes("KEY")
  ) {
    return value.length === 0 ? value : "[REDACTED]";
  }
  if (upper.includes("DIGEST") || upper.endsWith("_REF")) {
    if (value.length <= 24) return value;
    return `${value.slice(0, 18)}...${value.slice(-8)}`;
  }
  return value;
}

function detectSource(configValues: Record<string, string>, key: string): SettingView["source"] {
  const fileValue = configValues[key];
  if (fileValue && fileValue.trim().length > 0) return "config";
  const envValue = process.env[key];
  if (envValue && envValue.trim().length > 0) return "env";
  if (DEFAULTS[key] !== undefined) return "default";
  return "unset";
}

function effectiveValue(configValues: Record<string, string>, key: string): string {
  const fileValue = configValues[key];
  if (fileValue && fileValue.trim().length > 0) return fileValue;
  const envValue = process.env[key];
  if (envValue && envValue.trim().length > 0) return envValue;
  return DEFAULTS[key] ?? "";
}

function defaultOutputFile(configValues: Record<string, string>): string {
  const outputDir = effectiveValue(configValues, "OUTPUT_DIR") || "./runtime/demo-pass";
  const runReport =
    configValues.RUN_REPORT?.trim() ||
    process.env.RUN_REPORT?.trim() ||
    `${outputDir}/run-report.json`;
  if (runReport.endsWith("run-report.json")) {
    return runReport.replace(/run-report\.json$/, "migration-doctor.json");
  }
  return "./runtime/migration-doctor.json";
}

function hasReadyRefInput(
  configValues: Record<string, string>,
  directKey: string,
  fileKey: string,
): boolean {
  const directValue = effectiveValue(configValues, directKey).trim();
  if (directValue.length > 0) {
    return true;
  }

  const fileValue = effectiveValue(configValues, fileKey).trim();
  if (fileValue.length === 0) {
    return false;
  }

  return existsSync(resolve(fileValue));
}

function main(): void {
  const { filePath, values } = loadConfig();
  const outputArg = parseOutputArg(process.argv.slice(2));
  const outputFile = resolve(outputArg ?? defaultOutputFile(values as Record<string, string>));

  const checks: CheckResult[] = [];

  const baseEnv = {
    ...process.env,
    MIGRATION_CONFIG_FILE: process.env.MIGRATION_CONFIG_FILE ?? filePath,
    MIGRATION_CONFIG_SCHEMA_FILE:
      process.env.MIGRATION_CONFIG_SCHEMA_FILE ??
      frameworkPath("config.schema.json"),
  };

  checks.push(
    runCommandCapture(
      "bun",
      ["run", frameworkScriptPath("00_config_lint.ts")],
      baseEnv,
    ),
  );
  checks.push(
    runCommandCapture(
      "bun",
      [
        "run",
        frameworkScriptPath("00_preflight.ts"),
        "--mode",
        "general",
      ],
      baseEnv,
    ),
  );

  if (hasReadyRefInput(values as Record<string, string>, "BUNDLE_REF", "BUNDLE_REF_FILE")) {
    checks.push(
      runCommandCapture(
        "bun",
        [
          "run",
          frameworkScriptPath("00_preflight.ts"),
          "--mode",
          "reports",
        ],
        baseEnv,
      ),
    );
  } else {
    checks.push({
      name: "preflight reports",
      status: "skipped",
      reason: "BUNDLE_REF not configured and BUNDLE_REF_FILE not generated yet",
    });
  }

  if (
    hasReadyRefInput(
      values as Record<string, string>,
      "REUSE_FROM_DIGEST_REF",
      "REUSE_FROM_DIGEST_REF_FILE",
    )
  ) {
    checks.push(
      runCommandCapture(
        "bun",
        [
          "run",
          frameworkScriptPath("00_preflight.ts"),
          "--mode",
          "refresh",
        ],
        baseEnv,
      ),
    );
  } else {
    checks.push({
      name: "preflight refresh",
      status: "skipped",
      reason: "REUSE_FROM_DIGEST_REF not configured and REUSE_FROM_DIGEST_REF_FILE not generated yet",
    });
  }

  const effective_settings: Record<string, SettingView> = {};
  for (const key of KNOWN_KEYS) {
    const raw = effectiveValue(values as Record<string, string>, key);
    effective_settings[key] = {
      value: redact(key, raw),
      source: detectSource(values as Record<string, string>, key),
    };
  }

  const passedCount = checks.filter((check) => check.status === "passed").length;
  const failedCount = checks.filter((check) => check.status === "failed").length;
  const skippedCount = checks.filter((check) => check.status === "skipped").length;

  const report: DoctorReport = {
    schema: "underlay.migration.doctor.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    config_file: filePath,
    output_file: outputFile,
    checks,
    effective_settings,
    summary: {
      passed_count: passedCount,
      failed_count: failedCount,
      skipped_count: skippedCount,
      status: failedCount === 0 ? "passed" : "failed",
    },
  };

  mkdirSync(dirname(outputFile), { recursive: true });
  writeFileSync(outputFile, JSON.stringify(report, null, 2) + "\n", "utf-8");

  console.log(`doctor report written: ${outputFile}`);
  console.log(
    `doctor summary: passed=${passedCount} failed=${failedCount} skipped=${skippedCount} status=${report.summary.status}`,
  );

  if (failedCount > 0) {
    console.error(withCode("MIG_CFG_012", `doctor checks failed: failed_count=${failedCount}`));
    process.exit(1);
  }
}

main();
