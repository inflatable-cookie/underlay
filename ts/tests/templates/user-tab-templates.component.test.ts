// @vitest-environment jsdom
import { beforeEach, describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";

import { configureAuth } from "../../src/patterns/auth";
import UserTabsHarness from "../fixtures/UserTabsHarness.svelte";

describe("templates/UserTabs", () => {
  beforeEach(() => {
    Object.defineProperty(window.navigator, "clipboard", {
      configurable: true,
      value: { writeText: () => Promise.resolve() }
    });
    configureAuth({
      getToken: () => "token-1",
      onRefresh: async () => "token-1",
      getAuthLoading: () => false,
      getCurrentUser: () => ({ id: "user-1" })
    });
  });

  it("lazy-loads sessions only after the tab activates", async () => {
    render(UserTabsHarness);

    expect(screen.queryByText("203.0.113.10")).toBeNull();
    expect(screen.getByTestId("session-requests").textContent).toBe("[]");

    await fireEvent.click(screen.getByTestId("activate-sessions"));

    expect(await screen.findByText("203.0.113.10")).toBeTruthy();
    await waitFor(() => {
      expect(screen.getByTestId("session-requests").textContent).toBe('["1:10"]');
      expect(screen.getByTestId("session-count").textContent).toBe("2");
    });
  });

  it("confirms revoke through the alert dialog and refetches", async () => {
    render(UserTabsHarness);
    await fireEvent.click(screen.getByTestId("activate-sessions"));

    expect(await screen.findByText("203.0.113.10")).toBeTruthy();

    // The revoked session only offers the copy action.
    await fireEvent.click(screen.getAllByRole("button", { name: /^Copy Session ID/ })[0]);
    expect(screen.queryByRole("menuitem", { name: "Revoke" })).toBeNull();

    // The active session offers Revoke behind the actions menu.
    await fireEvent.click(screen.getByRole("button", { name: /^Actions for/ }));
    await fireEvent.click(await screen.findByRole("menuitem", { name: "Revoke" }));

    expect(await screen.findByText("Revoke session")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Revoke" }));

    await waitFor(() => {
      expect(screen.getByTestId("last-revoked").textContent).toBe("session-1");
      expect(screen.getByTestId("session-requests").textContent).toBe('["1:10","1:10"]');
    });

    // After the refetch the revoked session no longer offers Revoke.
    await waitFor(() => {
      expect(screen.queryByRole("button", { name: /^Actions for/ })).toBeNull();
    });
  });

  it("lazy-loads activity and hides resource copy when resourceId is null", async () => {
    render(UserTabsHarness);

    expect(screen.getByTestId("activity-requests").textContent).toBe("[]");

    await fireEvent.click(screen.getByTestId("activate-activity"));

    expect(await screen.findByText("admin@example.com")).toBeTruthy();
    await waitFor(() => {
      expect(screen.getByTestId("activity-requests").textContent).toBe('["1:10"]');
      expect(screen.getByTestId("activity-count").textContent).toBe("45");
    });

    // Fixed-limit mode never renders pagination even when total exceeds the limit.
    expect(screen.queryByRole("button", { name: "Next page" })).toBeNull();

    // Row without resourceId only offers the activity copy action.
    const copyOnlyButtons = screen.getAllByRole("button", { name: /^Copy Activity ID/ });
    expect(copyOnlyButtons.length).toBeGreaterThan(0);

    // Row with resourceId offers both copy actions behind the menu.
    await fireEvent.click(screen.getAllByRole("button", { name: /^Actions for/ })[0]);
    expect(await screen.findByRole("menuitem", { name: "Copy Activity ID" })).toBeTruthy();
    expect(screen.getByRole("menuitem", { name: "Copy Resource ID" })).toBeTruthy();
  });

  it("paginates activity through the loader when paginated", async () => {
    render(UserTabsHarness, { activityPaginated: true });
    await fireEvent.click(screen.getByTestId("activate-activity"));

    expect(await screen.findByText("admin@example.com")).toBeTruthy();
    await waitFor(() => {
      expect(screen.getByTestId("activity-requests").textContent).toBe('["1:20"]');
    });

    await fireEvent.click(screen.getByRole("button", { name: "Next page" }));

    await waitFor(() => {
      expect(screen.getByTestId("activity-requests").textContent).toBe('["1:20","2:20"]');
    });
  });
});
