import { describe, expect, it } from "vitest";
import { UnderlayHttpError } from "@decodelabs/underlay/client";
import {
  appendPaginationParams,
  type PaginatedResponse,
} from "@decodelabs/underlay/runtime";
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
});
