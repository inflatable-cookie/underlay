import type { Meta, StoryObj } from "@storybook/svelte-vite";

import PasswordRequirementsDemo from "../support/PasswordRequirementsDemo.svelte";

const meta = {
  title: "Auth/PasswordRequirements",
  component: PasswordRequirementsDemo,
  tags: ["autodocs"]
} satisfies Meta<PasswordRequirementsDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Interactive: Story = {};
