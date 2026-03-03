import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";

type CheckStatus = "passed" | "failed" | "skipped";

type PromotionDecision = {
  project_name: string;
  run_scope: "demo" | "refresh" | "pre_production";
  run_date_utc: string;
  recommendation: "promote" | "hold";
  can_promote: boolean;
  checks: Array<{ id: string; status: CheckStatus; required: boolean }>;
  blocking_reasons: string[];
  files?: { artifact_manifest_file?: string };
};

type PromotionLintResult = {
  status: "passed" | "failed";
  error_count: number;
  errors: string[];
};

type ArtifactManifest = {
  artifacts: Array<{
    artifact_name: string;
    path: string;
    sha256: string;
  }>;
};

type ReleaseNote = {
  schema: "underlay.migration.promotion_release_note.v1";
  schema_version: 1;
  generated_at: string;
  project_name: string;
  run_scope: "demo" | "refresh" | "pre_production";
  run_date_utc: string;
  recommendation: "promote" | "hold";
  can_promote: boolean;
  decision_lint_status: "passed" | "failed";
  decision_lint_error_count: number;
  decision_lint_errors: string[];
  digest_refs: {
    bundle_ref: string;
    reuse_from_digest_ref: string;
  };
  gate_statuses: Array<{
    gate: string;
    status: CheckStatus;
    required: boolean;
  }>;
  blocking_reasons: string[];
  artifact_checksums: Array<{
    artifact_name: string;
    path: string;
    sha256: string;
  }>;
  source_files: {
    promotion_decision_file: string;
    promotion_decision_lint_file: string;
    artifact_manifest_file: string;
  };
  outputs: {
    json_file: string;
    markdown_file: string;
  };
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): {
  decision?: string;
  lint?: string;
  manifest?: string;
  outputJson?: string;
  outputMd?: string;
} {
  const parsed: {
    decision?: string;
    lint?: string;
    manifest?: string;
    outputJson?: string;
    outputMd?: string;
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
    if (token === "--manifest" && i + 1 < argv.length) {
      parsed.manifest = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--manifest=")) {
      parsed.manifest = token.slice("--manifest=".length).trim();
      continue;
    }
    if (token === "--output-json" && i + 1 < argv.length) {
      parsed.outputJson = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--output-json=")) {
      parsed.outputJson = token.slice("--output-json=".length).trim();
      continue;
    }
    if (token === "--output-md" && i + 1 < argv.length) {
      parsed.outputMd = argv[i + 1].trim();
      i += 1;
      continue;
    }
    if (token.startsWith("--output-md=")) {
      parsed.outputMd = token.slice("--output-md=".length).trim();
      continue;
    }
    throw new Error(`unknown argument: ${token}`);
  }
  return parsed;
}

function readJson(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function requireFile(path: string, label: string): void {
  if (!existsSync(path)) {
    throw new Error(`missing ${label}: ${path}`);
  }
}

function defaultDecisionPath(config: Record<string, string>): string {
  const configured = readOptional(config, "PROMOTION_DECISION_FILE");
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.promotion-decision.json`);
}

function defaultLintPath(config: Record<string, string>): string {
  const configured = readOptional(config, "PROMOTION_DECISION_LINT_FILE");
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.promotion-decision-lint.json`);
}

function defaultManifestPath(config: Record<string, string>): string {
  const configured = readOptional(config, "ARTIFACT_MANIFEST_FILE");
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.artifact-manifest.json`);
}

function defaultReleaseNoteJsonPath(config: Record<string, string>): string {
  const configured = readOptional(config, "PROMOTION_RELEASE_NOTE_JSON_FILE");
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.promotion-release-note.json`);
}

function defaultReleaseNoteMdPath(config: Record<string, string>): string {
  const configured = readOptional(config, "PROMOTION_RELEASE_NOTE_MD_FILE");
  if (configured.length > 0) return resolve(configured);
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/demo-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.promotion-release-note.md`);
}

function toMarkdown(note: ReleaseNote): string {
  const lines: string[] = [];
  lines.push(`# Migration Promotion Summary`);
  lines.push("");
  lines.push(`- Project: \`${note.project_name}\``);
  lines.push(`- Scope: \`${note.run_scope}\``);
  lines.push(`- Run Date (UTC): \`${note.run_date_utc}\``);
  lines.push(`- Recommendation: \`${note.recommendation}\``);
  lines.push(`- Can Promote: \`${String(note.can_promote)}\``);
  lines.push(`- Decision Lint: \`${note.decision_lint_status}\` (errors: ${note.decision_lint_error_count})`);
  lines.push("");

  lines.push(`## Digests`);
  lines.push(`- Bundle Ref: \`${note.digest_refs.bundle_ref || "unset"}\``);
  lines.push(`- Reuse Baseline: \`${note.digest_refs.reuse_from_digest_ref || "unset"}\``);
  lines.push("");

  lines.push(`## Gate Statuses`);
  for (const gate of note.gate_statuses) {
    lines.push(`- ${gate.gate}: \`${gate.status}\` (required=${String(gate.required)})`);
  }
  lines.push("");

  lines.push(`## Blocking Reasons`);
  if (note.blocking_reasons.length === 0) {
    lines.push(`- none`);
  } else {
    for (const reason of note.blocking_reasons) {
      lines.push(`- ${reason}`);
    }
  }
  lines.push("");

  lines.push(`## Artifact Checksums`);
  for (const artifact of note.artifact_checksums) {
    lines.push(`- ${artifact.artifact_name}: \`${artifact.sha256}\` (${artifact.path})`);
  }
  lines.push("");

  lines.push(`## Source Files`);
  lines.push(`- Promotion Decision: \`${note.source_files.promotion_decision_file}\``);
  lines.push(`- Decision Lint: \`${note.source_files.promotion_decision_lint_file}\``);
  lines.push(`- Artifact Manifest: \`${note.source_files.artifact_manifest_file}\``);
  lines.push("");
  lines.push(`## Output Files`);
  lines.push(`- JSON: \`${note.outputs.json_file}\``);
  lines.push(`- Markdown: \`${note.outputs.markdown_file}\``);
  lines.push("");

  return lines.join("\n");
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const decisionPath = resolve(args.decision || defaultDecisionPath(values as Record<string, string>));
  const lintPath = resolve(args.lint || defaultLintPath(values as Record<string, string>));
  const outputJsonPath = resolve(args.outputJson || defaultReleaseNoteJsonPath(values as Record<string, string>));
  const outputMdPath = resolve(args.outputMd || defaultReleaseNoteMdPath(values as Record<string, string>));

  requireFile(decisionPath, "promotion decision file");
  requireFile(lintPath, "promotion decision lint file");

  const decision = readJson(decisionPath) as PromotionDecision;
  const lint = readJson(lintPath) as PromotionLintResult;
  const configuredManifest = readOptional(values as Record<string, string>, "ARTIFACT_MANIFEST_FILE");
  const manifestPath = resolve(
    args.manifest ||
      configuredManifest ||
      decision.files?.artifact_manifest_file ||
      defaultManifestPath(values as Record<string, string>),
  );
  requireFile(manifestPath, "artifact manifest file");
  const manifest = readJson(manifestPath) as ArtifactManifest;

  const note: ReleaseNote = {
    schema: "underlay.migration.promotion_release_note.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    project_name: decision.project_name,
    run_scope: decision.run_scope,
    run_date_utc: decision.run_date_utc,
    recommendation: decision.recommendation,
    can_promote: decision.can_promote,
    decision_lint_status: lint.status,
    decision_lint_error_count: lint.error_count,
    decision_lint_errors: lint.errors,
    digest_refs: {
      bundle_ref: readOptional(values, "BUNDLE_REF"),
      reuse_from_digest_ref: readOptional(values, "REUSE_FROM_DIGEST_REF"),
    },
    gate_statuses: decision.checks.map((check) => ({
      gate: check.id,
      status: check.status,
      required: check.required,
    })),
    blocking_reasons: decision.blocking_reasons,
    artifact_checksums: manifest.artifacts.map((artifact) => ({
      artifact_name: artifact.artifact_name,
      path: artifact.path,
      sha256: artifact.sha256,
    })),
    source_files: {
      promotion_decision_file: decisionPath,
      promotion_decision_lint_file: lintPath,
      artifact_manifest_file: manifestPath,
    },
    outputs: {
      json_file: outputJsonPath,
      markdown_file: outputMdPath,
    },
  };

  mkdirSync(dirname(outputJsonPath), { recursive: true });
  mkdirSync(dirname(outputMdPath), { recursive: true });
  writeFileSync(outputJsonPath, JSON.stringify(note, null, 2) + "\n", "utf-8");
  writeFileSync(outputMdPath, toMarkdown(note), "utf-8");

  console.log(`promotion release note json: ${outputJsonPath}`);
  console.log(`promotion release note markdown: ${outputMdPath}`);
}

main();
