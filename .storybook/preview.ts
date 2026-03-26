import type { Preview } from "@storybook/svelte-vite";

import "../ts/src/styles/tokens.css";
import "../ts/src/styles/base.css";
import "../ts/src/styles/forms.css";
import "../ts/src/styles/optimistic.css";

const preview: Preview = {
  parameters: {
    layout: "centered",
    controls: {
      expanded: true
    },
    options: {
      storySort: {
        order: ["Overview", "Auth", "Patterns", "Components"]
      }
    }
  }
};

export default preview;
