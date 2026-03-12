import { mkdirSync, writeFileSync } from "node:fs";
import { basename, dirname, extname, join } from "node:path";

import {
  loadConfig,
  readOptional,
  readString,
  readStringFromFile,
  validateDigestRef,
} from "./config.ts";
import {
  maybeRunAppMigrationRunner,
  repositoryFromTaggedRef,
  requireRunnerArtifacts,
  runStandardReports,
} from "./runner_support.ts";
import { requireCommand, runCommandText, underlayDevtoolsCommand } from "./tooling.ts";

function extractDigest(output: string): string {
  const match = output.match(/digest=(sha256:[0-9a-f]{64})/);
  if (!match) {
    throw new Error("failed to parse digest from publish output");
  }
  return match[1];
}

function defaultBundleRefFile(bundleFile: string): string {
  const extension = extname(bundleFile);
  const stem = extension.length > 0 ? basename(bundleFile, extension) : basename(bundleFile);
  return join(dirname(bundleFile), `${stem}.digest-ref.txt`);
}

function main(): void {
  const { filePath, values } = loadConfig();
  console.log(`using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`);
  const underlayDevtools = underlayDevtoolsCommand(values);
  requireCommand(underlayDevtools);

  const reuseFromDigestRef = readStringFromFile(
    values,
    "REUSE_FROM_DIGEST_REF",
    "REUSE_FROM_DIGEST_REF_FILE",
  );
  validateDigestRef(reuseFromDigestRef, "REUSE_FROM_DIGEST_REF");

  const sourceSystem = readString(values, "SOURCE_SYSTEM", "legacy_site");
  const targetSchemaVersion = readString(values, "TARGET_SCHEMA_VERSION", "schema-v1");
  const mediaDir = readString(values, "MEDIA_DIR", "./legacy-export/media");
  const bundleFile = readString(values, "BUNDLE_FILE", "./dist/migration-bundle-refresh.oci");
  const bundleRefFile = readOptional(values, "BUNDLE_REF_FILE") || defaultBundleRefFile(bundleFile);
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
  const governancePolicyFile = readString(
    values,
    "GOVERNANCE_POLICY_FILE",
    "./runtime/governance-policy.json",
  );

  mkdirSync(dirname(bundleFile), { recursive: true });

  runCommandText(underlayDevtools, [
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

  const publishOutput = runCommandText(underlayDevtools, [
    "migration",
    "bundle",
    "publish",
    "--bundle",
    bundleFile,
    "--oci-ref",
    ociRefTag,
  ]);

  const digest = extractDigest(publishOutput);
  const repository = repositoryFromTaggedRef(ociRefTag);
  const bundleRef = `${repository}@${digest}`;
  mkdirSync(dirname(bundleRefFile), { recursive: true });
  writeFileSync(bundleRefFile, `${bundleRef}\n`, "utf-8");

  const decisionIndexFile = readString(values, "DECISION_INDEX_FILE", `${outputDir}/decision_index.json`);
  const decisionJournalFile = readString(
    values,
    "DECISION_JOURNAL_FILE",
    `${outputDir}/decision_journal.ndjson`,
  );

  maybeRunAppMigrationRunner({
    underlayDevtoolsCmd: underlayDevtools,
    bundleRef,
    outputDir,
    appMigrationRunnerCmd,
    artifacts: {
      runReport,
      decisionIndexFile,
      decisionJournalFile,
    },
    runnerEnv: {
      REUSE_FROM_DIGEST_REF: reuseFromDigestRef,
    },
  });

  requireRunnerArtifacts(
    {
      runReport,
      decisionIndexFile,
      decisionJournalFile,
    },
    appMigrationRunnerCmd,
  );

  runStandardReports({
    underlayDevtoolsCmd: underlayDevtools,
    runReport,
    outputDir,
    governancePolicyFile,
  });

  runCommandText(underlayDevtools, [
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
  console.log("REFRESH_BUNDLE_REF_FILE=" + bundleRefFile);
  console.log("REUSE_FROM_DIGEST_REF=" + reuseFromDigestRef);
}

main();
