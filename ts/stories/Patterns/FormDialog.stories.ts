import type { Meta, StoryObj } from "@storybook/svelte-vite";

import FormDialogDemo from "../support/FormDialogDemo.svelte";

const meta = {
  title: "Patterns/FormDialog",
  component: FormDialogDemo,
  tags: ["autodocs"]
} satisfies Meta<FormDialogDemo>;

export default meta;

type Story = StoryObj<typeof meta>;

export const InteractiveShell: Story = {};
