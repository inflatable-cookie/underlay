import { mkdirSync, writeFileSync, unlinkSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

import {
  loadConfig,
  readOptional,
  readString,
  readStringFromFile,
  validateDigestRef,
} from "./config.ts";
import { fail } from "./error_codes.ts";
import { requireCommand, underlayDevtoolsCommand } from "./tooling.ts";

type Mode = "general" | "reports" | "refresh";

function parseMode(argv: string[]): Mode {
  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];
    if (token === "--mode" && i + 1 < argv.length) {
      return normalizeMode(argv[i + 1]);
    }
    if (token.startsWith("--mode=")) {
      return normalizeMode(token.slice("--mode=".length));
    }
  }
  return "general";
}

function normalizeMode(value: string): Mode {
  if (value === "general" || value === "reports" || value === "refresh") {
    return value;
  }
  fail("MIG_CLI_001", `unsupported mode: ${value} (expected general|reports|refresh)`);
}

function requireTool(name: string): void {
  const result = spawnSync("which", [name], { stdio: "ignore" });
  if (result.status !== 0) {
    fail("MIG_CFG_006", `${name} is required in PATH`);
  }
}

function assertWritablePath(pathValue: string, label: string): void {
  const abs = resolve(pathValue);
  const targetDir = abs.endsWith(".json") || abs.endsWith(".oci") || abs.endsWith(".ndjson")
    ? dirname(abs)
    : abs;
  mkdirSync(targetDir, { recursive: true });
  const probe = resolve(targetDir, `.preflight-${Date.now()}-${Math.random().toString(16).slice(2)}.tmp`);
  writeFileSync(probe, "ok\n", { encoding: "utf-8" });
  unlinkSync(probe);
  console.log(`ok writable ${label}: ${targetDir}`);
}

function main(): void {
  const mode = parseMode(process.argv.slice(2));
  const { filePath, values } = loadConfig();
  const hasFileValues = Object.keys(values).length > 0;

  requireTool("bun");
  try {
    requireCommand(underlayDevtoolsCommand(values));
  } catch (error) {
    fail("MIG_CFG_006", error instanceof Error ? error.message : "underlay-devtools command unavailable");
  }

  console.log(`mode=${mode}`);
  console.log(`config=${filePath}${hasFileValues ? "" : " (not found/empty; env+defaults only)"}`);

  // Baseline keys used by all flows.
  const sourceSystem = readString(values, "SOURCE_SYSTEM", "legacy_site");
  const targetSchemaVersion = readString(values, "TARGET_SCHEMA_VERSION", "schema-v1");
  const mediaDir = readString(values, "MEDIA_DIR", "./legacy-export/media");
  const bundleFile = readString(values, "BUNDLE_FILE", "./dist/migration-bundle.oci");
  const outputDir = readString(values, "OUTPUT_DIR", "./runtime/demo-pass");

  if (sourceSystem.trim().length === 0 || targetSchemaVersion.trim().length === 0) {
    fail("MIG_CFG_007", "SOURCE_SYSTEM and TARGET_SCHEMA_VERSION must be non-empty");
  }

  assertWritablePath(bundleFile, "BUNDLE_FILE");
  assertWritablePath(outputDir, "OUTPUT_DIR");

  if (mode === "reports") {
    const bundleRef = readStringFromFile(values, "BUNDLE_REF", "BUNDLE_REF_FILE");
    validateDigestRef(bundleRef, "BUNDLE_REF");
    console.log("ok digest BUNDLE_REF");
  }

  if (mode === "refresh") {
    const reuseFromDigestRef = readStringFromFile(
      values,
      "REUSE_FROM_DIGEST_REF",
      "REUSE_FROM_DIGEST_REF_FILE",
    );
    validateDigestRef(reuseFromDigestRef, "REUSE_FROM_DIGEST_REF");
    console.log("ok digest REUSE_FROM_DIGEST_REF");
  }

  if (mode === "reports" || mode === "refresh") {
    const appMigrationRunnerCmd = readOptional(values, "APP_MIGRATION_RUNNER_CMD");
    if (appMigrationRunnerCmd.trim().length > 0) {
      console.log("ok APP_MIGRATION_RUNNER_CMD configured");
    } else {
      console.warn(
        "warn APP_MIGRATION_RUNNER_CMD not configured; report and refresh flows expect the consuming app runner to write run-report.json, decision_index.json, and decision_journal.ndjson",
      );
    }
  }

  const governancePolicyFile = readOptional(values, "GOVERNANCE_POLICY_FILE");
  if (governancePolicyFile.trim().length > 0) {
    if (existsSync(governancePolicyFile)) {
      console.log(`ok found GOVERNANCE_POLICY_FILE: ${governancePolicyFile}`);
    } else {
      console.warn(`warn missing GOVERNANCE_POLICY_FILE: ${governancePolicyFile}`);
    }
  }

  if (!existsSync(mediaDir)) {
    console.warn(`warn MEDIA_DIR not found (allowed for data-only migrations): ${mediaDir}`);
  }

  console.log("preflight passed");
}

main();
