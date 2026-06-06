import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import { createOptimisticCounter } from "../../../src/patterns/optimistic";

describe("createOptimisticCounter", () => {
  describe("initialization", () => {
    it("starts with 0 by default", () => {
      const counter = createOptimisticCounter();
      expect(get(counter)).toBe(0);
    });

    it("starts with initial value", () => {
      const counter = createOptimisticCounter(42);
      expect(get(counter)).toBe(42);
    });
  });

  describe("increment", () => {
    it("increments by 1 by default", () => {
      const counter = createOptimisticCounter(5);
      counter.increment();

      expect(get(counter)).toBe(6);
    });

    it("increments by custom amount", () => {
      const counter = createOptimisticCounter(5);
      counter.increment(10);

      expect(get(counter)).toBe(15);
    });

    it("sets pending state", () => {
      const counter = createOptimisticCounter(5);
      counter.increment();

      expect(get(counter.pending)).toBe(true);
    });

    it("rollback restores previous value", () => {
      const counter = createOptimisticCounter(5);
      const { rollback } = counter.increment(10);

      expect(get(counter)).toBe(15);

      rollback();

      expect(get(counter)).toBe(5);
    });

    it("confirm clears pending state", () => {
      const counter = createOptimisticCounter(5);
      const { confirm } = counter.increment(2);
      expect(get(counter.pending)).toBe(true);

      confirm();

      expect(get(counter)).toBe(7);
      expect(get(counter.pending)).toBe(false);
    });
  });

  describe("decrement", () => {
    it("decrements by 1 by default", () => {
      const counter = createOptimisticCounter(5);
      counter.decrement();

      expect(get(counter)).toBe(4);
    });

    it("decrements by custom amount", () => {
      const counter = createOptimisticCounter(15);
      counter.decrement(10);

      expect(get(counter)).toBe(5);
    });

    it("rollback restores previous value", () => {
      const counter = createOptimisticCounter(5);
      const { rollback } = counter.decrement(3);

      expect(get(counter)).toBe(2);

      rollback();

      expect(get(counter)).toBe(5);
    });
  });

  describe("set", () => {
    it("sets value optimistically", () => {
      const counter = createOptimisticCounter(5);
      counter.set(100);

      expect(get(counter)).toBe(100);
    });

    it("confirm with server value updates count", () => {
      const counter = createOptimisticCounter(5);
      const { confirm } = counter.set(10);

      confirm(12);

      expect(get(counter)).toBe(12);
    });

    it("confirm without server value keeps optimistic count", () => {
      const counter = createOptimisticCounter(5);
      const { confirm } = counter.set(10);

      confirm();

      expect(get(counter)).toBe(10);
      expect(get(counter.pending)).toBe(false);
    });

    it("no-op when setting same value", () => {
      const counter = createOptimisticCounter(5);
      counter.set(5);

      expect(get(counter.pending)).toBe(false);
    });

    it("rollback restores previous count after set", () => {
      const counter = createOptimisticCounter(5);
      const { rollback } = counter.set(10);
      expect(get(counter)).toBe(10);

      rollback();

      expect(get(counter)).toBe(5);
      expect(get(counter.pending)).toBe(false);
    });
  });
});
