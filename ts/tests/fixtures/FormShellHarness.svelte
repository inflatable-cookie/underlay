<script lang="ts">
  import FormShell from "../../src/patterns/FormShell.svelte";
</script>

{#snippet formFields()}
  <label>
    <span>Name</span>
    <input name="name" type="text" />
  </label>
{/snippet}

<FormShell
  title="Edit project"
  backHref="/projects"
  backLabel="Back to projects"
>
  {#snippet children()}
    {@render formFields()}
  {/snippet}
</FormShell>
