<script lang="ts">
  type PrepareHook = ((formData: FormData) => void) | null;

  // Optional progressive-enhancement hook (e.g. SvelteKit's `$app/forms` enhance).
  // The component stays framework-agnostic by accepting this as a prop.
  type EnhanceHook =
    | ((
        node: HTMLFormElement,
        submit?: (options: { formData: FormData }) => void
      ) => { destroy?: () => void } | void)
    | null;

  export let method: "post" | "get" = "post";

  /**
   * Optional hook that can mutate `FormData` just before submit.
   */
  export let prepare: PrepareHook = null;

  /**
   * Optional enhancement function; if provided, it is invoked with a submit
   * callback that runs `prepare(formData)`.
   */
  export let enhance: EnhanceHook = null;

  function handleFormData(event: Event) {
    // When `enhance` is provided, it is responsible for calling `prepare`.
    // Avoid running twice if both enhancement and native `formdata` fire.
    if (enhance) return;

    if (!prepare) return;
    const formDataEvent = event as FormDataEvent;
    prepare(formDataEvent.formData);
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

      if (enhanced && typeof (enhanced as any).destroy === "function") {
        teardown = () => (enhanced as any).destroy();
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
  on:formdata={handleFormData}
  use:useEnhanced={{ enhance, prepare }}
  {...$$restProps}
>
  <slot />
</form>
