<script lang="ts">
  import { Button, Field, TextInput } from "@poodle/svelte";
  import SpaFormShell from "../../src/patterns/SpaFormShell.svelte";

  let titleValue = $state("Release checklist");
  let mode = $state<"success" | "field-error" | "global-error">("success");

  async function onSubmit(formData: FormData) {
    const title = String(formData.get("title") ?? "").trim();

    if (mode === "field-error" || !title) {
      return {
        success: false,
        error: null,
        fieldErrors: { title: "Title is required" }
      };
    }

    if (mode === "global-error") {
      return {
        success: false,
        error: "The release window is locked for deployment freeze.",
        fieldErrors: null
      };
    }

    return {
      success: true,
      error: null,
      fieldErrors: null
    };
  }
</script>

<div class="spa-form-shell-demo__controls">
  <Button variant={mode === "success" ? "primary" : "ghost"} onclick={() => (mode = "success")}>Success</Button>
  <Button variant={mode === "field-error" ? "primary" : "ghost"} onclick={() => (mode = "field-error")}>Field error</Button>
  <Button variant={mode === "global-error" ? "primary" : "ghost"} onclick={() => (mode = "global-error")}>Global error</Button>
</div>

<SpaFormShell
  title="Release notes"
  subtitle="Retained SPA form orchestration with caller-owned submit logic."
  backHref="/releases"
  backLabel="Back to releases"
  successMessage="Changes saved successfully."
  {onSubmit}
>
  {#snippet children()}
    <Field id="storybook-release-title" label="Title" required>
      <TextInput
        id="storybook-release-title"
        name="title"
        value={titleValue}
        oninput={(value) => {
          titleValue = value;
        }}
      />
    </Field>

    <div class="spa-form-shell-demo__actions">
      <Button type="submit" variant="primary">Save changes</Button>
    </div>
  {/snippet}
</SpaFormShell>

<style>
  .spa-form-shell-demo__controls {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    margin-bottom: 1rem;
  }

  .spa-form-shell-demo__actions {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
  }
</style>
