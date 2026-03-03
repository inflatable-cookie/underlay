import { createHash } from "node:crypto";
import { existsSync, readFileSync, statSync } from "node:fs";
import { isAbsolute, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { fail, withCode } from "./error_codes.ts";

type ArtifactEntry = {
  artifact_name: string;
  path: string;
  sha256: string;
};

type EvidenceManifest = {
  schema: string;
  schema_version: number;
  run_scope?: string;
  output_dir: string;
  artifacts: ArtifactEntry[];
};

const REQUIRED_COMMON_ARTIFACT_NAMES = [
  "migration-doctor",
  "run-report",
  "decision_index",
  "decision_journal",
  "verify_artifact",
  "audit_artifact",
] as const;
const REQUIRED_REFRESH_ARTIFACT_NAMES = [
  "decision_reuse_summary",
  "decision_reuse_summary_lint",
  "refresh_drift_gate",
  "refresh_drift_gate_lint",
] as const;

function parseArgs(argv: string[]): { input?: string } {
  const parsed: { input?: string } = {};
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
    fail("MIG_CLI_001", `unknown argument: ${token}`);
  }
  return parsed;
}

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function requireFile(path: string, label: string): void {
  if (!existsSync(path)) {
    fail("MIG_EVID_004", `missing file (${label}): ${path}`);
  }
  const stat = statSync(path);
  if (!stat.isFile()) {
    fail("MIG_EVID_005", `expected file (${label}): ${path}`);
  }
}

function sha256File(path: string): string {
  const hash = createHash("sha256");
  hash.update(readFileSync(path));
  return hash.digest("hex");
}

function resolveArtifactPath(outputDir: string, entryPath: string): string {
  if (isAbsolute(entryPath)) {
    return entryPath;
  }
  return resolve(outputDir, entryPath);
}

function defaultManifestPath(
  config: Record<string, string>,
  outputDir: string,
  argsInput?: string,
): string {
  if (argsInput && argsInput.length > 0) {
    return resolve(argsInput);
  }
  const configured = readOptional(config, "ARTIFACT_MANIFEST_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "demo");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.artifact-manifest.json`);
}

function parseManifest(path: string): EvidenceManifest {
  const raw = readFileSync(path, "utf-8");
  const parsed = JSON.parse(raw) as unknown;
  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    fail("MIG_EVID_006", `manifest must be a JSON object: ${path}`);
  }

  const manifest = parsed as Partial<EvidenceManifest>;
  if (manifest.schema !== "underlay.migration.evidence_manifest.v1") {
    fail("MIG_EVID_007", `unsupported manifest schema: ${String(manifest.schema)}`);
  }
  if (manifest.schema_version !== 1) {
    fail("MIG_EVID_008", `unsupported manifest schema_version: ${String(manifest.schema_version)}`);
  }
  if (!manifest.output_dir || typeof manifest.output_dir !== "string") {
    fail("MIG_EVID_009", "manifest missing output_dir");
  }
  if (!Array.isArray(manifest.artifacts)) {
    fail("MIG_EVID_010", "manifest missing artifacts array");
  }
  return manifest as EvidenceManifest;
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const outputDir = resolve(readString(values, "OUTPUT_DIR", "./runtime/demo-pass"));
  const manifestFile = defaultManifestPath(values as Record<string, string>, outputDir, args.input);
  requireFile(manifestFile, "artifact_manifest");

  const manifest = parseManifest(manifestFile);
  const manifestOutputDir = resolve(manifest.output_dir);

  const artifactNames = new Set<string>();
  const mismatches: string[] = [];
  const verified: string[] = [];

  for (const artifact of manifest.artifacts) {
    artifactNames.add(artifact.artifact_name);
    const artifactPath = resolveArtifactPath(manifestOutputDir, artifact.path);
    requireFile(artifactPath, artifact.artifact_name);
    const actual = sha256File(artifactPath);
    if (actual !== artifact.sha256) {
      mismatches.push(
        `${artifact.artifact_name}: checksum mismatch expected=${artifact.sha256} actual=${actual} path=${artifact.path}`,
      );
    } else {
      verified.push(`${artifact.artifact_name} ${artifact.sha256} ${artifact.path}`);
    }
  }

  for (const required of REQUIRED_COMMON_ARTIFACT_NAMES) {
    if (!artifactNames.has(required)) {
      mismatches.push(`missing required artifact entry in manifest: ${required}`);
    }
  }
  if (manifest.run_scope === "refresh") {
    for (const required of REQUIRED_REFRESH_ARTIFACT_NAMES) {
      if (!artifactNames.has(required)) {
        mismatches.push(`missing required refresh artifact entry in manifest: ${required}`);
      }
    }
  }

  if (mismatches.length > 0) {
    console.error(withCode("MIG_EVID_011", `evidence verify failed: ${manifestFile}`));
    for (const issue of mismatches) {
      console.error(withCode("MIG_EVID_012", `evidence mismatch ${issue}`));
    }
    process.exit(1);
  }

  console.log(`evidence verify passed: ${manifestFile}`);
  for (const line of verified) {
    console.log(`evidence verified ${line}`);
  }
}

main();
