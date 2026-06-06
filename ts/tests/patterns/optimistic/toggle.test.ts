import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import { createOptimisticToggle } from "../../../src/patterns/optimistic";

describe("createOptimisticToggle", () => {
  describe("initialization", () => {
    it("starts with false by default", () => {
      const toggle = createOptimisticToggle();
      expect(get(toggle)).toBe(false);
    });

    it("starts with initial value", () => {
      const toggle = createOptimisticToggle(true);
      expect(get(toggle)).toBe(true);
    });

    it("starts with no pending operation", () => {
      const toggle = createOptimisticToggle();
      expect(get(toggle.pending)).toBe(false);
    });
  });

  describe("toggle", () => {
    it("toggles value optimistically", () => {
      const toggle = createOptimisticToggle(false);
      toggle.toggle();

      expect(get(toggle)).toBe(true);
    });

    it("sets pending state", () => {
      const toggle = createOptimisticToggle(false);
      toggle.toggle();

      expect(get(toggle.pending)).toBe(true);
    });

    it("confirm clears pending state", () => {
      const toggle = createOptimisticToggle(false);
      const { confirm } = toggle.toggle();

      confirm();

      expect(get(toggle)).toBe(true);
      expect(get(toggle.pending)).toBe(false);
    });

    it("rollback restores previous value", () => {
      const toggle = createOptimisticToggle(false);
      const { rollback } = toggle.toggle();

      expect(get(toggle)).toBe(true);

      rollback();

      expect(get(toggle)).toBe(false);
      expect(get(toggle.pending)).toBe(false);
    });
  });

  describe("set", () => {
    it("sets value optimistically", () => {
      const toggle = createOptimisticToggle(false);
      toggle.set(true);

      expect(get(toggle)).toBe(true);
    });

    it("no-op when setting same value", () => {
      const toggle = createOptimisticToggle(true);
      const { rollback } = toggle.set(true);

      expect(get(toggle.pending)).toBe(false);

      rollback();
      expect(get(toggle)).toBe(true);
    });

    it("rollback restores previous value", () => {
      const toggle = createOptimisticToggle(false);
      const { rollback } = toggle.set(true);

      rollback();

      expect(get(toggle)).toBe(false);
    });

    it("confirm clears pending state after set", () => {
      const toggle = createOptimisticToggle(false);
      const { confirm } = toggle.set(true);
      expect(get(toggle.pending)).toBe(true);

      confirm();

      expect(get(toggle)).toBe(true);
      expect(get(toggle.pending)).toBe(false);
    });
  });
});
