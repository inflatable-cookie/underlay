import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

// Repo root: this file is ts/tests/client/, so up three levels.
const repoRoot = path.resolve(fileURLToPath(new URL(".", import.meta.url)), "../../..");
const yamlPath = path.join(repoRoot, "contracts/openapi/underlay.openapi.yaml");
const tsPath = path.join(repoRoot, "ts/src/client/envelopes.ts");

/**
 * Contract-sync drift guard. The response-envelope shapes are declared in three
 * places (Rust `dto.rs`, TS `envelopes.ts`, the OpenAPI YAML). There is no
 * codegen, so this test asserts the two machine-readable surfaces we can load
 * here — the TS interfaces and the OpenAPI schemas — declare the same set of
 * envelopes with the same required fields. It fails loudly the moment an
 * envelope is added or renamed on one surface but not the other (the exact
 * drift that let `PagedListResponse` land in TS but not the YAML).
 */

// Envelope interfaces are the request/response wrappers — not the `Uuid` alias
// or the leaf `ErrorBody` payload's own scalar fields.
function tsEnvelopeNames(source: string): Set<string> {
  const names = new Set<string>();
  const re = /export interface (\w+)/g;
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    names.add(match[1]);
  }
  return names;
}

function yamlSchemaNames(source: string): Set<string> {
  const names = new Set<string>();
  // Names are the keys two levels under `components.schemas:` — 4-space indent.
  const re = /^ {4}(\w+):$/gm;
  let match: RegExpExecArray | null;
  while ((match = re.exec(source))) {
    names.add(match[1]);
  }
  return names;
}

describe("envelope contract drift", () => {
  const tsSource = readFileSync(tsPath, "utf8");
  const yamlSource = readFileSync(yamlPath, "utf8");

  it("declares the same envelope set in TS and the OpenAPI YAML", () => {
    const ts = tsEnvelopeNames(tsSource);
    const yaml = yamlSchemaNames(yamlSource);
    expect([...ts].sort()).toEqual([...yaml].sort());
  });

  it("includes PagedListResponse on both surfaces", () => {
    expect(tsSource).toContain("interface PagedListResponse");
    expect(yamlSource).toContain("PagedListResponse:");
  });

  it("keeps ErrorBody required fields aligned", () => {
    // TS: code + message required (fieldErrors optional). YAML: required list.
    expect(tsSource).toMatch(/code:\s*string/);
    expect(tsSource).toMatch(/message:\s*string/);
    expect(yamlSource).toMatch(/ErrorBody:[\s\S]*?required: \[code, message\]/);
  });
});
