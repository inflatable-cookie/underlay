import { existsSync } from "node:fs";

import { runCommandText, spawnCommand } from "./tooling.ts";

type RunnerArtifacts = {
  runReport: string;
  decisionIndexFile: string;
  decisionJournalFile: string;
};

type RunnerExecutionOptions = {
  underlayDevtoolsCmd: string;
  bundleRef: string;
  outputDir: string;
  artifacts: RunnerArtifacts;
  appMigrationRunnerCmd: string;
  runnerEnv?: Record<string, string>;
};

type ReportSuiteOptions = {
  underlayDevtoolsCmd: string;
  runReport: string;
  outputDir: string;
  governancePolicyFile: string;
};

function renderMissingArtifacts(artifacts: string[]): string {
  return artifacts.map((artifact) => `- ${artifact}`).join("\n");
}

export function runCommand(command: string, args: string[]): string {
  return runCommandText(command, args);
}

export function runShell(command: string, env: Record<string, string>): void {
  const result = spawnCommand(command, [], { env });

  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);

  if (result.status !== 0) {
    throw new Error(`command failed: ${command}`);
  }
}

export function repositoryFromTaggedRef(ociRef: string): string {
  const digestIndex = ociRef.indexOf("@");
  if (digestIndex >= 0) {
    return ociRef.slice(0, digestIndex);
  }

  const lastSlash = ociRef.lastIndexOf("/");
  const lastColon = ociRef.lastIndexOf(":");
  if (lastColon > lastSlash) {
    return ociRef.slice(0, lastColon);
  }

  return ociRef;
}

export function requireTool(name: string): void {
  const result = spawnCommand("which", [name]);
  if (result.status !== 0) {
    throw new Error(`${name} is required in PATH`);
  }
}

export function maybeRunAppMigrationRunner(options: RunnerExecutionOptions): void {
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "run",
    "--bundle",
    options.bundleRef,
    "--output",
    options.outputDir,
  ]);

  if (options.appMigrationRunnerCmd.trim().length > 0) {
    runShell(options.appMigrationRunnerCmd, {
      BUNDLE_REF: options.bundleRef,
      OUTPUT_DIR: options.outputDir,
      RUN_REPORT: options.artifacts.runReport,
      DECISION_INDEX_FILE: options.artifacts.decisionIndexFile,
      DECISION_JOURNAL_FILE: options.artifacts.decisionJournalFile,
      ...options.runnerEnv,
    });
  }
}

export function requireRunnerArtifacts(
  artifacts: RunnerArtifacts,
  appMigrationRunnerCmd: string,
): void {
  const missing: string[] = [];
  if (!existsSync(artifacts.runReport)) missing.push(artifacts.runReport);
  if (!existsSync(artifacts.decisionIndexFile)) missing.push(artifacts.decisionIndexFile);
  if (!existsSync(artifacts.decisionJournalFile)) missing.push(artifacts.decisionJournalFile);

  if (missing.length === 0) {
    return;
  }

  const runnerHint =
    appMigrationRunnerCmd.trim().length > 0
      ? `configured APP_MIGRATION_RUNNER_CMD did not write required artifacts:\n${appMigrationRunnerCmd}`
      : "set APP_MIGRATION_RUNNER_CMD to invoke your migration orchestrator, or generate the artifacts before rerunning";

  throw new Error(
    [
      "required migration runner artifacts not found:",
      renderMissingArtifacts(missing),
      runnerHint,
      "required artifacts: run-report.json, decision_index.json, decision_journal.ndjson",
    ].join("\n"),
  );
}

export function runStandardReports(options: ReportSuiteOptions): void {
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "report",
    "governance",
    "--input",
    options.runReport,
    "--limit",
    "20",
  ]);
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "report",
    "integrity",
    "--input",
    options.runReport,
  ]);
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "report",
    "recovery",
    "--input",
    options.runReport,
  ]);
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "report",
    "verify",
    "--input",
    options.runReport,
    "--output-dir",
    options.outputDir,
  ]);
  runCommand(options.underlayDevtoolsCmd, [
    "migration",
    "report",
    "audit",
    "--input",
    options.outputDir,
    "--output-dir",
    options.outputDir,
  ]);

  if (existsSync(options.governancePolicyFile)) {
    runCommand(options.underlayDevtoolsCmd, [
      "migration",
      "report",
      "policy",
      "--input",
      options.governancePolicyFile,
    ]);
  } else {
    console.warn(
      `governance policy file not found (skipping policy report): ${options.governancePolicyFile}`,
    );
  }
}
