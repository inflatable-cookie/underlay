<script lang="ts">
  import type { Snippet } from "svelte";

  import type { HTMLFormAttributes } from "svelte/elements";

  type PrepareHook = ((formData: FormData) => void) | null;

  // Optional progressive-enhancement hook (e.g. SvelteKit's `$app/forms` enhance).
  // The component stays framework-agnostic by accepting this as a prop.
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
    class?: string;
    autocomplete?: HTMLFormAttributes["autocomplete"];
    children?: Snippet;
    onformdata?: (event: FormDataEvent) => void;
    [key: string]: unknown;
  }

  let {
    method = "post",
    prepare = null,
    enhance = null,
    class: className = "",
    autocomplete,
    children,
    ...restProps
  }: Props = $props();

  function handleFormData(event: FormDataEvent) {
    // When `enhance` is provided, it is responsible for calling `prepare`.
    // Avoid running twice if both enhancement and native `formdata` fire.
    if (enhance) return;

    if (!prepare) return;
    prepare(event.formData);
  }

  const useEnhanced = (
    node: HTMLFormElement,
    params: { enhance: EnhanceHook; prepare: PrepareHook }
  ) => {
    let teardown: (() => void) | null = null;

    function apply(next: { enhance: EnhanceHook; prepare: PrepareHook }) {
      if (teardown) {
        teardown();
        teardown = null;
      }

      if (!next.enhance) {
        return;
      }

      const enhanced = next.enhance(node, ({ formData }) => {
        next.prepare?.(formData);
      });

      if (enhanced && typeof (enhanced as { destroy?: () => void }).destroy === "function") {
        teardown = () => (enhanced as { destroy: () => void }).destroy();
      }
    }

    apply(params);

    return {
      update(next: { enhance: EnhanceHook; prepare: PrepareHook }) {
        apply(next);
      },
      destroy() {
        if (teardown) {
          teardown();
          teardown = null;
        }
      }
    };
  };
</script>

<form
  {method}
  onformdata={handleFormData}
  use:useEnhanced={{ enhance, prepare }}
  class={className}
  {autocomplete}
  {...restProps}
>
  {@render children?.()}
</form>
