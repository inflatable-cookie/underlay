import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { commandString, requireCommand, spawnCommand, underlayDevtoolsCommand } from "./tooling.ts";

type GateStatus = "passed" | "failed";

type RefreshDriftGate = {
  schema: "underlay.migration.refresh_drift_gate.v1";
  schema_version: 1;
  generated_at: string;
  status: GateStatus;
  command: string;
  exit_code: number;
  output_excerpt: string;
  files: {
    config_file: string;
    run_report: string;
    decision_index_file: string;
    decision_journal_file: string;
    gate_file: string;
  };
  thresholds: {
    drift_max_unresolved: number;
    drift_max_governance: number;
    drift_max_lineage: number;
  };
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): { output?: string; json?: boolean } {
  const parsed: { output?: string; json?: boolean } = {};
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
    if (token === "--json") {
      parsed.json = true;
      continue;
    }
    throw new Error(`unknown argument: ${token}`);
  }
  return parsed;
}

function parseNumber(value: string, key: string): number {
  if (!/^\d+$/.test(value)) {
    throw new Error(`${key} must be an unsigned integer string`);
  }
  return Number(value);
}

function defaultOutputPath(config: Record<string, string>): string {
  const configured = readOptional(config, "REFRESH_DRIFT_GATE_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.refresh.${runDate}.refresh-drift-gate.json`);
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );
  const underlayDevtools = underlayDevtoolsCommand(values);
  requireCommand(underlayDevtools);

  const runScope = readString(values, "RUN_SCOPE", "refresh");
  if (runScope !== "refresh") {
    throw new Error(`13_refresh_drift_gate.ts only supports RUN_SCOPE=refresh (got: ${runScope})`);
  }

  const outputDir = resolve(readString(values, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const runReport = resolve(readString(values, "RUN_REPORT", `${outputDir}/run-report.json`));
  const decisionIndexFile = resolve(
    readString(values, "DECISION_INDEX_FILE", `${outputDir}/decision_index.json`),
  );
  const decisionJournalFile = resolve(
    readString(values, "DECISION_JOURNAL_FILE", `${outputDir}/decision_journal.ndjson`),
  );

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

  const gateFile = resolve(args.output || defaultOutputPath(values as Record<string, string>));

  const driftArgs = [
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
    "--decision-index",
    decisionIndexFile,
    "--decision-journal",
    decisionJournalFile,
  ];

  const command = commandString(underlayDevtools, driftArgs);
  const result = spawnCommand(underlayDevtools, driftArgs);

  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);

  const gate: RefreshDriftGate = {
    schema: "underlay.migration.refresh_drift_gate.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    status: result.status === 0 ? "passed" : "failed",
    command,
    exit_code: result.status ?? -1,
    output_excerpt: `${result.stdout ?? ""}${result.stderr ?? ""}`.trim().slice(0, 3000),
    files: {
      config_file: filePath,
      run_report: runReport,
      decision_index_file: decisionIndexFile,
      decision_journal_file: decisionJournalFile,
      gate_file: gateFile,
    },
    thresholds: {
      drift_max_unresolved: driftMaxUnresolved,
      drift_max_governance: driftMaxGovernance,
      drift_max_lineage: driftMaxLineage,
    },
  };

  mkdirSync(dirname(gateFile), { recursive: true });
  writeFileSync(gateFile, JSON.stringify(gate, null, 2) + "\n", "utf-8");

  if (args.json) {
    console.log(JSON.stringify(gate, null, 2));
  } else {
    console.log(`refresh drift gate ${gate.status}: ${gateFile}`);
  }

  if (gate.status === "failed") {
    process.exit(1);
  }
}

main();
