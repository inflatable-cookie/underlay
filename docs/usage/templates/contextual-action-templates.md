# Contextual Action Templates

Status: active shared surface.

These components provide a small reusable shell for route-aware AI actions. They
do not own agent execution, routing, prompts, persistence, or app-specific
schema handling.

## Components

- `ContextActionBar` renders the fixed right-edge strip and a simple right
  drawer.
- `ContextActionList` renders matched actions as name/description rows.
- `ContextActionDialog` renders the large execution dialog, optional model
  picker, schema fields, custom form surface, error text, and submit/cancel
  actions.
- `createContextActionController` provides local Svelte state for the bar,
  selected action, values, model alias, run state, and error message.

## Contract

Actions are app-owned `ContextActionDefinition` objects:

```ts
const generateQuestionAction = {
  id: "quiz-question-prefill",
  name: "Generate question",
  description: "Draft a quiz question from course notes.",
  resultMode: "client_prefill",
  defaultModelAlias: "fast-balanced",
  modelOptions: [
    { alias: "fast-balanced", label: "Fast balanced" },
    { alias: "deep-reasoning", label: "Deep reasoning" }
  ],
  fields: [
    {
      id: "source_note_id",
      label: "Source notes",
      type: "select",
      required: true,
      options: noteOptions
    },
    {
      id: "difficulty",
      label: "Difficulty",
      type: "select",
      defaultValue: "standard",
      options: [
        { value: "intro", label: "Intro" },
        { value: "standard", label: "Standard" },
        { value: "hard", label: "Hard" }
      ]
    }
  ]
} satisfies ContextActionDefinition;
```

`resultMode` tells the consuming app how to handle the response:

- `client_prefill` forwards structured agent JSON back to the current UI.
- `backend_mutation` lets the app backend create/update records first.
- `suggestion_review` keeps the result reviewable before applying it.

## Usage

```svelte
<script lang="ts">
  import {
    ContextActionBar,
    ContextActionDialog,
    type ContextActionSubmitDetail
  } from "@decodelabs/underlay/templates";
  import { createContextActionController } from "@decodelabs/underlay/patterns";

  const contextActions = createContextActionController({ actions });

  async function runAction(detail: ContextActionSubmitDetail) {
    contextActions.setRunState("running");
    contextActions.setError(null);

    try {
      await executeContextAction(detail);
      contextActions.setRunState("succeeded");
      contextActions.closeAction();
    } catch (error) {
      contextActions.setError(error instanceof Error ? error.message : "Action failed");
    }
  }
</script>

<ContextActionBar
  open={contextActions.open}
  actions={contextActions.actions}
  onOpenChange={contextActions.setOpen}
  onActionSelect={contextActions.selectAction}
/>

<ContextActionDialog
  open={Boolean(contextActions.selectedAction)}
  action={contextActions.selectedAction}
  values={contextActions.values}
  selectedModelAlias={contextActions.selectedModelAlias}
  runState={contextActions.runState}
  errorMessage={contextActions.errorMessage}
  onCancel={contextActions.closeAction}
  onValueChange={contextActions.setValue}
  onSelectedModelChange={contextActions.setSelectedModelAlias}
  onSubmit={runAction}
/>
```

## Rules

- Keep action matching in the app route layer.
- Keep prompts and JSON schemas server-side or in app-owned contracts.
- Pass model aliases, not raw provider credentials.
- Use `fields` for simple forms and `form` only when an action needs custom
  layout.
- Do not add conversation state to this shell. That is a separate workflow.
