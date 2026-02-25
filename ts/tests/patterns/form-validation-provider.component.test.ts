// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import FormValidationProviderHarness from "../fixtures/FormValidationProviderHarness.svelte";

describe("components/FormValidationProvider.svelte", () => {
	it("tracks required-value validity and unregister flow", async () => {
		render(FormValidationProviderHarness);

		expect(screen.getByTestId("provider-validity").textContent).toBe("true");
		await fireEvent.click(screen.getByTestId("register-invalid"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("false");

		await fireEvent.click(screen.getByTestId("register-valid"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("true");

		await fireEvent.click(screen.getByTestId("unregister"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("true");
	});

	it("marks form invalid while validating and valid once validation succeeds", async () => {
		render(FormValidationProviderHarness);

		await fireEvent.click(screen.getByTestId("register-valid"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("true");

		await fireEvent.click(screen.getByTestId("set-validating"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("false");

		await fireEvent.click(screen.getByTestId("set-valid"));
		expect(screen.getByTestId("provider-validity").textContent).toBe("true");
	});
});
