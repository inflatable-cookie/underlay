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

if (typeof cargoVersion !== "string" || typeof packageVersion !== "string") {
	console.error("Release version sync check failed: Cargo.toml or package.json has no string version");
	process.exit(1);
}

if (cargoVersion !== packageVersion) {
	console.error(
		`Release version sync check failed: Cargo.toml is ${cargoVersion}, package.json is ${packageVersion}`,
	);
	process.exit(1);
}

console.log(`Release versions are synchronized at ${cargoVersion}.`);
