import { describe, expect, it } from "vitest";
import {
  emailSchema,
  loginRequestSchema,
  passwordSchema,
  registerRequestSchema,
  slugSchema,
} from "../../src/validation";

describe("validation zod exports", () => {
  it("accepts the canonical primitive schemas", () => {
    expect(emailSchema.parse("person@example.com")).toBe("person@example.com");
    expect(passwordSchema.parse("password-1234")).toBe("password-1234");
    expect(slugSchema.parse("hello-world")).toBe("hello-world");
  });

  it("validates the shared login and register request shapes", () => {
    expect(
      loginRequestSchema.parse({
        email: "person@example.com",
        password: "secret",
      }),
    ).toEqual({
      email: "person@example.com",
      password: "secret",
    });

    expect(
      registerRequestSchema.parse({
        email: " person@example.com ",
        password: "password-1234",
        displayName: " Person ",
      }),
    ).toEqual({
      email: "person@example.com",
      password: "password-1234",
      displayName: "Person",
    });
  });
});
