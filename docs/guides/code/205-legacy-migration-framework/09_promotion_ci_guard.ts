import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { fail, withCode } from "./error_codes.ts";

type Decision = {
  schema?: string;
  schema_version?: number;
  project_name?: string;
  run_scope?: string;
  run_date_utc?: string;
  recommendation?: string;
  can_promote?: boolean;
};

type Lint = {
  schema?: string;
  schema_version?: number;
  status?: string;
  error_count?: number;
  errors?: unknown[];
};

type ReleaseNote = {
  schema?: string;
  schema_version?: number;
  project_name?: string;
  run_scope?: string;
  run_date_utc?: string;
  recommendation?: string;
  can_promote?: boolean;
  decision_lint_status?: string;
  decision_lint_error_count?: number;
};

type GuardReport = {
  schema: "underlay.migration.promotion_ci_guard.v1";
  schema_version: 1;
  generated_at: string;
  status: "passed" | "failed";
  reason_count: number;
  reasons: string[];
  files: {
    decision_file: string;
    lint_file: string;
    release_note_file: string;
    guard_file: string;
  };
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): {
  decision?: string;
  lint?: string;
  releaseNote?: string;
  output?: string;
  json?: boolean;
} {
  const parsed: {
    decision?: string;
    lint?: string;
    releaseNote?: string;
    output?: string;
    json?: boolean;
  } = {};
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--decision" && i + 1 < argv.length) {
      parsed.decision = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--decision=")) {
      parsed.decision = token.slice("--decision=".length).trim();
      continue;
    }
    if (token === "--lint" && i + 1 < argv.length) {
      parsed.lint = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--lint=")) {
      parsed.lint = token.slice("--lint=".length).trim();
      continue;
    }
    if (token === "--release-note" && i + 1 < argv.length) {
      parsed.releaseNote = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--release-note=")) {
      parsed.releaseNote = token.slice("--release-note=".length).trim();
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

function requireFile(path: string, label: string, reasons: string[]): void {
  if (!existsSync(path)) {
    reasons.push(`missing ${label}: ${path}`);
  }
}

function defaultPath(
  config: Record<string, string>,
  key:
    | "PROMOTION_DECISION_FILE"
    | "PROMOTION_DECISION_LINT_FILE"
    | "PROMOTION_RELEASE_NOTE_JSON_FILE"
    | "PROMOTION_CI_GUARD_FILE",
  suffix: string,
): string {
  const configured = readOptional(config, key);
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.${suffix}`);
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const configValues = values as Record<string, string>;
  const decisionFile = resolve(args.decision || defaultPath(configValues, "PROMOTION_DECISION_FILE", "promotion-decision.json"));
  const lintFile = resolve(args.lint || defaultPath(configValues, "PROMOTION_DECISION_LINT_FILE", "promotion-decision-lint.json"));
  const releaseNoteFile = resolve(
    args.releaseNote || defaultPath(configValues, "PROMOTION_RELEASE_NOTE_JSON_FILE", "promotion-release-note.json"),
  );
  const guardFile = resolve(
    args.output || defaultPath(configValues, "PROMOTION_CI_GUARD_FILE", "promotion-ci-guard.json"),
  );

  const reasons: string[] = [];
  requireFile(decisionFile, "promotion decision file", reasons);
  requireFile(lintFile, "promotion decision lint file", reasons);
  requireFile(releaseNoteFile, "promotion release note file", reasons);

  let decision: Decision = {};
  let lint: Lint = {};
  let release: ReleaseNote = {};

  if (reasons.length === 0) {
    decision = readJson(decisionFile) as Decision;
    lint = readJson(lintFile) as Lint;
    release = readJson(releaseNoteFile) as ReleaseNote;

    if (decision.schema !== "underlay.migration.promotion_check.v1") {
      reasons.push(`invalid decision schema: ${String(decision.schema)}`);
    }
    if (decision.schema_version !== 1) {
      reasons.push(`invalid decision schema_version: ${String(decision.schema_version)}`);
    }
    if (decision.recommendation !== "promote") {
      reasons.push(`decision recommendation is not promote: ${String(decision.recommendation)}`);
    }
    if (decision.can_promote !== true) {
      reasons.push(`decision can_promote must be true`);
    }

    if (lint.schema !== "underlay.migration.promotion_decision_lint.v1") {
      reasons.push(`invalid lint schema: ${String(lint.schema)}`);
    }
    if (lint.schema_version !== 1) {
      reasons.push(`invalid lint schema_version: ${String(lint.schema_version)}`);
    }
    if (lint.status !== "passed") {
      reasons.push(`lint status is not passed: ${String(lint.status)}`);
    }
    if ((lint.error_count ?? 1) !== 0) {
      reasons.push(`lint error_count must be 0: ${String(lint.error_count)}`);
    }

    if (release.schema !== "underlay.migration.promotion_release_note.v1") {
      reasons.push(`invalid release note schema: ${String(release.schema)}`);
    }
    if (release.schema_version !== 1) {
      reasons.push(`invalid release note schema_version: ${String(release.schema_version)}`);
    }
    if (release.recommendation !== "promote") {
      reasons.push(`release note recommendation is not promote: ${String(release.recommendation)}`);
    }
    if (release.can_promote !== true) {
      reasons.push(`release note can_promote must be true`);
    }
    if (release.decision_lint_status !== "passed") {
      reasons.push(`release note decision_lint_status is not passed: ${String(release.decision_lint_status)}`);
    }
    if ((release.decision_lint_error_count ?? 1) !== 0) {
      reasons.push(`release note decision_lint_error_count must be 0: ${String(release.decision_lint_error_count)}`);
    }

    if (decision.project_name && release.project_name && decision.project_name !== release.project_name) {
      reasons.push(`project mismatch decision=${decision.project_name} release_note=${release.project_name}`);
    }
    if (decision.run_scope && release.run_scope && decision.run_scope !== release.run_scope) {
      reasons.push(`run_scope mismatch decision=${decision.run_scope} release_note=${release.run_scope}`);
    }
    if (decision.run_date_utc && release.run_date_utc && decision.run_date_utc !== release.run_date_utc) {
      reasons.push(`run_date_utc mismatch decision=${decision.run_date_utc} release_note=${release.run_date_utc}`);
    }
  }

  const report: GuardReport = {
    schema: "underlay.migration.promotion_ci_guard.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    status: reasons.length === 0 ? "passed" : "failed",
    reason_count: reasons.length,
    reasons,
    files: {
      decision_file: decisionFile,
      lint_file: lintFile,
      release_note_file: releaseNoteFile,
      guard_file: guardFile,
    },
  };

  mkdirSync(dirname(guardFile), { recursive: true });
  writeFileSync(guardFile, JSON.stringify(report, null, 2) + "\n", "utf-8");

  if (args.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    console.log(`promotion ci guard ${report.status}: ${guardFile}`);
    console.log(`reason_count=${report.reason_count}`);
  }

  if (report.status === "failed") {
    for (const reason of reasons) {
      console.error(withCode("MIG_PROMO_003", `guard failure: ${reason}`));
    }
    process.exit(1);
  }
}

main();
