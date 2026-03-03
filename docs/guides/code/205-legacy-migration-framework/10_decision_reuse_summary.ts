import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";

type DecideStage = {
  decision_count?: number;
  reused_count?: number;
  resolved_count?: number;
  invalidated_count?: number;
  unresolved_count?: number;
};

type RunReport = {
  decide?: DecideStage;
};

type DecisionReuseSummary = {
  schema: "underlay.migration.decision_reuse_summary.v1";
  schema_version: 1;
  generated_at: string;
  project_name: string;
  run_scope: "refresh";
  run_date_utc: string;
  reuse_from_digest_ref: string;
  reuse_stats: {
    reused_count: number;
    new_count: number;
    invalidated_count: number;
    unresolved_count: number;
  };
  sidecar_merge: {
    deduped_count: number;
    replaced_count: number;
    invalidated_count: number;
    corrupted_count: number;
  };
  suppression_kpi: {
    candidate_decisions_total: number;
    reused_decisions_total: number;
    new_ai_calls_total: number;
    new_human_required_total: number;
    invalidated_decisions_total: number;
    ai_call_suppression_ratio: number;
    reuse_ratio: number;
    human_queue_ratio: number;
  };
  targets: {
    ai_call_suppression_ratio_min: number;
    reuse_ratio_min: number;
    human_queue_ratio_max: number;
  };
};

function utcDate(): string {
  return new Date().toISOString().slice(0, 10);
}

function parseArgs(argv: string[]): { input?: string; output?: string; json?: boolean } {
  const parsed: { input?: string; output?: string; json?: boolean } = {};
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

function readJson(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function asCount(value: unknown, fallback = 0): number {
  if (typeof value === "number" && Number.isFinite(value) && value >= 0) {
    return Math.floor(value);
  }
  return fallback;
}

function ratio(numerator: number, denominator: number): number {
  if (denominator <= 0) {
    return 0;
  }
  return Math.max(0, Math.min(1, numerator / denominator));
}

function defaultOutputPath(config: Record<string, string>): string {
  const configured = readOptional(config, "DECISION_REUSE_SUMMARY_FILE");
  if (configured.length > 0) {
    return resolve(configured);
  }
  const outputDir = resolve(readString(config, "OUTPUT_DIR", "./runtime/refresh-pass"));
  const project = readString(config, "PROJECT_NAME", "migration");
  const scope = readString(config, "RUN_SCOPE", "refresh");
  const runDate = readString(config, "RUN_DATE_UTC", utcDate());
  return resolve(`${outputDir}/metadata/${project}.${scope}.${runDate}.decision-reuse-summary.json`);
}

function defaultRunReportPath(config: Record<string, string>): string {
  return resolve(readString(config, "RUN_REPORT", `${readString(config, "OUTPUT_DIR", "./runtime/refresh-pass")}/run-report.json`));
}

function main(): void {
  const args = parseArgs(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  console.log(
    `using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env+defaults only)" : ""}`,
  );

  const runScope = readString(values, "RUN_SCOPE", "refresh");
  if (runScope !== "refresh") {
    throw new Error(`10_decision_reuse_summary.ts only supports RUN_SCOPE=refresh (got: ${runScope})`);
  }

  const runReportPath = resolve(args.input || defaultRunReportPath(values as Record<string, string>));
  const outputPath = resolve(args.output || defaultOutputPath(values as Record<string, string>));

  const reuseFromDigestRef = readString(values, "REUSE_FROM_DIGEST_REF");
  if (!/@sha256:[0-9a-f]{64}$/.test(reuseFromDigestRef)) {
    throw new Error("REUSE_FROM_DIGEST_REF must be digest-pinned (<repo>@sha256:<64 hex>)");
  }

  const runReport = readJson(runReportPath) as RunReport;
  const decide = runReport.decide ?? {};

  const candidateDecisionsTotal = asCount(decide.decision_count, 0);
  const reusedCount = asCount(decide.reused_count, 0);
  const invalidatedCount = asCount(decide.invalidated_count, 0);
  const unresolvedCount = asCount(decide.unresolved_count, 0);
  const resolvedCount = asCount(decide.resolved_count, 0);

  const newCount = Math.max(0, candidateDecisionsTotal - reusedCount);
  const newHumanRequiredTotal = unresolvedCount;
  const newAiCallsTotal = Math.max(0, resolvedCount - unresolvedCount);

  const summary: DecisionReuseSummary = {
    schema: "underlay.migration.decision_reuse_summary.v1",
    schema_version: 1,
    generated_at: new Date().toISOString(),
    project_name: readString(values, "PROJECT_NAME", "migration"),
    run_scope: "refresh",
    run_date_utc: readString(values, "RUN_DATE_UTC", utcDate()),
    reuse_from_digest_ref: reuseFromDigestRef,
    reuse_stats: {
      reused_count: reusedCount,
      new_count: newCount,
      invalidated_count: invalidatedCount,
      unresolved_count: unresolvedCount,
    },
    sidecar_merge: {
      deduped_count: reusedCount,
      replaced_count: resolvedCount,
      invalidated_count: invalidatedCount,
      corrupted_count: 0,
    },
    suppression_kpi: {
      candidate_decisions_total: candidateDecisionsTotal,
      reused_decisions_total: reusedCount,
      new_ai_calls_total: newAiCallsTotal,
      new_human_required_total: newHumanRequiredTotal,
      invalidated_decisions_total: invalidatedCount,
      ai_call_suppression_ratio: ratio(candidateDecisionsTotal - newAiCallsTotal, candidateDecisionsTotal),
      reuse_ratio: ratio(reusedCount, candidateDecisionsTotal),
      human_queue_ratio: ratio(newHumanRequiredTotal, candidateDecisionsTotal),
    },
    targets: {
      ai_call_suppression_ratio_min: 0.85,
      reuse_ratio_min: 0.8,
      human_queue_ratio_max: 0.05,
    },
  };

  mkdirSync(dirname(outputPath), { recursive: true });
  writeFileSync(outputPath, JSON.stringify(summary, null, 2) + "\n", "utf-8");

  if (args.json) {
    console.log(JSON.stringify(summary, null, 2));
  } else {
    console.log(`decision reuse summary written: ${outputPath}`);
    console.log(`candidate_decisions_total=${summary.suppression_kpi.candidate_decisions_total}`);
    console.log(`ai_call_suppression_ratio=${summary.suppression_kpi.ai_call_suppression_ratio}`);
    console.log(`reuse_ratio=${summary.suppression_kpi.reuse_ratio}`);
    console.log(`human_queue_ratio=${summary.suppression_kpi.human_queue_ratio}`);
  }
}

main();
