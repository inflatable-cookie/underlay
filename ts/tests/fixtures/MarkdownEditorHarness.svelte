<script lang="ts">
  import MarkdownEditor from "../../src/components/MarkdownEditor.svelte";

  interface Props {
    label?: string | null;
    hint?: string | null;
    name?: string | null;
    initialValue?: string | null;
    required?: boolean;
    loading?: boolean;
    showPreview?: boolean;
    className?: string;
    placeholder?: string | null;
  }

  let {
    label = "Body",
    hint = "Supports markdown",
    name = "body",
    initialValue = "Initial text",
    required = false,
    loading = false,
    showPreview = false,
    className = "",
    placeholder = "Write here"
  }: Props = $props();

  let value = $state("");
  let initialized = $state(false);
  $effect(() => {
    if (!initialized) {
      value = initialValue;
      initialized = true;
    }
  });
</script>

<MarkdownEditor
  {label}
  {hint}
  {name}
  bind:value
  {required}
  {loading}
  {showPreview}
  {className}
  {placeholder}
/>

<p data-testid="markdown-value">{value}</p>
