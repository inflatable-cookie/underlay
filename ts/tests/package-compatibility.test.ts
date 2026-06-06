import { describe, expect, it } from "vitest";
import { UnderlayHttpError } from "@decodelabs/underlay/client";
import { appendSuggestionParams } from "@decodelabs/underlay/client/suggestions";
import {
  appendPaginationParams,
  type PaginatedResponse,
} from "@decodelabs/underlay/runtime";
import { createMockHttpClient } from "@decodelabs/underlay/testing";
import { scanFiles } from "@decodelabs/underlay/tools/guardrails";
import { loadConfig } from "@decodelabs/underlay/tools/guardrails-config";
import { bannedPatterns } from "@decodelabs/underlay/tools/templates/banned-apis";
import { moduleScopeChecks } from "@decodelabs/underlay/tools/templates/sveltekit-ssr";
import { getBlockEditor } from "@decodelabs/underlay/nightfire";

describe("package compatibility barrels", () => {
  it("exposes retained client, runtime, and nightfire compatibility subpaths", () => {
    const error = new UnderlayHttpError(500, "failed");
    const response: PaginatedResponse<string> = {
      data: ["item"],
      nextCursor: null,
      prevCursor: null,
      hasMore: false,
      total: 1,
    };

    expect(error.status).toBe(500);
    expect(appendPaginationParams("/items", { limit: 10 })).toBe(
      "/items?limit=10",
    );
    expect(response.data).toEqual(["item"]);
    expect(getBlockEditor("missing", "missing")).toBeNull();
  });

  it("exposes retained runtime, testing, and tools public subpaths", () => {
    expect(typeof appendSuggestionParams).toBe("function");
    expect(typeof createMockHttpClient).toBe("function");
    expect(typeof scanFiles).toBe("function");
    expect(typeof loadConfig).toBe("function");
    expect(bannedPatterns.length).toBeGreaterThan(0);
    expect(moduleScopeChecks.length).toBeGreaterThan(0);
  });
});
