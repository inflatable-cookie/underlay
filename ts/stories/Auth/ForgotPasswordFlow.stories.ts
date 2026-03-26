import type { Meta, StoryObj } from "@storybook/svelte-vite";

import ForgotPasswordFlowDemo from "../support/ForgotPasswordFlowDemo.svelte";

const meta = {
  title: "Auth/ForgotPasswordFlow",
  component: ForgotPasswordFlowDemo,
  tags: ["autodocs"]
} satisfies Meta<ForgotPasswordFlowDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};
