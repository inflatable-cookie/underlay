<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
  import { FormDialog } from "../../src/patterns";

  let open = $state(false);
  let mode = $state<"default" | "error" | "success">("default");
</script>

<div class="form-dialog-demo__controls">
  <Button variant="primary" onclick={() => {
    mode = "default";
    open = true;
  }}>Open dialog</Button>
  <Button variant="secondary" onclick={() => {
    mode = "error";
    open = true;
  }}>Open with error</Button>
  <Button variant="ghost" onclick={() => {
    mode = "success";
    open = true;
  }}>Open with success</Button>
</div>

<FormDialog
  bind:open
  title="Publish release notes"
  subtitle="This shell remains in Underlay because it owns flexible modal-form workflow composition."
  error={mode === "error" ? "A publish target is required before continuing." : null}
  success={mode === "success" ? "Draft saved successfully." : null}
  onCancel={() => {
    open = false;
  }}
>
  {#snippet children(submitting)}
    <div class="form-dialog-demo__body">
      <p>Use retained dialog orchestration here, but keep low-level dialog chrome in Poodle.</p>
      <div class="form-dialog-demo__actions">
        <Button variant="ghost" onclick={() => (open = false)} disabled={submitting}>Cancel</Button>
        <Button variant="primary" disabled={submitting}>Save changes</Button>
      </div>
    </div>
  {/snippet}
</FormDialog>

<style>
  .form-dialog-demo__controls {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .form-dialog-demo__body p {
    margin-top: 0;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .form-dialog-demo__actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1rem;
  }
</style>
