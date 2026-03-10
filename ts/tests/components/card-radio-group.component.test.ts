/// <reference types="vitest" />
// @vitest-environment jsdom

import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import Search from "lucide-svelte/icons/search";
import Copy from "lucide-svelte/icons/copy";
import CardRadioGroup from "../../src/components/CardRadioGroup.svelte";

afterEach(() => {
  cleanup();
});

describe("components/CardRadioGroup.svelte", () => {
  it("renders card options with descriptions and submits the selected hidden value", () => {
    const { container } = render(CardRadioGroup, {
      name: "questionPolicy",
      value: "lookup_or_duplicate",
      options: [
        {
          value: "lookup_or_duplicate",
          title: "Lookup or duplicate",
          description: "Reuse equivalent target question when available, else duplicate.",
          icon: Search,
        },
        {
          value: "duplicate_in_target_module",
          title: "Duplicate in target module",
          description: "Always duplicate linked questions for the target module.",
          icon: Copy,
        },
      ],
      ariaLabel: "Exam question handling",
    });

    expect(screen.getByRole("radiogroup", { name: "Exam question handling" })).toBeTruthy();
    expect(screen.getByText("Lookup or duplicate")).toBeTruthy();
    expect(screen.getByText("Reuse equivalent target question when available, else duplicate.")).toBeTruthy();
    expect((container.querySelector('input[name="questionPolicy"]') as HTMLInputElement)?.value).toBe("lookup_or_duplicate");
  });

  it("renders multiple icons for combined options", () => {
    const { container } = render(CardRadioGroup, {
      value: "lookup_or_duplicate",
      options: [
        {
          value: "lookup_or_duplicate",
          title: "Lookup or duplicate",
          icon: [Search, Copy],
        },
      ],
      ariaLabel: "Exam question handling",
    });

    expect(
      container.querySelectorAll(".underlay-card-radio-group__icon-tile").length,
    ).toBe(2);
  });

  it("applies the configured intent tone to the selected card", () => {
    const { container } = render(CardRadioGroup, {
      value: "lookup_or_remove",
      options: [
        {
          value: "lookup_or_remove",
          title: "Lookup or remove",
          icon: [Search, Copy],
          tone: "warning",
        },
      ],
      ariaLabel: "Exam question handling",
    });

    expect(
      container.querySelector(".underlay-card-radio-group__option--tone-warning"),
    ).toBeTruthy();
  });

  it("updates selection on click and arrow-key navigation", async () => {
    let value = "lookup_or_duplicate";

    const view = render(CardRadioGroup, {
      value,
      options: [
        {
          value: "lookup_or_duplicate",
          title: "Lookup or duplicate",
          icon: Search,
        },
        {
          value: "duplicate_in_target_module",
          title: "Duplicate in target module",
          icon: Copy,
        },
      ],
      onchange: (next: string) => {
        value = next;
      },
      ariaLabel: "Exam question handling",
    });

    const radios = screen.getAllByRole("radio");
    await fireEvent.click(radios[1]);
    expect(value).toBe("duplicate_in_target_module");

    await view.rerender({
      value,
      options: [
        {
          value: "lookup_or_duplicate",
          title: "Lookup or duplicate",
          icon: Search,
        },
        {
          value: "duplicate_in_target_module",
          title: "Duplicate in target module",
          icon: Copy,
        },
      ],
      onchange: (next: string) => {
        value = next;
      },
      ariaLabel: "Exam question handling",
    });

    await fireEvent.keyDown(radios[1], { key: "ArrowLeft", code: "ArrowLeft" });
    expect(value).toBe("lookup_or_duplicate");
  });
});
