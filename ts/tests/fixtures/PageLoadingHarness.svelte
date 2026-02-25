<script lang="ts">
  import PageLoading from "../../src/components/PageLoading.svelte";

  interface Props {
    message?: string;
    size?: "sm" | "md" | "lg";
  }

  let {
    message = "Loading...",
    size = "md"
  }: Props = $props();
</script>

<PageLoading {message} {size} />
