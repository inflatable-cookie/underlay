import * as z from "zod";
import { describe, expect, it, vi } from "vitest";

type ValidatedFormModule = typeof import("../../src/patterns/validated-form.svelte.ts");

const registerRequestSchema = z.object({
  email: z.string().trim().email(),
  password: z.string().min(12, "String must contain at least 12 character(s)"),
  displayName: z.string().trim().min(1, "String must contain at least 1 character(s)")
});

async function loadValidatedFormModule(): Promise<ValidatedFormModule> {
  vi.resetModules();
  (globalThis as any).$state = <T>(initial: T) => initial;
  return await import("../../src/patterns/validated-form.svelte.ts");
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
    expect(form.errors.email).toContain("Invalid email");
    expect(form.errors.password).toContain("at least 12 character(s)");
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
    expect(form.errors.email).toContain("Invalid email");

    form.setField("email", "person@example.com");
    expect(form.errors.email).toBeUndefined();

    expect(await form.submit()).toBe(true);
    expect(onSubmit).toHaveBeenCalledWith({
      email: "person@example.com",
      password: "password-1234",
      displayName: "Person",
    });
  });

  it("ignores a second submit while one is already in flight", async () => {
    const mod = await loadValidatedFormModule();
    let release: () => void = () => {};
    const onSubmit = vi.fn(
      () => new Promise<void>((resolve) => {
        release = resolve;
      }),
    );
    const form = mod.useValidatedForm({
      schema: registerRequestSchema,
      initialValues: {
        email: "person@example.com",
        password: "password-1234",
        displayName: "Person",
      },
      onSubmit,
    });

    const first = form.submit();
    expect(form.isSubmitting).toBe(true);
    expect(await form.submit()).toBe(false);
    expect(onSubmit).toHaveBeenCalledTimes(1);

    release();
    expect(await first).toBe(true);
    expect(form.isSubmitting).toBe(false);
  });

  it("clones plain and nested initial values so edits never leak back into the caller", async () => {
    const mod = await loadValidatedFormModule();
    const initialValues = {
      email: "person@example.com",
      password: "password-1234",
      displayName: "Person",
      tags: ["a"],
    };
    const schema = registerRequestSchema.extend({ tags: z.array(z.string()) });
    const form = mod.useValidatedForm({
      schema,
      initialValues,
      onSubmit: async () => undefined,
    });

    (form.values.tags as string[]).push("b");
    expect(initialValues.tags).toEqual(["a"]);

    form.reset();
    expect(form.values.tags).toEqual(["a"]);
  });
});
