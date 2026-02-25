<script lang="ts">
  import Form from "../../src/components/Form.svelte";

  type PrepareHook = ((formData: FormData) => void) | null;
  type EnhanceHook =
    | ((
        node: HTMLFormElement,
        submit?: (options: { formData: FormData }) => void
      ) => { destroy?: () => void } | void)
    | null;

  interface Props {
    method?: "post" | "get";
    prepare?: PrepareHook;
    enhance?: EnhanceHook;
    className?: string;
    autocomplete?: "on" | "off";
  }

  let {
    method = "post",
    prepare = null,
    enhance = null,
    className = "",
    autocomplete = "on"
  }: Props = $props();
</script>

<Form {method} {prepare} {enhance} class={className} {autocomplete} data-testid="form-root">
  {#snippet children()}
    <input name="name" value="Alice" data-testid="form-input" />
  {/snippet}
</Form>
