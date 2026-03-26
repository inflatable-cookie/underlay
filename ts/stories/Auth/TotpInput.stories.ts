import type { Meta, StoryObj } from "@storybook/svelte-vite";

import TotpInput from "../../src/components/auth/TotpInput.svelte";

const meta = {
  title: "Auth/TotpInput",
  component: TotpInput,
  tags: ["autodocs"],
  args: {
    label: "Authenticator code",
    hint: "Enter the 6-digit code from your authenticator app",
    name: "code",
    length: 6,
    value: ""
  }
} satisfies Meta<TotpInput>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const WithValue: Story = {
  args: {
    value: "123456"
  }
};

export const Invalid: Story = {
  args: {
    value: "123",
    error: "Enter a valid 6-digit code"
  }
};

export const Disabled: Story = {
  args: {
    disabled: true,
    value: "987654"
  }
};
