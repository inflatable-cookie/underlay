import { describe, expect, it, vi } from "vitest";
import { registerRequestSchema } from "../../src/validation";

type ValidatedFormModule = typeof import("../../src/patterns/validated-form.svelte");

async function loadValidatedFormModule(): Promise<ValidatedFormModule> {
  vi.resetModules();
  (globalThis as any).$state = <T>(initial: T) => initial;
  return await import("../../src/patterns/validated-form.svelte");
}

describe("patterns/useValidatedForm", () => {
  it("collects schema errors and blocks submit until the values are valid", async () => {
    const mod = await loadValidatedFormModule();
    const onSubmit = vi.fn(async () => undefined);
    const form = mod.useValidatedForm({
      schema: registerRequestSchema,
      initialValues: {
        email: "not-an-email",
        password: "short",
        displayName: "",
      },
      onSubmit,
    });

    expect(form.validate()).toBe(false);
    expect(form.errors.email).toContain("Invalid email address");
    expect(form.errors.password).toContain("at least 12 characters");
    expect(await form.submit()).toBe(false);
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("submits parsed values and supports validate-on-change updates", async () => {
    const mod = await loadValidatedFormModule();
    const onSubmit = vi.fn(async () => undefined);
    const form = mod.useValidatedForm({
      schema: registerRequestSchema,
      initialValues: {
        email: "person@example.com",
        password: "password-1234",
        displayName: " Person ",
      },
      onSubmit,
      validateOnChange: true,
    });

    form.setField("email", "bad-email");
    expect(form.errors.email).toContain("Invalid email address");

    form.setField("email", "person@example.com");
    expect(form.errors.email).toBeUndefined();

    expect(await form.submit()).toBe(true);
    expect(onSubmit).toHaveBeenCalledWith({
      email: "person@example.com",
      password: "password-1234",
      displayName: "Person",
    });
  });
});
