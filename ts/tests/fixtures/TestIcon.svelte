<script lang="ts">
  interface Props {
    size?: number;
  }

  let { size = 16 }: Props = $props();
</script>

<svg data-testid="test-icon" width={size} height={size} aria-hidden="true">
  <circle cx="8" cy="8" r="6"></circle>
</svg>
