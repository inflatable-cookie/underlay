import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";

import { loadConfig, readString, validateDigestRef } from "./config.ts";

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

function requireTool(name: string): void {
  const result = spawnSync("which", [name], { stdio: "ignore" });
  if (result.status !== 0) {
    throw new Error(`${name} is required in PATH`);
  }
}

function main(): void {
  requireTool("underlay-devtools");

  const { filePath, values } = loadConfig();
  console.log(`using config file: ${filePath}${Object.keys(values).length === 0 ? " (not found/empty; env only)" : ""}`);

  const bundleRef = readString(values, "BUNDLE_REF");
  validateDigestRef(bundleRef, "BUNDLE_REF");

  const outputDir = readString(values, "OUTPUT_DIR", "./runtime/demo-pass");
  const runReport = readString(values, "RUN_REPORT", `${outputDir}/run-report.json`);
  const governancePolicyFile = readString(
    values,
    "GOVERNANCE_POLICY_FILE",
    "./runtime/governance-policy.json",
  );

  runCommand("underlay-devtools", [
    "migration",
    "run",
    "--bundle",
    bundleRef,
    "--output",
    outputDir,
  ]);

  if (!existsSync(runReport)) {
    throw new Error(
      [
        `expected run report not found at: ${runReport}`,
        "the bundle run command prepares deterministic replay input;",
        "run your app migration orchestrator to produce run-report.json, then retry.",
      ].join("\n"),
    );
  }

  runCommand("underlay-devtools", [
    "migration",
    "report",
    "governance",
    "--input",
    runReport,
    "--limit",
    "20",
  ]);
  runCommand("underlay-devtools", ["migration", "report", "integrity", "--input", runReport]);
  runCommand("underlay-devtools", ["migration", "report", "recovery", "--input", runReport]);
  runCommand("underlay-devtools", [
    "migration",
    "report",
    "verify",
    "--input",
    runReport,
    "--output-dir",
    outputDir,
  ]);
  runCommand("underlay-devtools", [
    "migration",
    "report",
    "audit",
    "--input",
    outputDir,
    "--output-dir",
    outputDir,
  ]);

  if (existsSync(governancePolicyFile)) {
    runCommand("underlay-devtools", [
      "migration",
      "report",
      "policy",
      "--input",
      governancePolicyFile,
    ]);
  } else {
    console.warn(
      `governance policy file not found (skipping policy report): ${governancePolicyFile}`,
    );
  }
}

main();
