// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import type { PasswordRequirementsPolicy } from "@inflatable-cookie/poodle-svelte";

import PasswordRequirements from "../../src/patterns/auth-workflows/PasswordRequirements.svelte";
import PasswordRequirementsParentHarness from "./PasswordRequirementsParentHarness.svelte";

const policy: PasswordRequirementsPolicy = {
	minLength: 12,
	requireMixedCase: true,
	requireDigit: true,
	requireSpecial: true,
	minStrengthScore: 3,
	description: "Use at least 12 characters with mixed case, digits, and symbols.",
};

describe("patterns/auth-workflows/PasswordRequirements", () => {
	it("fetches once on mount and does not refetch on parent password keystrokes", async () => {
		const onFetch = vi.fn(async () => policy);
		render(PasswordRequirementsParentHarness, { onFetch });

		await waitFor(() => {
			expect(onFetch).toHaveBeenCalledTimes(1);
		});

		const input = screen.getByLabelText("Password") as HTMLInputElement;
		await fireEvent.input(input, { target: { value: "a" } });
		await fireEvent.input(input, { target: { value: "ab" } });
		await fireEvent.input(input, { target: { value: "Ab1!longerpass" } });

		await new Promise((resolve) => setTimeout(resolve, 20));
		expect(onFetch).toHaveBeenCalledTimes(1);
	});

	it("refetches when reloadKey explicitly changes", async () => {
		const onFetch = vi.fn(async () => policy);
		const { rerender } = render(PasswordRequirementsParentHarness, {
			onFetch,
			reloadKey: "v1",
		});

		await waitFor(() => {
			expect(onFetch).toHaveBeenCalledTimes(1);
		});

		await rerender({ onFetch, reloadKey: "v2" });
		await waitFor(() => {
			expect(onFetch).toHaveBeenCalledTimes(2);
		});
	});

	it("renders the error message when fetching requirements fails", async () => {
		const fetchRequirements = vi.fn(async () => {
			throw new Error("Policy service unavailable");
		});
		render(PasswordRequirements, {
			password: "",
			fetchRequirements,
		});

		const alert = await screen.findByRole("alert");
		expect(alert.textContent).toContain("Policy service unavailable");
	});
});
