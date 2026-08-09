<script lang="ts">
  import { IconProvider } from "@inflatable-cookie/poodle-svelte";
  import type { IconSet } from "@inflatable-cookie/poodle-svelte";
  import EntityListCard from "../../src/templates/EntityListCard.svelte";

  const testIcons: IconSet = {
    "briefcase-business": [
      ["path", { d: "M12 12h.01" }],
      ["path", { d: "M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" }],
      ["path", { d: "M22 13a18.15 18.15 0 0 1-20 0" }],
      ["rect", { width: "20", height: "14", x: "2", y: "6", rx: "2" }]
    ],
    "list-todo": [
      ["path", { d: "M13 5h8" }],
      ["path", { d: "M13 12h8" }],
      ["path", { d: "M13 19h8" }],
      ["path", { d: "m3 17 2 2 4-4" }],
      ["rect", { x: "3", y: "4", width: "6", height: "6", rx: "1" }]
    ],
    "message-square": [
      [
        "path",
        {
          d: "M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"
        }
      ]
    ]
  };

  let selected = $state(false);
  let lastAction = $state("none");
</script>

<IconProvider icons={testIcons}>
<EntityListCard
  title="Project Apollo"
  subtitle="3 open tasks"
  footerText="Last updated today"
  leadingIcon="briefcase-business"
  selectionMode
  {selected}
  badges={[
    { label: "Active", tone: "success" },
    { label: "Development" }
  ]}
  counters={[
    { icon: "list-todo", count: 3, tooltip: "Tasks" },
    { icon: "message-square", count: 12, tooltip: "Comments" }
  ]}
  contextMenuItems={[
    { value: "archive", label: "Archive" },
    { value: "delete", label: "Delete", tone: "danger" }
  ]}
  contextMenuAriaLabel="Project actions"
  onSelectionChange={(next) => {
    selected = next;
  }}
  onContextAction={(value) => {
    lastAction = value;
  }}
/>

<EntityListCard
  title="Project Mercury"
  subtitle="Should collapse in reorder mode"
  leadingIcon="briefcase-business"
  reorderMode
  badges={[
    { label: "Archived", tone: "neutral" }
  ]}
  counters={[
    { icon: "list-todo", count: 9, tooltip: "Tasks" }
  ]}
  footerText="This should be hidden in reorder mode"
/>

<EntityListCard
  title="Project Artemis"
  subtitle="Context actions available"
  leadingIcon="briefcase-business"
  interactive
  contextMenuItems={[
    { value: "archive", label: "Archive" },
    { value: "delete", label: "Delete", tone: "danger" }
  ]}
  contextMenuAriaLabel="Project actions"
  onContextAction={(value) => {
    lastAction = value;
  }}
/>

<EntityListCard
  title="Project Gemini"
  subtitle="Should hide subtitle in selection mode"
  footerText="This should be hidden in selection mode"
  leadingIcon="briefcase-business"
  selectionMode
  selectionDisplay={{
    layout: "compact",
    showSubtitle: false,
    showFooter: false,
    showBadges: true
  }}
  badges={[
    { label: "Research" }
  ]}
/>

<output data-testid="selected-state">{selected ? "selected" : "idle"}</output>
<output data-testid="context-action">{lastAction}</output>
</IconProvider>
