import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { parse, type TomlTable } from "smol-toml";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..", "..");

const cargo = parse(fs.readFileSync(path.join(repoRoot, "Cargo.toml"), "utf8")) as TomlTable;
const workspace = cargo.workspace as TomlTable | undefined;
const workspacePackage = workspace?.package as TomlTable | undefined;
const cargoVersion = workspacePackage?.version;

const packageManifest = JSON.parse(
	fs.readFileSync(path.join(repoRoot, "package.json"), "utf8"),
) as { version?: unknown };
const packageVersion = packageManifest.version;

const openapi = Bun.YAML.parse(
	fs.readFileSync(path.join(repoRoot, "contracts/openapi/underlay.openapi.yaml"), "utf8"),
);
const openapiVersion: unknown = openapi?.info?.version;

if (
	typeof cargoVersion !== "string" ||
	typeof packageVersion !== "string" ||
	typeof openapiVersion !== "string"
) {
	console.error(
		"Release version sync check failed: Cargo.toml, package.json, or OpenAPI info has no string version",
	);
	process.exit(1);
}

if (cargoVersion !== packageVersion || cargoVersion !== openapiVersion) {
	console.error(
		`Release version sync check failed: Cargo.toml is ${cargoVersion}, package.json is ${packageVersion}, OpenAPI is ${openapiVersion}`,
	);
	process.exit(1);
}

console.log(`Release versions are synchronized at ${cargoVersion}.`);
