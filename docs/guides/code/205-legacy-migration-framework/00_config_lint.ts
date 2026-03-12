import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

import { fail } from "./error_codes.ts";
import { frameworkPath } from "./script_paths.ts";

type JsonSchemaProperty = {
  type?: string;
  minLength?: number;
  pattern?: string;
};

type JsonSchemaRoot = {
  type?: string;
  additionalProperties?: boolean;
  properties?: Record<string, JsonSchemaProperty>;
};

function loadJson(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function main(): void {
  const configPath = resolve(process.env.MIGRATION_CONFIG_FILE?.trim() || "./migration.config.json");
  const schemaPath = resolve(
    process.env.MIGRATION_CONFIG_SCHEMA_FILE?.trim() || frameworkPath("config.schema.json"),
  );

  if (!existsSync(configPath)) {
    fail("MIG_CFG_001", `config file not found: ${configPath}`);
  }
  if (!existsSync(schemaPath)) {
    fail("MIG_CFG_002", `schema file not found: ${schemaPath}`);
  }

  const config = loadJson(configPath);
  const schema = loadJson(schemaPath) as JsonSchemaRoot;

  if (!config || typeof config !== "object" || Array.isArray(config)) {
    fail("MIG_CFG_003", "migration config must be a JSON object");
  }
  if (schema.type !== "object") {
    fail("MIG_CFG_004", "config schema must define type=object");
  }

  const properties = schema.properties ?? {};
  const configRecord = config as Record<string, unknown>;
  const errors: string[] = [];

  for (const [key, value] of Object.entries(configRecord)) {
    const prop = properties[key];
    if (!prop) {
      if (schema.additionalProperties === false) {
        errors.push(`unknown key: ${key}`);
      }
      continue;
    }

    if (prop.type === "string") {
      if (typeof value !== "string") {
        errors.push(`key ${key} must be string`);
        continue;
      }
      if (typeof prop.minLength === "number" && value.length < prop.minLength) {
        errors.push(`key ${key} length must be >= ${prop.minLength}`);
      }
      if (prop.pattern) {
        const re = new RegExp(prop.pattern);
        if (!re.test(value)) {
          errors.push(`key ${key} does not match pattern ${prop.pattern}`);
        }
      }
    }
  }

  if (errors.length > 0) {
    const message = [
      `migration config validation failed (${errors.length} issue${errors.length === 1 ? "" : "s"})`,
      ...errors.map((error) => `- ${error}`),
    ].join("\n");
    fail("MIG_CFG_005", message);
  }

  console.log(`config lint passed: ${configPath}`);
  console.log(`schema: ${schemaPath}`);
}

main();
