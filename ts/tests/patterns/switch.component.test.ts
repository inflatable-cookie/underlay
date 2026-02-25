// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import SwitchHarness from "../fixtures/SwitchHarness.svelte";

describe("components/Switch.svelte", () => {
  it("toggles checked state and keeps hidden checkbox in sync", async () => {
    const view = render(SwitchHarness, {
      name: "published",
      initialChecked: false,
      leftLabel: "Draft",
      rightLabel: "Live"
    });

    const toggle = screen.getByRole("switch", { name: "Draft" });
    const hidden = view.container.querySelector('input[type="checkbox"][name="published"]') as HTMLInputElement;
    expect(toggle.getAttribute("aria-checked")).toBe("false");
    expect(hidden.checked).toBe(false);

    await fireEvent.click(toggle);
    expect(screen.getByRole("switch", { name: "Live" }).getAttribute("aria-checked")).toBe("true");
    expect(screen.getByTestId("switch-checked").textContent).toBe("true");
    expect(hidden.checked).toBe(true);
  });

  it("applies legacy and state variant classes and respects disabled mode", async () => {
    const view = render(SwitchHarness, {
      initialChecked: false,
      variant: "danger-off",
      leftVariant: "success",
      rightVariant: "danger",
      disabled: true
    });

    const root = view.container.querySelector(".underlay-switch") as HTMLButtonElement;
    expect(root.classList.contains("underlay-switch--danger-off")).toBe(true);
    expect(root.classList.contains("underlay-switch--left-success")).toBe(true);
    expect(root.classList.contains("underlay-switch--right-danger")).toBe(true);
    expect(root.disabled).toBe(true);

    await fireEvent.click(root);
    expect(screen.getByTestId("switch-checked").textContent).toBe("false");
  });
});
