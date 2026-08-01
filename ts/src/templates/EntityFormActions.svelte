<script lang="ts">
  import {
    Button as PoodleButton,
    FormActions as PoodleFormActions,
    SplitButton as PoodleSplitButton,
  } from "@poodle/svelte";
  import { navigateOnCancel } from "../client/navigation";

  type SaveIntent = "save" | "save-close";

  interface Props {
    /** create: single submit button; edit: save/save-close split button. */
    mode: "create" | "edit";
    /** Binds the chosen intent into the hidden `intent` field. */
    intent?: SaveIntent;
    /** Optional hidden `returnTo` field value. */
    returnTo?: string | null;
    /** Disable the submit control(s) (e.g. form invalid). */
    disabled?: boolean;
    createLabel?: string;
    editLabel?: string;
    editCloseLabel?: string;
    /** Cancel navigation target. Uses navigateOnCancel when no onCancel given. */
    cancelHref?: string;
    onCancel?: () => void;
    /** Passthrough for the poodle FormActions chrome. */
    align?: "start" | "end" | "between";
    showTopBorder?: boolean;
    dangerItems?: { label: string; onSelect: () => void; destructive?: boolean }[];
  }

  let {
    mode,
    intent = $bindable("save"),
    returnTo = null,
    disabled = false,
    createLabel = "Create",
    editLabel = "Save changes",
    editCloseLabel = "Save & close",
    cancelHref,
    onCancel,
    align = "end",
    showTopBorder = false,
    dangerItems = [],
  }: Props = $props();

  function handleCancel() {
    if (onCancel) {
      onCancel();
    } else {
      navigateOnCancel(cancelHref);
    }
  }
</script>

<PoodleFormActions {align} {showTopBorder} {dangerItems}>
  {#snippet danger()}
    <PoodleButton variant="ghost" type="button" onClick={handleCancel}>
      Cancel
    </PoodleButton>
  {/snippet}

  <input type="hidden" name="intent" value={intent} />
  {#if returnTo}
    <input type="hidden" name="returnTo" value={returnTo} />
  {/if}

  {#if mode === "create"}
    <PoodleButton type="submit" variant="primary" {disabled}>
      {createLabel}
    </PoodleButton>
  {:else}
    <PoodleSplitButton
      type="submit"
      variant="primary"
      {disabled}
      items={[
        { value: "save", label: editLabel },
        { value: "save-close", label: editCloseLabel },
      ]}
      onAction={(value) => {
        intent = value as SaveIntent;
      }}
    >
      {intent === "save" ? editLabel : editCloseLabel}
    </PoodleSplitButton>
  {/if}
</PoodleFormActions>
