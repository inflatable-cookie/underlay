import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

import { loadConfig, readOptional, readString } from "./config.ts";
import { fail, withCode } from "./error_codes.ts";

type RunScope = "demo" | "refresh" | "pre_production";
type CheckStatus = "passed" | "failed" | "skipped";

type CheckRecord = {
  id: string;
  required: boolean;
  status: CheckStatus;
  command?: string;
  exit_code?: number;
  output_excerpt?: string;
  reason?: string;
};

type PromotionDecision = {
  schema: "underlay.migration.promotion_check.v1";
  schema_version: 1;
  generated_at: string;
  project_name: string;
  run_scope: RunScope;
  run_date_utc: string;
  recommendation: "promote" | "hold";
  can_promote: boolean;
  thresholds: {
    drift_max_unresolved: number;
    drift_max_governance: number;
    drift_max_lineage: number;
  };
  checks: CheckRecord[];
  blocking_reasons: string[];
  files: {
    config_file: string;
    run_report: string;
    output_dir: string;
    governance_policy_file: string;
    artifact_manifest_file: string;
    decision_file: string;
  };
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): { output?: string } {
  const parsed: { output?: string } = {};
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--output" && i + 1 < argv.length) {
      parsed.output = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--output=")) {
      parsed.output = token.slice("--output=".length).trim();
      continue;
    }
    fail("MIG_CLI_001", `unknown argument: ${token}`);
  }
  return parsed;
}

function requireTool(name: string): void {
  const result = spawnSync("which", [name], { stdio: "ignore" });
  if (result.status !== 0) {
    fail("MIG_CFG_006", `${name} is required in PATH`);
  }
}

function parseRunScope(value: string): RunScope {
  if (value === "demo" || value === "refresh" || value === "pre_production") {
    return value;
  }
  fail("MIG_CFG_008", `RUN_SCOPE must be one of: demo, refresh, pre_production (got: ${value})`);
}

function runCheck(
  id: string,
  required: boolean,
  command: string,
  args: string[],
  env?: NodeJS.ProcessEnv,
): CheckRecord {
  const cmd = `${command} ${args.join(" ")}`;
  const result = spawnSync(command, args, {
    encoding: "utf-8",
    stdio: ["ignore", "pipe", "pipe"],
    env,
  });
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`.trim();
  return {
    id,
    required,
    status: result.status === 0 ? "passed" : "failed",
    command: cmd,
    exit_code: result.status ?? -1,
    output_excerpt: output.slice(0, 3000),
  };
}

function runSkipped(id: string, required: boolean, reason: string): CheckRecord {
  return {
    id,
    required,
    status: "skipped",
    reason,
  };
}

function parseNumber(value: string, key: string): number {
  if (!/^\d+$/.test(value)) {
    fail("MIG_CFG_011", `${key} must be an unsigned integer string`);
  }
  return Number(value);
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  requireTool("bun");
  requireTool("underlay-devtools");

  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const outputDir = resolve(readString(values, "OUTPUT_DIR", "./runtime/demo-pass"));
  const runReport = resolve(readString(values, "RUN_REPORT", `${outputDir}/run-report.json`));
  const governancePolicyFile = resolve(
    readString(values, "GOVERNANCE_POLICY_FILE", "./runtime/governance-policy.json"),
  );
  const projectName = readString(values, "PROJECT_NAME", "migration");
  const runScope = parseRunScope(readString(values, "RUN_SCOPE", "demo"));
  const runDateUtc = readString(values, "RUN_DATE_UTC", utcDate());

  const driftMaxUnresolved = parseNumber(
    readString(values, "DRIFT_MAX_UNRESOLVED", "0"),
    "DRIFT_MAX_UNRESOLVED",
  );
  const driftMaxGovernance = parseNumber(
    readString(values, "DRIFT_MAX_GOVERNANCE", "0"),
    "DRIFT_MAX_GOVERNANCE",
  );
  const driftMaxLineage = parseNumber(
    readString(values, "DRIFT_MAX_LINEAGE", "0"),
    "DRIFT_MAX_LINEAGE",
  );

  const artifactManifestFile = resolve(
    readOptional(values, "ARTIFACT_MANIFEST_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.artifact-manifest.json`,
  );
  const decisionFile = resolve(
    args.output ||
      readOptional(values, "PROMOTION_DECISION_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.promotion-decision.json`,
  );

  const configEnv: NodeJS.ProcessEnv = {
    ...process.env,
    MIGRATION_CONFIG_FILE: process.env.MIGRATION_CONFIG_FILE ?? filePath,
    RUN_SCOPE: process.env.RUN_SCOPE ?? runScope,
    PROJECT_NAME: process.env.PROJECT_NAME ?? projectName,
    RUN_DATE_UTC: process.env.RUN_DATE_UTC ?? runDateUtc,
  };

  const checks: CheckRecord[] = [];
  checks.push(
    runCheck(
      "evidence_generate",
      true,
      "bun",
      ["run", "./docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts"],
      configEnv,
    ),
  );
  checks.push(
    runCheck(
      "evidence_verify",
      true,
      "bun",
      [
        "run",
        "./docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts",
        "--input",
        artifactManifestFile,
      ],
      configEnv,
    ),
  );
  checks.push(
    runCheck("report_integrity", true, "underlay-devtools", [
      "migration",
      "report",
      "integrity",
      "--input",
      runReport,
    ]),
  );
  checks.push(
    runCheck("report_drift", true, "underlay-devtools", [
      "migration",
      "report",
      "drift",
      "--input",
      runReport,
      "--max-unresolved",
      String(driftMaxUnresolved),
      "--max-governance",
      String(driftMaxGovernance),
      "--max-lineage",
      String(driftMaxLineage),
    ]),
  );
  checks.push(
    runCheck("report_verify", true, "underlay-devtools", [
      "migration",
      "report",
      "verify",
      "--input",
      runReport,
      "--output-dir",
      outputDir,
    ]),
  );

  if (existsSync(governancePolicyFile)) {
    checks.push(
      runCheck("report_policy", true, "underlay-devtools", [
        "migration",
        "report",
        "policy",
        "--input",
        governancePolicyFile,
      ]),
    );
  } else {
    checks.push(
      runSkipped("report_policy", true, `governance policy file missing: ${governancePolicyFile}`),
    );
  }

  const blocking = checks
    .filter((check) => check.required && check.status !== "passed")
    .map((check) => check.reason || `${check.id} failed`);
  const canPromote = blocking.length === 0;

  const decision: PromotionDecision = {
    schema: "underlay.migration.promotion_check.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    project_name: projectName,
    run_scope: runScope,
    run_date_utc: runDateUtc,
    recommendation: canPromote ? "promote" : "hold",
    can_promote: canPromote,
    thresholds: {
      drift_max_unresolved: driftMaxUnresolved,
      drift_max_governance: driftMaxGovernance,
      drift_max_lineage: driftMaxLineage,
    },
    checks,
    blocking_reasons: blocking,
    files: {
      config_file: filePath,
      run_report: runReport,
      output_dir: outputDir,
      governance_policy_file: governancePolicyFile,
      artifact_manifest_file: artifactManifestFile,
      decision_file: decisionFile,
    },
  };

  mkdirSync(dirname(decisionFile), { recursive: true });
  writeFileSync(decisionFile, JSON.stringify(decision, null, 2) + "\n", "utf-8");

  console.log(`promotion decision written: ${decisionFile}`);
  console.log(`promotion recommendation: ${decision.recommendation}`);
  if (!canPromote) {
    for (const reason of blocking) {
      console.error(withCode("MIG_PROMO_001", `promotion blocker: ${reason}`));
    }
    process.exit(1);
  }
}

main();
