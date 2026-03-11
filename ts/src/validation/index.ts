import { z } from "zod";

export const emailSchema = z
  .string()
  .trim()
  .email("Invalid email address")
  .max(254, "Email must be 254 characters or fewer");

export const passwordSchema = z
  .string()
  .min(12, "Password must be at least 12 characters")
  .max(128, "Password must be 128 characters or fewer");

export const slugSchema = z
  .string()
  .min(1, "Slug is required")
  .max(100, "Slug must be 100 characters or fewer")
  .regex(/^[a-z0-9-]+$/, "Use lowercase letters, numbers, and hyphens only");

export const displayNameSchema = z
  .string()
  .trim()
  .min(1, "Display name is required")
  .max(100, "Display name must be 100 characters or fewer");

export const registerRequestSchema = z.object({
  email: emailSchema,
  password: passwordSchema,
  displayName: displayNameSchema.optional(),
});

export const loginRequestSchema = z.object({
  email: emailSchema,
  password: z.string().min(1, "Password is required"),
});

export type RegisterRequestInput = z.input<typeof registerRequestSchema>;
export type RegisterRequest = z.output<typeof registerRequestSchema>;
export type LoginRequestInput = z.input<typeof loginRequestSchema>;
export type LoginRequest = z.output<typeof loginRequestSchema>;
