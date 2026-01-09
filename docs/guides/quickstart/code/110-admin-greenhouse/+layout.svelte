<script lang="ts">
  import "@decodelabs/underlay/styles/base.css";
  import "@decodelabs/underlay/styles/tokens.css";

  let { children } = $props();
</script>

<nav class="p-4 bg-green-100 border-b border-green-200">
  <a href="/admin" class="mr-4 font-bold">Admin Panel</a>
  <a href="/admin/users">Users</a>
  <a href="/admin/settings">Settings</a>
</nav>

<main class="p-4">
  {@render children()}
</main>
