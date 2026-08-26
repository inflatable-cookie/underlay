import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

// Repo root: this file is ts/tests/client/, so up three levels.
const repoRoot = path.resolve(fileURLToPath(new URL(".", import.meta.url)), "../../..");
const yamlPath = path.join(repoRoot, "contracts/openapi/underlay.openapi.yaml");
const tsPath = path.join(repoRoot, "ts/src/client/envelopes.ts");
const rustPath = path.join(repoRoot, "rust/crates/underlay-http/src/page_list.rs");

/**
 * Contract-sync drift guard. Response envelopes are declared across Rust, TS,
 * and OpenAPI without a shared code generator. The schema names stay aligned,
 * while the page-list field guard preserves the intentional raw `has_more` to
 * public-client `hasMore` normalization.
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
  const rustSource = readFileSync(rustPath, "utf8");

  it("declares the same envelope set in TS and the OpenAPI YAML", () => {
    const ts = tsEnvelopeNames(tsSource);
    const yaml = yamlSchemaNames(yamlSource);
    expect([...ts].sort()).toEqual([...yaml].sort());
  });

  it("includes PagedListResponse on both surfaces", () => {
    expect(tsSource).toContain("interface PagedListResponse");
    expect(yamlSource).toContain("PagedListResponse:");
  });

  it("keeps raw page-list casing separate from the TS client boundary", () => {
    const rustPageList = rustSource.match(/pub struct PageList<T> \{[\s\S]*?\n\}/)?.[0];
    const yamlPageList = yamlSource
      .split(/^    PagedListResponse:\s*$/m)[1]
      ?.split(/^    \w+:\s*$/m)[0];
    const tsPageList = tsSource.match(
      /export interface PagedListResponse<T> \{[\s\S]*?\n\}/
    )?.[0];

    expect(rustPageList).toContain("pub has_more: bool");
    expect(rustPageList).not.toContain("hasMore");
    expect(yamlPageList).toContain("required: [data, total, has_more]");
    expect(yamlPageList).toContain("has_more:");
    expect(yamlPageList).not.toContain("hasMore");
    expect(tsPageList).toContain("hasMore: boolean");
    expect(tsPageList).not.toContain("has_more");
  });

  it("keeps ErrorBody required fields aligned", () => {
    // TS: code + message required (fieldErrors optional). YAML: required list.
    expect(tsSource).toMatch(/code:\s*string/);
    expect(tsSource).toMatch(/message:\s*string/);
    expect(yamlSource).toMatch(/ErrorBody:[\s\S]*?required: \[code, message\]/);
  });
});
