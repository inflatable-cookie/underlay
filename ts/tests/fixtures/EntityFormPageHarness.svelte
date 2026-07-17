<script lang="ts">
  import EntityFormPage from "../../src/templates/EntityFormPage.svelte";
  import type { SpaFormResult } from "../../src/patterns/spa-form-types";

  interface Props {
    onSubmit: (formData: FormData) => Promise<SpaFormResult>;
  }

  let { onSubmit }: Props = $props();
</script>

<EntityFormPage title="Edit Thing" {onSubmit}>
  {#snippet children()}
    <label>
      <span>Name</span>
      <input name="name" type="text" />
    </label>
  {/snippet}
</EntityFormPage>
