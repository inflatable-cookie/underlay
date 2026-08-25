import { describe, expect, it } from "vitest";
import { UnderlayHttpError } from "@inflatable-cookie/underlay/client";
import type { User } from "@inflatable-cookie/underlay/client/auth-types";
import type { SingleResponse } from "@inflatable-cookie/underlay/client/envelopes";
import { isRestoreBlockedResult } from "@inflatable-cookie/underlay/client/restore";
import { appendSuggestionParams } from "@inflatable-cookie/underlay/client/suggestions";
import * as runtimeData from "@inflatable-cookie/underlay/runtime/data";
import {
  appendPaginationParams,
  type PaginatedResponse,
} from "@inflatable-cookie/underlay/runtime";
import { createMockHttpClient } from "@inflatable-cookie/underlay/testing";
import { scanFiles } from "@inflatable-cookie/underlay/tools/guardrails";
import { loadConfig } from "@inflatable-cookie/underlay/tools/guardrails-config";
import { bannedPatterns } from "@inflatable-cookie/underlay/tools/templates/banned-apis";
import { moduleScopeChecks } from "@inflatable-cookie/underlay/tools/templates/sveltekit-ssr";
import {
  checkWorkspaceShape,
  runWorkspaceShapeCli,
} from "@inflatable-cookie/underlay/tools/workspace-shape";
import { getBlockEditor } from "@inflatable-cookie/underlay/nightfire";

describe("package compatibility barrels", () => {
  it("exposes retained client, runtime, and nightfire compatibility subpaths", () => {
    const error = new UnderlayHttpError(500, "failed");
    const single: SingleResponse<User> = {
      data: {
        id: "user-1",
        email: "user@example.com",
        displayName: "User",
        status: "active",
        createdAt: "2026-06-06T00:00:00Z",
        updatedAt: "2026-06-06T00:00:00Z",
      },
    };
    const response: PaginatedResponse<string> = {
      data: ["item"],
      nextCursor: null,
      prevCursor: null,
      hasMore: false,
      total: 1,
    };

    expect(error.status).toBe(500);
    expect(single.data.status).toBe("active");
    expect(isRestoreBlockedResult({})).toBe(false);
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
    expect(typeof checkWorkspaceShape).toBe("function");
    expect(typeof runWorkspaceShapeCli).toBe("function");
    expect(bannedPatterns.length).toBeGreaterThan(0);
    expect(moduleScopeChecks.length).toBeGreaterThan(0);
  });

  it("does not expose suggestion request helpers through runtime data", () => {
    expect("appendSuggestionParams" in runtimeData).toBe(false);
    expect("buildSuggestionParams" in runtimeData).toBe(false);
    expect("formatHintsParam" in runtimeData).toBe(false);
    expect("parseHintsParam" in runtimeData).toBe(false);
  });
});
