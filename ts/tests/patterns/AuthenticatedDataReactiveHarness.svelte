<script lang="ts">
  import { useAuthenticatedData } from "../../src/patterns/authenticated-data.svelte";

  interface Props {
    fetcher: (fetchFn: typeof fetch, token: string) => Promise<string>;
  }

  let { fetcher }: Props = $props();

  const pageData = useAuthenticatedData<string>(fetcher, {
    defaultValue: "loading"
  });
</script>

<div data-testid="result">{pageData.data}</div>
