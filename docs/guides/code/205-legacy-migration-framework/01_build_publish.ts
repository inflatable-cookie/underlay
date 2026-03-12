import { mkdirSync, writeFileSync } from "node:fs";
import { basename, dirname, extname, join } from "node:path";

import { loadConfig, readOptional, readString } from "./config.ts";
import { repositoryFromTaggedRef } from "./runner_support.ts";
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

  const sourceSystem = readString(values, "SOURCE_SYSTEM", "legacy_site");
  const targetSchemaVersion = readString(values, "TARGET_SCHEMA_VERSION", "schema-v1");
  const mediaDir = readString(values, "MEDIA_DIR", "./legacy-export/media");
  const bundleFile = readString(values, "BUNDLE_FILE", "./dist/migration-bundle.oci");
  const bundleRefFile = readOptional(values, "BUNDLE_REF_FILE") || defaultBundleRefFile(bundleFile);
  const ociRefTag = readString(
    values,
    "OCI_REF_TAG",
    `registry.example.com/underlay/site-migration:demo-${new Date()
      .toISOString()
      .replace(/[-:]/g, "")
      .replace(/\.\d+Z$/, "Z")}`,
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
  const digestRef = `${repository}@${digest}`;
  mkdirSync(dirname(bundleRefFile), { recursive: true });
  writeFileSync(bundleRefFile, `${digestRef}\n`, "utf-8");

  console.log("\nDIGEST_REF=" + digestRef);
  console.log("BUNDLE_DIGEST=" + digest);
  console.log("BUNDLE_REF_FILE=" + bundleRefFile);
}

main();
