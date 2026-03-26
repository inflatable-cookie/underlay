import type { StorybookConfig } from "@storybook/svelte-vite";

const config: StorybookConfig = {
  stories: ["../ts/stories/**/*.stories.@(js|jsx|mjs|ts|tsx|svelte)"],
  addons: [
    "@storybook/addon-docs",
    "@storybook/addon-a11y",
    "@storybook/addon-svelte-csf"
  ],
  framework: {
    name: "@storybook/svelte-vite",
    options: {}
  },
  docs: {
    autodocs: "tag"
  }
};

export default config;
