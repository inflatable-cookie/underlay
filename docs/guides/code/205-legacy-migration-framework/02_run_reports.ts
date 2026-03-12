import {
  loadConfig,
  readString,
  readStringFromFile,
  validateDigestRef,
} from "./config.ts";
import {
  maybeRunAppMigrationRunner,
  requireRunnerArtifacts,
  runStandardReports,
} from "./runner_support.ts";
import { requireCommand, underlayDevtoolsCommand } from "./tooling.ts";

function main(): void {
  const { filePath, values } = loadConfig();
  console.log(`using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env only)" : ""}`);
  const underlayDevtools = underlayDevtoolsCommand(values);
  requireCommand(underlayDevtools);

  const bundleRef = readStringFromFile(values, "BUNDLE_REF", "BUNDLE_REF_FILE");
  validateDigestRef(bundleRef, "BUNDLE_REF");

  const outputDir = readString(values, "OUTPUT_DIR", "./runtime/demo-pass");
  const runReport = readString(values, "RUN_REPORT", `${outputDir}/run-report.json`);
  const appMigrationRunnerCmd = readString(values, "APP_MIGRATION_RUNNER_CMD", "");
  const decisionIndexFile = readString(values, "DECISION_INDEX_FILE", `${outputDir}/decision_index.json`);
  const decisionJournalFile = readString(
    values,
    "DECISION_JOURNAL_FILE",
    `${outputDir}/decision_journal.ndjson`,
  );
  const governancePolicyFile = readString(
    values,
    "GOVERNANCE_POLICY_FILE",
    "./runtime/governance-policy.json",
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
}

main();
