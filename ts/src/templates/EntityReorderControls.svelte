<script lang="ts">
  import { IconButton } from "@poodle/svelte";
  import type { SemanticControlSizeRole } from "@poodle/svelte";

  interface Props {
    active: boolean;
    available: boolean;
    dirty?: boolean;
    saving?: boolean;
    disabled?: boolean;
    sizeRole?: SemanticControlSizeRole;
    onEnter: () => void | Promise<void>;
    onSave?: () => void | Promise<void>;
    onCancel?: () => void;
  }

  let {
    active,
    available,
    dirty = false,
    saving = false,
    disabled = false,
    sizeRole = "control",
    onEnter,
    onSave,
    onCancel
  }: Props = $props();
</script>

{#if active}
  <IconButton
    type="button"
    variant="secondary"
    tone="danger"
    sizeRole={sizeRole}
    icon="arrow-up-down"
    ariaLabel="Cancel reorder"
    tooltip="Cancel Reorder"
    disabled={disabled || saving}
    onClick={() => onCancel?.()}
  />
  <IconButton
    type="button"
    variant="primary"
    tone="success"
    sizeRole={sizeRole}
    icon="check"
    ariaLabel="Save order"
    tooltip="Save Order"
    disabled={disabled || !dirty || saving}
    loading={saving}
    onClick={() => onSave?.()}
  />
{:else if available}
  <IconButton
    type="button"
    variant="secondary"
    sizeRole={sizeRole}
    icon="arrow-up-down"
    ariaLabel="Reorder items"
    tooltip="Reorder Items"
    disabled={disabled}
    onClick={onEnter}
  />
{/if}
