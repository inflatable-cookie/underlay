<script lang="ts">
  export let explode = false;

  $: if (explode) {
    throw new Error("The demo child crashed while rendering");
  }
</script>
