import type { Meta, StoryObj } from "@storybook/svelte-vite";

import SpaFormShellDemo from "../support/SpaFormShellDemo.svelte";

const meta = {
  title: "Patterns/SpaFormShell",
  component: SpaFormShellDemo,
  tags: ["autodocs"]
} satisfies Meta<SpaFormShellDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Interactive: Story = {};
