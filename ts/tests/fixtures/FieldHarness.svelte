<script lang="ts">
  import Field from "../../src/components/Field.svelte";

  interface Props {
    label?: string;
    forId?: string;
    hint?: string;
    error?: string;
    required?: boolean;
    span?: "full" | 1 | 2 | 3 | 4 | 5 | 6;
    wide?: boolean;
  }

  let {
    label = "Name",
    forId = undefined,
    hint = undefined,
    error = undefined,
    required = false,
    span = undefined,
    wide = false
  }: Props = $props();
</script>

<Field {label} {forId} {hint} {error} {required} {span} {wide}>
  {#snippet children()}
    <input id={forId} data-testid="field-input" />
  {/snippet}
</Field>
