import type { Meta, StoryObj } from "@storybook/svelte-vite";

import LoginPageDemo from "../support/LoginPageDemo.svelte";

const meta = {
  title: "Auth/LoginPage",
  component: LoginPageDemo,
  tags: ["autodocs"]
} satisfies Meta<LoginPageDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const MultiMethodFlow: Story = {};
