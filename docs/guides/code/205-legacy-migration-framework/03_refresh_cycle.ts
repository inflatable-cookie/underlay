import { existsSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { spawnSync } from "node:child_process";

import { loadConfig, readOptional, readString, validateDigestRef } from "./config.ts";

function runCommand(command: string, args: string[]): string {
  const result = spawnSync(command, args, {
    encoding: "utf-8",
    stdio: ["inherit", "pipe", "pipe"],
  });

  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);

  if (result.status !== 0) {
    throw new Error(`command failed: ${command} ${args.join(" ")}`);
  }

  return result.stdout ?? "";
}

function runShell(command: string, env: Record<string, string>): void {
  const result = spawnSync("zsh", ["-lc", command], {
    encoding: "utf-8",
    stdio: ["inherit", "pipe", "pipe"],
    env: { ...process.env, ...env },
  });

  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);

  if (result.status !== 0) {
    throw new Error(`command failed: ${command}`);
  }
}

function requireTool(name: string): void {
  const result = spawnSync("which", [name], { stdio: "ignore" });
  if (result.status !== 0) {
    throw new Error(`${name} is required in PATH`);
  }
}

function extractDigest(output: string): string {
  const match = output.match(/digest=(sha256:[0-9a-f]{64})/);
  if (!match) {
    throw new Error("failed to parse digest from publish output");
  }
  return match[1];
}

function main(): void {
  requireTool("underlay-devtools");

  const { filePath, values } = loadConfig();
  console.log(`using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`);

  const reuseFromDigestRef = readString(values, "REUSE_FROM_DIGEST_REF");
  validateDigestRef(reuseFromDigestRef, "REUSE_FROM_DIGEST_REF");

  const sourceSystem = readString(values, "SOURCE_SYSTEM", "legacy_site");
  const targetSchemaVersion = readString(values, "TARGET_SCHEMA_VERSION", "schema-v1");
  const mediaDir = readString(values, "MEDIA_DIR", "./legacy-export/media");
  const bundleFile = readString(values, "BUNDLE_FILE", "./dist/migration-bundle-refresh.oci");
  const ociRefTag = readString(
    values,
    "OCI_REF_TAG",
    `registry.example.com/underlay/site-migration:refresh-${new Date()
      .toISOString()
      .replace(/[-:]/g, "")
      .replace(/\.\d+Z$/, "Z")}`,
  );
  const outputDir = readString(values, "OUTPUT_DIR", "./runtime/refresh-pass");
  const runReport = readString(values, "RUN_REPORT", `${outputDir}/run-report.json`);
  const appMigrationRunnerCmd = readOptional(values, "APP_MIGRATION_RUNNER_CMD");

  mkdirSync(dirname(bundleFile), { recursive: true });

  runCommand("underlay-devtools", [
    "migration",
    "bundle",
    "build",
    "--output",
    bundleFile,
    "--source-system",
    sourceSystem,
    "--target-schema-version",
    targetSchemaVersion,
    "--media-dir",
    mediaDir,
  ]);

  const publishOutput = runCommand("underlay-devtools", [
    "migration",
    "bundle",
    "publish",
    "--bundle",
    bundleFile,
    "--oci-ref",
    ociRefTag,
  ]);

  const digest = extractDigest(publishOutput);
  const repository = ociRefTag.split(":")[0];
  const bundleRef = `${repository}@${digest}`;

  runCommand("underlay-devtools", [
    "migration",
    "run",
    "--bundle",
    bundleRef,
    "--output",
    outputDir,
  ]);

  if (appMigrationRunnerCmd.trim().length > 0) {
    runShell(appMigrationRunnerCmd, {
      REUSE_FROM_DIGEST_REF: reuseFromDigestRef,
      BUNDLE_REF: bundleRef,
      OUTPUT_DIR: outputDir,
    });
  }

  if (!existsSync(runReport)) {
    throw new Error(
      [
        `expected run report not found at: ${runReport}`,
        "set APP_MIGRATION_RUNNER_CMD to invoke your migration orchestrator so it writes run-report.json.",
      ].join("\n"),
    );
  }

  const decisionIndexFile = readString(values, "DECISION_INDEX_FILE", `${outputDir}/decision_index.json`);
  const decisionJournalFile = readString(
    values,
    "DECISION_JOURNAL_FILE",
    `${outputDir}/decision_journal.ndjson`,
  );

  runCommand("underlay-devtools", [
    "migration",
    "report",
    "drift",
    "--input",
    runReport,
    "--max-unresolved",
    "0",
    "--max-governance",
    "0",
    "--max-lineage",
    "0",
    "--decision-index",
    decisionIndexFile,
    "--decision-journal",
    decisionJournalFile,
    "--expected-bundle-digest",
    digest,
  ]);

  console.log("\nREFRESH_BUNDLE_REF=" + bundleRef);
  console.log("REUSE_FROM_DIGEST_REF=" + reuseFromDigestRef);
}

main();
