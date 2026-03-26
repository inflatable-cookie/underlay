import type { Meta, StoryObj } from "@storybook/svelte-vite";

import AuthLayoutDemo from "../support/AuthLayoutDemo.svelte";

const meta = {
  title: "Auth/AuthLayout",
  component: AuthLayoutDemo,
  tags: ["autodocs"]
} satisfies Meta<AuthLayoutDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
