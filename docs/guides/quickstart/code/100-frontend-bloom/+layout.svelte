<script lang="ts">
  import "@decodelabs/underlay/styles/base.css";
  import "@decodelabs/underlay/styles/tokens.css";

  let { children } = $props();
</script>

<nav class="p-4 bg-gray-100">
  <a href="/" class="mr-4">Home</a>
  <a href="/dashboard">Dashboard</a>
</nav>

<main class="p-4">
  {@render children()}
</main>
