import { describe, it, expect } from "vitest";
import { MemoryTokenStore } from "../../../src/client/http";

describe("MemoryTokenStore", () => {
  it("should store and retrieve access token", () => {
    const store = new MemoryTokenStore();
    expect(store.getAccessToken()).toBeNull();

    store.setAccessToken("test-token");
    expect(store.getAccessToken()).toBe("test-token");
  });

  it("should store and retrieve refresh token", () => {
    const store = new MemoryTokenStore();
    expect(store.getRefreshToken()).toBeNull();

    store.setRefreshToken("refresh-token");
    expect(store.getRefreshToken()).toBe("refresh-token");
  });

  it("should clear all tokens", () => {
    const store = new MemoryTokenStore();
    store.setAccessToken("access");
    store.setRefreshToken("refresh");

    store.clear();

    expect(store.getAccessToken()).toBeNull();
    expect(store.getRefreshToken()).toBeNull();
  });
});
