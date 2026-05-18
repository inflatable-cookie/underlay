# ErrorLogListPage

Status: active

`ErrorLogListPage` is the retained page shell for repeated admin error-log
surfaces.

It owns the repeated error-log posture:

- URL-friendly query seam via `query` / `onQueryChange`
- built-in query variants for `all`, `5xx`, `4xx`, `500`, and `404`
- compact request/error table layout
- optional stats cards
- expandable detail rows with message and context

Use it when an app is rendering the standard platform error-log surface and the
main variation is just which client command family supplies list, detail, and
stats data.

## Usage

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { buildQueryString, parseQueryParams } from "@decodelabs/underlay/client/query";
  import { ErrorLogListPage, toPagedListResult } from "@decodelabs/underlay/templates";
  import { adminCommands } from "@api-client";

  const currentQuery = $derived(parseQueryParams($page.url.searchParams));

  function updateUrl(nextQuery) {
    const url = new URL($page.url);
    url.search = buildQueryString(nextQuery);
    goto(url.toString(), { replaceState: true, keepFocus: true });
  }

  async function loadList(fetchFn, token, request) {
    const response = await adminCommands.listErrorLogs(fetchFn, token, {
      status_class: request.statusClass,
      status_code: request.statusCode,
      limit: request.limit,
      offset: request.offset
    });
    return toPagedListResult(response);
  }

  async function loadDetail(id, fetchFn, token) {
    return await adminCommands.getErrorLog(id, fetchFn, token);
  }

  async function loadStats(fetchFn, token) {
    return await adminCommands.getErrorLogStats(fetchFn, token);
  }
</script>

<ErrorLogListPage
  query={currentQuery}
  onQueryChange={updateUrl}
  {loadList}
  {loadDetail}
  {loadStats}
/>
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `loadList` | `(fetch, token, request) => Promise<PagedListResult<ErrorLogListItem>>` | Yes | Loads the filtered paged list |
| `loadDetail` | `(id, fetch, token) => Promise<ErrorLogDetailItem>` | Yes | Loads one expanded row detail record |
| `loadStats` | `(fetch, token) => Promise<ErrorLogStatsSummary \| null>` | No | Loads the optional summary cards |
| `query` | `QueryParams` | No | External query state for URL sync |
| `onQueryChange` | `(query) => void` | No | Query change callback for URL sync |
| `title` | `string` | No | Page title, defaults to `Error Log` |
| `backHref` | `string` | No | Back-link target, defaults to `/system` |
| `backLabel` | `string` | No | Back-link label, defaults to `Back to system` |

## Query Variants

`ErrorLogListPage` treats status classes as durable baseline views, not
ephemeral filters:

- `all` shows all captured errors and is the default
- `5xx` passes `request.statusClass = "5xx"` to the loader
- `4xx` passes `request.statusClass = "4xx"` to the loader
- `500` and `404` pass `request.statusCode`

App API clients should serialize `statusClass` as `status_class=4xx|5xx`.
Exact status-code variants still serialize as `status_code=500` or
`status_code=404`.

## App wrapper policy

Keep `ErrorLogListPage` inside an app-local wrapper such as
`src/lib/lists/ErrorLogList.svelte`, then thin-mount that wrapper in the route.

The wrapper should own:

- route URL sync
- app-specific command wiring
- any honest local deviations that still belong to the error-log workflow

Do not copy the compact table, query variants, stats card, or expanded-detail
composition into each app again.
