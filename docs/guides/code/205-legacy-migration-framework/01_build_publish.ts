import { spawnSync } from "node:child_process";
import { mkdirSync } from "node:fs";
import { dirname } from "node:path";

import { loadConfig, readString } from "./config.ts";

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

  const sourceSystem = readString(values, "SOURCE_SYSTEM", "legacy_site");
  const targetSchemaVersion = readString(values, "TARGET_SCHEMA_VERSION", "schema-v1");
  const mediaDir = readString(values, "MEDIA_DIR", "./legacy-export/media");
  const bundleFile = readString(values, "BUNDLE_FILE", "./dist/migration-bundle.oci");
  const ociRefTag = readString(
    values,
    "OCI_REF_TAG",
    `registry.example.com/underlay/site-migration:demo-${new Date()
      .toISOString()
      .replace(/[-:]/g, "")
      .replace(/\.\d+Z$/, "Z")}`,
  );

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
  const digestRef = `${repository}@${digest}`;

  console.log("\nDIGEST_REF=" + digestRef);
  console.log("BUNDLE_DIGEST=" + digest);
}

main();
