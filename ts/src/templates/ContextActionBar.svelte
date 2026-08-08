<script lang="ts">
  import { Button, Drawer } from "@inflatable-cookie/poodle-svelte";
  import { default as ContextActionList } from "./ContextActionList.svelte";
  import type { ContextActionDefinition } from "./contextual-action.types";

  interface Props {
    open?: boolean;
    actions?: ContextActionDefinition[];
    title?: string;
    description?: string;
    stripLabel?: string;
    emptyMessage?: string;
    busy?: boolean;
    onOpenChange?: (open: boolean) => void;
    onActionSelect?: (action: ContextActionDefinition) => void;
  }

  let {
    open = $bindable(false),
    actions = [],
    title = "AI actions",
    description = "Actions available for this page.",
    stripLabel = "AI",
    emptyMessage = "No actions available here.",
    busy = false,
    onOpenChange = undefined,
    onActionSelect = undefined
  }: Props = $props();

  function setOpen(nextOpen: boolean): void {
    open = nextOpen;
    onOpenChange?.(nextOpen);
  }
</script>

<Button
  type="button"
  variant="primary"
  className="underlay-context-action-bar__strip"
  ariaLabel="Open AI actions"
  ariaExpanded={open}
  onClick={() => setOpen(!open)}
>
  {stripLabel}
</Button>

<Drawer
  open={open}
  edge="right"
  modal={false}
  {title}
  {description}
  dismissOnBackdrop={false}
  onOpenChange={setOpen}
  onRequestClose={() => setOpen(false)}
>
  <ContextActionList
    {actions}
    {busy}
    {emptyMessage}
    onActionSelect={(action) => {
      onActionSelect?.(action);
      setOpen(false);
    }}
  />
</Drawer>

<style>
  :global(.underlay-context-action-bar__strip) {
    position: fixed;
    top: 50%;
    right: 0;
    z-index: calc(var(--poodle-overlay-z-dialog) - 1);
    width: 2.5rem;
    min-width: 2.5rem;
    height: 4.25rem;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
    transform: translateY(-50%);
    writing-mode: vertical-rl;
    text-orientation: mixed;
  }
</style>
