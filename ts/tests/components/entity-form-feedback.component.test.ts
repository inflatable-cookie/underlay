// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { tick } from "svelte";

import EntityFormPageHarness from "../fixtures/EntityFormPageHarness.svelte";
import LoginGoogleTab from "../../src/patterns/auth-workflows/LoginGoogleTab.svelte";

describe("templates/EntityFormPage submit feedback", () => {
  it("keeps submit-result feedback after the prop-sync effect runs", async () => {
    const onSubmit = vi.fn(async () => ({
      success: false,
      error: "Name is required",
      fieldErrors: { name: "Required" },
    }));

    const { container } = render(EntityFormPageHarness, { onSubmit });

    const form = container.querySelector("form");
    expect(form).not.toBeNull();

    form!.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));

    await vi.waitFor(() => {
      expect(onSubmit).toHaveBeenCalledTimes(1);
    });

    // Let any pending effects run; the feedback must survive them.
    await tick();
    await tick();

    expect(container.textContent).toContain("Required");
  });

  it("ignores a second submit while one is in flight", async () => {
    let resolveSubmit: (value: { success: boolean }) => void = () => {};
    const onSubmit = vi.fn(
      () =>
        new Promise<{ success: boolean }>((resolve) => {
          resolveSubmit = resolve;
        })
    );

    const { container } = render(EntityFormPageHarness, { onSubmit });

    const form = container.querySelector("form")!;
    form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
    form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));

    await tick();
    expect(onSubmit).toHaveBeenCalledTimes(1);

    resolveSubmit({ success: true });
  });
});

describe("patterns/auth-workflows LoginGoogleTab", () => {
  it("invokes the click handler on the Google button", async () => {
    const onGoogleClick = vi.fn();

    render(LoginGoogleTab, {
      googleHint: "Use your Google account",
      loading: false,
      onGoogleLogin: async () => {},
      onGoogleClick,
    });

    const button = await screen.findByRole("button", { name: /continue with google/i });
    button.click();

    expect(onGoogleClick).toHaveBeenCalledTimes(1);
  });
});
