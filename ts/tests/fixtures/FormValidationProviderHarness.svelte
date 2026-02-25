<script lang="ts">
  import FormValidationProvider from "../../src/components/FormValidationProvider.svelte";
  import FormValidationProbe from "./FormValidationProbe.svelte";

  let isValid = $state(true);
</script>

<FormValidationProvider bind:isValid>
  {#snippet children()}
    <FormValidationProbe />
  {/snippet}
</FormValidationProvider>

<div data-testid="provider-validity">{isValid ? "true" : "false"}</div>
