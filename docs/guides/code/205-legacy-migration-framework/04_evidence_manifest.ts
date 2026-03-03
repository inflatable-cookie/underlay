import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve, sep } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { fail } from "./error_codes.ts";

type RunScope = "demo" | "refresh" | "pre_production";

type ArtifactEntry = {
  artifact_name: string;
  path: string;
  sha256: string;
  size_bytes: number;
  generated_at: string;
};

type EvidenceManifest = {
  schema: "underlay.migration.evidence_manifest.v1";
  schema_version: 1;
  generated_at: string;
  project_name: string;
  run_scope: RunScope;
  run_date_utc: string;
  output_dir: string;
  artifacts: ArtifactEntry[];
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): {
  scope?: string;
  project?: string;
  runDate?: string;
  output?: string;
} {
  const parsed: { scope?: string; project?: string; runDate?: string; output?: string } = {};
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--scope" && i + 1 < argv.length) {
      parsed.scope = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--scope=")) {
      parsed.scope = token.slice("--scope=".length).trim();
      continue;
    }
    if (token === "--project" && i + 1 < argv.length) {
      parsed.project = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--project=")) {
      parsed.project = token.slice("--project=".length).trim();
      continue;
    }
    if (token === "--run-date" && i + 1 < argv.length) {
      parsed.runDate = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--run-date=")) {
      parsed.runDate = token.slice("--run-date=".length).trim();
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
    fail("MIG_CLI_001", `unknown argument: ${token}`);
  }
  return parsed;
}

function asRunScope(value: string): RunScope {
  if (value === "demo" || value === "refresh" || value === "pre_production") {
    return value;
  }
  fail("MIG_CFG_008", `RUN_SCOPE must be one of: demo, refresh, pre_production (got: ${value})`);
}

function validateProjectName(value: string): string {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(value)) {
    fail(
      "MIG_CFG_009",
      `PROJECT_NAME must be lowercase kebab-case (example: acowtancy-demo), got: ${value}`,
    );
  }
  return value;
}

function validateRunDateUtc(value: string): string {
  if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) {
    fail("MIG_CFG_010", `RUN_DATE_UTC must be YYYY-MM-DD, got: ${value}`);
  }
  return value;
}

function defaultDoctorReport(runReport: string): string {
  if (runReport.endsWith("run-report.json")) {
    return runReport.replace(/run-report\.json$/, "migration-doctor.json");
  }
  return "./runtime/migration-doctor.json";
}

function readRunId(runReportPath: string): string {
  const raw = readFileSync(runReportPath, "utf-8");
  const parsed = JSON.parse(raw) as { run_id?: unknown };
  if (!parsed || typeof parsed !== "object" || typeof parsed.run_id !== "string") {
    fail("MIG_EVID_001", `RUN_REPORT missing run_id: ${runReportPath}`);
  }
  return parsed.run_id;
}

function sha256File(path: string): string {
  const content = readFileSync(path);
  const hash = createHash("sha256");
  hash.update(content);
  return hash.digest("hex");
}

function renderPathForManifest(outputDir: string, filePath: string): string {
  const rel = relative(outputDir, filePath);
  if (rel.length > 0 && !rel.startsWith("..") && !rel.startsWith(sep)) {
    return rel.split(sep).join("/");
  }
  return filePath.split(sep).join("/");
}

function requireFile(path: string, label: string): void {
  if (!existsSync(path)) {
    fail("MIG_EVID_002", `required artifact missing (${label}): ${path}`);
  }
  const stat = statSync(path);
  if (!stat.isFile()) {
    fail("MIG_EVID_003", `artifact must be a file (${label}): ${path}`);
  }
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const outputDir = resolve(readString(values, "OUTPUT_DIR", "./runtime/demo-pass"));
  const runReport = resolve(readString(values, "RUN_REPORT", `${outputDir}/run-report.json`));
  const runId = readRunId(runReport);

  const projectName = validateProjectName(
    args.project || readString(values, "PROJECT_NAME", "migration"),
  );
  const runScope = asRunScope(args.scope || readString(values, "RUN_SCOPE", "demo"));
  const runDateUtc = validateRunDateUtc(args.runDate || readString(values, "RUN_DATE_UTC", utcDate()));

  const doctorReport = resolve(
    readOptional(values, "DOCTOR_REPORT") || defaultDoctorReport(runReport),
  );
  const decisionIndexFile = resolve(
    readString(values, "DECISION_INDEX_FILE", `${outputDir}/decision_index.json`),
  );
  const decisionJournalFile = resolve(
    readString(values, "DECISION_JOURNAL_FILE", `${outputDir}/decision_journal.ndjson`),
  );
  const decisionReuseSummaryFile = resolve(
    readOptional(values, "DECISION_REUSE_SUMMARY_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.decision-reuse-summary.json`,
  );
  const decisionReuseSummaryLintFile = resolve(
    readOptional(values, "DECISION_REUSE_SUMMARY_LINT_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.decision-reuse-summary-lint.json`,
  );
  const refreshDriftGateFile = resolve(
    readOptional(values, "REFRESH_DRIFT_GATE_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.refresh-drift-gate.json`,
  );
  const refreshDriftGateLintFile = resolve(
    readOptional(values, "REFRESH_DRIFT_GATE_LINT_FILE") ||
      `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.refresh-drift-gate-lint.json`,
  );
  const verifyArtifactFile = resolve(
    readOptional(values, "VERIFY_ARTIFACT_FILE") ||
      `${outputDir}/verification-artifacts/${runId}.json`,
  );
  const auditArtifactFile = resolve(
    readOptional(values, "AUDIT_ARTIFACT_FILE") || `${outputDir}/audit-artifacts/${runId}.json`,
  );

  const defaultManifestFile = `${outputDir}/metadata/${projectName}.${runScope}.${runDateUtc}.artifact-manifest.json`;
  const manifestFile = resolve(
    args.output || readOptional(values, "ARTIFACT_MANIFEST_FILE") || defaultManifestFile,
  );

  const required = [
    { artifact_name: "migration-doctor", path: doctorReport },
    { artifact_name: "run-report", path: runReport },
    { artifact_name: "decision_index", path: decisionIndexFile },
    { artifact_name: "decision_journal", path: decisionJournalFile },
    { artifact_name: "verify_artifact", path: verifyArtifactFile },
    { artifact_name: "audit_artifact", path: auditArtifactFile },
  ];
  if (runScope === "refresh") {
    required.push({
      artifact_name: "decision_reuse_summary",
      path: decisionReuseSummaryFile,
    });
    required.push({
      artifact_name: "decision_reuse_summary_lint",
      path: decisionReuseSummaryLintFile,
    });
    required.push({
      artifact_name: "refresh_drift_gate",
      path: refreshDriftGateFile,
    });
    required.push({
      artifact_name: "refresh_drift_gate_lint",
      path: refreshDriftGateLintFile,
    });
  }

  for (const item of required) {
    requireFile(item.path, item.artifact_name);
  }

  const artifacts: ArtifactEntry[] = required.map((item) => {
    const stat = statSync(item.path);
    return {
      artifact_name: item.artifact_name,
      path: renderPathForManifest(outputDir, item.path),
      sha256: sha256File(item.path),
      size_bytes: stat.size,
      generated_at: stat.mtime.toISOString(),
    };
  });

  const manifest: EvidenceManifest = {
    schema: "underlay.migration.evidence_manifest.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    project_name: projectName,
    run_scope: runScope,
    run_date_utc: runDateUtc,
    output_dir: outputDir,
    artifacts,
  };

  mkdirSync(dirname(manifestFile), { recursive: true });
  writeFileSync(manifestFile, JSON.stringify(manifest, null, 2) + "\n", "utf-8");

  console.log(`evidence manifest written: ${manifestFile}`);
  for (const artifact of artifacts) {
    console.log(`evidence artifact ${artifact.artifact_name} ${artifact.sha256} ${artifact.path}`);
  }
}

main();
