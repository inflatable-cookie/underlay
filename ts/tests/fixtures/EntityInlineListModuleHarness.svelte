<script lang="ts">
  import { EntityInlineListModule } from "../../src/templates";
  import type {
    InlineListDialogContext,
    InlineListItemActionConfig,
    InlineListItemDeleteConfig
  } from "../../src/templates";

  type Level = {
    id: string;
    title: string;
  };

  let levels = $state<Level[]>([
    { id: "level-1", title: "Knowledge" },
    { id: "level-2", title: "Comprehension" },
    { id: "level-3", title: "Application" },
    { id: "level-4", title: "Analysis" },
    { id: "level-5", title: "Evaluation" },
    { id: "level-6", title: "Synthesis" }
  ]);
  let lastAction = $state("none");

  async function loadLevels(_fetch: typeof fetch, _token: string, query: { page?: number; limit?: number }) {
    const page = Math.max(1, query.page ?? 1);
    const limit = Math.max(1, query.limit ?? 5);
    const start = (page - 1) * limit;
    const end = start + limit;

    return {
      data: levels.slice(start, end),
      total: levels.length
    };
  }

  function buildItemActions(level: Level): InlineListItemActionConfig<Level>[] {
    return [
      {
        label: `Rename ${level.title}`,
        handler: async () => {
          lastAction = `rename:${level.id}`;
        }
      }
    ];
  }

  const deleteConfig: InlineListItemDeleteConfig<Level> = {
    title: "Delete level",
    description: "This action removes the level from the pathway.",
    confirmLabel: "Delete level",
    entityLabel: (level) => level.title,
    handler: async (level) => {
      levels = levels.filter((entry) => entry.id !== level.id);
      lastAction = `delete:${level.id}`;
    }
  };
</script>

{#snippet levelRow(level: Level)}
  <div class="level-row">
    <span>{level.title}</span>
  </div>
{/snippet}

{#snippet addLevelDialog(context: InlineListDialogContext)}
  <div class="dialog-body">
    <p>Create a new level.</p>
    <button
      type="button"
      onclick={async () => {
        levels = [
          ...levels,
          { id: `level-${levels.length + 1}`, title: "Reflection" }
        ];
        await context.refetch();
      }}
    >
      Create level
    </button>
    <button type="button" onclick={context.close}>Cancel</button>
  </div>
{/snippet}

<EntityInlineListModule
  title="Levels"
  dataLoader={loadLevels}
  pageSize={5}
  addLabel="Add level"
  addDialog={{
    title: "Add level",
    content: addLevelDialog
  }}
  item={levelRow}
  itemActions={buildItemActions}
  itemDelete={deleteConfig}
  emptyMessage="No levels yet."
/>

<p data-testid="inline-list-last-action">{lastAction}</p>
